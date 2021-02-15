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
    let (our_occupancy, enemy_occupancy) = board.split_occupancy();
    let total_occupancy = our_occupancy | enemy_occupancy;

    //a white pawn cannot exist on row 8
    let pos_front = position << 8;
    let free_square_in_front = !intersects(pos_front, total_occupancy);
    if free_square_in_front {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_front;
        new.piece_positions_tzcnt[pawn_piece_index] = pos_front.tzcnt() as u8;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 8
    }

    if free_square_in_front && (position & ROW_2 != 0) {
        // pawn double square move
        let pos_twofront = pos_front << 8;
        if !intersects(pos_twofront, total_occupancy) {
            //All clear, sir
            let mut new = board.clone_and_advance(pos_front, true);
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_twofront;
            new.piece_positions_tzcnt[pawn_piece_index] = pos_twofront.tzcnt() as u8;
            outvec.push(new);
        }
    }

    if position & FILE_A == 0 {
        let front_left_pos = position << 7;
        if intersects(front_left_pos, enemy_occupancy) {
            pawn_capture_pos(
                &board,
                position,
                front_left_pos,
                pawn_piece_index,
                true,
                outvec,
            );
        }
    }
    if position & FILE_H == 0 {
        let front_right_pos = position << 9;
        if intersects(front_right_pos, enemy_occupancy) {
            pawn_capture_pos(
                &board,
                position,
                front_right_pos,
                pawn_piece_index,
                true,
                outvec,
            );
        }
    }
    //TODO en passant capture
}

pub fn black_pawn_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    outvec: &mut Vec<Board>,
) {
    let (our_occupancy, enemy_occupancy) = board.split_occupancy();
    let total_occupancy = our_occupancy | enemy_occupancy;

    //a black pawn cannot exist on row 0
    let pos_front = position >> 8;
    let free_square_in_front = !intersects(pos_front, total_occupancy);
    if free_square_in_front {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_front;
        new.piece_positions_tzcnt[pawn_piece_index] = pos_front.tzcnt() as u8;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 0
    }

    if free_square_in_front && intersects(position, ROW_7) {
        // pawn double square move
        let pos_twofront = pos_front >> 8;
        if !intersects(pos_twofront, total_occupancy) {
            //All clear, sir
            let mut new = board.clone_and_advance(pos_front, true);
            new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_twofront;
            new.piece_positions_tzcnt[pawn_piece_index] = pos_twofront.tzcnt() as u8;
            outvec.push(new);
        }
    }

    if position & FILE_A == 0 {
        let pos_file_lower = position >> 9;
        if intersects(pos_file_lower, enemy_occupancy) {
            pawn_capture_pos(
                &board,
                position,
                pos_file_lower,
                pawn_piece_index,
                false,
                outvec,
            );
        }
    }
    if position & FILE_H == 0 {
        let pos_file_higher = position >> 7;
        if intersects(pos_file_higher, enemy_occupancy) {
            pawn_capture_pos(
                &board,
                position,
                pos_file_higher,
                pawn_piece_index,
                false,
                outvec,
            );
        }
    }
    //TODO en passant capture
}

pub fn rooklike_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    queen: bool,
    outvec: &mut Vec<Board>,
) {
    let white = board.white_to_move();
    file_slide_moves(board, position, piece_index, white, queen, outvec);
    row_slide_moves(board, position, piece_index, white, queen, outvec);
}

#[allow(clippy::too_many_arguments)]
fn rooklike_target_square(
    white: bool,
    queen: bool,
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
            new.bitboard.white_rooklike = (new.bitboard.white_rooklike ^ position) | target_pos;
            if queen {
                new.bitboard.white_bishoplike =
                    (new.bitboard.white_bishoplike ^ position) | target_pos;
            }
            new.bitboard.unset_black_piece(target_pos);
        } else {
            new.bitboard.black_rooklike = (new.bitboard.black_rooklike ^ position) | target_pos;
            if queen {
                new.bitboard.black_bishoplike =
                    (new.bitboard.black_bishoplike ^ position) | target_pos;
            }
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
            new.bitboard.white_rooklike = (new.bitboard.white_rooklike ^ position) | target_pos;
            if queen {
                new.bitboard.white_bishoplike =
                    (new.bitboard.white_bishoplike ^ position) | target_pos;
            }
        } else {
            new.bitboard.black_rooklike = (new.bitboard.black_rooklike ^ position) | target_pos;
            if queen {
                new.bitboard.black_bishoplike =
                    (new.bitboard.black_bishoplike ^ position) | target_pos;
            }
        }
        outvec.push(new);
    }
    true
}

fn file_slide_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    white: bool,
    queen: bool,
    outvec: &mut Vec<Board>,
) {
    let (our_occupancy, enemy_occupancy) = board.split_occupancy();

    if position & ROW_8 == 0 {
        // Not in row 8, ie can move upwards
        let mut target_pos = position << 8;
        loop {
            let should_continue = rooklike_target_square(
                white,
                queen,
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
            let should_continue = rooklike_target_square(
                white,
                queen,
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
    queen: bool,
    outvec: &mut Vec<Board>,
) {
    let (our_occupancy, enemy_occupancy) = board.split_occupancy();

    if !intersects(position, FILE_H) {
        // Not in file H, ie can move in positive file direction
        let mut target_pos = position << 1;
        loop {
            let should_continue = rooklike_target_square(
                white,
                queen,
                piece_index,
                position,
                target_pos,
                our_occupancy,
                enemy_occupancy,
                board,
                outvec,
            );
            if !should_continue || intersects(target_pos, FILE_H) {
                break;
            }
            target_pos <<= 1;
        }
    }

    if !intersects(position, FILE_A) {
        // Not in file A, ie can move negative file direction
        let mut target_pos = position >> 1;
        loop {
            let should_continue = rooklike_target_square(
                white,
                queen,
                piece_index,
                position,
                target_pos,
                our_occupancy,
                enemy_occupancy,
                board,
                outvec,
            );
            if !should_continue || intersects(target_pos, FILE_A) {
                break;
            }
            target_pos >>= 1;
        }
    }
}

#[inline]
fn get_knight_possible_targets(pos: u64) -> [u64; 8] {
    KNIGHT_ATTACK[pos.tzcnt() as usize]
}

pub fn knight_moves(board: &Board, position: u64, piece_index: usize, outvec: &mut Vec<Board>) {
    let white = board.white_to_move();
    let targets = get_knight_possible_targets(position);
    let (our_occupancy, enemy_occupancy) = board.split_occupancy();
    let total_occupancy = our_occupancy | enemy_occupancy;

    for t in &targets {
        let target_pos = *t;
        if target_pos == 0 {
            continue;
        }

        let target_pos_tzcnt = target_pos.tzcnt() as u8;
        if !intersects(target_pos, total_occupancy) {
            let mut new = board.clone_and_advance(0, false);
            new.piece_positions_tzcnt[piece_index] = target_pos_tzcnt;

            if white {
                new.bitboard.white_knights = (new.bitboard.white_knights ^ position) | *t;
            } else {
                new.bitboard.black_knights = (new.bitboard.black_knights ^ position) | *t;
            }

            outvec.push(new);
        } else if intersects(target_pos, enemy_occupancy) {
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
    outvec: &mut Vec<Board>,
) {
    let white = board.white_to_move();
    let trailing = position.tzcnt() as usize;
    let (our_occupancy, enemy_occupancy) = board.split_occupancy();
    let total_occupancy = our_occupancy | enemy_occupancy;

    let targets: [u64; 8] = KING_ATTACK[trailing];
    for t in &targets {
        let target_pos = *t;
        if target_pos == 0 {
            continue;
        }
        let target_pos_tzcnt = target_pos.tzcnt() as u8;

        if !intersects(target_pos, total_occupancy) {
            // Move to empty square
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

        if intersects(target_pos, our_occupancy) {
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
