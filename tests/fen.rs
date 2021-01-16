use fisk::board::Board;
use fisk::constants::*;
use fisk::fen::FEN_DEFAULT_BOARD;

#[test]
fn test_default_board_fen() {
    let a = Board::from_fen(FEN_DEFAULT_BOARD).unwrap();
    assert_eq!(a.halfmove_clock, 0, "No turns have been made");
    assert_eq!(a.en_passant, 0, "No en passant in initial state");
    assert_eq!(
        a.bitboard.white_pawns & ROW_2,
        ROW_2,
        "Row 2 is filled with white pawns"
    );
    assert_eq!(
        a.bitboard.black_pawns & ROW_7,
        ROW_7,
        "Row 7 is filled with black pawns"
    );

    for p in a.pieces.iter() {
        assert_ne!(p.kind, EMPTY_SQUARE, "Piece list is filled");
    }
    assert!(a.white_to_move())
}

#[test]
fn compare_board_constructor_fen() {
    let a = Board::default();
    let b = Board::from_fen(FEN_DEFAULT_BOARD).unwrap();

    assert_eq!(a.bitboard, b.bitboard);
    assert_eq!(a.pieces.len(), b.pieces.len());
    assert_eq!(a.halfmove_clock, b.halfmove_clock);
    // assert_eq!(a.castling, b.castling); //TODO enable
    assert!(a.white_to_move());
    assert!(b.white_to_move());
}
