use crate::constants::*;
use crate::engine::{BitBoard, Board, Piece};

/// Create Board from Forsyth–Edwards Notation
/// https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation
pub fn board_from_fen(fen: &str) -> Board {
    let split = fen.split_whitespace().collect::<Vec<_>>();
    if split.len() != 8 {
        panic!("Misformed FEN string");
    }

    let board = split.get(0).unwrap();
    let (bitboard, pieces) = parse_board_string(board);

    Board {
        halfturn: 0, //TODO
        en_passant: 0, //TODO
        bitboard,
        pieces,
        castling: 0, //TODO
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
        _ => panic!("Invalid piece: {}", piece)
    }
}

fn parse_board_string(board: &str) -> (BitBoard, [Piece; 32]) {
    let board_rows: Vec<&str> = board.split("/").collect::<Vec<_>>();
    if board_rows.len() != 8 {
        panic!("Missing board row(s)");
    }

    let mut bb = BitBoard {
        white_pawns: 0,
        white_queen: 0,
        white_king: 0,
        white_rooks: 0,
        white_bishops: 0,
        white_knights: 0,
        black_pawns: 0,
        black_queen: 0,
        black_king: 0,
        black_rooks: 0,
        black_bishops: 0,
        black_knights: 0,
    };

    let placeholder = Piece {
        kind: EMPTY_SQUARE,
        position: 0,
    };
    let mut pieces = [placeholder; 32]; // Limitation: only supports positions with <= 32 pieces
    let mut piece_i: usize = 0;

    for (i, pieces_str) in board_rows.iter().enumerate() {
        let row = 7 - i;
        let row_mask = 0xFF << (row * 8);
        for (j, c) in pieces_str.chars().enumerate() {
            if c.is_digit(10) {
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
                _ => ()
            };

            pieces[piece_i] = Piece {
                kind,
                position: pos,
            };
            piece_i += 1;
        }

        println!("{}, {}", row, pieces_str);
        println!("{:b}", pieces.get(0).unwrap().kind);
    }
    return (bb, pieces);
}