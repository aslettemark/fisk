use std::io::{self, BufRead, BufReader, Result};

use vampirc_uci::{UciFen, UciMessage, UciMove, UciSquare};

use crate::{
    board::{Board, PieceKind},
    constants::{self, intersects},
    fen,
    move_representation::Move,
};

pub struct UciState {
    debug: bool,
    board: Option<Board>,
}

impl UciState {
    pub fn new() -> Self {
        UciState {
            debug: true,
            board: None,
        }
    }

    pub fn run_uci_input(
        &mut self,
        input: &mut impl io::Read,
        output: &mut impl io::Write,
    ) -> Result<()> {
        let buf_read = BufReader::new(input);
        for read_line in buf_read.lines() {
            let line = read_line?;
            let message = vampirc_uci::parse_one(&line);
            eprintln!("Received message: {}", line);

            match message {
                UciMessage::Uci => {
                    writeln!(output, "id name fisk")?;
                    writeln!(output, "id author Aksel Slettemark")?;
                    writeln!(output, "uciok")?;
                }
                UciMessage::Debug(dbg) => self.debug = dbg,
                UciMessage::IsReady => writeln!(output, "readyok")?,
                UciMessage::Register { .. } => todo!(),
                UciMessage::Position {
                    startpos,
                    fen,
                    moves,
                } => self.position(startpos, fen, moves),
                UciMessage::SetOption { name: _, value: _ } => todo!(),
                UciMessage::UciNewGame => todo!(),
                UciMessage::Stop => todo!(),
                UciMessage::PonderHit => todo!(),
                UciMessage::Quit => todo!(),
                UciMessage::Go {
                    time_control: _,
                    search_control: _,
                } => todo!(),
                UciMessage::CopyProtection(_) => todo!(),
                UciMessage::Registration(_) => todo!(),
                UciMessage::Unknown(_, _) => todo!(),
                _ => {}
            }
        }

        Ok(())
    }

    fn position(&mut self, startpos: bool, fen: Option<UciFen>, moves: Vec<UciMove>) {
        dbg!(&moves);
        let fen_string = match (startpos, &fen) {
            (true, _) => fen::FEN_DEFAULT_BOARD,
            (false, Some(uci_fen)) => uci_fen.0.as_str(),
            (false, None) => panic!("Invalid position command"),
        };

        let mut board = Board::from_fen(fen_string).expect("Invalid fen string");
        if self.debug {
            eprint!("Initial board");
            eprintln!("{}", board);
        }
        for uci_move in moves {
            let fisk_move = uci_move_to_fisk_move(uci_move, &board).expect("Invalid UciMove");
            board.make_move_in_place(&fisk_move);
            if self.debug {
                eprintln!("After {}", uci_move);
                eprintln!("{}", board);
            }
        }

        self.board = Some(board);
    }
}

impl Default for UciState {
    fn default() -> Self {
        Self::new()
    }
}

fn uci_move_to_fisk_move(uci_move: UciMove, board: &Board) -> Option<Move> {
    let from = uci_square_to_tzcnt_pos(&uci_move.from)?;
    let to = uci_square_to_tzcnt_pos(&uci_move.to)?;
    let to_pos = 1u64 << to;
    let coverage = board.bitboard.coverage();

    let capture = intersects(coverage, to_pos);

    if let Some(promotion) = uci_move.promotion {
        return match promotion {
            vampirc_uci::UciPiece::Knight => Some(Move::new(from, to, capture, 0)),
            vampirc_uci::UciPiece::Bishop => Some(Move::new(from, to, capture, 0x1)),
            vampirc_uci::UciPiece::Rook => Some(Move::new(from, to, capture, 0x2)),
            vampirc_uci::UciPiece::Queen => Some(Move::new(from, to, capture, 0x3)),
            _ => None,
        };
    }

    let from_piece = board.slow_kind_at(1u64 << (from as u64));
    let bitboard_square_index_abs_diff = (from as i32 - to as i32).abs() as u32;

    if from_piece.is_pawn() {
        if bitboard_square_index_abs_diff == 16 {
            // Double pawn push
            return Some(Move::new(from, to, false, 0x1));
        }
        if !intersects(coverage, to_pos)
            && (bitboard_square_index_abs_diff == 7 || bitboard_square_index_abs_diff == 9)
        {
            // EP capture
            return Some(Move::new(from, to, true, 0x1));
        }
    }

    if from_piece == PieceKind::WhiteKing && from == 4 {
        if to == 6 {
            // Kingside castle
            return Some(Move::new(from, to, false, 0x10));
        }
        if to == 2 {
            // Queenside castle
            return Some(Move::new(from, to, false, 0x11));
        }
    }

    if from_piece == PieceKind::BlackKing && from == 60 {
        if to == 62 {
            // Kingside castle
            return Some(Move::new(from, to, false, 0x10));
        }
        if to == 58 {
            // Queenside castle
            return Some(Move::new(from, to, false, 0x11));
        }
    }

    // Quiet move or normal capture
    Some(Move::new(from, to, capture, 0))
}

fn uci_square_to_tzcnt_pos(square: &UciSquare) -> Option<u8> {
    let square_name = format!("{}{}", square.file, square.rank);
    constants::SQUARE_NAME
        .iter()
        .position(|x| x == &square_name)
        .map(|x| x as u8)
}
