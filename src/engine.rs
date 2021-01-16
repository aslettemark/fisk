use crate::board::{Board, is_kind_white};
use crate::constants::*;
use crate::move_generation::*;

impl Board {
    pub fn clone_and_advance(&self, en_passant: u64, reset_halfmove: bool) -> Board {
        let mut new = *self;
        new.en_passant = en_passant;
        new.toggle_white_to_move();

        if reset_halfmove {
            new.reset_halfmove_clock();
        } else {
            new.increment_halfmove_clock();
        }

        if new.white_to_move() {
            new.increment_fullmove_counter();
        }

        new
    }

    #[inline]
    fn piece_moves(
        &self,
        white: bool,
        piece_index: usize,
        mut outvec: &mut Vec<Board>,
    ) {
        // TODO Keep pieces ordered with empty square pieces at the end to abort entire
        //  iteration when an empty square is found.

        let piece_position = self.piece_positions[piece_index];
        let piece_kind = self.piece_kinds[piece_index];

        if piece_kind == EMPTY_SQUARE {
            return;
        }

        if white ^ is_kind_white(piece_kind) {
            return;
        }

        match piece_kind {
            WHITE_PAWN => white_pawn_moves(&self, piece_position, piece_index, &mut outvec),
            BLACK_PAWN => black_pawn_moves(&self, piece_position, piece_index, &mut outvec),
            WHITE_ROOK | BLACK_ROOK => rook_moves(&self, piece_position, piece_index, white, &mut outvec),
            WHITE_KNIGHT | BLACK_KNIGHT => {
                knight_moves(&self, piece_position, piece_index, white, &mut outvec)
            }
            WHITE_KING | BLACK_KING => king_moves(&self, piece_position, piece_index, white, &mut outvec),

            //TODO remaining kinds
            _ => {}
        }
    }

    pub fn generate_successors(&self) -> Vec<Board> {
        let white = self.white_to_move();
        let mut states = Vec::with_capacity(32);

        for i in 0..32 {
            self.piece_moves(white, i, &mut states);
        }

        states
    }

    pub fn delete_piece(&mut self, capture_pos: u64) {
        let piece_positions = &mut self.piece_positions;
        for (i, p) in piece_positions.iter().enumerate() {
            if *p == capture_pos {
                piece_positions[i] = 0;
                self.piece_kinds[i] = EMPTY_SQUARE;
                break;
            }
        }
    }

    pub fn iter_successors(&self) -> SuccessorIter {
        SuccessorIter {
            board: self,
            buf: Vec::with_capacity(16),
            piece_index: 0,
        }
    }
}

pub struct SuccessorIter<'a> {
    board: &'a Board,
    buf: Vec<Board>,
    piece_index: usize,
}

impl<'a> Iterator for SuccessorIter<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buf.is_empty() {
            let board = self.buf.swap_remove(0);
            return Some(board);
        }

        let piece_kinds = &self.board.piece_kinds;
        while self.piece_index < 32 {
            let piece_kind = piece_kinds[self.piece_index];
            if piece_kind == EMPTY_SQUARE {
                self.piece_index += 1;
                continue;
            }

            self.board.piece_moves(
                self.board.white_to_move(),
                self.piece_index,
                &mut self.buf,
            );
            self.piece_index += 1;

            if !self.buf.is_empty() {
                return Some(self.buf.swap_remove(0));
            }
        }
        None
    }
}
