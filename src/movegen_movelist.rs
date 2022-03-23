use bitintr::*;

use crate::board::Board;
use crate::constants::*;
use crate::move_representation::Move;

pub fn knight_moves(
    position_tzcnt: u8,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let targets = get_knight_possible_targets_tzcnt(position_tzcnt);
    let total_occupancy = our_occupancy | enemy_occupancy;

    for target_pos in targets {
        if target_pos == 0 {
            continue;
        }

        let target_pos_tzcnt = target_pos.tzcnt() as u8;
        if !intersects(target_pos, total_occupancy) {
            let mov = Move::new(position_tzcnt, target_pos_tzcnt, false, 0);
            outvec.push(mov);
        } else if intersects(target_pos, enemy_occupancy) {
            let mov = Move::new(position_tzcnt, target_pos_tzcnt, true, 0);
            outvec.push(mov);
        }
    }
}

fn pawn_capture_pos(pawn_pos_tzcnt: u8, capture_pos: u64, outvec: &mut Vec<Move>) {
    let capture_pos_tzcnt = capture_pos.tzcnt() as u8;

    // Capture and promote
    if intersects(capture_pos, ROW_1 | ROW_8) {
        let mov1 = Move::new(pawn_pos_tzcnt, capture_pos_tzcnt, true, 0b1011);
        let mov2 = Move::new(pawn_pos_tzcnt, capture_pos_tzcnt, true, 0b1001);
        let mov3 = Move::new(pawn_pos_tzcnt, capture_pos_tzcnt, true, 0b1010);
        let mov4 = Move::new(pawn_pos_tzcnt, capture_pos_tzcnt, true, 0b1000);

        outvec.reserve(4);
        outvec.push(mov1);
        outvec.push(mov2);
        outvec.push(mov3);
        outvec.push(mov4);

        return;
    }

    let mov = Move::new(pawn_pos_tzcnt, capture_pos_tzcnt, true, 0);
    outvec.push(mov);
}

pub fn white_pawn_moves(
    board: &Board,
    pawn_pos_tzcnt: u8,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let total_occupancy = our_occupancy | enemy_occupancy;
    let position = 1u64 << pawn_pos_tzcnt;

    // A white pawn cannot exist on row 8
    let pos_front_tzcnt = pawn_pos_tzcnt + 8;
    let pos_front = 1 << pos_front_tzcnt;
    let free_square_in_front = !intersects(pos_front, total_occupancy);
    if free_square_in_front && intersects(pos_front, ROW_8) {
        // Promote
        let mov1 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1011);
        let mov2 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1001);
        let mov3 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1010);
        let mov4 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1000);

        outvec.reserve(4);
        outvec.push(mov1);
        outvec.push(mov2);
        outvec.push(mov3);
        outvec.push(mov4);
    } else if free_square_in_front {
        // pawn short forward move
        let mov = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0);
        outvec.push(mov);
    }

    if free_square_in_front && intersects(position, ROW_2) {
        // pawn double square move
        let pos_twofront = pos_front << 8;
        if !intersects(pos_twofront, total_occupancy) {
            //All clear, sir
            let mov = Move::new(pawn_pos_tzcnt, pos_front_tzcnt + 8, false, 0b1);
            outvec.push(mov);
        }
    }

    if !intersects(position, FILE_A) {
        let front_left_pos = position << 7;
        if intersects(front_left_pos, enemy_occupancy) {
            pawn_capture_pos(pawn_pos_tzcnt, front_left_pos, outvec);
        }
    }
    if !intersects(position, FILE_H) {
        let front_right_pos = position << 9;
        if intersects(front_right_pos, enemy_occupancy) {
            pawn_capture_pos(pawn_pos_tzcnt, front_right_pos, outvec);
        }
    }

    let ep_file = board.get_en_passant_file();
    if ep_file != 0 {
        let shift = (ep_file as u64 - 1) + 4 * 8;
        let twofront_square = 1 << shift;
        let eligible_pos = ep_eligible_positions_mask(twofront_square);
        if intersects(position, eligible_pos) {
            // en passant capture
            let mov = Move::new(pawn_pos_tzcnt, shift as u8 + 8, true, 0b0101);
            outvec.push(mov);
        }
    }
}

pub fn black_pawn_moves(
    board: &Board,
    pawn_pos_tzcnt: u8,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let total_occupancy = our_occupancy | enemy_occupancy;
    let position = 1u64 << pawn_pos_tzcnt;

    //a black pawn cannot exist on row 0
    let pos_front_tzcnt = pawn_pos_tzcnt - 8;
    let pos_front = 1 << pos_front_tzcnt;
    let free_square_in_front = !intersects(pos_front, total_occupancy);
    if free_square_in_front && intersects(pos_front, ROW_1) {
        // Promote
        let mov1 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1011);
        let mov2 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1001);
        let mov3 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1010);
        let mov4 = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0b1000);

        outvec.reserve(4);
        outvec.push(mov1);
        outvec.push(mov2);
        outvec.push(mov3);
        outvec.push(mov4);
    } else if free_square_in_front {
        // pawn short forward move
        let mov = Move::new(pawn_pos_tzcnt, pos_front_tzcnt, false, 0);
        outvec.push(mov);
    }

    if free_square_in_front && intersects(position, ROW_7) {
        // pawn double square move
        let pos_twofront = pos_front >> 8;
        if !intersects(pos_twofront, total_occupancy) {
            //All clear, sir
            let mov = Move::new(pawn_pos_tzcnt, pawn_pos_tzcnt - 16, false, 0b1);
            outvec.push(mov);
        }
    }

    if position & FILE_A == 0 {
        let pos_file_lower = position >> 9;
        if intersects(pos_file_lower, enemy_occupancy) {
            pawn_capture_pos(pawn_pos_tzcnt, pos_file_lower, outvec);
        }
    }
    if position & FILE_H == 0 {
        let pos_file_higher = position >> 7;
        if intersects(pos_file_higher, enemy_occupancy) {
            pawn_capture_pos(pawn_pos_tzcnt, pos_file_higher, outvec);
        }
    }

    let ep_file = board.get_en_passant_file();
    if ep_file != 0 {
        let shift = (ep_file as u64 - 1) + 3 * 8;
        let twofront_square = 1 << shift;
        let eligible_pos = ep_eligible_positions_mask(twofront_square);
        if intersects(position, eligible_pos) {
            // en passant capture
            let new_pawn_pos_tzcnt = shift as u8 - 8;
            let mov = Move::new(pawn_pos_tzcnt, new_pawn_pos_tzcnt, true, 0b0101);
            outvec.push(mov);
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

fn bishoplike_target_square(
    position_tzcnt: u8,
    target_pos: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) -> bool {
    if (target_pos & our_occupancy) != 0 {
        // Abort
        return false;
    }
    let target_pos_tzcnt = target_pos.tzcnt() as u8;
    let capture = (target_pos & enemy_occupancy) != 0;
    let mov = Move::new(position_tzcnt, target_pos_tzcnt, capture, 0);
    outvec.push(mov);

    !capture
}

pub fn bishoplike_moves(
    position: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let pos_tzcnt = position.tzcnt() as u8;

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
                pos_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
                pos_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
                pos_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
                pos_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
    position: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    file_slide_moves(position, our_occupancy, enemy_occupancy, outvec);
    row_slide_moves(position, our_occupancy, enemy_occupancy, outvec);
}

fn rooklike_target_square(
    position_tzcnt: u8,
    target_pos: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) -> bool {
    if (target_pos & our_occupancy) != 0 {
        // Abort
        return false;
    }

    let capture = (target_pos & enemy_occupancy) != 0;
    let target_pos_tzcnt = target_pos.tzcnt() as u8;
    let mov = Move::new(position_tzcnt, target_pos_tzcnt, capture, 0);
    outvec.push(mov);

    !capture
}

fn file_slide_moves(
    position: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let position_tzcnt = position.tzcnt() as u8;
    if position & ROW_8 == 0 {
        // Not in row 8, ie can move upwards
        let mut target_pos = position << 8;
        loop {
            let should_continue = rooklike_target_square(
                position_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
                position_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
    position: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let position_tzcnt = position.tzcnt() as u8;
    if !intersects(position, FILE_H) {
        // Not in file H, ie can move in positive file direction
        let mut target_pos = position << 1;
        loop {
            let should_continue = rooklike_target_square(
                position_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
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
                position_tzcnt,
                target_pos,
                our_occupancy,
                enemy_occupancy,
                outvec,
            );
            if !should_continue || intersects(target_pos, FILE_A) {
                break;
            }
            target_pos >>= 1;
        }
    }
}

pub fn king_moves(
    board: &Board,
    position: u64,
    our_occupancy: u64,
    enemy_occupancy: u64,
    outvec: &mut Vec<Move>,
) {
    let white = board.white_to_move();
    let position_tzcnt = position.tzcnt() as u8;
    let total_occupancy = our_occupancy | enemy_occupancy;

    let targets: [u64; 8] = KING_ATTACK[position_tzcnt as usize];
    for target_pos in targets {
        if target_pos == 0 {
            continue;
        }
        if intersects(target_pos, our_occupancy) {
            // Can't capture our own pieces
            continue;
        }

        let target_pos_tzcnt = target_pos.tzcnt() as u8;
        let capture = intersects(target_pos, enemy_occupancy);
        let mov = Move::new(position_tzcnt, target_pos_tzcnt, capture, 0);
        outvec.push(mov);
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
            // Kingside white castle
            let mov = Move::new(position_tzcnt, 6, false, 0b10);
            outvec.push(mov);
        }
        if queenside
            && !board.is_square_attacked_by_black(position >> 1)
            && !board.is_square_attacked_by_black(position >> 2)
        {
            // Queenside white castle
            let mov = Move::new(position_tzcnt, 2, false, 0b11);
            outvec.push(mov);
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
            // Kingside black castle
            let mov = Move::new(position_tzcnt, 62, false, 0b10);
            outvec.push(mov);
        }
        if queenside
            && !board.is_square_attacked_by_white(position >> 1)
            && !board.is_square_attacked_by_white(position >> 2)
        {
            // Queenside black castle
            let mov = Move::new(position_tzcnt, 58, false, 0b11);
            outvec.push(mov);
        }
    }
}
