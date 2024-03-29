use fisk::board::{Board, PieceKind};
use fisk::fen::FEN_DEFAULT_BOARD;
use std::mem::size_of;

fn fen(fen: &str) -> Board {
    Board::from_fen(fen).unwrap()
}

fn test_check(fenstr: &str, white: bool, black: bool) {
    let board = fen(fenstr);
    assert_eq!(board.is_in_check(true), white);
    assert_eq!(board.is_in_check(false), black);
}

#[test]
fn memsizes() {
    assert_eq!(size_of::<PieceKind>(), 1); // Not using more memory than u8
    assert_eq!(size_of::<Board>(), 152); // We don't want to accidentally change the Board size
}

#[test]
fn rooklike_check() {
    test_check(FEN_DEFAULT_BOARD, false, false);
    test_check("8/5k2/8/8/8/8/1K2r3/8 w - - 0 1", true, false);
    test_check("8/5k2/8/8/8/8/rK6/8 w - - 0 1", true, false);
    test_check("8/5k2/8/8/1q6/8/1K6/8 w - - 0 1", true, false);
    test_check("8/1r3k2/8/8/1b6/8/1K6/8 w - - 0 1", false, false);
    test_check("8/1r3k2/8/8/1b6/8/1K6/1q6 w - - 0 1", true, false);
}

#[test]
fn knight_check() {
    test_check("8/5k2/8/8/3n4/8/1K6/8 w - - 0 1", false, false);
    test_check("8/5k2/8/8/3n4/3n4/1K6/8 w - - 0 1", true, false);
    test_check("8/5k2/8/8/2n5/3n4/1K6/8 w - - 0 1", true, false);
    test_check("8/5k2/8/8/1n6/8/7K/8 w - - 0 1", false, false);
    test_check("8/5k2/8/6N1/1n6/8/7K/8 w - - 0 1", false, true);
}

#[test]
fn bishoplike_check() {
    test_check("8/5k2/8/8/2B5/1n6/7K/8 w - - 0 1", false, true);
    test_check("8/5k2/8/3n4/2B5/8/7K/8 w - - 0 1", false, false);
    test_check("8/5k2/2b5/3n4/2B5/8/6K1/8 w - - 0 1", false, false);
}

#[test]
fn pawn_check() {
    test_check(
        "rnbqkbnr/ppp1pppp/8/8/8/8/PPPpPPPP/RNBQKBNR w KQkq - 0 1",
        true,
        false,
    );
    test_check(
        "rnbqkbnr/8/8/8/8/8/ppp1pPpp/RNBQKBNR w KQkq - 0 1",
        false,
        false,
    );
    test_check(
        "rnbqkbnr/8/8/8/8/8/ppp2ppp/RNBQKBNR w KQkq - 0 1",
        true,
        false,
    );
    test_check(
        "rnbqkbnr/PPP1P1PP/8/8/8/8/ppp2ppp/RNBQKBNR w KQkq - 0 1",
        true,
        false,
    );
    test_check("8/8/k7/7P/8/p7/1K6/8 w - - 0 1", true, false);
    test_check("8/8/6k1/7P/8/p7/1K6/8 w - - 0 1", true, true);
}
