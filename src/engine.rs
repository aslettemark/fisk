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
//TODO

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
    //TODO
}

struct Piece {
    pub kind: u8,
    pub position: u64,
}

impl Board {
    // Create default chess board
    pub fn new() -> Board {
        let ps = vec![Piece { kind: WHITE_PAWN, position: ROW_2 & FILE_A }];
        Board {
            halfturn: 0,
            en_passant: 0,
            bitboard: BitBoard {
                white_pawns: ROW_2,
                white_queen: ROW_1 & FILE_D,
                white_king: ROW_1 & FILE_E,
            },
            pieces: ps, //TODO
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



        for p in self.pieces.iter() {
            println!("{:#064b}: {}", p.position, self.piece_representation(p.kind));
        }
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
            //TODO
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
        //TODO

        return EMPTY_SQUARE;
    }
}
