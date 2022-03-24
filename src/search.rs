use crate::board::Board;
use crate::eval::{INF, NEGINF};
use crate::move_representation::Move;

impl Board {
    pub fn best_move(&self, depth: usize) -> (Option<Move>, i32) {
        let white = self.white_to_move();

        let best = get_best_move_and_eval(self, white, depth);

        if best.is_none() {
            if self.is_in_check(white) {
                return if white { (None, NEGINF) } else { (None, INF) }; // Mated
            }
            return (None, 0); // Stalemate
        }
        let (next_board, eval) = best.unwrap();

        (Some(next_board), eval)
    }

    fn minimax(&self, depth: usize, alpha: i32, beta: i32) -> i32 {
        let white = self.white_to_move();

        if depth == 0 {
            return self.eval();
        }

        let best = get_best_eval(self, white, depth);

        if let Some(eval) = best {
            eval
        } else {
            if self.is_in_check(white) {
                // Mated
                let depth_penalty = 50 - (depth as i32);
                return if white {
                    NEGINF + depth_penalty
                } else {
                    INF - depth_penalty
                };
            }

            // Stalemate
            0
        }
    }
}

#[inline]
fn get_best_eval(board: &Board, white: bool, depth: usize) -> Option<i32> {
    let max = white;
    let moves = board.generate_pseudo_legal_moves();
    let mut best = None;

    if max {
        for m in moves.iter() {
            let b = board.make_move(m);
            if b.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = b.minimax(depth - 1, 0, 0);
            if best.is_none() || e > best.unwrap() {
                best = Some(e);
            }
        }
    } else {
        for m in moves.iter() {
            let b = board.make_move(m);
            if b.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = b.minimax(depth - 1, 0, 0);
            if best.is_none() || e < best.unwrap() {
                best = Some(e);
            }
        }
    }

    if best.is_none() {
        let in_check = board.is_in_check(white);
        return if in_check {
            if white {
                Some(NEGINF)
            } else {
                Some(INF)
            }
        } else {
            Some(0) // Stalemate
        };
    }

    best
}

#[inline]
fn get_best_move_and_eval(board: &Board, white: bool, depth: usize) -> Option<(Move, i32)> {
    let moves = board.generate_pseudo_legal_moves();
    let mut best: Option<(Move, i32)> = None;
    if white {
        for m in moves.iter() {
            let b = board.make_move(m);
            if b.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = b.minimax(depth - 1, 0, 0);
            if best.is_none() || e > best.unwrap().1 {
                best = Some((*m, e));
            }
        }
    } else {
        for m in moves.iter() {
            let b = board.make_move(m);
            if b.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = b.minimax(depth - 1, 0, 0);
            if best.is_none() || e < best.unwrap().1 {
                best = Some((*m, e));
            }
        }
    }

    best
}
