use bitintr::Popcnt;

use fisk::board::PieceKind::EmptySquare;
use fisk::board::{Board, PieceKind};
use fisk::constants::*;
use std::mem::size_of;

fn fen(fen: &str) -> Board {
    Board::from_fen(fen).unwrap()
}

fn succ(fen: &str) -> Vec<Board> {
    let b = Board::from_fen(fen).unwrap();
    let succ = b.generate_successors();

    for s in &succ {
        assert_ne!(b.bitboard, s.bitboard);

        let pawn_move = (s.bitboard.white_pawns != b.bitboard.white_pawns)
            || (s.bitboard.black_pawns != b.bitboard.black_pawns);
        let has_captured = (b.bitboard.white_coverage() != s.bitboard.white_coverage())
            && (b.bitboard.black_coverage() != s.bitboard.black_coverage())
            && (b.bitboard.coverage().popcnt() > s.bitboard.coverage().popcnt());

        if s.get_halfmove_clock() == 0 {
            assert!(pawn_move || has_captured);
        } else {
            assert!(!pawn_move);
            assert!(!has_captured);
        }
    }

    succ
}

fn test_alive(board: &Board, n_alive: u64) {
    let mut alive = 0;
    for (i, kind) in board.piece_kinds.iter().enumerate() {
        if *kind != EmptySquare {
            alive += 1;
            assert_ne!(board.piece_positions[i], 0u64); // alive piece has location
        }
    }
    assert_eq!(alive, n_alive);
    let cover = board.bitboard.coverage();
    assert_eq!(cover.popcnt(), alive);
}

#[test]
fn test_default_board_movegen() {
    test_starting_board_movegen(Board::default());
    test_starting_board_movegen(fen(fisk::fen::FEN_DEFAULT_BOARD));
}

#[test]
fn test_basic_pawn_moves() {
    let a = fen("8/8/8/8/8/6p1/5P2/8 w KQkq -");
    let succ = a.generate_successors();
    assert!(a.white_to_move());
    assert_eq!(succ.len(), 3);

    let b = fen("8/8/8/8/6p1/5P2/8/8 w KQkq -");
    let succ = b.generate_successors();
    assert_eq!(succ.len(), 2);

    let c = fen("8/8/8/3p4/2QR4/8/8/8 b - - 0 1");
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
    let a = fen("8/8/8/1P1P4/P3P3/2N5/P3P3/1P1P4 w - - 0 1");
    assert_eq!(a.generate_successors().len(), 8);
}

#[test]
fn test_white_knight_capture() {
    {
        let a = fen("8/6p1/8/8/1k6/1P6/2p5/N7 w - - 0 1");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 1);
        let b = succ.get(0).unwrap();
        assert_eq!(b.get_halfmove_clock(), 0);
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
        let a = fen("7n/5P2/4P3/8/8/8/8/8 b - - 0 1");
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
            cumulative |= king_mask;
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
            cumulative |= king_mask;
        }
    }

    let s3 = succ("8/8/8/8/8/8/PPP5/1K6 w - - 0 1");
    assert_eq!(s3.len(), 8);

    let s4 = succ("8/8/8/8/7k/8/8/8 b - - 0 1");
    assert_eq!(s4.len(), 5);

    let s5 = succ("8/8/8/8/7k/8/8/8 w - - 0 1");
    assert_eq!(s5.len(), 0);
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

#[test]
fn test_iter() {
    let b = Board::default();

    let s1 = b.generate_successors();
    let s2c = b.iter_successors().count();

    assert_eq!(s1.len(), s2c);
}

#[test]
fn test_piece_kind_memsize() {
    assert_eq!(size_of::<PieceKind>(), 1); // Not using more memory than u8
}
