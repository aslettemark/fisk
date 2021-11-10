use bitintr::Popcnt;

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

    fn piece_eval_diff(&self) -> i32 {
        let bb = self.bitboard;
        let mut eval = 0;

        let white_queen_count = count(bb.white_queen_coverage());
        let white_bishoplike_count = count(bb.white_bishoplike);
        eval += count(bb.white_pawns) * PAWN;
        eval += count(bb.white_knights) * KNIGHT;
        eval += white_bishoplike_count * BISHOP;
        eval += count(bb.white_rooklike) * ROOK;
        eval += white_queen_count * QUEEN_BONUS;
        if white_bishoplike_count == white_queen_count + 2 {
            // Likely bishop pair
            eval += BISHOP_PAIR_BONUS;
        }

        eval -= count(bb.black_pawns) * PAWN;
        eval -= count(bb.black_knights) * KNIGHT;
        eval -= count(bb.black_bishoplike) * BISHOP;
        eval -= count(bb.black_rooklike) * ROOK;
        eval -= count(bb.black_queen_coverage()) * QUEEN_BONUS;

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
}
