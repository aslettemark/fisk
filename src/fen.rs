use crate::constants::*;
use crate::engine::{BitBoard, Board, Piece};

pub const FEN_DEFAULT_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
    /// Create Board from Forsyth–Edwards Notation
    /// https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation
    pub fn from_fen(fen: &str) -> Board {
        let split: Vec<&str> = fen.split_whitespace().collect::<Vec<_>>();
        if split.len() < 4 {
            panic!("Malformed FEN string");
        }
        let has_move_data = split.len() == 8;

        let board = split.get(0).unwrap();
        let (bitboard, pieces) = parse_board_string(board);

        let fullmove = if has_move_data {
            split.get(5).unwrap().parse::<u16>().unwrap()
        } else {
            1
        };
        let halfmove = (fullmove - 1) * 2; //TODO Half-moves are broken! See FEN spec
        let white = split.get(1).unwrap() == &"w";

        Board {
            halfturn: halfmove,
            en_passant: 0, //TODO
            bitboard,
            pieces,
            castling: 0, //TODO
            white_to_move: white,
        }
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

fn parse_board_string(board: &str) -> (BitBoard, [Piece; 32]) {
    let board_rows: Vec<&str> = board.split('/').collect::<Vec<_>>();
    if board_rows.len() != 8 {
        panic!("Missing board row(s)");
    }

    let mut bb = BitBoard::empty();

    let mut pieces = [Piece {
        kind: EMPTY_SQUARE,
        position: 0,
    }; 32]; // Limitation: only supports positions with <= 32 pieces
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

            pieces[piece_i].kind = kind;
            pieces[piece_i].position = pos;
            piece_i += 1;
            j += 1;
        }
    }
    (bb, pieces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_board_fen() {
        let a = Board::from_fen(FEN_DEFAULT_BOARD);
        assert_eq!(a.halfturn, 0, "No turns have been made");
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
    }

    #[test]
    fn compare_board_constructor_fen() {
        let a = Board::new();
        let b = Board::from_fen(FEN_DEFAULT_BOARD);

        assert_eq!(a.bitboard, b.bitboard);
    }
}
