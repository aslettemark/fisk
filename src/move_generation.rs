use bitintr::*;

use crate::board::Board;
use crate::board::PieceKind::*;
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
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Board>,
) {
    let total_occupancy = our_occupancy | enemy_occupancy;

    //a white pawn cannot exist on row 8
    let pos_front = position << 8;
    let pos_front_tzcnt = pos_front.tzcnt() as u8;
    let free_square_in_front = !intersects(pos_front, total_occupancy);
    if free_square_in_front && intersects(pos_front, ROW_8) {
        // Promote
        let mut new1 = board.clone_and_advance(0, true);
        new1.bitboard.white_pawns &= !position;
        new1.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new1.piece_kinds[pawn_piece_index] = WhiteQueen;

        let mut new2 = board.clone_and_advance(0, true);
        new2.bitboard.white_pawns &= !position;
        new2.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new2.piece_kinds[pawn_piece_index] = WhiteBishop;

        let mut new3 = board.clone_and_advance(0, true);
        new3.bitboard.white_pawns &= !position;
        new3.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new3.piece_kinds[pawn_piece_index] = WhiteRook;

        let mut new4 = board.clone_and_advance(0, true);
        new4.bitboard.white_pawns &= !position;
        new4.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new4.piece_kinds[pawn_piece_index] = WhiteKnight;

        outvec.reserve(4);
        outvec.push(new1);
        outvec.push(new2);
        outvec.push(new3);
        outvec.push(new4);
    } else if free_square_in_front {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_front;
        new.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        outvec.push(new);
    }

    if free_square_in_front && intersects(position, ROW_2) {
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

    if !intersects(position, FILE_A) {
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
    if !intersects(position, FILE_H) {
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

    let ep_file = board.get_en_passant_file();
    if ep_file != 0 {
        let shift = (ep_file as u64 - 1) + 4 * 8;
        let twofront_square = 1 << shift;
        let eligible_pos = ep_eligible_positions_mask(twofront_square);
        if intersects(position, eligible_pos) {
            // en passant capture
            let mut new = board.clone_and_advance(0, true);
            let new_pawn_pos = twofront_square << 8;

            new.piece_positions_tzcnt[pawn_piece_index] = new_pawn_pos.tzcnt() as u8;
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | new_pawn_pos;

            new.delete_piece(twofront_square.tzcnt() as u8);
            new.bitboard.unset_black_piece(twofront_square);

            outvec.push(new);
        }
    }
}

pub fn black_pawn_moves(
    board: &Board,
    position: u64,
    pawn_piece_index: usize,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Board>,
) {
    let total_occupancy = our_occupancy | enemy_occupancy;

    //a black pawn cannot exist on row 0
    let pos_front = position >> 8;
    let pos_front_tzcnt = pos_front.tzcnt() as u8;
    let free_square_in_front = !intersects(pos_front, total_occupancy);
    if free_square_in_front && intersects(pos_front, ROW_1) {
        // Promote
        let mut new1 = board.clone_and_advance(0, true);
        new1.bitboard.black_pawns &= !position;
        new1.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new1.piece_kinds[pawn_piece_index] = BlackQueen;

        let mut new2 = board.clone_and_advance(0, true);
        new2.bitboard.black_pawns &= !position;
        new2.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new2.piece_kinds[pawn_piece_index] = BlackBishop;

        let mut new3 = board.clone_and_advance(0, true);
        new3.bitboard.black_pawns &= !position;
        new3.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new3.piece_kinds[pawn_piece_index] = BlackRook;

        let mut new4 = board.clone_and_advance(0, true);
        new4.bitboard.black_pawns &= !position;
        new4.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        new4.piece_kinds[pawn_piece_index] = BlackKnight;

        outvec.reserve(4);
        outvec.push(new1);
        outvec.push(new2);
        outvec.push(new3);
        outvec.push(new4);
    } else if free_square_in_front {
        // pawn short forward move
        let mut new = board.clone_and_advance(0, true);
        new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | pos_front;
        new.piece_positions_tzcnt[pawn_piece_index] = pos_front_tzcnt;
        outvec.push(new);
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

    let ep_file = board.get_en_passant_file();
    if ep_file != 0 {
        let shift = (ep_file as u64 - 1) + 3 * 8;
        let twofront_square = 1 << shift;
        let eligible_pos = ep_eligible_positions_mask(twofront_square);
        if intersects(position, eligible_pos) {
            // en passant capture
            let mut new = board.clone_and_advance(0, true);
            let new_pawn_pos = twofront_square >> 8;

            new.piece_positions_tzcnt[pawn_piece_index] = new_pawn_pos.tzcnt() as u8;
            new.bitboard.black_pawns = (new.bitboard.black_pawns ^ position) | new_pawn_pos;

            new.delete_piece(twofront_square.tzcnt() as u8);
            new.bitboard.unset_white_piece(twofront_square);

            outvec.push(new);
        }
    }
}

fn ep_eligible_positions_mask(twofront_square: u64) -> u64 {
    if intersects(twofront_square, FILE_A) {
        twofront_square << 1
    } else if intersects(twofront_square, FILE_H) {
        twofront_square >> 1
    } else {
        (twofront_square << 1) | (twofront_square >> 1)
    }
}

#[allow(clippy::too_many_arguments)]
fn bishoplike_target_square(
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
            new.bitboard.white_bishoplike = (new.bitboard.white_bishoplike ^ position) | target_pos;
            if queen {
                new.bitboard.white_rooklike = (new.bitboard.white_rooklike ^ position) | target_pos;
            }
            new.bitboard.unset_black_piece(target_pos);
        } else {
            new.bitboard.black_bishoplike = (new.bitboard.black_bishoplike ^ position) | target_pos;
            if queen {
                new.bitboard.black_rooklike = (new.bitboard.black_rooklike ^ position) | target_pos;
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
            new.bitboard.white_bishoplike = (new.bitboard.white_bishoplike ^ position) | target_pos;
            if queen {
                new.bitboard.white_rooklike = (new.bitboard.white_rooklike ^ position) | target_pos;
            }
        } else {
            new.bitboard.black_bishoplike = (new.bitboard.black_bishoplike ^ position) | target_pos;
            if queen {
                new.bitboard.black_rooklike = (new.bitboard.black_rooklike ^ position) | target_pos;
            }
        }
        outvec.push(new);
    }
    true
}

pub fn bishoplike_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    our_occupancy: u64,
    enemy_occupancy: u64,
    queen: bool,
    outvec: &mut Vec<Board>,
) {
    let white = board.white_to_move();

    fn is_at_top(position: u64) -> bool {
        intersects(position, ROW_8)
    }
    fn is_at_bottom(position: u64) -> bool {
        intersects(position, ROW_1)
    }
    fn is_at_left(position: u64) -> bool {
        intersects(position, FILE_A)
    }
    fn is_at_right(position: u64) -> bool {
        intersects(position, FILE_H)
    }

    let top = is_at_top(position);
    let bottom = is_at_bottom(position);
    let right = is_at_right(position);
    let left = is_at_left(position);

    if !top && !right {
        let mut target_pos = position << 9;
        loop {
            let should_continue = bishoplike_target_square(
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
            if !should_continue || is_at_top(target_pos) || is_at_right(target_pos) {
                break;
            }
            target_pos <<= 9;
        }
    }

    if !top && !left {
        let mut target_pos = position << 7;
        loop {
            let should_continue = bishoplike_target_square(
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
            if !should_continue || is_at_top(target_pos) || is_at_left(target_pos) {
                break;
            }
            target_pos <<= 7;
        }
    }

    if !bottom && !right {
        let mut target_pos = position >> 7;
        loop {
            let should_continue = bishoplike_target_square(
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
            if !should_continue || is_at_bottom(target_pos) || is_at_right(target_pos) {
                break;
            }
            target_pos >>= 7;
        }
    }

    if !bottom && !left {
        let mut target_pos = position >> 9;
        loop {
            let should_continue = bishoplike_target_square(
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
            if !should_continue || is_at_bottom(target_pos) || is_at_left(target_pos) {
                break;
            }
            target_pos >>= 9;
        }
    }
}

pub fn rooklike_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    our_occupancy: u64,
    enemy_occupancy: u64,
    queen: bool,
    outvec: &mut Vec<Board>,
) {
    let white = board.white_to_move();
    file_slide_moves(
        board,
        position,
        piece_index,
        white,
        queen,
        our_occupancy,
        enemy_occupancy,
        outvec,
    );
    row_slide_moves(
        board,
        position,
        piece_index,
        white,
        queen,
        our_occupancy,
        enemy_occupancy,
        outvec,
    );
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
            } else if position == ROW_1 & FILE_A {
                new.disqualify_white_queenside_castling();
            } else if position == ROW_1 & FILE_H {
                new.disqualify_white_kingside_castling();
            }
            new.bitboard.unset_black_piece(target_pos);
        } else {
            new.bitboard.black_rooklike = (new.bitboard.black_rooklike ^ position) | target_pos;
            if queen {
                new.bitboard.black_bishoplike =
                    (new.bitboard.black_bishoplike ^ position) | target_pos;
            } else if position == ROW_8 & FILE_A {
                new.disqualify_black_queenside_castling();
            } else if position == ROW_8 & FILE_H {
                new.disqualify_black_kingside_castling();
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

#[allow(clippy::too_many_arguments)]
fn file_slide_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    white: bool,
    queen: bool,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Board>,
) {
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

#[allow(clippy::too_many_arguments)]
fn row_slide_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    white: bool,
    queen: bool,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Board>,
) {
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
        // Not in file A, ie can move in negative file direction
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

pub fn knight_moves(
    board: &Board,
    position: u64,
    piece_index: usize,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Board>,
) {
    let white = board.white_to_move();
    let targets = get_knight_possible_targets(position);
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

pub fn king_moves(board: &Board, position: u64, piece_index: usize, outvec: &mut Vec<Board>) {
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

    if white {
        let kingside = board.can_white_castle_kingside()
            && !intersects(total_occupancy, ROW_1 & (FILE_F | FILE_G));
        let queenside = board.can_white_castle_queenside()
            && !intersects(total_occupancy, ROW_1 & (FILE_B | FILE_C | FILE_D));
        if !(kingside || queenside) {
            return;
        }

        // Can't castle out of check
        if board.is_in_check(true) {
            return;
        }

        if kingside
            && !board.is_square_attacked_by_black(position << 1)
            && !board.is_square_attacked_by_black(position << 2)
        {
            // perform kingside castle
            let mut new = board.clone_and_advance(0, false);

            // slow? :/
            let old_rook_pos = ROW_1 & FILE_H;
            let rook_pos_tzcnt = old_rook_pos.tzcnt() as u8;
            let rook_piece_index = slow_get_index_of_pos(&new, rook_pos_tzcnt);

            let new_king_pos = ROW_1 & FILE_G;
            let new_rook_pos = ROW_1 & FILE_F;
            update_white_castled_board(
                &mut new,
                new_king_pos,
                old_rook_pos,
                new_rook_pos,
                piece_index,
                rook_piece_index,
            );

            outvec.push(new);
        }
        if queenside
            && !board.is_square_attacked_by_black(position >> 1)
            && !board.is_square_attacked_by_black(position >> 2)
        {
            // perform queenside castle
            let mut new = board.clone_and_advance(0, false);

            // slow? :/
            let old_rook_pos = ROW_1 & FILE_A;
            let rook_pos_tzcnt = old_rook_pos.tzcnt() as u8;
            let rook_piece_index = slow_get_index_of_pos(&new, rook_pos_tzcnt);

            let new_king_pos = ROW_1 & FILE_C;
            let new_rook_pos = ROW_1 & FILE_D;
            update_white_castled_board(
                &mut new,
                new_king_pos,
                old_rook_pos,
                new_rook_pos,
                piece_index,
                rook_piece_index,
            );

            outvec.push(new);
        }
    } else {
        let kingside = board.can_black_castle_kingside()
            && !intersects(total_occupancy, ROW_8 & (FILE_F | FILE_G));
        let queenside = board.can_black_castle_queenside()
            && !intersects(total_occupancy, ROW_8 & (FILE_B | FILE_C | FILE_D));
        if !(kingside || queenside) {
            return;
        }

        // Can't castle out of check
        if board.is_in_check(false) {
            return;
        }

        if kingside
            && !board.is_square_attacked_by_white(position << 1)
            && !board.is_square_attacked_by_white(position << 2)
        {
            // perform kingside castle
            let mut new = board.clone_and_advance(0, false);

            // slow? :/
            let old_rook_pos = ROW_8 & FILE_H;
            let rook_pos_tzcnt = old_rook_pos.tzcnt() as u8;
            let rook_piece_index = slow_get_index_of_pos(&new, rook_pos_tzcnt);

            let new_king_pos = ROW_8 & FILE_G;
            let new_rook_pos = ROW_8 & FILE_F;
            update_black_castled_board(
                &mut new,
                new_king_pos,
                old_rook_pos,
                new_rook_pos,
                piece_index,
                rook_piece_index,
            );

            outvec.push(new);
        }
        if queenside
            && !board.is_square_attacked_by_white(position >> 1)
            && !board.is_square_attacked_by_white(position >> 2)
        {
            // perform queenside castle
            let mut new = board.clone_and_advance(0, false);

            // slow? :/
            let old_rook_pos = ROW_8 & FILE_A;
            let rook_pos_tzcnt = old_rook_pos.tzcnt() as u8;
            let rook_piece_index = slow_get_index_of_pos(&new, rook_pos_tzcnt);

            let new_king_pos = ROW_8 & FILE_C;
            let new_rook_pos = ROW_8 & FILE_D;
            update_black_castled_board(
                &mut new,
                new_king_pos,
                old_rook_pos,
                new_rook_pos,
                piece_index,
                rook_piece_index,
            );

            outvec.push(new);
        }
    }
}

fn slow_get_index_of_pos(board: &Board, target_pos_tzcnt: u8) -> usize {
    let (piece_index, _) = board
        .piece_positions_tzcnt
        .iter()
        .enumerate()
        .find(|(_, p)| **p == target_pos_tzcnt)
        .unwrap();
    piece_index
}

fn update_white_castled_board(
    board: &mut Board,
    new_king_pos: u64,
    old_rook_pos: u64,
    new_rook_pos: u64,
    king_piece_i: usize,
    rook_piece_i: usize,
) {
    board.piece_positions_tzcnt[king_piece_i] = new_king_pos.tzcnt() as u8;
    board.piece_positions_tzcnt[rook_piece_i] = new_rook_pos.tzcnt() as u8;
    board.bitboard.white_king = new_king_pos;
    board.bitboard.white_rooklike &= !old_rook_pos;
    board.bitboard.white_rooklike |= new_rook_pos;

    board.disqualify_white_castling();
}

fn update_black_castled_board(
    board: &mut Board,
    new_king_pos: u64,
    old_rook_pos: u64,
    new_rook_pos: u64,
    king_piece_i: usize,
    rook_piece_i: usize,
) {
    board.piece_positions_tzcnt[king_piece_i] = new_king_pos.tzcnt() as u8;
    board.piece_positions_tzcnt[rook_piece_i] = new_rook_pos.tzcnt() as u8;
    board.bitboard.black_king = new_king_pos;
    board.bitboard.black_rooklike &= !old_rook_pos;
    board.bitboard.black_rooklike |= new_rook_pos;

    board.disqualify_black_castling();
}
