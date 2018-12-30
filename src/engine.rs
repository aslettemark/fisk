const ROW_1: u64 = 0xFF;
const ROW_2: u64 = ROW_1 << 8;
const ROW_3: u64 = ROW_1 << (8 * 2);
const ROW_4: u64 = ROW_1 << (8 * 3);
const ROW_5: u64 = ROW_1 << (8 * 4);
const ROW_6: u64 = ROW_1 << (8 * 5);
const ROW_7: u64 = ROW_1 << (8 * 6);
const ROW_8: u64 = ROW_1 << (8 * 7);

const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = FILE_A << 1;
const FILE_C: u64 = FILE_A << 2;
const FILE_D: u64 = FILE_A << 3;
const FILE_E: u64 = FILE_A << 4;
const FILE_F: u64 = FILE_A << 5;
const FILE_G: u64 = FILE_A << 6;
const FILE_H: u64 = FILE_A << 7;

const EMPTY_SQUARE: u8 = 0;

const WHITE_PAWN: u8 = 1;
const WHITE_BISHOP: u8 = 1 << 1;
const WHITE_KNIGHT: u8 = 1 << 2;
const WHITE_ROOK: u8 = 1 << 3;
const WHITE_QUEEN: u8 = 1 << 4;
const WHITE_KING: u8 = 1 << 5;

const BLACK_PAWN: u8 = WHITE_PAWN | (1 << 7);
const BLACK_BISHOP: u8 = WHITE_BISHOP | (1 << 7);
const BLACK_KNIGHT: u8 = WHITE_KNIGHT | (1 << 7);
const BLACK_ROOK: u8 = WHITE_ROOK | (1 << 7);
const BLACK_QUEEN: u8 = WHITE_QUEEN | (1 << 7);
const BLACK_KING: u8 = WHITE_KING | (1 << 7);

pub struct Board {
    pub halfturn: u16,
    pub en_passant: u64,
    bitboard: BitBoard,
    pieces: Vec<Piece>,
    pub castling: u8,
}

struct BitBoard {
    // little-endian https://www.chessprogramming.org/Square_Mapping_Considerations
    // bit 0 is a1, bit 7 is f1, bit 63 is h8

    pub white_pawns: u64,
    pub white_queen: u64,
    pub white_king: u64,
    pub white_rooks: u64,
    pub white_bishops: u64,
    pub white_knights: u64,

    pub black_pawns: u64,
    pub black_queen: u64,
    pub black_king: u64,
    pub black_rooks: u64,
    pub black_bishops: u64,
    pub black_knights: u64,
}

struct Piece {
    pub kind: u8,
    pub position: u64,
}

impl Piece {
    pub fn new(kind: u8, position: u64) -> Piece {
        Piece {
            kind: kind,
            position: position,
        }
    }
}

impl Board {
    // Create default chess board
    pub fn new() -> Board {
        let mut ps: Vec<Piece> = Vec::with_capacity(32);

        for i in 0..8 {
            ps.push(Piece::new(WHITE_PAWN, ROW_2 & (FILE_A << i)));
            ps.push(Piece::new(BLACK_PAWN, ROW_7 & (FILE_A << i)));
        }

        ps.push(Piece::new(WHITE_KING, ROW_1 & FILE_E));
        ps.push(Piece::new(WHITE_QUEEN, ROW_1 & FILE_D));
        ps.push(Piece::new(WHITE_BISHOP, ROW_1 & FILE_C));
        ps.push(Piece::new(WHITE_BISHOP, ROW_1 & FILE_F));
        ps.push(Piece::new(WHITE_KNIGHT, ROW_1 & FILE_B));
        ps.push(Piece::new(WHITE_KNIGHT, ROW_1 & FILE_G));
        ps.push(Piece::new(WHITE_ROOK, ROW_1 & FILE_A));
        ps.push(Piece::new(WHITE_ROOK, ROW_1 & FILE_H));

        ps.push(Piece::new(BLACK_KING, ROW_8 & FILE_E));
        ps.push(Piece::new(BLACK_QUEEN, ROW_8 & FILE_D));
        ps.push(Piece::new(BLACK_BISHOP, ROW_8 & FILE_C));
        ps.push(Piece::new(BLACK_BISHOP, ROW_8 & FILE_F));
        ps.push(Piece::new(BLACK_KNIGHT, ROW_8 & FILE_B));
        ps.push(Piece::new(BLACK_KNIGHT, ROW_8 & FILE_G));
        ps.push(Piece::new(BLACK_ROOK, ROW_8 & FILE_A));
        ps.push(Piece::new(BLACK_ROOK, ROW_8 & FILE_H));

        Board {
            halfturn: 0,
            en_passant: 0,
            bitboard: BitBoard {
                white_pawns: ROW_2,
                white_queen: ROW_1 & FILE_D,
                white_king: ROW_1 & FILE_E,
                white_rooks: ROW_1 & (FILE_A | FILE_H),
                white_bishops: ROW_1 & (FILE_C | FILE_F),
                white_knights: ROW_1 & (FILE_B | FILE_G),
                black_pawns: ROW_7,
                black_queen: ROW_8 & FILE_D,
                black_king: ROW_8 & FILE_E,
                black_rooks: ROW_8 & (FILE_A | FILE_H),
                black_bishops: ROW_8 & (FILE_C | FILE_F),
                black_knights: ROW_8 & (FILE_B | FILE_G),
            },
            pieces: ps,
            castling: 0b11101110,
        }
    }

    pub fn print(&self) {
        println!(" A B C D E F G H");

        for i in 0..8 {
            let row = ROW_1 << (8 * (7 - i));
            print!("{}", 7 - i + 1);

            for j in 0..8 {
                let file = FILE_A << j;
                let pos = row & file;
                let c = self.piece_representation(self.kind_at(pos));

                print!("{} ", c);
            }

            println!();
        }

        println!(" A B C D E F G H");

        /*for p in self.pieces.iter() {
            println!("{:#066b}: {}", p.position, self.piece_representation(p.kind));
        }*/
    }

    fn piece_representation(&self, kind: u8) -> char {
        match kind {
            WHITE_PAWN => '♙',
            WHITE_BISHOP => '♗',
            WHITE_KNIGHT => '♘',
            WHITE_ROOK => '♖',
            WHITE_QUEEN => '♕',
            WHITE_KING => '♔',
            BLACK_PAWN => '♟',
            BLACK_BISHOP => '♝',
            BLACK_KNIGHT => '♞',
            BLACK_ROOK => '♜',
            BLACK_QUEEN => '♛',
            BLACK_KING => '♚',
            _ => ' '
        }
    }

    fn kind_at(&self, pos: u64) -> u8 {
        let b = &self.bitboard;
        if pos & b.white_pawns != 0 {
            return WHITE_PAWN;
        }
        if pos & b.white_queen != 0 {
            return WHITE_QUEEN;
        }
        if pos & b.white_king != 0 {
            return WHITE_KING;
        }
        if pos & b.white_bishops != 0 {
            return WHITE_BISHOP;
        }
        if pos & b.white_knights != 0 {
            return WHITE_KNIGHT;
        }
        if pos & b.white_rooks != 0 {
            return WHITE_ROOK;
        }

        if pos & b.black_pawns != 0 {
            return BLACK_PAWN;
        }
        if pos & b.black_queen != 0 {
            return BLACK_QUEEN;
        }
        if pos & b.black_king != 0 {
            return BLACK_KING;
        }
        if pos & b.black_bishops != 0 {
            return BLACK_BISHOP;
        }
        if pos & b.black_knights != 0 {
            return BLACK_KNIGHT;
        }
        if pos & b.black_rooks != 0 {
            return BLACK_ROOK;
        }

        return EMPTY_SQUARE;
    }
}
