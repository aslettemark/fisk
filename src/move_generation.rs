use super::constants::*;
use super::engine::{Board, Piece};

pub fn white_pawn_moves(board: &Board, position: u64, pawn_piece_index: usize, outvec: &mut Vec<Board>) {
    //a white pawn cannot exist on row 8
    let pos_front = position << 8;
    let kind_front = board.kind_at(pos_front);
    if kind_front == EMPTY_SQUARE {
        // pawn short forward move
        let mut new: Board = board.clone();
        new.en_passant = 0; //No en passant in a short pawn move
        new.halfturn += 1;
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_front;
        let p = Piece {
            kind: WHITE_PAWN,
            position: pos_front,
        };
        new.pieces[pawn_piece_index] = p;
        outvec.push(new);
    }

    if kind_front == EMPTY_SQUARE && (position & ROW_2 != 0) {
        // pawn double square move
        let pos_twofront = pos_front << 8;
        if board.kind_at(pos_twofront) == EMPTY_SQUARE {
            //All clear, sir
            let mut new = board.clone();
            new.en_passant = pos_front; // Setting en passant to where another pawn can capture
            new.halfturn += 1;
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_twofront;
            let p = Piece {
                kind: WHITE_PAWN,
                position: pos_twofront,
            };
            new.pieces[pawn_piece_index] = p;
            outvec.push(new);
        }
    }

    if position & FILE_A == 0 {
        // white pawn capture left
        pawn_capture_pos(&board, position, position << 7, pawn_piece_index, outvec);
    }
    if position & FILE_H == 0 {
        // capture right
        pawn_capture_pos(&board, position, position << 9, pawn_piece_index, outvec);
    }

    fn pawn_capture_pos(board: &Board, pawn_pos: u64, capture_pos: u64, pawn_piece_index: usize, outvec: &mut Vec<Board>) {
        let kind = board.kind_at(capture_pos);

        let black_piece = kind & BLACK_BIT != 0;
        if black_piece {
            //capture
            let mut new = board.clone();
            new.en_passant = 0;
            new.halfturn += 1;
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ pawn_pos) | capture_pos;
            let p = Piece {
                kind: WHITE_PAWN,
                position: capture_pos,
            };
            new.pieces[pawn_piece_index] = p;

            let mut bb = new.bitboard;
            bb.black_pawns ^= capture_pos;
            bb.black_bishops ^= capture_pos;
            bb.black_rooks ^= capture_pos;
            bb.black_knights ^= capture_pos;
            bb.black_queen ^= capture_pos;
            bb.black_king ^= capture_pos;
            new.bitboard = bb;

            //TODO remove black piece from piece list (simple iteration and comparison on position field)

            outvec.push(new);
        }
    }

    //TODO en passant capture
}