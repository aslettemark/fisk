use crate::board::{BitBoard, Board};
use crate::constants::*;

pub const FEN_DEFAULT_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
    /// Create Board from Forsyth–Edwards Notation
    /// https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation
    pub fn from_fen(fen: &str) -> Option<Board> {
        let split: Vec<&str> = fen.split_whitespace().collect::<Vec<_>>();
        if split.len() < 4 {
            return None;
        }
        let has_move_data = split.len() == 6;

        let board = split.get(0).unwrap();
        let (bitboard, pieces) = parse_board_string(board);

        let (halfmove_clock, fullmove_counter) = if has_move_data {
            (
                split.get(4).unwrap().parse::<u16>().ok()?,
                split.get(5).unwrap().parse::<u16>().ok()?,
            )
        } else {
            (0, 1)
        };

        let white_to_move = split.get(1).unwrap() == &"w";

        Some(Board::new(
            bitboard,
            pieces.map(|(_, p)| p),
            pieces.map(|(k, _)| k),
            halfmove_clock,
            fullmove_counter,
            0, // TODO
            white_to_move,
            0, // TODO
        ))
    }
}

/// Map FEN pieces to kinds
fn fen_kind(piece: char) -> u8 {
    match piece {
        'r' => BLACK_ROOK,
        'n' => BLACK_KNIGHT,
        'b' => BLACK_BISHOP,
        'q' => BLACK_QUEEN,
        'k' => BLACK_KING,
        'p' => BLACK_PAWN,
        'R' => WHITE_ROOK,
        'N' => WHITE_KNIGHT,
        'B' => WHITE_BISHOP,
        'Q' => WHITE_QUEEN,
        'K' => WHITE_KING,
        'P' => WHITE_PAWN,
        _ => panic!("Invalid piece: {}", piece),
    }
}

fn parse_board_string(board: &str) -> (BitBoard, [(u8, u64); 32]) {
    let board_rows: Vec<&str> = board.split('/').collect::<Vec<_>>();
    if board_rows.len() != 8 {
        panic!("Missing board row(s)");
    }

    let mut bb = BitBoard::empty();

    let mut pieces = [(EMPTY_SQUARE, 0u64); 32]; // Limitation: only supports positions with <= 32 pieces
    let mut piece_i: usize = 0;

    for (i, pieces_str) in board_rows.iter().enumerate() {
        let row = 7 - i;
        let row_mask = ROWS[row];

        let mut j = 0u64;
        for c in pieces_str.chars() {
            if c.is_digit(10) {
                j += c.to_digit(10).unwrap() as u64;
                continue;
            }
            let kind = fen_kind(c);
            let file_mask = FILE_A << j;
            let pos = row_mask & file_mask;

            match kind {
                WHITE_PAWN => bb.white_pawns ^= pos,
                WHITE_BISHOP => bb.white_bishops ^= pos,
                WHITE_KNIGHT => bb.white_knights ^= pos,
                WHITE_ROOK => bb.white_rooks ^= pos,
                WHITE_QUEEN => bb.white_queen ^= pos,
                WHITE_KING => bb.white_king ^= pos,
                BLACK_PAWN => bb.black_pawns ^= pos,
                BLACK_BISHOP => bb.black_bishops ^= pos,
                BLACK_KNIGHT => bb.black_knights ^= pos,
                BLACK_ROOK => bb.black_rooks ^= pos,
                BLACK_QUEEN => bb.black_queen ^= pos,
                BLACK_KING => bb.black_king ^= pos,
                _ => (),
            };

            pieces[piece_i] = (kind, pos);
            piece_i += 1;
            j += 1;
        }
    }
    (bb, pieces)
}
