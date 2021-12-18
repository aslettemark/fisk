use bitintr::Popcnt;

use fisk::board::Board;
use fisk::board::PieceKind::EmptySquare;
use fisk::constants::*;

fn fen(fen: &str) -> Board {
    Board::from_fen(fen).unwrap()
}

fn succ(fen: &str) -> Vec<Board> {
    let b = Board::from_fen(fen).unwrap();
    println!("Parent board:");
    b.print();
    println!(
        "Fullmove {} Halfmove {} White to move {}",
        b.get_fullmove_counter(),
        b.get_halfmove_clock(),
        b.white_to_move()
    );

    let succ = b.generate_successors();
    println!("{} successors:", succ.len());

    for s in &succ {
        s.print();
        println!(
            "Fullmove {} Halfmove {} White to move {}",
            s.get_fullmove_counter(),
            s.get_halfmove_clock(),
            s.white_to_move()
        );

        let sbb = s.bitboard;
        if b.bitboard == sbb {
            println!("Two identical boards:");
            b.print();
        }
        assert_ne!(b.bitboard, sbb);

        let pawn_move = (sbb.white_pawns != b.bitboard.white_pawns)
            || (sbb.black_pawns != b.bitboard.black_pawns);
        let has_captured = (b.bitboard.white_coverage() != sbb.white_coverage())
            && (b.bitboard.black_coverage() != sbb.black_coverage())
            && (b.bitboard.coverage().popcnt() > sbb.coverage().popcnt());

        if s.get_halfmove_clock() == 0 {
            assert!(pawn_move || has_captured);
        } else {
            assert!(!pawn_move);
            assert!(!has_captured);
        }

        if pawn_move {
            assert_eq!(sbb.white_pawns & ROW_1, 0);
            assert_eq!(sbb.white_pawns & ROW_8, 0);
            assert_eq!(sbb.black_pawns & ROW_1, 0);
            assert_eq!(sbb.black_pawns & ROW_8, 0);
        }
    }

    succ
}

fn gen(fen: &str, expected_succ: usize) -> Vec<Board> {
    let v = succ(fen);
    assert_eq!(v.len(), expected_succ);
    v
}

fn count_alive(board: &Board) -> usize {
    board
        .piece_kinds
        .iter()
        .filter(|pk| **pk != EmptySquare)
        .count()
}

fn assert_alive(board: &Board, n_alive: u64) {
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
    let a = fen("k7/8/8/8/8/6p1/5P2/K7 w - - 0 1");
    let succ = a.generate_successors();
    assert!(a.white_to_move());
    assert_eq!(succ.len(), 6);

    gen("k7/8/8/8/6p1/5P2/8/K7 w - - 0 1", 5);

    let c = fen("k7/8/8/3p4/2QR4/8/8/K7 b - - 0 1");
    assert_eq!(c.bitboard.white_rooklike.popcnt(), 2);
    assert_eq!(c.bitboard.white_bishoplike.popcnt(), 1);
    let succ = c.generate_successors();
    assert_eq!(succ.len(), 1 + 3);

    assert_eq!(succ.iter().filter(|b| count_alive(*b) == 4).count(), 1);
    assert_eq!(succ.iter().filter(|b| count_alive(*b) == 5).count(), 3);
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
fn pawn_always_promotes() {
    gen("3r4/k3P3/8/8/8/8/8/2K5 w - - 0 1", 4 + 4 + 5);
    gen("8/k7/8/8/8/8/2K1p3/5R2 b - - 0 1", 4 + 4 + 5);
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
fn white_knight_capture() {
    {
        let a = fen("8/6p1/8/8/1k6/1P6/2p5/N6K w - - 0 1");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 1 + 3);
        let b = succ
            .iter()
            .filter(|b| b.get_halfmove_clock() == 0)
            .collect::<Vec<&Board>>();
        assert_eq!(b.len(), 1);

        let b = b[0];
        assert_eq!(b.get_halfmove_clock(), 0);
        let bb = b.bitboard;
        assert_ne!(bb.white_pawns, 0);
        assert_ne!(bb.black_pawns, 0);
        assert_ne!(bb.black_king, 0);
        assert_ne!(bb.black_pawns, a.bitboard.black_pawns);
        assert_eq!(bb.white_coverage().popcnt(), 3);
        assert_eq!(bb.black_coverage().popcnt(), 2);
        assert_alive(b, 5);
    }
    {
        let a = fen("k6n/5P2/4P3/8/8/8/8/K7 b - - 0 1");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 2 + 3);
        for s in &succ {
            if s.bitboard.white_pawns != a.bitboard.white_pawns {
                assert_eq!(s.bitboard.white_coverage().popcnt(), 2);
                assert_alive(s, 4);
                let sb = s.generate_successors();
                assert_eq!(sb.len(), 2 + 3);
            } else {
                assert_eq!(s.bitboard.white_coverage().popcnt(), 3);
                assert_alive(s, 5);
            }
        }
    }
}

#[test]
fn king_movement() {
    {
        let s1 = gen("8/4k3/8/8/3K4/8/8/8 w - - 0 1", 8);
        let mut cumulative = 0u64;
        for s in &s1 {
            assert_eq!(s.bitboard.white_coverage().popcnt(), 1);
            assert_eq!(s.bitboard.black_coverage().popcnt(), 1);

            let king_mask = s.bitboard.white_king;
            assert_eq!(cumulative & king_mask, 0); // All 8 moves are different
            cumulative |= king_mask;
        }
    }
    {
        let s2 = gen("8/8/8/8/3k4/8/6K1/8 b - - 0 1", 8);
        let mut cumulative = 0u64;
        for s in &s2 {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 1);
            assert_eq!(s.bitboard.white_coverage().popcnt(), 1);

            let king_mask = s.bitboard.black_king;
            assert_eq!(cumulative & king_mask, 0); // All 8 moves are different
            cumulative |= king_mask;
        }
    }

    gen("8/8/5k2/8/8/8/PPP5/1K6 w - - 0 1", 8);
    gen("8/8/8/8/7k/8/1K6/8 b - - 0 1", 5);
}

#[test]
fn king_capture() {
    gen("8/8/8/8/7k/7P/8/K7 b - - 0 1", 5);

    let s2 = gen("8/8/8/6RR/5RRk/6RR/8/K7 b - - 0 1", 5);
    for s in &s2 {
        assert_ne!(s.bitboard.black_coverage(), 0);
        assert_alive(s, 7);
    }

    gen("8/5k2/8/8/8/1rr5/1Kr5/1r6 w - - 0 1", 8);
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
            assert_alive(s, 5 + 4);
        } else {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 6);
            assert_alive(s, 6 + 4);
        }
    }
}

#[test]
fn rook_row_file_slide() {
    let s1 = gen(
        "k6R/6R1/5R2/4R3/3R4/2R5/1R6/R6K w - - 0 1",
        8 * (2 * 7) - 2 + 3,
    );

    let mut capture_count = 0;
    for s in &s1 {
        if s.get_halfmove_clock() == 0 {
            capture_count += 1;
            assert_eq!(s.bitboard.black_coverage().popcnt(), 0);
        } else {
            assert_eq!(s.bitboard.black_coverage().popcnt(), 1);
        }
        expect_queens(s, 0, 0);
    }
    assert_eq!(capture_count, 2);
}

#[test]
fn queen_moves() {
    gen("8/1r6/3k4/3p4/3P4/8/1Q6/7K w - - 0 1", 20);

    let s2 = gen("1k6/8/8/3p2K1/3P4/rrr5/rQr5/rrr5 w - - 0 1", 8 + 8);
    for s in &s2 {
        expect_queens(s, 1, 0);
    }
}

#[test]
fn bishop_moves() {
    let s1 = gen("b6k/1Q6/8/8/8/8/8/K7 b - - 0 1", 4);
    assert_eq!(
        s1.iter()
            .map(|s| s.bitboard.white_bishoplike.popcnt())
            .sum::<u64>(),
        3
    );
    assert_eq!(
        s1.iter()
            .map(|s| s.bitboard.white_rooklike.popcnt())
            .sum::<u64>(),
        3
    );
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
fn black_castling() {
    gen("r3k2r/8/8/8/8/8/8/3K4 b kq - 0 1", 5 + 5 + 2 * 7 + 2);
    gen("r3k2r/8/8/8/8/8/8/3K4 b q - 0 1", 5 + 5 + 2 * 7 + 1);
    gen("r3k2r/8/8/8/8/8/8/3K4 b k - 0 1", 5 + 5 + 2 * 7 + 1);
}

#[test]
fn pawn_promotion() {
    gen("2k5/5P2/8/8/8/8/8/K7 w - g6 0 1", 3 + 4);
    gen("2k2r2/5P2/8/8/8/8/8/K7 w - g6 0 1", 3);
    gen("2k5/5P2/8/8/8/8/3p4/K7 b - - 0 1", 5 + 4);
    gen("2k5/5P2/8/8/8/8/3p4/3K4 b - - 0 1", 5);
    gen("r3k2r/4P3/8/8/8/8/8/3K4 w q - 0 1", 5);
}

#[test]
fn perft_pos5_regression() {
    gen(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        44,
    );
}
