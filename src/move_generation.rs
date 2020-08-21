extern crate bitintr;

use bitintr::*;

use crate::constants::*;
use crate::engine::{BitBoard, Board, Piece};

fn delete_piece(capture_pos: u64, piece_list: &mut [Piece; 32]) {
    for (i, p) in piece_list.iter().enumerate() {
        if p.position == capture_pos {
            piece_list[i].kind = EMPTY_SQUARE;
            piece_list[i].position = 0;
            break;
        }
    }
}

fn unset_white_piece(capture_pos: u64, bb: &mut BitBoard) {
    bb.white_pawns &= !capture_pos;
    bb.white_bishops &= !capture_pos;
    bb.white_rooks &= !capture_pos;
    bb.white_knights &= !capture_pos;
    bb.white_queen &= !capture_pos;
    bb.white_king &= !capture_pos;
}

fn unset_black_piece(capture_pos: u64, bb: &mut BitBoard) {
    bb.black_pawns &= !capture_pos;
    bb.black_bishops &= !capture_pos;
    bb.black_rooks &= !capture_pos;
    bb.black_knights &= !capture_pos;
    bb.black_queen &= !capture_pos;
    bb.black_king &= !capture_pos;
}

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
        return;
    }

    let white_piece = kind & BLACK_BIT == 0;
    if white ^ white_piece {
        //capture
        let mut new = board.clone_and_advance(0, true);

        delete_piece(capture_pos, &mut new.pieces);
        new.pieces[pawn_piece_index].position = capture_pos;

        if white {
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ pawn_pos) | capture_pos;

            // TODO consider putting this in the piece list iteration, where a specific board may be identified
            unset_black_piece(capture_pos, &mut new.bitboard);
        } else {
            new.bitboard.black_pawns = (new.bitboard.black_pawns ^ pawn_pos) | capture_pos;

            // TODO consider putting this in the piece list iteration, where a specific board may be identified
            unset_white_piece(capture_pos, &mut new.bitboard);
        }

        outvec.push(new);
    }
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
        new.pieces[pawn_piece_index].position = pos_front;
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
            new.pieces[pawn_piece_index].position = pos_twofront;
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
        new.pieces[pawn_piece_index].position = pos_front;
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
            new.pieces[pawn_piece_index].position = pos_twofront;
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
            new.pieces[piece_index].position = *t;

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
                delete_piece(capture_pos, &mut new.pieces);
                new.pieces[piece_index].position = capture_pos;

                let mut bb = &mut new.bitboard;
                if white {
                    bb.white_knights = (bb.white_knights ^ position) | capture_pos;

                    unset_black_piece(capture_pos, &mut new.bitboard);
                } else {
                    bb.black_knights = (bb.black_knights ^ position) | capture_pos;

                    unset_white_piece(capture_pos, &mut new.bitboard);
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
            new.pieces[piece_index].position = *t;
            if white {
                new.bitboard.white_king = (new.bitboard.white_king ^ position) | *t;
            } else {
                new.bitboard.black_king = (new.bitboard.black_king ^ position) | *t;
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
        delete_piece(capture_pos, &mut new.pieces);
        new.pieces[piece_index].position = capture_pos;

        let mut bb = &mut new.bitboard;
        if white {
            bb.white_king = (bb.white_king ^ position) | capture_pos;
            unset_black_piece(capture_pos, &mut new.bitboard);
        } else {
            bb.black_king = (bb.black_king ^ position) | capture_pos;
            unset_white_piece(capture_pos, &mut new.bitboard);
        }

        outvec.push(new);
    }

    //TODO: castling
}
