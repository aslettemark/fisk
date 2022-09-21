use bitintr::*;

use crate::board::PieceKind::*;
use crate::board::{BitBoard, Board, PieceKind};
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

        let board = split.first().unwrap();
        let (bitboard, pieces) = parse_board_string(board)?;

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

        let mut castling_availability = 0u8;
        let castling_flags = split.get(2).unwrap();
        for c in castling_flags.chars() {
            match c {
                'K' => castling_availability |= 1 << 3,
                'Q' => castling_availability |= 1 << 2,
                'k' => castling_availability |= 1 << 1,
                'q' => castling_availability |= 1 << 0,
                '-' => {
                    castling_availability = 0;
                    break;
                }
                _ => return None,
            }
        }

        let ep_pos = {
            let ep_field = split.get(3).unwrap();
            let square_index = SQUARE_NAME.iter().position(|x| x == ep_field);

            if let Some(i) = square_index {
                1u64 << i
            } else {
                0
            }
        };

        let board = Board::new(
            bitboard,
            piece_positions,
            piece_kinds,
            halfmove_clock,
            fullmove_counter,
            ep_pos,
            white_to_move,
            castling_availability,
        );

        if board.is_sane_position() {
            return Some(board);
        }

        None
    }

    pub fn is_sane_position(&self) -> bool {
        let bb = self.bitboard;

        (bb.white_pawns.popcnt() <= 8)
            && (bb.black_pawns.popcnt() <= 8)
            && (bb.white_king.popcnt() == 1)
            && (bb.black_king.popcnt() == 1)
            && (((bb.white_pawns | bb.black_pawns) & (ROW_1 | ROW_8)) == 0)
    }
}

/// Map FEN pieces to kinds
fn fen_kind(piece: char) -> Option<PieceKind> {
    match piece {
        'r' => Some(BlackRook),
        'n' => Some(BlackKnight),
        'b' => Some(BlackBishop),
        'q' => Some(BlackQueen),
        'k' => Some(BlackKing),
        'p' => Some(BlackPawn),
        'R' => Some(WhiteRook),
        'N' => Some(WhiteKnight),
        'B' => Some(WhiteBishop),
        'Q' => Some(WhiteQueen),
        'K' => Some(WhiteKing),
        'P' => Some(WhitePawn),
        _ => None,
    }
}

fn parse_board_string(board: &str) -> Option<(BitBoard, [(PieceKind, u64); 32])> {
    let board_rows: Vec<&str> = board.split('/').collect::<Vec<_>>();
    if board_rows.len() != 8 {
        return None;
    }

    let mut bb = BitBoard::empty();

    let mut pieces = [(EmptySquare, 0u64); 32]; // Limitation: only supports positions with <= 32 pieces
    let mut piece_i: usize = 0;

    for (i, pieces_str) in board_rows.iter().enumerate() {
        let row = 7 - i;
        let row_mask = ROWS[row];

        let mut j = 0u64;
        for c in pieces_str.chars() {
            if c.is_ascii_digit() {
                j += c.to_digit(10).unwrap() as u64;
                continue;
            }
            let kind = fen_kind(c)?;
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
                }
                WhiteKing => bb.white_king ^= pos,
                BlackPawn => bb.black_pawns ^= pos,
                BlackBishop => bb.black_bishoplike ^= pos,
                BlackKnight => bb.black_knights ^= pos,
                BlackRook => bb.black_rooklike ^= pos,
                BlackQueen => {
                    bb.black_rooklike ^= pos;
                    bb.black_bishoplike ^= pos;
                }
                BlackKing => bb.black_king ^= pos,
                _ => (),
            };

            if piece_i >= 32 {
                return None;
            }
            pieces[piece_i] = (kind, pos);
            piece_i += 1;
            j += 1;
        }
    }
    Some((bb, pieces))
}
