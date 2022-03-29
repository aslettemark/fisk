use std::io::{self, BufRead, BufReader, Result};

use vampirc_uci::{UciFen, UciMessage, UciMove, UciSearchControl, UciSquare, UciTimeControl};

use crate::{
    board::{Board, PieceKind},
    constants::{self, intersects, SQUARE_NAME},
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
                UciMessage::UciNewGame => self.board = None,
                UciMessage::Stop => todo!(),
                UciMessage::PonderHit => todo!(),
                UciMessage::Quit => {
                    return Ok(());
                }
                UciMessage::Go {
                    time_control,
                    search_control,
                } => self.go(time_control, search_control, output),
                UciMessage::Unknown(_, _) => todo!(),
                _ => {}
            }
        }

        Ok(())
    }

    fn position(&mut self, startpos: bool, fen: Option<UciFen>, moves: Vec<UciMove>) {
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

    fn go(
        &mut self,
        time_control: Option<UciTimeControl>,
        search_control: Option<UciSearchControl>,
        output: &mut impl io::Write,
    ) {
        let board = if let Some(board) = self.board {
            board
        } else {
            eprint!("Go with no position");
            return;
        };

        // TODO if-let-chain
        let depth = if let Some(control) = search_control {
            if let Some(depth) = control.depth {
                depth
            } else {
                6
            }
        } else {
            6
        };

        let (_eval, best_move) = board.best_move(depth as usize);
        writeln!(
            output,
            "bestmove {}",
            fisk_move_to_uci_text(&best_move.unwrap())
        )
        .unwrap();
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
    let from_pos = 1u64 << from;
    let to_pos = 1u64 << to;
    let coverage = board.bitboard.coverage();

    let capture = intersects(coverage, to_pos);
    if let Some(promotion) = uci_move.promotion {
        return match promotion {
            vampirc_uci::UciPiece::Knight => Some(Move::new(from, to, capture, 0b1000)),
            vampirc_uci::UciPiece::Bishop => Some(Move::new(from, to, capture, 0b1001)),
            vampirc_uci::UciPiece::Rook => Some(Move::new(from, to, capture, 0b1010)),
            vampirc_uci::UciPiece::Queen => Some(Move::new(from, to, capture, 0b1011)),
            _ => None,
        };
    }

    let from_piece = board.slow_kind_at(from_pos);
    let bitboard_square_index_abs_diff = (from as i32 - to as i32).abs() as u32;

    if from_piece.is_pawn() {
        if bitboard_square_index_abs_diff == 16 {
            // Double pawn push
            return Some(Move::new(from, to, false, 0b1));
        }
        if !intersects(coverage, to_pos)
            && (bitboard_square_index_abs_diff == 7 || bitboard_square_index_abs_diff == 9)
        {
            // EP capture
            return Some(Move::new(from, to, true, 0b1));
        }
    }

    if from_piece == PieceKind::WhiteKing && from == 4 {
        if to == 6 {
            // Kingside castle
            return Some(Move::new(from, to, false, 0b10));
        }
        if to == 2 {
            // Queenside castle
            return Some(Move::new(from, to, false, 0b11));
        }
    }

    if from_piece == PieceKind::BlackKing && from == 60 {
        if to == 62 {
            // Kingside castle
            return Some(Move::new(from, to, false, 0b10));
        }
        if to == 58 {
            // Queenside castle
            return Some(Move::new(from, to, false, 0b11));
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

fn fisk_move_to_uci_text(mov: &Move) -> String {
    let from = SQUARE_NAME[mov.from() as usize];
    let to = SQUARE_NAME[mov.to() as usize];

    if mov.is_promotion() {
        let char = match mov.flags_nibble() & 0b11 {
            0b00 => "k",
            0b01 => "b",
            0b10 => "r",
            0b11 => "q",
            _ => unreachable!(),
        };
        return format!("{}{}{}", from, to, char);
    }

    format!("{}{}", from, to)
}
