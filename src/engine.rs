use crate::board::Board;
use crate::board::PieceKind::*;
use crate::constants::{pos_to_file_index, TZCNT_U64_ZEROS};
use crate::move_representation::Move;
use crate::movegen_movelist::*;

impl Board {
    pub fn clone_and_advance(&self, en_passant: u64, reset_halfmove: bool) -> Board {
        let mut new = *self;
        new.toggle_side_to_move();

        if reset_halfmove {
            new.reset_halfmove_clock();
        } else {
            new.increment_halfmove_clock();
        }

        if new.white_to_move() {
            new.increment_fullmove_counter();
        }

        if en_passant != 0 {
            let file = (pos_to_file_index(en_passant) + 1) as u8;
            new.set_en_passant(file);
        } else {
            new.reset_en_passant();
        }

        new
    }

    #[inline]
    fn piece_moves(
        &self,
        piece_index: usize,
        our_occupancy: u64,
        enemy_occupancy: u64,
        white_to_move: bool,
        outvec: &mut Vec<Move>,
    ) {
        let piece_position_tzcnt = self.piece_positions_tzcnt[piece_index];
        if piece_position_tzcnt == TZCNT_U64_ZEROS {
            // No position => empty square
            return;
        }
        let piece_position = 1u64 << piece_position_tzcnt;
        let piece_kind = self.piece_kinds[piece_index];

        if white_to_move ^ piece_kind.is_white() {
            return;
        }

        match piece_kind {
            WhitePawn => {
                white_pawn_moves(
                    self,
                    piece_position_tzcnt,
                    our_occupancy,
                    enemy_occupancy,
                    outvec,
                );
            }
            BlackPawn => {
                black_pawn_moves(
                    self,
                    piece_position_tzcnt,
                    our_occupancy,
                    enemy_occupancy,
                    outvec,
                );
            }
            WhiteRook | BlackRook => {
                rooklike_moves(piece_position, our_occupancy, enemy_occupancy, outvec);
            }
            WhiteKnight | BlackKnight => {
                knight_moves(piece_position_tzcnt, our_occupancy, enemy_occupancy, outvec);
            }
            WhiteKing | BlackKing => {
                king_moves(self, piece_position, our_occupancy, enemy_occupancy, outvec);
            }
            WhiteQueen | BlackQueen => {
                rooklike_moves(piece_position, our_occupancy, enemy_occupancy, outvec);
                bishoplike_moves(piece_position, our_occupancy, enemy_occupancy, outvec);
            }
            WhiteBishop | BlackBishop => {
                bishoplike_moves(piece_position, our_occupancy, enemy_occupancy, outvec);
            }

            EmptySquare => {
                unreachable!()
            }
        }
    }

    pub fn generate_pseudo_legal_moves(&self) -> Vec<Move> {
        let (our_occupancy, enemy_occupancy) = self.split_occupancy();
        let mut moves = Vec::with_capacity(64);
        for i in 0..32 {
            self.piece_moves(
                i,
                our_occupancy,
                enemy_occupancy,
                self.white_to_move(),
                &mut moves,
            );
        }

        moves
    }

    pub fn generate_successors(&self) -> Vec<Board> {
        let moves = self.generate_pseudo_legal_moves();
        let mut states = Vec::with_capacity(moves.len());
        for m in &moves {
            let b = self.make_move(m);
            states.push(b);
        }
        states
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
    buf: Vec<Move>,
    piece_index: usize,
}

impl<'a> Iterator for SuccessorIter<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buf.is_empty() {
            let mov = self.buf.swap_remove(0);
            return Some(self.board.make_move(&mov));
        }

        let piece_kinds = &self.board.piece_kinds;
        while self.piece_index < 32 {
            let piece_kind = piece_kinds[self.piece_index];
            if piece_kind == EmptySquare {
                self.piece_index += 1;
                continue;
            }
            let (our_occupancy, enemy_occupancy) = self.board.split_occupancy();
            self.board.piece_moves(
                self.piece_index,
                our_occupancy,
                enemy_occupancy,
                self.board.white_to_move(),
                &mut self.buf,
            );
            self.piece_index += 1;

            if !self.buf.is_empty() {
                let mov = self.buf.swap_remove(0);
                return Some(self.board.make_move(&mov));
            }
        }
        None
    }
}
