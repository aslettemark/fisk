use crate::board::Board;
use crate::eval::{INF, NEGINF};
use crate::move_representation::Move;

impl Board {
    pub fn best_move(&self, depth: usize) -> (i32, Option<Move>) {
        self.minimax(depth, NEGINF, INF)
    }

    fn minimax(&self, depth: usize, mut alpha: i32, mut beta: i32) -> (i32, Option<Move>) {
        let white = self.white_to_move();

        if depth == 0 {
            return (self.eval(), None);
        }

        let moves = self.generate_pseudo_legal_moves();

        let max = white;
        let mut best: Option<(i32, Move)> = None;

        if max {
            for m in moves.iter() {
                let b = self.make_move(m);
                if b.is_in_check(white) {
                    // Illegal, can't move into check
                    continue;
                }
                let score = b.minimax(depth - 1, alpha, beta).0;

                if score >= beta {
                    return (beta, None); // fail hard beta-cutoff
                }

                if best.is_none() || score > best.unwrap().0 {
                    alpha = score;
                    best = Some((score, *m));
                }
            }
        } else {
            for m in moves.iter() {
                let b = self.make_move(m);
                if b.is_in_check(white) {
                    // Illegal, can't move into check
                    continue;
                }
                let score = b.minimax(depth - 1, alpha, beta).0;

                if score <= alpha {
                    return (alpha, None); // fail hard alpha-cutoff
                }

                if best.is_none() || score < best.unwrap().0 {
                    beta = score;
                    best = Some((score, *m));
                }
            }
        }

        if let Some((eval, m)) = best {
            (eval, Some(m))
        } else {
            if self.is_in_check(white) {
                // Mated
                let depth_penalty = 50 - (depth as i32);
                return if white {
                    (NEGINF + depth_penalty, None)
                } else {
                    (INF - depth_penalty, None)
                };
            }

            // Stalemate
            (0, None)
        }
    }
}
