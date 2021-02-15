use crate::board::Board;
use crate::board::PieceKind::*;
use crate::constants::TZCNT_U64_ZEROS;
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
    fn piece_moves(&self, white: bool, piece_index: usize, mut outvec: &mut Vec<Board>) {
        // TODO Keep pieces ordered with empty square pieces at the end to abort entire
        //  iteration when an empty square is found.

        let piece_position_tzcnt = self.piece_positions_tzcnt[piece_index];
        if piece_position_tzcnt == TZCNT_U64_ZEROS {
            // No position => empty square
            return;
        }
        let piece_position = 1u64 << piece_position_tzcnt;
        let piece_kind = self.piece_kinds[piece_index];

        if white ^ piece_kind.is_white() {
            return;
        }

        match piece_kind {
            WhitePawn => white_pawn_moves(&self, piece_position, piece_index, &mut outvec),
            BlackPawn => black_pawn_moves(&self, piece_position, piece_index, &mut outvec),
            WhiteRook | BlackRook => {
                rook_moves(&self, piece_position, piece_index, white, &mut outvec)
            }
            WhiteKnight | BlackKnight => {
                knight_moves(&self, piece_position, piece_index, white, &mut outvec)
            }
            WhiteKing | BlackKing => {
                king_moves(&self, piece_position, piece_index, white, &mut outvec)
            }

            // TODO
            WhiteQueen => {}
            WhiteBishop => {}
            BlackQueen => {}
            BlackBishop => {}

            EmptySquare => {
                unreachable!()
            }
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

    pub fn delete_piece(&mut self, capture_pos_tzcnt: u8) {
        let piece_positions_tzcnt = &mut self.piece_positions_tzcnt;
        for (i, p) in piece_positions_tzcnt.iter().enumerate() {
            if *p == capture_pos_tzcnt {
                piece_positions_tzcnt[i] = TZCNT_U64_ZEROS;
                self.piece_kinds[i] = EmptySquare;
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
            if piece_kind == EmptySquare {
                self.piece_index += 1;
                continue;
            }

            self.board
                .piece_moves(self.board.white_to_move(), self.piece_index, &mut self.buf);
            self.piece_index += 1;

            if !self.buf.is_empty() {
                return Some(self.buf.swap_remove(0));
            }
        }
        None
    }
}
