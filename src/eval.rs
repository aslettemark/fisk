use bitintr::{Popcnt, Tzcnt};

use crate::board::Board;

pub const INF: i32 = i32::MAX;
pub const NEGINF: i32 = i32::MIN + 1;

pub const PAWN: i32 = 100;
pub const KNIGHT: i32 = 320;
pub const BISHOP: i32 = 330;
pub const ROOK: i32 = 500;
pub const QUEEN: i32 = 900;
pub const KING: i32 = 30000;
pub const QUEEN_BONUS: i32 = QUEEN - (ROOK + BISHOP);
pub const BISHOP_PAIR_BONUS: i32 = 10;

pub const WHITE_BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 5, 0, 0, 0, 0, 5, -10, -10, 10, 10, 10, 10, 10,
    10, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

pub const BLACK_BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

pub const WHITE_ROOK_TABLE: [i32; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0,
    0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0,
];

pub const BLACK_ROOK_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

pub const WHITE_PAWN_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0, 0,
    20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50, 50,
    50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const BLACK_PAWN_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const WHITE_KNIGHT_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15, 15, 10,
    5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 10, 15, 15, 10,
    0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

pub const BLACK_KNIGHT_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

pub const WHITE_QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 5, 0, 0, 0, 0, -10, -10, 5, 5, 5, 5, 5, 0, -10,
    0, 0, 5, 5, 5, 5, 0, -5, -5, 0, 5, 5, 5, 5, 0, -5, -10, 0, 5, 5, 5, 5, 0, -10, -10, 0, 0, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

pub const BLACK_QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

pub const WHITE_KING_TABLE_MIDGAME: [i32; 64] = [
    20, 30, 10, 0, 0, 10, 30, 20, 20, 20, 0, 0, 0, 0, 20, 20, -10, -20, -20, -20, -20, -20, -20,
    -10, -20, -30, -30, -40, -40, -30, -30, -20, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
    -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50,
    -40, -40, -30,
];

pub const BLACK_KING_TABLE_MIDGAME: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

pub const WHITE_KING_TABLE_ENDGAME: [i32; 64] = [
    -50, -30, -30, -30, -30, -30, -30, -50, -30, -30, 0, 0, 0, 0, -30, -30, -30, -10, 20, 30, 30,
    20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10,
    20, 30, 30, 20, -10, -30, -30, -20, -10, 0, 0, -10, -20, -30, -50, -40, -30, -20, -20, -30,
    -40, -50,
];

pub const BLACK_KING_TABLE_ENDGAME: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];

#[inline]
fn count(bits: u64) -> i32 {
    bits.popcnt() as i32
}

impl Board {
    pub fn eval(&self) -> i32 {
        let king = self.king_eval_diff();
        if king != 0 {
            return king;
        }

        self.piece_eval_diff()
    }

    // Loosely based on https://www.chessprogramming.org/Simplified_Evaluation_Function
    fn piece_eval_diff(&self) -> i32 {
        let bb = self.bitboard;
        let is_endgame = self.is_probably_endgame();
        let mut eval = 0;

        let white_queens = bb.white_queen_coverage();
        let white_queen_count = count(white_queens);
        let real_white_bishops = bb.white_bishoplike & !bb.white_rooklike;
        let real_white_rooks = bb.white_rooklike & !bb.white_bishoplike;
        let white_knights = bb.white_knights;
        let white_pawns = bb.white_pawns;

        eval += count(white_pawns) * PAWN;
        eval += count(white_knights) * KNIGHT;
        eval += count(bb.white_bishoplike) * BISHOP;
        eval += count(bb.white_rooklike) * ROOK;
        eval += white_queen_count * QUEEN_BONUS;

        if real_white_bishops != 0 {
            if real_white_bishops.popcnt() == 2 {
                // Likely bishop pair on opposite square colors
                eval += BISHOP_PAIR_BONUS;
            }
            eval += bitboard_position_boost_table_eval(real_white_bishops, &WHITE_BISHOP_TABLE);
        }
        if real_white_rooks != 0 {
            eval += bitboard_position_boost_table_eval(real_white_rooks, &WHITE_ROOK_TABLE);
        }
        if white_pawns != 0 {
            eval += bitboard_position_boost_table_eval(white_pawns, &WHITE_PAWN_TABLE);
        }
        if white_knights != 0 {
            eval += bitboard_position_boost_table_eval(white_knights, &WHITE_KNIGHT_TABLE);
        }
        if white_queens != 0 {
            eval += bitboard_position_boost_table_eval(white_queens, &WHITE_QUEEN_TABLE);
        }
        if is_endgame {
            eval += WHITE_KING_TABLE_ENDGAME[bb.white_king.tzcnt() as usize];
        } else {
            eval += WHITE_KING_TABLE_MIDGAME[bb.white_king.tzcnt() as usize];
        }

        let black_queens = bb.black_queen_coverage();
        let black_queen_count = count(black_queens);
        let real_black_bishops = bb.black_bishoplike & !bb.black_rooklike;
        let real_black_rooks = bb.black_rooklike & !bb.black_bishoplike;
        let black_knights = bb.black_knights;
        let black_pawns = bb.black_pawns;
        eval -= count(bb.black_pawns) * PAWN;
        eval -= count(bb.black_knights) * KNIGHT;
        eval -= count(bb.black_bishoplike) * BISHOP;
        eval -= count(bb.black_rooklike) * ROOK;
        eval -= black_queen_count * QUEEN_BONUS;

        if real_black_bishops != 0 {
            if real_black_bishops.popcnt() == 2 {
                // Likely bishop pair on opposite square colors
                eval -= BISHOP_PAIR_BONUS;
            }
            eval -= bitboard_position_boost_table_eval(real_black_bishops, &BLACK_BISHOP_TABLE);
        }
        if real_black_rooks != 0 {
            eval -= bitboard_position_boost_table_eval(real_black_rooks, &BLACK_ROOK_TABLE);
        }
        if black_pawns != 0 {
            eval -= bitboard_position_boost_table_eval(black_pawns, &BLACK_PAWN_TABLE);
        }
        if black_knights != 0 {
            eval -= bitboard_position_boost_table_eval(black_knights, &BLACK_KNIGHT_TABLE);
        }
        if black_queens != 0 {
            eval -= bitboard_position_boost_table_eval(black_queens, &BLACK_QUEEN_TABLE);
        }
        if is_endgame {
            eval -= BLACK_KING_TABLE_ENDGAME[bb.black_king.tzcnt() as usize];
        } else {
            eval -= BLACK_KING_TABLE_MIDGAME[bb.black_king.tzcnt() as usize];
        }

        eval
    }

    pub fn king_eval_diff(&self) -> i32 {
        let white_king = self.bitboard.white_king;
        let black_king = self.bitboard.black_king;
        if white_king == 0 {
            return -KING;
        }
        if black_king == 0 {
            return KING;
        }

        0
    }

    pub fn is_probably_endgame(&self) -> bool {
        // TODO better heuristic
        let white_queen_count = count(self.bitboard.white_queen_coverage());
        let black_queen_count = count(self.bitboard.black_queen_coverage());

        white_queen_count == 0 && black_queen_count == 0
    }
}

#[inline]
fn bitboard_position_boost_table_eval(bb: u64, table: &[i32; 64]) -> i32 {
    let mut e = 0;
    for i in (bb.tzcnt())..63 {
        if (bb & (1 << i)) != 0 {
            e += table[i as usize];
        }
    }
    e
}
