extern crate bitintr;

use bitintr::*;

use crate::board::Board;
use crate::constants::*;

fn pawn_capture_pos(
    board: &Board,
    pawn_pos: u64,
    capture_pos: u64,
    pawn_piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    let kind = board.kind_at(capture_pos);
    if kind == EMPTY_SQUARE {
        // TODO optimize the check for emptiness?
        return;
    }

    let white_piece = kind & BLACK_BIT == 0;
    if !(white ^ white_piece) {
        return;
    }

    //capture
    let mut new = board.clone_and_advance(0, true);
    new.delete_piece(capture_pos);
    new.piece_positions[pawn_piece_index] = capture_pos;

    if white {
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ pawn_pos) | capture_pos;

        // TODO consider putting this in the piece list iteration, where a specific board may be identified
        new.bitboard.unset_black_piece(capture_pos);
    } else {
        new.bitboard.black_pawns = (new.bitboard.black_pawns ^ pawn_pos) | capture_pos;

        // TODO consider putting this in the piece list iteration, where a specific board may be identified
        new.bitboard.unset_white_piece(capture_pos);
    }

    outvec.push(new);
}

pub fn white_pawn_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    outvec: &mut Vec<Board>,
) {
    //a white pawn cannot exist on row 8
    let pos_front = position << 8;
    let kind_front = board.kind_at(pos_front);
    if kind_front == EMPTY_SQUARE {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_front;
        new.piece_positions[pawn_piece_index] = pos_front;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 8
    }

    if kind_front == EMPTY_SQUARE && (position & ROW_2 != 0) {
        // pawn double square move
        let pos_twofront = pos_front << 8;
        if board.kind_at(pos_twofront) == EMPTY_SQUARE {
            //All clear, sir
            let mut new = board.clone_and_advance(pos_front, true);
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_twofront;
            new.piece_positions[pawn_piece_index] = pos_twofront;
            outvec.push(new);
        }
    }

    if position & FILE_A == 0 {
        // white pawn capture left
        pawn_capture_pos(
            &board,
            position,
            position << 7,
            pawn_piece_index,
            true,
            outvec,
        );
    }
    if position & FILE_H == 0 {
        // capture right
        pawn_capture_pos(
            &board,
            position,
            position << 9,
            pawn_piece_index,
            true,
            outvec,
        );
    }
    //TODO en passant capture
}

pub fn black_pawn_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    outvec: &mut Vec<Board>,
) {
    //a black pawn cannot exist on row 0
    let pos_front = position >> 8;
    let kind_front = board.kind_at(pos_front);
    if kind_front == EMPTY_SQUARE {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_front;
        new.piece_positions[pawn_piece_index] = pos_front;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 0
    }

    if kind_front == EMPTY_SQUARE && (position & ROW_7 != 0) {
        // pawn double square move
        let pos_twofront = pos_front >> 8;
        if board.kind_at(pos_twofront) == EMPTY_SQUARE {
            //All clear, sir
            let mut new = board.clone_and_advance(pos_front, true);
            new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_twofront;
            new.piece_positions[pawn_piece_index] = pos_twofront;
            outvec.push(new);
        }
    }

    if position & FILE_A == 0 {
        pawn_capture_pos(
            &board,
            position,
            position >> 9,
            pawn_piece_index,
            false,
            outvec,
        );
    }
    if position & FILE_H == 0 {
        pawn_capture_pos(
            &board,
            position,
            position >> 7,
            pawn_piece_index,
            false,
            outvec,
        );
    }
    //TODO en passant capture
}

pub fn rook_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    file_slide_moves(board, position, pawn_piece_index, white, outvec);
    row_slide_moves(board, position, pawn_piece_index, white, outvec);
}

fn file_slide_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    //TODO
    if position & ROW_8 == 0 { //Not in row 8, ie can move upwards
    }
    if position & ROW_1 == 0 { //Not in row 1, ie can move downwards
    }
}

fn row_slide_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    //TODO
}

#[inline]
fn get_knight_possible_targets(pos: u64) -> [u64; 8] {
    KNIGHT_ATTACK[pos.tzcnt() as usize]
}

pub fn knight_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    let targets = get_knight_possible_targets(position);

    for t in &targets {
        if *t == 0 {
            continue;
        }
        let target_kind = board.kind_at(*t);
        if target_kind == EMPTY_SQUARE {
            let mut new = board.clone_and_advance(0, false);
            new.piece_positions[piece_index] = *t;

            if white {
                new.bitboard.white_knights = (new.bitboard.white_knights ^ position) | *t;
            } else {
                new.bitboard.black_knights = (new.bitboard.black_knights ^ position) | *t;
            }

            outvec.push(new);
        } else {
            let target_white = (target_kind & BLACK_BIT) == 0;
            if white ^ target_white {
                let capture_pos = *t;
                let mut new = board.clone_and_advance(0, true);
                new.delete_piece(capture_pos);
                new.piece_positions[piece_index] = capture_pos;

                let mut bb = &mut new.bitboard;
                if white {
                    bb.white_knights = (bb.white_knights ^ position) | capture_pos;

                    new.bitboard.unset_black_piece(capture_pos);
                } else {
                    bb.black_knights = (bb.black_knights ^ position) | capture_pos;

                    new.bitboard.unset_white_piece(capture_pos);
                }

                outvec.push(new);
            }
        }
    }
}

pub fn king_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    let trailing = position.tzcnt() as usize;
    let targets: [u64; 8] = KING_ATTACK[trailing];
    for t in &targets {
        if *t == 0 {
            continue;
        }
        let target_kind = board.kind_at(*t);
        if target_kind == EMPTY_SQUARE {
            let mut new = board.clone_and_advance(0, false);
            new.piece_positions[piece_index] = *t;
            if white {
                new.bitboard.white_king = (new.bitboard.white_king ^ position) | *t;
                new.disqualify_white_castling();
            } else {
                new.bitboard.black_king = (new.bitboard.black_king ^ position) | *t;
                new.disqualify_black_castling();
            }
            outvec.push(new);
            continue;
        }

        let target_white = (target_kind & BLACK_BIT) == 0;
        if !(white ^ target_white) {
            // Can't capture our own pieces
            continue;
        }

        // Capture
        let capture_pos = *t;
        let mut new = board.clone_and_advance(0, true);
        new.delete_piece(capture_pos);
        new.piece_positions[piece_index] = capture_pos;

        let mut bb = &mut new.bitboard;
        if white {
            bb.white_king = (bb.white_king ^ position) | capture_pos;
            new.bitboard.unset_black_piece(capture_pos);
            new.disqualify_white_castling();
        } else {
            bb.black_king = (bb.black_king ^ position) | capture_pos;
            new.bitboard.unset_white_piece(capture_pos);
            new.disqualify_black_castling();
        }

        outvec.push(new);
    }

    //TODO: castling
}
