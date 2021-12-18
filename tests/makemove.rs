use bitintr::Popcnt;
use fisk::board::Board;
use fisk::constants::SQUARE_NAME;
use fisk::move_representation::Move;

fn get_move(from: &str, to: &str, capture: bool, special_bits: u8) -> Move {
    let from_tzcnt = SQUARE_NAME.iter().position(|x| *x == from).unwrap() as u8;
    let to_tzcnt = SQUARE_NAME.iter().position(|x| *x == to).unwrap() as u8;

    Move::new(from_tzcnt, to_tzcnt, capture, special_bits)
}

#[test]
fn white_pawn_double_forward() {
    let b1 = Board::default();
    let pawn_double_move = get_move("a2", "a4", false, 1);

    let b2 = b1.make_move(pawn_double_move);

    let b2_correct_pawn_bitboard = b1.bitboard.white_pawns ^ ((1 << 8) | (1 << 24));
    assert_eq!(b2.bitboard.white_pawns, b2_correct_pawn_bitboard);
    assert!(b2.piece_positions_tzcnt.iter().any(|x| *x == 24));
    assert!(!b2.piece_positions_tzcnt.iter().any(|x| *x == 8));
    assert_eq!(b2.get_en_passant_file(), 1);
}

#[test]
fn white_pawn_capture() {
    let b1 = Board::from_fen("4k3/8/8/2pp4/3P4/8/8/4K3 w - - 0 1").unwrap();
    let capture_move = get_move("d4", "c5", true, 0);

    let b2 = b1.make_move(capture_move);

    let b2_correct_pawn_bb = b1.bitboard.white_pawns ^ ((1 << 27) | (1 << 34));
    assert_eq!(b2.bitboard.white_pawns, b2_correct_pawn_bb);
    assert!(b2.piece_positions_tzcnt.iter().any(|x| *x == 34));
    assert!(!b2.piece_positions_tzcnt.iter().any(|x| *x == 27));
}

#[test]
fn white_kingside_castle() {
    let b1 = Board::from_fen("4k3/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let castling_move = get_move("e1", "g1", false, 0b10);

    let b2 = b1.make_move(castling_move);

    assert_eq!(b2.bitboard.coverage().popcnt(), 3);
    assert!(!b2.can_white_castle_kingside());
    assert!(!b2.can_white_castle_queenside());
    assert_eq!(b2.bitboard.white_king, 1 << 6);
}
