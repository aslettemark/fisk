use crate::board::{Board, Piece};
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
        piece: &Piece,
        mut outvec: &mut Vec<Board>,
    ) {
        // TODO Keep pieces ordered with empty square pieces at the end to abort entire
        //  iteration when an empty square is found.
        if piece.is_empty_square() {
            return;
        }

        if white ^ piece.is_white() {
            return;
        }

        let position = piece.position;

        match piece.kind {
            WHITE_PAWN => white_pawn_moves(&self, position, piece_index, &mut outvec),
            BLACK_PAWN => black_pawn_moves(&self, position, piece_index, &mut outvec),
            WHITE_ROOK | BLACK_ROOK => rook_moves(&self, position, piece_index, white, &mut outvec),
            WHITE_KNIGHT | BLACK_KNIGHT => {
                knight_moves(&self, position, piece_index, white, &mut outvec)
            }
            WHITE_KING | BLACK_KING => king_moves(&self, position, piece_index, white, &mut outvec),

            //TODO remaining kinds
            _ => {}
        }
    }

    pub fn generate_successors(&self) -> Vec<Board> {
        let white = self.white_to_move();
        let mut states = Vec::with_capacity(32);

        for (i, piece) in self.pieces.iter().enumerate() {
            self.piece_moves(white, i, piece, &mut states);
        }

        states
    }

    pub fn delete_piece(&mut self, capture_pos: u64) {
        let mut piece_list = &mut self.pieces;
        for (i, p) in piece_list.iter().enumerate() {
            if p.position == capture_pos {
                piece_list[i].kind = EMPTY_SQUARE;
                piece_list[i].position = 0;
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

        let piece_list = &self.board.pieces;
        while self.piece_index < 32 {
            let piece = piece_list[self.piece_index];
            if piece.is_empty_square() {
                self.piece_index += 1;
                continue;
            }

            self.board.piece_moves(
                self.board.white_to_move(),
                self.piece_index,
                &piece,
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
