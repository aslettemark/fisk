use crate::constants::*;
use crate::move_generation::*;


#[derive(Copy, Clone)]
pub struct Board {
    pub halfturn: u16,
    pub en_passant: u64,
    pub bitboard: BitBoard,
    pub pieces: [Piece; 32],
    pub castling: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard {
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    pub kind: u8,
    pub position: u64,
}

impl BitBoard {
    pub const fn empty() -> BitBoard {
        BitBoard {
            white_pawns: 0,
            white_queen: 0,
            white_king: 0,
            white_rooks: 0,
            white_bishops: 0,
            white_knights: 0,
            black_pawns: 0,
            black_queen: 0,
            black_king: 0,
            black_rooks: 0,
            black_bishops: 0,
            black_knights: 0,
        }
    }

    pub fn white_coverage(&self) -> u64 {
        self.white_king | self.white_queen | self.white_knights | self.white_rooks | self.white_bishops | self.white_pawns
    }

    pub fn black_coverage(&self) -> u64 {
        self.black_king | self.black_queen | self.black_knights | self.black_rooks | self.black_bishops | self.black_pawns
    }
}

impl Piece {
    pub fn new(kind: u8, position: u64) -> Piece {
        Piece {
            kind,
            position,
        }
    }
}

impl Board {
    // Create default chess board
    pub fn new() -> Board {
        let mut ps = [Piece { kind: EMPTY_SQUARE, position: 0 }; 32];

        for i in 0usize..8 {
            let iu = i as u64;
            ps[i] = Piece::new(WHITE_PAWN, ROW_2 & (FILE_A << iu));
            ps[i + 8] = Piece::new(BLACK_PAWN, ROW_7 & (FILE_A << iu));
        }

        ps[16] = Piece::new(WHITE_KING, ROW_1 & FILE_E);
        ps[17] = Piece::new(WHITE_QUEEN, ROW_1 & FILE_D);
        ps[18] = Piece::new(WHITE_BISHOP, ROW_1 & FILE_C);
        ps[19] = Piece::new(WHITE_BISHOP, ROW_1 & FILE_F);
        ps[20] = Piece::new(WHITE_KNIGHT, ROW_1 & FILE_B);
        ps[21] = Piece::new(WHITE_KNIGHT, ROW_1 & FILE_G);
        ps[22] = Piece::new(WHITE_ROOK, ROW_1 & FILE_A);
        ps[23] = Piece::new(WHITE_ROOK, ROW_1 & FILE_H);

        ps[24] = Piece::new(BLACK_KING, ROW_8 & FILE_E);
        ps[25] = Piece::new(BLACK_QUEEN, ROW_8 & FILE_D);
        ps[26] = Piece::new(BLACK_BISHOP, ROW_8 & FILE_C);
        ps[27] = Piece::new(BLACK_BISHOP, ROW_8 & FILE_F);
        ps[28] = Piece::new(BLACK_KNIGHT, ROW_8 & FILE_B);
        ps[29] = Piece::new(BLACK_KNIGHT, ROW_8 & FILE_G);
        ps[30] = Piece::new(BLACK_ROOK, ROW_8 & FILE_A);
        ps[31] = Piece::new(BLACK_ROOK, ROW_8 & FILE_H);

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


    pub fn generate_successors(&self) -> Vec<Board> {
        let white = self.halfturn % 2 == 0;
        let mut states = Vec::new();

        for (i, piece) in self.pieces.iter().enumerate() {
            // TODO Keep pieces ordered with empty square pieces at the end to abort entire
            // iteration when an empty square is found.
            if piece.kind == EMPTY_SQUARE {
                continue;
            }

            let white_piece = piece.kind & (1 << 7) == 0;
            if (white_piece && !white) || (!white_piece && white) {
                continue;
            }

            let position = piece.position;

            match piece.kind {
                WHITE_PAWN => white_pawn_moves(&self, position, i, &mut states),
                WHITE_ROOK | BLACK_ROOK => rook_moves(&self, position, i, white, &mut states),

                //TODO remaining kinds
                _ => {}
            }
        }

        return states;
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

    pub fn kind_at(&self, pos: u64) -> u8 {
        let b = &self.bitboard;
        if pos & b.white_pawns != 0 {
            return WHITE_PAWN;
        }
        if pos & b.black_pawns != 0 {
            return BLACK_PAWN;
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
        if pos & b.black_bishops != 0 {
            return BLACK_BISHOP;
        }
        if pos & b.black_knights != 0 {
            return BLACK_KNIGHT;
        }
        if pos & b.black_rooks != 0 {
            return BLACK_ROOK;
        }


        if pos & b.white_queen != 0 {
            return WHITE_QUEEN;
        }
        if pos & b.white_king != 0 {
            return WHITE_KING;
        }

        if pos & b.black_queen != 0 {
            return BLACK_QUEEN;
        }
        if pos & b.black_king != 0 {
            return BLACK_KING;
        }

        return EMPTY_SQUARE;
    }
}

