use crate::constants::*;
use crate::engine::{Board, Piece};

pub fn white_pawn_moves(board: &Board, position: u64, pawn_piece_index: usize, outvec: &mut Vec<Board>) {
    //a white pawn cannot exist on row 8
    let pos_front = position << 8;
    let kind_front = board.kind_at(pos_front);
    if kind_front == EMPTY_SQUARE {
        // pawn short forward move
        let mut new: Board = board.clone();
        new.en_passant = 0; //No en passant in a short pawn move
        new.halfturn += 1;
        new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_front;
        let p = Piece {
            kind: WHITE_PAWN,
            position: pos_front,
        };
        new.pieces[pawn_piece_index] = p;
        outvec.push(new);
        //TODO turn into queen, rook, bishop, knight if row == 8
    }

    if kind_front == EMPTY_SQUARE && (position & ROW_2 != 0) {
        // pawn double square move
        let pos_twofront = pos_front << 8;
        if board.kind_at(pos_twofront) == EMPTY_SQUARE {
            //All clear, sir
            let mut new = board.clone();
            new.en_passant = pos_front; // Setting en passant to where another pawn can capture
            new.halfturn += 1;
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ position) | pos_twofront;
            let p = Piece {
                kind: WHITE_PAWN,
                position: pos_twofront,
            };
            new.pieces[pawn_piece_index] = p;
            outvec.push(new);
        }
    }

    if position & FILE_A == 0 {
        // white pawn capture left
        pawn_capture_pos(&board, position, position << 7, pawn_piece_index, outvec);
    }
    if position & FILE_H == 0 {
        // capture right
        pawn_capture_pos(&board, position, position << 9, pawn_piece_index, outvec);
    }

    fn pawn_capture_pos(board: &Board, pawn_pos: u64, capture_pos: u64, pawn_piece_index: usize, outvec: &mut Vec<Board>) {
        let kind = board.kind_at(capture_pos);

        let black_piece = kind & BLACK_BIT != 0;
        if black_piece {
            //capture
            let mut new = board.clone();
            new.en_passant = 0;
            new.halfturn += 1;
            new.bitboard.white_pawns = (new.bitboard.white_pawns ^ pawn_pos) | capture_pos;
            let p = Piece {
                kind: WHITE_PAWN,
                position: capture_pos,
            };
            new.pieces[pawn_piece_index] = p;

            // TODO consider putting this in the piece list iteration, where a specific board may be identified
            let mut bb = new.bitboard;
            bb.black_pawns &= !capture_pos;
            bb.black_bishops &= !capture_pos;
            bb.black_rooks &= !capture_pos;
            bb.black_knights &= !capture_pos;
            bb.black_queen &= !capture_pos;
            bb.black_king &= !capture_pos;
            new.bitboard = bb;

            for (i, p) in new.pieces.iter().enumerate() {
                if p.position == capture_pos {
                    new.pieces[i] = Piece {
                        kind: EMPTY_SQUARE,
                        position: capture_pos, // Doesn't really matter...
                    };
                    break;
                }
            }

            outvec.push(new);
        }
    }

    //TODO en passant capture
}

pub fn rook_moves(board: &Board, position: u64, pawn_piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
    file_slide_moves(board, position, pawn_piece_index, white, outvec);
    row_slide_moves(board, position, pawn_piece_index, white, outvec);
}

fn file_slide_moves(board: &Board, position: u64, pawn_piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
    //TODO
    if position & ROW_8 == 0 { //Not in row 8, ie can move upwards
    }
    if position & ROW_1 == 0 { //Not in row 1, ie can move downwards
    }
}

fn row_slide_moves(board: &Board, position: u64, pawn_piece_index: usize, white: bool, outvec: &mut Vec<Board>) {
//TODO
}

#[cfg(test)]
mod tests {
    use crate::fen::*;

    use super::*;

    #[test]
    fn test_test() {
        assert!(true);
    }

    #[test]
    fn test_default_board_movegen() {
        test_starting_board_movegen(Board::new());
        test_starting_board_movegen(board_from_fen(FEN_DEFAULT_BOARD));
    }

    #[test]
    fn test_basic_pawn_moves() {
        let a = board_from_fen("8/8/8/8/8/6p1/5P2/8 w KQkq -");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 3);

        let b = board_from_fen("8/8/8/8/6p1/5P2/8/8 w KQkq -");
        let succ = b.generate_successors();
        assert_eq!(succ.len(), 2);
    }

    /* TODO
    #[test]
    fn test_white_pawn_en_passant() {
        let a = board_from_fen("8/8/8/5Pp1/8/8/8/8 w - g6");
        let succ = a.generate_successors();
        assert_eq!(succ.len(), 2);

        let b = board_from_fen("8/8/8/5Pp1/8/8/8/8 w - e6");
        let succ = b.generate_successors();
        assert_eq!(succ.len(), 2);
    }
    */

    fn test_starting_board_movegen(a: Board) {
        let succ = a.generate_successors();
        //TODO should eventually be 2*8 + 2*2
        assert_eq!(succ.len(), 16); // 16 moves as of right now: only white pawns

        let mut count_en_passant = 0;
        for s in succ.iter() {
            if s.en_passant != 0 {
                count_en_passant += 1;
                assert_ne!(s.en_passant & ROW_3, 0); // white pawn en passant appears on row 3
            }
        }
        assert_eq!(count_en_passant, 8); // 8 of the pawn moves should produce an en passant square

        for s in succ.iter() {
            assert_eq!(s.generate_successors().len(), 0);
        }
    }
}
