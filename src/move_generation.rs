extern crate bitintr;

use bitintr::Tzcnt;
use bitintr::Popcnt;
use crate::constants::*;
use crate::engine::{Board, Piece};

fn pawn_capture_pos(board: &Board, pawn_pos: u64, capture_pos: u64, pawn_piece_index: usize, outvec: &mut Vec<Board>) {
    let kind = board.kind_at(capture_pos);

    let black_piece = kind & BLACK_BIT != 0;
    if black_piece {
        //capture
        let mut new = board.clone();
        new.en_passant = 0;
        new.halfturn += 1;
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ pawn_pos) | capture_pos;

        for (i, p) in new.pieces.iter().enumerate() {
            if p.position == capture_pos {
                new.pieces[i].kind = EMPTY_SQUARE;
                new.pieces[i].position = 0;
                break;
            }
        }

        new.pieces[pawn_piece_index].position = capture_pos;

        // TODO consider putting this in the piece list iteration, where a specific board may be identified
        let mut bb = &mut new.bitboard;
        bb.black_pawns &= !capture_pos;
        bb.black_bishops &= !capture_pos;
        bb.black_rooks &= !capture_pos;
        bb.black_knights &= !capture_pos;
        bb.black_queen &= !capture_pos;
        bb.black_king &= !capture_pos;

        outvec.push(new);
    }
}

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
        new.pieces[pawn_piece_index].position = pos_front;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 8
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
    //TODO en passant capture
}

pub fn rook_moves(board: &Board, position: u64, pawn_piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
    file_slide_moves(board, position, pawn_piece_index, white, outvec);
    row_slide_moves(board, position, pawn_piece_index, white, outvec);
}

fn file_slide_moves(board: &Board, position: u64, pawn_piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
    //TODO
    if position & ROW_8 == 0 { //Not in row 8, ie can move upwards
    }
    if position & ROW_1 == 0 { //Not in row 1, ie can move downwards
    }
}

fn row_slide_moves(board: &Board, position: u64, pawn_piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
//TODO
}

#[inline]
fn get_knight_possible_targets(pos: u64) -> [u64; 8] {
    let trailing = pos.tzcnt() as usize;
    return KNIGHT_ATTACK[trailing];
}

pub fn knight_moves(board: &Board, position: u64, piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
    let targets = get_knight_possible_targets(position);

    for t in targets.iter() {
        if *t == 0 {
            continue;
        }
        let target_kind = board.kind_at(*t);
        if target_kind == EMPTY_SQUARE {
            let mut new = board.clone();
            new.en_passant = 0;
            new.halfturn += 1;
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
                let mut new = board.clone();
                new.en_passant = 0;
                new.halfturn += 1;
                for (i, p) in new.pieces.iter().enumerate() {
                    if p.position == capture_pos {
                        new.pieces[i].kind = EMPTY_SQUARE;
                        new.pieces[i].position = 0;
                        break;
                    }
                }
                new.pieces[piece_index].position = capture_pos;

                let mut bb = &mut new.bitboard;
                if white {
                    bb.white_knights = (bb.white_knights ^ position) | capture_pos;

                    bb.black_pawns &= !capture_pos;
                    bb.black_bishops &= !capture_pos;
                    bb.black_rooks &= !capture_pos;
                    bb.black_knights &= !capture_pos;
                    bb.black_queen &= !capture_pos;
                    bb.black_king &= !capture_pos;
                } else {
                    bb.black_knights = (bb.black_knights ^ position) | capture_pos;

                    bb.white_pawns &= !capture_pos;
                    bb.white_bishops &= !capture_pos;
                    bb.white_rooks &= !capture_pos;
                    bb.white_knights &= !capture_pos;
                    bb.white_queen &= !capture_pos;
                    bb.white_king &= !capture_pos;
                }

                outvec.push(new);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fen::*;

    use super::*;

    #[test]
    fn test_test() {
        assert!(true);
    }

    #[test]
    fn test_default_board_movegen() {
        test_starting_board_movegen(Board::new());
        test_starting_board_movegen(board_from_fen(FEN_DEFAULT_BOARD));
    }

    #[test]
    fn test_basic_pawn_moves() {
        let a = board_from_fen("8/8/8/8/8/6p1/5P2/8 w KQkq -");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 3);

        let b = board_from_fen("8/8/8/8/6p1/5P2/8/8 w KQkq -");
        let succ = b.generate_successors();
        assert_eq!(succ.len(), 2);
    }

    /* TODO
    #[test]
    fn test_white_pawn_en_passant() {
        let a = board_from_fen("8/8/8/5Pp1/8/8/8/8 w - g6");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 2);

        let b = board_from_fen("8/8/8/5Pp1/8/8/8/8 w - e6");
        let succ = b.generate_successors();
        assert_eq!(succ.len(), 2);
    }
    */

    fn test_starting_board_movegen(a: Board) {
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 20);

        let mut count_en_passant = 0;
        for s in succ.iter() {
            if s.en_passant != 0 {
                count_en_passant += 1;
                assert_ne!(s.en_passant & ROW_3, 0); // white pawn en passant appears on row 3
            }
        }
        assert_eq!(count_en_passant, 8); // 8 of the pawn moves should produce an en passant square

        for s in succ.iter() {
            assert_eq!(s.generate_successors().len(), 4); // Black doesn't have pawns yet
        }
    }

    #[test]
    fn test_locked_knight() {
        let a = board_from_fen("8/8/8/1P1P4/P3P3/2N5/P3P3/1P1P4 w - - 0 1");
        assert_eq!(a.generate_successors().len(), 8);
    }

    fn test_alive(board: &Board, n_alive: u64) {
        let mut alive = 0;
        for p in board.pieces.iter() {
            if p.kind != EMPTY_SQUARE {
                alive += 1;
            }
        }
        assert_eq!(alive, n_alive);
        let cover = board.bitboard.white_coverage() | board.bitboard.black_coverage();
        assert_eq!(cover.popcnt(), alive);
    }

    #[test]
    fn test_white_knight_capture() {
        {
            let a = board_from_fen("8/6p1/8/8/1k6/1P6/2p5/N7 w - - 0 1");
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
            let a = board_from_fen("7n/5P2/4P3/8/8/8/8/8 b - - 0 1");
            let succ = a.generate_successors();
            assert_eq!(succ.len(), 2);
            for s in succ.iter() {
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
}
