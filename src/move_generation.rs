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
        let mut new = board.clone_and_advance(0);

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
        let mut new = board.clone_and_advance(0);
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
            let mut new = board.clone_and_advance(pos_front);
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
        let mut new = board.clone_and_advance(0);
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
            let mut new = board.clone_and_advance(pos_front);
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
    let trailing = pos.tzcnt() as usize;
    return KNIGHT_ATTACK[trailing];
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
            let mut new = board.clone_and_advance(0);
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
                let mut new = board.clone_and_advance(0);
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
            let mut new = board.clone_and_advance(0);
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
        let mut new = board.clone_and_advance(0);
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

#[cfg(test)]
mod tests {
    use crate::fen::*;

    use super::*;

    fn succ(fen: &str) -> Vec<Board> {
        return Board::from_fen(fen).generate_successors();
    }

    fn test_alive(board: &Board, n_alive: u64) {
        let mut alive = 0;
        for p in &board.pieces {
            if p.kind != EMPTY_SQUARE {
                alive += 1;
            }
        }
        assert_eq!(alive, n_alive);
        let cover = board.bitboard.coverage();
        assert_eq!(cover.popcnt(), alive);
    }

    #[test]
    fn test_test() {
        assert!(true);
    }

    #[test]
    fn test_default_board_movegen() {
        test_starting_board_movegen(Board::new());
        test_starting_board_movegen(Board::from_fen(FEN_DEFAULT_BOARD));
    }

    #[test]
    fn test_basic_pawn_moves() {
        let a = Board::from_fen("8/8/8/8/8/6p1/5P2/8 w KQkq -");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 3);

        let b = Board::from_fen("8/8/8/8/6p1/5P2/8/8 w KQkq -");
        let succ = b.generate_successors();
        assert_eq!(succ.len(), 2);

        let c = Board::from_fen("8/8/8/3p4/2QR4/8/8/8 b - - 0 1");
        let succ = c.generate_successors();
        assert_eq!(succ.len(), 1);
        test_alive(&succ[0], 2);
        assert_eq!(succ[0].bitboard.white_queen, 0);
    }

    /* TODO
    #[test]
    fn test_white_pawn_en_passant() {
        let a = Board::from_fen("8/8/8/5Pp1/8/8/8/8 w - g6");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 2);

        let b = Board::from_fen("8/8/8/5Pp1/8/8/8/8 w - e6");
        let succ = b.generate_successors();
        assert_eq!(succ.len(), 2);
    }
    */

    fn test_starting_board_movegen(a: Board) {
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 20);

        let mut count_en_passant = 0;
        for s in &succ {
            if s.en_passant != 0 {
                count_en_passant += 1;
                assert_ne!(s.en_passant & ROW_3, 0); // white pawn en passant appears on row 3
            }
        }
        assert_eq!(count_en_passant, 8); // 8 of the pawn moves should produce an en passant square

        for s in &succ {
            let ss = s.generate_successors();
            for s in &ss {
                s.print();
                println!();
            }
            assert_eq!(ss.len(), 20);
        }
    }

    #[test]
    fn test_locked_knight() {
        let a = Board::from_fen("8/8/8/1P1P4/P3P3/2N5/P3P3/1P1P4 w - - 0 1");
        assert_eq!(a.generate_successors().len(), 8);
    }

    #[test]
    fn test_white_knight_capture() {
        {
            let a = Board::from_fen("8/6p1/8/8/1k6/1P6/2p5/N7 w - - 0 1");
            let succ = a.generate_successors();
            assert_eq!(succ.len(), 1);
            let b = succ.get(0).unwrap();
            let bb = b.bitboard;
            assert_ne!(bb.white_pawns, 0);
            assert_ne!(bb.black_pawns, 0);
            assert_ne!(bb.black_king, 0);
            assert_ne!(bb.black_pawns, a.bitboard.black_pawns);
            assert_eq!(bb.white_coverage().popcnt(), 2);
            assert_eq!(bb.black_coverage().popcnt(), 2);
            test_alive(b, 4);
            //assert_eq!(b.generate_successors().len(), 9); // TODO enable (black king, black pawn)
        }
        {
            let a = Board::from_fen("7n/5P2/4P3/8/8/8/8/8 b - - 0 1");
            let succ = a.generate_successors();
            assert_eq!(succ.len(), 2);
            for s in &succ {
                if s.bitboard.white_pawns != a.bitboard.white_pawns {
                    assert_eq!(s.bitboard.white_coverage().popcnt(), 1);
                    test_alive(s, 2);
                    let sb = s.generate_successors();
                    assert_eq!(sb.len(), 2);
                } else {
                    assert_eq!(s.bitboard.white_coverage().popcnt(), 2);
                    test_alive(s, 3);
                }
            }
        }
    }

    #[test]
    fn test_king_movement() {
        {
            let s1 = succ("8/8/8/8/3K4/8/8/8 w - - 0 1");
            assert_eq!(s1.len(), 8);
            let mut cumulative = 0u64;
            for s in &s1 {
                assert_eq!(s.bitboard.white_coverage().popcnt(), 1);
                assert_eq!(s.bitboard.black_coverage(), 0);

                let king_mask = s.bitboard.white_king;
                assert_eq!(cumulative & king_mask, 0); // All 8 moves are different
                cumulative &= king_mask;
            }
        }
        {
            let s2 = succ("8/8/8/8/3k4/8/8/8 b - - 0 1");
            assert_eq!(s2.len(), 8);
            let mut cumulative = 0u64;
            for s in &s2 {
                assert_eq!(s.bitboard.black_coverage().popcnt(), 1);
                assert_eq!(s.bitboard.white_coverage(), 0);

                let king_mask = s.bitboard.white_king;
                assert_eq!(cumulative & king_mask, 0); // All 8 moves are different
                cumulative &= king_mask;
            }
        }

        let s3 = succ("8/8/8/8/8/8/PPP5/1K6 w - - 0 1");
        assert_eq!(s3.len(), 8);

        let s4 = succ("8/8/8/8/7k/8/8/8 b - - 0 1");
        assert_eq!(s4.len(), 5);

        let s4 = succ("8/8/8/8/7k/8/8/8 w - - 0 1");
        assert_eq!(s4.len(), 0);
    }

    #[test]
    fn test_king_capture() {
        let s1 = succ("8/8/8/8/7k/7P/8/8 b - - 0 1");
        assert_eq!(s1.len(), 5);

        let s2 = succ("8/8/8/6RR/5RRk/6RR/8/8 b - - 0 1");
        assert_eq!(s2.len(), 5);
        for s in &s2 {
            assert_ne!(s.bitboard.black_coverage(), 0);
            test_alive(s, 6);
        }

        let s3 = succ("8/8/8/8/8/1rr5/1Kr5/1r6 w - - 0 1");
        assert_eq!(s3.len(), 8);
    }
}
