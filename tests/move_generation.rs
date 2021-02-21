use bitintr::Popcnt;

use fisk::board::Board;
use fisk::board::PieceKind::EmptySquare;
use fisk::constants::*;

fn fen(fen: &str) -> Board {
    Board::from_fen(fen).unwrap()
}

fn succ(fen: &str) -> Vec<Board> {
    let b = Board::from_fen(fen).unwrap();
    let succ = b.generate_successors();

    println!("{} successors:", succ.len());
    for s in &succ {
        s.print();
    }

    for s in &succ {
        if b.bitboard == s.bitboard {
            println!("Two identical boards:");
            b.print();
        }
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

fn gen(fen: &str, expected_succ: usize) -> Vec<Board> {
    let v = succ(fen);
    assert_eq!(v.len(), expected_succ);
    v
}

fn alive(board: &Board, n_alive: u64) {
    let mut alive = 0;
    for (i, kind) in board.piece_kinds.iter().enumerate() {
        if *kind != EmptySquare {
            alive += 1;
            assert_ne!(board.piece_positions_tzcnt[i], TZCNT_U64_ZEROS); // alive piece has location
        }
    }
    assert_eq!(alive, n_alive);
    let cover = board.bitboard.coverage();
    assert_eq!(cover.popcnt(), alive);
}

fn expect_queens(board: &Board, white: u64, black: u64) {
    assert_eq!(
        (board.bitboard.white_bishoplike & board.bitboard.white_rooklike).popcnt(),
        white
    );
    assert_eq!(
        (board.bitboard.black_bishoplike & board.bitboard.black_rooklike).popcnt(),
        black
    );
}

#[test]
fn default_board_movegen() {
    test_starting_board_movegen(Board::default());
    test_starting_board_movegen(fen(fisk::fen::FEN_DEFAULT_BOARD));
}

#[test]
fn basic_pawn_moves() {
    let a = fen("8/8/8/8/8/6p1/5P2/8 w KQkq -");
    let succ = a.generate_successors();
    assert!(a.white_to_move());
    assert_eq!(succ.len(), 3);

    gen("8/8/8/8/6p1/5P2/8/8 w KQkq -", 2);

    let c = fen("8/8/8/3p4/2QR4/8/8/8 b - - 0 1");
    assert_eq!(c.bitboard.white_rooklike.popcnt(), 2);
    assert_eq!(c.bitboard.white_bishoplike.popcnt(), 1);
    let succ = c.generate_successors();
    assert_eq!(succ.len(), 1);
    let s = succ[0];
    alive(&s, 2);
    assert_eq!(s.bitboard.white_rooklike.popcnt(), 1);
    assert_eq!(s.bitboard.white_bishoplike.popcnt(), 0);
}

fn ep_file(fen: &str, file: u8) {
    let ss = succ(fen);
    let mut ep_count = 0;
    for s in ss {
        if s.get_en_passant_file() != 0 {
            ep_count += 1;
            assert_eq!(s.get_en_passant_file(), file);
        }
    }
    assert_eq!(ep_count, 1);
}

#[test]
fn en_passant() {
    ep_file("8/4k3/8/8/8/8/P7/2K5 w - - 0 1", 1);
    ep_file("8/4k3/8/8/8/8/7P/2K5 w - - 0 1", 8);
    ep_file("8/p3k3/8/8/8/8/7P/2K5 w - - 0 1", 8);
    ep_file("8/p3k3/8/8/8/8/7P/2K5 b - - 0 1", 1);
    ep_file("8/4k2p/8/8/8/8/7P/2K5 b - - 0 1", 8);
    ep_file("8/4k2p/8/8/7r/8/6PP/2K5 w - - 0 1", 7);
}

#[test]
fn white_pawn_en_passant_capture() {
    /* TODO Old
    let a = Board::from_fen("8/8/8/5Pp1/8/8/8/8 w - g6").unwrap();
    let succ = a.generate_successors();
    assert_eq!(succ.len(), 2);*/

    /* TODO old
    let b = Board::from_fen("8/8/8/5Pp1/8/8/8/8 w - e6").unwrap();
    let succ = b.generate_successors();
    assert_eq!(succ.len(), 2);
    */

    let s1 = succ("2k5/4p3/8/3P4/8/8/8/1K6 b - g6 0 1");
    let mut ep_count = 0;
    for s in &s1 {
        if s.get_en_passant_file() != 0 {
            ep_count += 1;
            assert_eq!(s.get_en_passant_file(), 5);

            println!("EP board:");
            s.print();

            let s11 = s.generate_successors();
            println!("Ep successors:");
            for ss in &s11 {
                ss.print();
            }
            assert_eq!(s11.len(), 5 + 2); // 5 king moves and 2 pawn moves
        }
    }
    assert_eq!(ep_count, 1);
}

#[test]
fn black_pawn_en_passant_capture() {
    let s1 = succ("2k5/8/8/8/5p2/8/4P3/1K6 w - g6 0 1");
    let mut ep_count = 0;
    for s in &s1 {
        if s.get_en_passant_file() != 0 {
            ep_count += 1;
            assert_eq!(s.get_en_passant_file(), 5);

            println!("EP board:");
            s.print();

            let s11 = s.generate_successors();
            println!("Ep successors:");
            for ss in &s11 {
                ss.print();
            }

            // At least one successor removed the white pawn
            assert!(s11
                .iter()
                .any(|board| board.bitboard.white_pawns.popcnt() == 0));

            assert_eq!(s11.len(), 5 + 2); // 5 king moves and 2 pawn moves
        }
    }
    assert_eq!(ep_count, 1);
}

fn test_starting_board_movegen(a: Board) {
    let succ = a.generate_successors();
    assert_eq!(succ.len(), 20);

    let mut count_en_passant = 0;
    for s in &succ {
        if s.get_en_passant_file() != 0 {
            count_en_passant += 1;
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
fn locked_knight() {
    gen("8/8/8/1P1P4/P3P3/2N5/P3P3/1P1P4 w - - 0 1", 8);
}

#[test]
fn white_knight_capture() {
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
        alive(b, 4);
        //assert_eq!(b.generate_successors().len(), 9); // TODO enable (black king, black pawn)
    }
    {
        let a = fen("7n/5P2/4P3/8/8/8/8/8 b - - 0 1");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 2);
        for s in &succ {
            if s.bitboard.white_pawns != a.bitboard.white_pawns {
                assert_eq!(s.bitboard.white_coverage().popcnt(), 1);
                alive(s, 2);
                let sb = s.generate_successors();
                assert_eq!(sb.len(), 2);
            } else {
                assert_eq!(s.bitboard.white_coverage().popcnt(), 2);
                alive(s, 3);
            }
        }
    }
}

#[test]
fn king_movement() {
    {
        let s1 = gen("8/8/8/8/3K4/8/8/8 w - - 0 1", 8);
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
        let s2 = gen("8/8/8/8/3k4/8/8/8 b - - 0 1", 8);
        let mut cumulative = 0u64;
        for s in &s2 {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 1);
            assert_eq!(s.bitboard.white_coverage(), 0);

            let king_mask = s.bitboard.white_king;
            assert_eq!(cumulative & king_mask, 0); // All 8 moves are different
            cumulative |= king_mask;
        }
    }

    gen("8/8/8/8/8/8/PPP5/1K6 w - - 0 1", 8);
    gen("8/8/8/8/7k/8/8/8 b - - 0 1", 5);
    gen("8/8/8/8/7k/8/8/8 w - - 0 1", 0);
}

#[test]
fn king_capture() {
    gen("8/8/8/8/7k/7P/8/8 b - - 0 1", 5);

    let s2 = gen("8/8/8/6RR/5RRk/6RR/8/8 b - - 0 1", 5);
    for s in &s2 {
        assert_ne!(s.bitboard.black_coverage(), 0);
        alive(s, 6);
    }

    gen("8/8/8/8/8/1rr5/1Kr5/1r6 w - - 0 1", 8);
}

#[test]
fn iter() {
    let b = Board::default();

    let s1 = b.generate_successors();
    let s2c = b.iter_successors().count();

    assert_eq!(s1.len(), s2c);
}

#[test]
fn rook_file_slide() {
    gen("8/7k/8/8/4p1p1/4PRP1/8/K7 w - - 0 1", 3 + 7);
    gen("8/5n1k/8/8/4p1p1/4PRP1/8/K7 w - - 0 1", 3 + 5 + 1);
    gen("8/5n1k/8/8/4ppp1/4PRP1/8/K7 w - - 0 1", 3 + 2 + 1 + 2);

    let s4 = gen("8/5n1k/8/8/4ppp1/4PRP1/5p2/K7 w - - 0 1", 3 + 2 + 2);
    for s in &s4 {
        if s.bitboard.white_king == 1u64 {
            // King didn't move
            // All moves are captures
            assert_eq!(s.bitboard.black_coverage().popcnt(), 5);
            alive(s, 5 + 4);
        } else {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 6);
            alive(s, 6 + 4);
        }
    }
}

#[test]
fn rook_row_file_slide() {
    let s1 = gen("7R/6R1/5R2/4R3/3R4/2R5/1R6/R7 w - - 0 1", 8 * (2 * 7));
    for s in &s1 {
        assert_eq!(s.get_halfmove_clock(), 1);
    }

    let s2 = gen("n6R/6R1/5R2/4R3/3R4/2R5/1R6/R6n w - - 0 1", 8 * (2 * 7));
    let mut capture_count = 0;
    for s in &s2 {
        if s.get_halfmove_clock() == 0 {
            capture_count += 1;
            assert_eq!(s.bitboard.black_coverage().popcnt(), 1);
        } else {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 2);
        }
        expect_queens(s, 0, 0);
    }
    assert_eq!(capture_count, 4);

    let s3 = gen("n6Q/6R1/5R2/4R3/3R4/2R5/1R6/Q6n w - - 0 1", 8 * (2 * 7));
    let mut capture_count = 0;
    for s in &s3 {
        if s.get_halfmove_clock() == 0 {
            capture_count += 1;
            assert_eq!(s.bitboard.black_coverage().popcnt(), 1);
        } else {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 2);
        }
        expect_queens(s, 2, 0);
    }
    assert_eq!(capture_count, 4);
}

#[test]
fn queen_moves() {
    gen("8/1r6/8/3p4/3P4/8/1Q6/8 w - - 0 1", 17);

    let s2 = gen("8/1r6/8/3p4/3P4/rrr5/rQr5/rrr5 w - - 0 1", 8);
    for s in &s2 {
        assert_eq!(s.get_halfmove_clock(), 0);
        assert_eq!(s.bitboard.black_rooklike.popcnt(), 8);
        expect_queens(s, 1, 0);
    }
}

#[test]
fn bishop_moves() {
    let s1 = gen("b7/1Q6/8/8/8/8/8/8 b - - 0 1", 1);
    expect_queens(&s1[0], 0, 0);
}

#[test]
fn white_kingside_castling() {
    gen("8/3k4/8/8/8/7p/7P/4K2R w K - 0 1", 5 + 1 + 2);
    gen("8/3k4/8/8/6r1/7p/7P/4K2R w K - 0 1", 5 + 2);
}

#[test]
fn white_queenside_castling() {
    gen("8/8/3k4/8/8/8/8/R3K2R w KQ - 0 1", 2 + 5 + 2 * 7 + 5);
    gen("8/8/3k4/8/8/1r6/8/R3K2R w KQ - 0 1", 2 + 5 + 2 * 7 + 5);
    gen("8/8/3k4/8/8/2r5/8/R3K2R w KQ - 0 1", 2 + 5 + 2 * 7 + 5 - 1);
}

#[test]
fn pawn_promotion() {
    gen("2k5/5P2/8/8/8/8/8/K7 w - g6 0 1", 3 + 4);
    gen("2k2r2/5P2/8/8/8/8/8/K7 w - g6 0 1", 3);
    gen("2k5/5P2/8/8/8/8/3p4/K7 b - - 0 1", 5 + 4);
    gen("2k5/5P2/8/8/8/8/3p4/3K4 b - - 0 1", 5);
}
