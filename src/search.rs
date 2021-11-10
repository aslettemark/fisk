use crate::board::Board;
use crate::eval::{INF, NEGINF};

impl Board {
    // TODO change to -> Move, bench
    pub fn best_move(&self, depth: usize) -> (Option<Board>, i32) {
        let white = self.white_to_move();

        let best = get_best_board_and_eval(self, white, depth);

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

        let depth_penalty = 50 - (depth as i32);
        if best.is_none() {
            if self.is_in_check(white) {
                // Mated
                return if white {
                    NEGINF + depth_penalty
                } else {
                    INF - depth_penalty
                };
            }

            // Stalemate
            return 0;
        }

        best.unwrap()
    }
}

#[inline]
fn get_best_eval(board: &Board, white: bool, depth: usize) -> Option<i32> {
    let max = white;
    //let mut iter = self.iter_successors();
    let ss = board.generate_successors();
    let mut iter = ss.iter();
    let first = iter.next();
    if first.is_none() {
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
    let first = first.unwrap();
    let mut best: Option<i32>;
    if first.is_in_check(white) {
        best = None;
    } else {
        best = Some(first.eval());
    }

    if max {
        for s in iter {
            if s.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = s.minimax(depth - 1, 0, 0);
            if best.is_none() || e > best.unwrap() {
                best = Some(e);
            }
        }
    } else {
        for s in iter {
            if s.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = s.minimax(depth - 1, 0, 0);
            if best.is_none() || e < best.unwrap() {
                best = Some(e);
            }
        }
    }

    best
}

#[inline]
fn get_best_board_and_eval(board: &Board, white: bool, depth: usize) -> Option<(Board, i32)> {
    let successors = board.generate_successors();
    if successors.is_empty() {
        // Checkmate or stalemate
        return None;
    }

    let mut best: Option<(&Board, i32)> = None;
    if white {
        for s in successors.iter() {
            if s.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = s.minimax(depth - 1, 0, 0);
            if best.is_none() || e > best.unwrap().1 {
                best = Some((s, e));
            }
        }
    } else {
        for s in successors.iter() {
            if s.is_in_check(white) {
                // Illegal, can't move into check
                continue;
            }
            let e = s.minimax(depth - 1, 0, 0);
            if best.is_none() || e < best.unwrap().1 {
                best = Some((s, e));
            }
        }
    }

    if let Some((b, e)) = best {
        Some((*b, e))
    } else {
        None
    }
}
