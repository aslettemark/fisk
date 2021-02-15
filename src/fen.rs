use crate::board::PieceKind::*;
use crate::board::{BitBoard, Board, PieceKind};
use crate::constants::*;

use bitintr::*;

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

        let mut piece_positions = [TZCNT_U64_ZEROS; 32];
        let mut piece_kinds = [PieceKind::EmptySquare; 32];

        for i in 0..32 {
            piece_kinds[i] = pieces[i].0;
            let pos = pieces[i].1;
            piece_positions[i] = pos.tzcnt() as u8;
        }

        Some(Board::new(
            bitboard,
            piece_positions,
            piece_kinds,
            halfmove_clock,
            fullmove_counter,
            0, // TODO
            white_to_move,
            0, // TODO
        ))
    }
}

/// Map FEN pieces to kinds
fn fen_kind(piece: char) -> PieceKind {
    match piece {
        'r' => BlackRook,
        'n' => BlackKnight,
        'b' => BlackBishop,
        'q' => BlackQueen,
        'k' => BlackKing,
        'p' => BlackPawn,
        'R' => WhiteRook,
        'N' => WhiteKnight,
        'B' => WhiteBishop,
        'Q' => WhiteQueen,
        'K' => WhiteKing,
        'P' => WhitePawn,
        _ => panic!("Invalid piece: {}", piece),
    }
}

fn parse_board_string(board: &str) -> (BitBoard, [(PieceKind, u64); 32]) {
    let board_rows: Vec<&str> = board.split('/').collect::<Vec<_>>();
    if board_rows.len() != 8 {
        panic!("Missing board row(s)");
    }

    let mut bb = BitBoard::empty();

    let mut pieces = [(EmptySquare, 0u64); 32]; // Limitation: only supports positions with <= 32 pieces
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
                WhitePawn => bb.white_pawns ^= pos,
                WhiteBishop => bb.white_bishoplike ^= pos,
                WhiteKnight => bb.white_knights ^= pos,
                WhiteRook => bb.white_rooklike ^= pos,
                WhiteQueen => {
                    bb.white_rooklike ^= pos;
                    bb.white_bishoplike ^= pos;
                },
                WhiteKing => bb.white_king ^= pos,
                BlackPawn => bb.black_pawns ^= pos,
                BlackBishop => bb.black_bishoplike ^= pos,
                BlackKnight => bb.black_knights ^= pos,
                BlackRook => bb.black_rooklike ^= pos,
                BlackQueen => {
                    bb.black_rooklike ^= pos;
                    bb.black_bishoplike ^= pos;
                },
                BlackKing => bb.black_king ^= pos,
                _ => (),
            };

            pieces[piece_i] = (kind, pos);
            piece_i += 1;
            j += 1;
        }
    }
    (bb, pieces)
}
