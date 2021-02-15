use bitintr::*;

use crate::board::Board;
use crate::board::PieceKind::EmptySquare;
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
    if kind == EmptySquare {
        // TODO optimize the check for emptiness?
        return;
    }

    if !(white ^ kind.is_white()) {
        return;
    }

    //capture
    let mut new = board.clone_and_advance(0, true);
    let capture_pos_tzcnt = capture_pos.tzcnt() as u8;
    new.delete_piece(capture_pos_tzcnt);
    new.piece_positions_tzcnt[pawn_piece_index] = capture_pos_tzcnt;

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
    if kind_front == EmptySquare {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_front;
        new.piece_positions_tzcnt[pawn_piece_index] = pos_front.tzcnt() as u8;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 8
    }

    if kind_front == EmptySquare && (position & ROW_2 != 0) {
        // pawn double square move
        let pos_twofront = pos_front << 8;
        if board.kind_at(pos_twofront) == EmptySquare {
            //All clear, sir
            let mut new = board.clone_and_advance(pos_front, true);
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_twofront;
            new.piece_positions_tzcnt[pawn_piece_index] = pos_twofront.tzcnt() as u8;
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
    if kind_front == EmptySquare {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_front;
        new.piece_positions_tzcnt[pawn_piece_index] = pos_front.tzcnt() as u8;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 0
    }

    if kind_front == EmptySquare && (position & ROW_7 != 0) {
        // pawn double square move
        let pos_twofront = pos_front >> 8;
        if board.kind_at(pos_twofront) == EmptySquare {
            //All clear, sir
            let mut new = board.clone_and_advance(pos_front, true);
            new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_twofront;
            new.piece_positions_tzcnt[pawn_piece_index] = pos_twofront.tzcnt() as u8;
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
    piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    rook_file_slide_moves(board, position, piece_index, white, outvec);
    row_slide_moves(board, position, piece_index, white, outvec);
}

#[allow(clippy::too_many_arguments)]
fn rook_target_square(
    white: bool,
    piece_index: usize,
    position: u64,
    target_pos: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    board: &Board,
    outvec: &mut Vec<Board>,
) -> bool {
    let target_pos_tzcnt = target_pos.tzcnt() as u8;
    if (target_pos & enemy_occupancy) != 0 {
        // Capture
        let mut new = board.clone_and_advance(0, true);
        new.delete_piece(target_pos_tzcnt);
        new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;
        if white {
            new.bitboard.white_rooks = (new.bitboard.white_rooks ^ position) | target_pos;
            new.bitboard.unset_black_piece(target_pos);
        } else {
            new.bitboard.black_rooks = (new.bitboard.black_rooks ^ position) | target_pos;
            new.bitboard.unset_white_piece(target_pos);
        }
        outvec.push(new);
        return false;
    } else if (target_pos & our_occupancy) != 0 {
        // Abort
        return false;
    } else {
        // Move to empty square
        let mut new = board.clone_and_advance(0, false);
        new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;
        if white {
            new.bitboard.white_rooks = (new.bitboard.white_rooks ^ position) | target_pos;
        } else {
            new.bitboard.black_rooks = (new.bitboard.black_rooks ^ position) | target_pos;
        }
        outvec.push(new);
    }
    true
}

fn rook_file_slide_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    white: bool,
    outvec: &mut Vec<Board>,
) {
    //TODO
    let our_occupancy;
    let enemy_occupancy;
    if white {
        our_occupancy = board.bitboard.white_coverage();
        enemy_occupancy = board.bitboard.black_coverage();
    } else {
        our_occupancy = board.bitboard.black_coverage();
        enemy_occupancy = board.bitboard.white_coverage();
    }

    if position & ROW_8 == 0 {
        // Not in row 8, ie can move upwards
        let mut target_pos = position << 8;
        loop {
            let should_continue = rook_target_square(
                white,
                piece_index,
                position,
                target_pos,
                our_occupancy,
                enemy_occupancy,
                board,
                outvec,
            );
            if !should_continue || (target_pos & ROW_8) != 0 {
                break;
            }
            target_pos <<= 8;
        }
    }

    if position & ROW_1 == 0 {
        // Not in row 1, ie can move downwards
        let mut target_pos = position >> 8;
        loop {
            let should_continue = rook_target_square(
                white,
                piece_index,
                position,
                target_pos,
                our_occupancy,
                enemy_occupancy,
                board,
                outvec,
            );
            if !should_continue || (target_pos & ROW_1) != 0 {
                break;
            }
            target_pos >>= 8;
        }
    }
}

fn row_slide_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
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
        let target_pos = *t;
        if target_pos == 0 {
            continue;
        }

        let target_kind = board.kind_at(target_pos);
        let target_pos_tzcnt = target_pos.tzcnt() as u8;
        if target_kind == EmptySquare {
            let mut new = board.clone_and_advance(0, false);
            new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;

            if white {
                new.bitboard.white_knights = (new.bitboard.white_knights ^ position) | *t;
            } else {
                new.bitboard.black_knights = (new.bitboard.black_knights ^ position) | *t;
            }

            outvec.push(new);
        } else if white ^ target_kind.is_white() {
            let mut new = board.clone_and_advance(0, true);
            new.delete_piece(target_pos_tzcnt);
            new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;

            let mut bb = &mut new.bitboard;
            if white {
                bb.white_knights = (bb.white_knights ^ position) | target_pos;

                new.bitboard.unset_black_piece(target_pos);
            } else {
                bb.black_knights = (bb.black_knights ^ position) | target_pos;

                new.bitboard.unset_white_piece(target_pos);
            }

            outvec.push(new);
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
        let target_pos = *t;
        if target_pos == 0 {
            continue;
        }
        let target_kind = board.kind_at(*t);
        let target_pos_tzcnt = target_pos.tzcnt() as u8;
        if target_kind == EmptySquare {
            let mut new = board.clone_and_advance(0, false);
            new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;
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

        if !(white ^ target_kind.is_white()) {
            // Can't capture our own pieces
            continue;
        }

        // Capture
        let mut new = board.clone_and_advance(0, true);
        new.delete_piece(target_pos_tzcnt);
        new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;

        let mut bb = &mut new.bitboard;
        if white {
            bb.white_king = (bb.white_king ^ position) | target_pos;
            new.bitboard.unset_black_piece(target_pos);
            new.disqualify_white_castling();
        } else {
            bb.black_king = (bb.black_king ^ position) | target_pos;
            new.bitboard.unset_white_piece(target_pos);
            new.disqualify_black_castling();
        }

        outvec.push(new);
    }

    //TODO: castling
}
