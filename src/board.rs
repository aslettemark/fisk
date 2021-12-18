use bitintr::*;

use crate::board::Color::{Black, Empty, White};
use crate::board::PieceKind::*;
use crate::constants::*;
use crate::move_representation::Move;

/// Bit overview:
/// 0: white to move
/// 1: black queenside castling availability
/// 2: black kingside castling
/// 3: white queenside castling
/// 4: white kingside castling
/// 5-7: unused
/// 8-15: en passant file (bit 8 set = file A en passant opportunity, bit 15 = file H)
/// 16-31: halfmove_clock
/// 32-47: fullmove_counter
#[derive(Copy, Clone, Debug)]
struct Flags(u64);

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub bitboard: BitBoard,
    pub piece_positions_tzcnt: [u8; 32],
    pub piece_kinds: [PieceKind; 32],
    flags: Flags,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard {
    // Little-Endian Rank-File (LERF) https://www.chessprogramming.org/Square_Mapping_Considerations
    // bit 0 is a1, bit 7 is h1, bit 63 is h8
    // TODO: Consider having array(s) of u64 to reduce branching when checking side, ie pawns[white]
    pub white_pawns: u64,
    pub white_king: u64,
    pub white_rooklike: u64,
    pub white_bishoplike: u64,
    pub white_knights: u64,

    pub black_pawns: u64,
    pub black_king: u64,
    pub black_rooklike: u64,
    pub black_bishoplike: u64,
    pub black_knights: u64,
}

impl BitBoard {
    #[inline]
    pub const fn empty() -> BitBoard {
        BitBoard {
            white_pawns: 0,
            white_king: 0,
            white_rooklike: 0,
            white_bishoplike: 0,
            white_knights: 0,
            black_pawns: 0,
            black_king: 0,
            black_rooklike: 0,
            black_bishoplike: 0,
            black_knights: 0,
        }
    }

    #[inline]
    pub fn white_coverage(&self) -> u64 {
        self.white_king
            | self.white_knights
            | self.white_rooklike
            | self.white_bishoplike
            | self.white_pawns
    }

    #[inline]
    pub fn black_coverage(&self) -> u64 {
        self.black_king
            | self.black_knights
            | self.black_rooklike
            | self.black_bishoplike
            | self.black_pawns
    }

    #[inline]
    pub fn coverage(&self) -> u64 {
        // Might be a silly premature opt to not trust the codegen for doing white_cover | black_cover
        self.white_king
            | self.white_knights
            | self.white_rooklike
            | self.white_bishoplike
            | self.white_pawns
            | self.black_king
            | self.black_knights
            | self.black_rooklike
            | self.black_bishoplike
            | self.black_pawns
    }

    #[inline]
    pub fn unset_white_piece(&mut self, capture_pos: u64) {
        self.white_pawns &= !capture_pos;
        self.white_bishoplike &= !capture_pos;
        self.white_rooklike &= !capture_pos;
        self.white_knights &= !capture_pos;
        self.white_king &= !capture_pos;
    }

    #[inline]
    pub fn unset_black_piece(&mut self, capture_pos: u64) {
        self.black_pawns &= !capture_pos;
        self.black_bishoplike &= !capture_pos;
        self.black_rooklike &= !capture_pos;
        self.black_knights &= !capture_pos;
        self.black_king &= !capture_pos;
    }

    #[inline]
    pub fn white_queen_coverage(&self) -> u64 {
        self.white_bishoplike & self.white_rooklike
    }

    #[inline]
    pub fn black_queen_coverage(&self) -> u64 {
        self.black_bishoplike & self.black_rooklike
    }

    #[inline]
    pub fn king_coverage(&self) -> u64 {
        self.black_king | self.white_king
    }
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
    Empty,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PieceKind {
    EmptySquare = EMPTY_SQUARE as isize,

    WhiteQueen = WHITE_QUEEN as isize,
    WhiteKing = WHITE_KING as isize,
    WhiteRook = WHITE_ROOK as isize,
    WhiteBishop = WHITE_BISHOP as isize,
    WhiteKnight = WHITE_KNIGHT as isize,
    WhitePawn = WHITE_PAWN as isize,

    BlackQueen = BLACK_QUEEN as isize,
    BlackKing = BLACK_KING as isize,
    BlackRook = BLACK_ROOK as isize,
    BlackBishop = BLACK_BISHOP as isize,
    BlackKnight = BLACK_KNIGHT as isize,
    BlackPawn = BLACK_PAWN as isize,
}

impl PieceKind {
    pub fn is_white(&self) -> bool {
        self.get_color() == White
    }

    pub fn get_color(&self) -> Color {
        if *self == EmptySquare {
            return Empty;
        }
        if ((*self as u8) & BLACK_BIT) != 0 {
            return Black;
        }
        White
    }

    pub fn is_pawn(&self) -> bool {
        *self == WhitePawn || *self == BlackPawn
    }
}

impl Board {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bitboard: BitBoard,
        piece_positions_tzcnt: [u8; 32],
        piece_kinds: [PieceKind; 32],
        halfmove_clock: u16,
        fullmove_counter: u16,
        en_passant: u64,
        white_to_move: bool,
        castling_availability: u8, // 0b0000KQkq
    ) -> Board {
        let mut board = Board {
            bitboard,
            piece_positions_tzcnt,
            piece_kinds,
            flags: Flags { 0: 0 },
        };

        board.flags.set_bit(0, white_to_move);
        for i in 0..4 {
            if (castling_availability & (1 << i)) != 0 {
                board.flags.set_bit(i + 1, true);
            }
        }
        board.set_halfmove_clock(halfmove_clock);
        board.set_fullmove_counter(fullmove_counter);

        if en_passant != 0 {
            let file = (pos_to_file_index(en_passant) + 1) as u8;
            board.set_en_passant(file);
        }

        board
    }

    pub fn print(&self) {
        println!(" A B C D E F G H");

        for (i, row) in ROWS.iter().rev().enumerate() {
            print!("{}", 7 - i + 1);

            for file in &FILES {
                let pos = row & file;
                let c = self.piece_representation(self.slow_kind_at(pos));

                print!("{} ", c);
            }

            println!();
        }

        println!(" A B C D E F G H");

        /*for p in self.pieces.iter() {
            println!("{:#066b}: {}", p.position, self.piece_representation(p.kind));
        }*/
    }

    fn piece_representation(&self, kind: PieceKind) -> char {
        match kind {
            WhitePawn => '♙',
            WhiteBishop => '♗',
            WhiteKnight => '♘',
            WhiteRook => '♖',
            WhiteQueen => '♕',
            WhiteKing => '♔',
            BlackPawn => '♟',
            BlackBishop => '♝',
            BlackKnight => '♞',
            BlackRook => '♜',
            BlackQueen => '♛',
            BlackKing => '♚',
            _ => ' ',
        }
    }

    pub fn slow_kind_at(&self, pos: u64) -> PieceKind {
        let pos_tzcnt = pos.tzcnt() as u8;
        for (i, ptz) in self.piece_positions_tzcnt.iter().enumerate() {
            if *ptz == pos_tzcnt {
                return self.piece_kinds[i];
            }
        }
        EmptySquare
    }

    #[inline]
    pub fn split_occupancy(&self) -> (u64, u64) {
        if self.white_to_move() {
            (
                self.bitboard.white_coverage(),
                self.bitboard.black_coverage(),
            )
        } else {
            (
                self.bitboard.black_coverage(),
                self.bitboard.white_coverage(),
            )
        }
    }

    #[inline]
    pub const fn white_to_move(self) -> bool {
        self.flags.get_bit(0)
    }

    pub fn toggle_side_to_move(&mut self) {
        self.flags.toggle_bit(0);
    }

    pub fn disqualify_white_castling(&mut self) {
        self.flags.0 &= !(0b11000);
    }

    pub fn disqualify_white_queenside_castling(&mut self) {
        self.flags.set_bit(3, false);
    }

    pub fn disqualify_white_kingside_castling(&mut self) {
        self.flags.set_bit(4, false);
    }

    pub fn disqualify_black_castling(&mut self) {
        self.flags.0 &= !(0b00110);
    }

    pub fn disqualify_black_queenside_castling(&mut self) {
        self.flags.set_bit(1, false);
    }

    pub fn disqualify_black_kingside_castling(&mut self) {
        self.flags.set_bit(2, false);
    }

    pub fn can_white_castle_kingside(&self) -> bool {
        self.flags.get_bit(4)
    }

    pub fn can_white_castle_queenside(&self) -> bool {
        self.flags.get_bit(3)
    }

    pub fn can_black_castle_kingside(&self) -> bool {
        self.flags.get_bit(2)
    }

    pub fn can_black_castle_queenside(&self) -> bool {
        self.flags.get_bit(1)
    }

    pub fn get_halfmove_clock(&self) -> u16 {
        ((self.flags.0 >> 16) & (0xFFFF_u64)) as u16
    }

    pub fn get_fullmove_counter(&self) -> u16 {
        ((self.flags.0 >> 32) & (0xFFFF_u64)) as u16
    }

    pub fn reset_halfmove_clock(&mut self) {
        self.flags.0 &= !(0xFFFF_u64 << 16);
    }

    pub fn increment_halfmove_clock(&mut self) {
        let clock = self.get_halfmove_clock() + 1;
        self.set_halfmove_clock(clock);
    }

    pub fn set_halfmove_clock(&mut self, new: u16) {
        self.reset_halfmove_clock();
        self.flags.0 |= (new as u64) << 16;
    }

    pub fn reset_fullmove_counter(&mut self) {
        self.flags.0 &= !(0xFFFF_u64 << 32);
    }

    pub fn increment_fullmove_counter(&mut self) {
        let counter = self.get_fullmove_counter() + 1;
        self.set_fullmove_counter(counter);
    }

    pub fn set_fullmove_counter(&mut self, new: u16) {
        self.reset_fullmove_counter();
        self.flags.0 |= (new as u64) << 32;
    }

    #[inline]
    pub fn set_en_passant(&mut self, file: u8) {
        self.reset_en_passant();
        self.flags.0 |= (file as u64) << 8;
    }

    #[inline]
    pub fn reset_en_passant(&mut self) {
        self.flags.0 &= !(0xFF << 8);
    }

    #[inline]
    pub fn get_en_passant_file(&self) -> u8 {
        (self.flags.0 >> 8) as u8
    }

    pub fn is_in_check(&self, white: bool) -> bool {
        // TODO Slow approach?

        let white_occupancy = self.bitboard.white_coverage();
        let black_occupancy = self.bitboard.black_coverage();
        let total_occupancy = white_occupancy | black_occupancy;

        let king_pos;
        let enemy_rooklike;
        let enemy_knights;
        let enemy_bishoplike;
        if white {
            king_pos = self.bitboard.white_king;
            enemy_rooklike = self.bitboard.black_rooklike;
            enemy_knights = self.bitboard.black_knights;
            enemy_bishoplike = self.bitboard.black_bishoplike;
        } else {
            king_pos = self.bitboard.black_king;
            enemy_rooklike = self.bitboard.white_rooklike;
            enemy_knights = self.bitboard.white_knights;
            enemy_bishoplike = self.bitboard.white_bishoplike;
        }

        if is_attacked_by_knight(king_pos, enemy_knights) {
            return true;
        }
        if is_attacked_by_rooklike(king_pos, enemy_rooklike, total_occupancy) {
            return true;
        }
        if is_attacked_by_bishoplike(king_pos, enemy_bishoplike, total_occupancy) {
            return true;
        }

        if white {
            if is_attacked_by_black_pawns(king_pos, self.bitboard.black_pawns) {
                return true;
            }
        } else if is_attacked_by_white_pawns(king_pos, self.bitboard.white_pawns) {
            return true;
        }

        false
    }

    pub fn is_square_attacked_by_white(&self, position: u64) -> bool {
        let white_occupancy = self.bitboard.white_coverage();
        let black_occupancy = self.bitboard.black_coverage();
        let total_occupancy = white_occupancy | black_occupancy;

        let enemy_rooklike = self.bitboard.white_rooklike;
        let enemy_knights = self.bitboard.white_knights;
        let enemy_bishoplike = self.bitboard.white_bishoplike;

        if is_attacked_by_knight(position, enemy_knights) {
            return true;
        }
        if is_attacked_by_rooklike(position, enemy_rooklike, total_occupancy) {
            return true;
        }
        if is_attacked_by_bishoplike(position, enemy_bishoplike, total_occupancy) {
            return true;
        }

        if is_attacked_by_white_pawns(position, self.bitboard.white_pawns) {
            return true;
        }

        if is_attacked_by_king(position, self.bitboard.white_king) {
            return true;
        }

        false
    }

    pub fn is_square_attacked_by_black(&self, position: u64) -> bool {
        let white_occupancy = self.bitboard.white_coverage();
        let black_occupancy = self.bitboard.black_coverage();
        let total_occupancy = white_occupancy | black_occupancy;

        let enemy_rooklike = self.bitboard.black_rooklike;
        let enemy_knights = self.bitboard.black_knights;
        let enemy_bishoplike = self.bitboard.black_bishoplike;

        if is_attacked_by_knight(position, enemy_knights) {
            return true;
        }
        if is_attacked_by_rooklike(position, enemy_rooklike, total_occupancy) {
            return true;
        }
        if is_attacked_by_bishoplike(position, enemy_bishoplike, total_occupancy) {
            return true;
        }

        if is_attacked_by_black_pawns(position, self.bitboard.black_pawns) {
            return true;
        }

        if is_attacked_by_king(position, self.bitboard.black_king) {
            return true;
        }

        false
    }

    pub fn make_move(&self, mov: &Move) -> Board {
        let mut new = *self;
        new.make_move_in_place(mov);
        new
    }

    pub fn make_move_in_place(&mut self, mov: &Move) {
        let white = self.white_to_move();
        if !white {
            // TODO huh??
            self.increment_fullmove_counter();
        }
        self.reset_en_passant();
        self.toggle_side_to_move();

        let from_tzcnt = mov.from();
        let to_tzcnt = mov.to();
        let from: u64 = 1 << from_tzcnt;
        let to: u64 = 1 << to_tzcnt;
        let from_piecelist_i = self.slow_get_piecelist_index_of_pos(from_tzcnt);
        let from_kind = self.piece_kinds[from_piecelist_i];
        let toggle_bits = from | to;

        let flags = mov.flags_nibble();
        let is_promotion = (flags & 0b1000) != 0;
        let is_capture = (flags & 0b0100) != 0;

        if is_capture || from_kind.is_pawn() {
            self.reset_halfmove_clock()
        } else {
            self.increment_halfmove_clock();
        }
        self.castling_maintenance(from_tzcnt);

        if is_promotion {
            if white {
                self.bitboard.white_pawns ^= from;
                match flags & 0b11 {
                    0b00 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::WhiteKnight;
                        self.bitboard.white_knights |= to;
                    }
                    0b01 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::WhiteBishop;
                        self.bitboard.white_bishoplike |= to;
                    }
                    0b10 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::WhiteRook;
                        self.bitboard.white_rooklike |= to;
                    }
                    0b11 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::WhiteQueen;
                        self.bitboard.white_bishoplike |= to;
                        self.bitboard.white_rooklike |= to;
                    }
                    _ => unreachable!(),
                }
                if is_capture {
                    self.delete_from_piecelist(to_tzcnt);
                    self.bitboard.unset_black_piece(to);
                }
            } else {
                self.bitboard.black_pawns ^= from;
                match flags & 0b11 {
                    0b00 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::BlackKnight;
                        self.bitboard.black_knights |= to;
                    }
                    0b01 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::BlackBishop;
                        self.bitboard.black_bishoplike |= to;
                    }
                    0b10 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::BlackRook;
                        self.bitboard.black_rooklike |= to;
                    }
                    0b11 => {
                        self.piece_kinds[from_piecelist_i] = PieceKind::BlackQueen;
                        self.bitboard.black_bishoplike |= to;
                        self.bitboard.black_rooklike |= to;
                    }
                    _ => unreachable!(),
                }
                if is_capture {
                    self.delete_from_piecelist(to_tzcnt);
                    self.bitboard.unset_white_piece(to);
                }
            }
            self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;

            return;
        }

        match flags & 0b111 {
            0b000 => {
                // "Normal" move
                if white {
                    match from_kind {
                        WhiteQueen => {
                            self.bitboard.white_rooklike ^= toggle_bits;
                            self.bitboard.white_bishoplike ^= toggle_bits;
                        }
                        WhiteKing => self.bitboard.white_king ^= toggle_bits,
                        WhiteRook => self.bitboard.white_rooklike ^= toggle_bits,
                        WhiteBishop => self.bitboard.white_bishoplike ^= toggle_bits,
                        WhiteKnight => self.bitboard.white_knights ^= toggle_bits,
                        WhitePawn => self.bitboard.white_pawns ^= toggle_bits,
                        _ => unreachable!(),
                    }
                } else {
                    match from_kind {
                        BlackQueen => {
                            self.bitboard.black_rooklike ^= toggle_bits;
                            self.bitboard.black_bishoplike ^= toggle_bits;
                        }
                        BlackKing => self.bitboard.black_king ^= toggle_bits,
                        BlackRook => self.bitboard.black_rooklike ^= toggle_bits,
                        BlackBishop => self.bitboard.black_bishoplike ^= toggle_bits,
                        BlackKnight => self.bitboard.black_knights ^= toggle_bits,
                        BlackPawn => self.bitboard.black_pawns ^= toggle_bits,
                        _ => unreachable!(),
                    }
                }
                self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;
            }
            0b001 => {
                // Double pawn push
                if white {
                    self.bitboard.white_pawns ^= toggle_bits;
                    let ep_file = (pos_to_file_index(from << 8) + 1) as u8;
                    self.set_en_passant(ep_file);
                } else {
                    self.bitboard.black_pawns ^= toggle_bits;
                    let ep_file = (pos_to_file_index(from >> 8) + 1) as u8;
                    self.set_en_passant(ep_file);
                }
                self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;
            }
            0b010 => {
                // Kingside castle
                if white {
                    self.bitboard.white_king = 1 << 6;
                    self.bitboard.white_rooklike ^= (1 << 7) | (1 << 5);
                    let piecelist_rook_i = self.slow_get_piecelist_index_of_pos(7);
                    self.piece_positions_tzcnt[piecelist_rook_i] = 5;
                } else {
                    self.bitboard.black_king = 1 << 62;
                    self.bitboard.black_rooklike ^= (1 << 61) | (1 << 63);
                    let piecelist_rook_i = self.slow_get_piecelist_index_of_pos(63);
                    self.piece_positions_tzcnt[piecelist_rook_i] = 61;
                }
                self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;
            }
            0b011 => {
                // Queenside castle
                if white {
                    self.bitboard.white_king = 1 << 2;
                    self.bitboard.white_rooklike ^= 1 | (1 << 3);
                    let piecelist_rook_i = self.slow_get_piecelist_index_of_pos(0);
                    self.piece_positions_tzcnt[piecelist_rook_i] = 3;
                } else {
                    self.bitboard.black_king = 1 << 58;
                    self.bitboard.black_rooklike ^= (1 << 56) | (1 << 59);
                    let piecelist_rook_i = self.slow_get_piecelist_index_of_pos(56);
                    self.piece_positions_tzcnt[piecelist_rook_i] = 59;
                }
                self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;
            }
            0b100 => {
                // "Normal" capture
                if white {
                    match from_kind {
                        WhiteQueen => {
                            self.bitboard.white_rooklike ^= toggle_bits;
                            self.bitboard.white_bishoplike ^= toggle_bits;
                        }
                        WhiteKing => self.bitboard.white_king ^= toggle_bits,
                        WhiteRook => self.bitboard.white_rooklike ^= toggle_bits,
                        WhiteBishop => self.bitboard.white_bishoplike ^= toggle_bits,
                        WhiteKnight => self.bitboard.white_knights ^= toggle_bits,
                        WhitePawn => self.bitboard.white_pawns ^= toggle_bits,
                        _ => unreachable!(),
                    }
                } else {
                    match from_kind {
                        BlackQueen => {
                            self.bitboard.black_rooklike ^= toggle_bits;
                            self.bitboard.black_bishoplike ^= toggle_bits;
                        }
                        BlackKing => self.bitboard.black_king ^= toggle_bits,
                        BlackRook => self.bitboard.black_rooklike ^= toggle_bits,
                        BlackBishop => self.bitboard.black_bishoplike ^= toggle_bits,
                        BlackKnight => self.bitboard.black_knights ^= toggle_bits,
                        BlackPawn => self.bitboard.black_pawns ^= toggle_bits,
                        _ => unreachable!(),
                    }
                }

                self.delete_from_piecelist(to_tzcnt);
                if white {
                    self.bitboard.unset_black_piece(to);
                } else {
                    self.bitboard.unset_white_piece(to);
                }

                self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;
            }
            0b101 => {
                // Ep capture
                let opponent_square;
                if white {
                    opponent_square = to >> 8;
                    self.bitboard.white_pawns ^= toggle_bits;
                    self.bitboard.black_pawns ^= opponent_square;
                } else {
                    opponent_square = to << 8;
                    self.bitboard.black_pawns ^= toggle_bits;
                    self.bitboard.white_pawns ^= opponent_square;
                }
                self.piece_positions_tzcnt[from_piecelist_i] = to_tzcnt;
                self.delete_from_piecelist(opponent_square.tzcnt() as u8);
            }
            _ => unreachable!(),
        }
    }

    pub fn slow_get_piecelist_index_of_pos(&self, pos_tzcnt: u8) -> usize {
        let (piece_index, _) = self
            .piece_positions_tzcnt
            .iter()
            .enumerate()
            .find(|(_, p)| **p == pos_tzcnt)
            .unwrap();
        piece_index
    }

    pub fn delete_from_piecelist(&mut self, capture_pos_tzcnt: u8) {
        let piece_positions_tzcnt = &mut self.piece_positions_tzcnt;
        for (i, p) in piece_positions_tzcnt.iter().enumerate() {
            if *p == capture_pos_tzcnt {
                piece_positions_tzcnt[i] = TZCNT_U64_ZEROS;
                self.piece_kinds[i] = EmptySquare;
                break;
            }
        }
    }

    #[inline]
    fn castling_maintenance(&mut self, pos_from_tzcnt: u8) {
        match pos_from_tzcnt {
            4 => self.disqualify_white_castling(),
            60 => self.disqualify_black_castling(),
            0 => self.disqualify_black_queenside_castling(),
            7 => self.disqualify_white_kingside_castling(),
            56 => self.disqualify_black_queenside_castling(),
            63 => self.disqualify_black_kingside_castling(),
            _ => {}
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut ps = [(PieceKind::EmptySquare, 0u64); 32];

        for i in 0usize..8 {
            let iu = i as u64;
            ps[i] = (WhitePawn, ROW_2 & (FILE_A << iu));
            ps[i + 8] = (BlackPawn, ROW_7 & (FILE_A << iu));
        }

        ps[16] = (WhiteKing, ROW_1 & FILE_E);
        ps[17] = (WhiteQueen, ROW_1 & FILE_D);
        ps[18] = (WhiteBishop, ROW_1 & FILE_C);
        ps[19] = (WhiteBishop, ROW_1 & FILE_F);
        ps[20] = (WhiteKnight, ROW_1 & FILE_B);
        ps[21] = (WhiteKnight, ROW_1 & FILE_G);
        ps[22] = (WhiteRook, ROW_1 & FILE_A);
        ps[23] = (WhiteRook, ROW_1 & FILE_H);

        ps[24] = (BlackKing, ROW_8 & FILE_E);
        ps[25] = (BlackQueen, ROW_8 & FILE_D);
        ps[26] = (BlackBishop, ROW_8 & FILE_C);
        ps[27] = (BlackBishop, ROW_8 & FILE_F);
        ps[28] = (BlackKnight, ROW_8 & FILE_B);
        ps[29] = (BlackKnight, ROW_8 & FILE_G);
        ps[30] = (BlackRook, ROW_8 & FILE_A);
        ps[31] = (BlackRook, ROW_8 & FILE_H);

        let mut piece_positions = [TZCNT_U64_ZEROS; 32];
        let mut piece_kinds = [PieceKind::EmptySquare; 32];

        for i in 0..32 {
            piece_kinds[i] = ps[i].0;
            piece_positions[i] = ps[i].1.tzcnt() as u8;
        }

        let white_queen = ROW_1 & FILE_D;
        let black_queen = ROW_8 & FILE_D;
        Board::new(
            BitBoard {
                white_pawns: ROW_2,
                white_king: ROW_1 & FILE_E,
                white_rooklike: (ROW_1 & (FILE_A | FILE_H)) | white_queen,
                white_bishoplike: (ROW_1 & (FILE_C | FILE_F)) | white_queen,
                white_knights: ROW_1 & (FILE_B | FILE_G),
                black_pawns: ROW_7,
                black_king: ROW_8 & FILE_E,
                black_rooklike: (ROW_8 & (FILE_A | FILE_H)) | black_queen,
                black_bishoplike: (ROW_8 & (FILE_C | FILE_F)) | black_queen,
                black_knights: ROW_8 & (FILE_B | FILE_G),
            },
            piece_positions,
            piece_kinds,
            0,
            1,
            0,
            true,
            0b00001111,
        )
    }
}

impl Flags {
    #[inline]
    const fn get_bit(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    #[inline]
    fn set_bit(&mut self, i: usize, value: bool) {
        self.0 &= !(1 << i);
        if value {
            self.0 |= 1 << i;
        }
    }

    #[inline]
    fn toggle_bit(&mut self, i: usize) {
        self.set_bit(i, !self.get_bit(i));
    }
}

fn is_attacked_by_knight(position: u64, enemy_knights: u64) -> bool {
    let knight_squares_to_check = get_knight_target_mask(position);
    intersects(knight_squares_to_check, enemy_knights)
}

fn is_attacked_by_rooklike(position: u64, enemy_rooklike: u64, total_occupancy: u64) -> bool {
    if !intersects(position, FILE_H) {
        let mut target_pos = position << 1;
        loop {
            if intersects(target_pos, enemy_rooklike) {
                return true;
            }

            if intersects(target_pos, total_occupancy | FILE_H) {
                break;
            }
            target_pos <<= 1;
        }
    }
    if !intersects(position, FILE_A) {
        let mut target_pos = position >> 1;
        loop {
            if intersects(target_pos, enemy_rooklike) {
                return true;
            }

            if intersects(target_pos, total_occupancy | FILE_A) {
                break;
            }
            target_pos >>= 1;
        }
    }
    if !intersects(position, ROW_8) {
        let mut target_pos = position << 8;
        loop {
            if intersects(target_pos, enemy_rooklike) {
                return true;
            }

            if intersects(target_pos, total_occupancy | ROW_8) {
                break;
            }
            target_pos <<= 8;
        }
    }
    if !intersects(position, ROW_1) {
        let mut target_pos = position >> 8;
        loop {
            if intersects(target_pos, enemy_rooklike) {
                return true;
            }

            if intersects(target_pos, total_occupancy | ROW_1) {
                break;
            }
            target_pos >>= 8;
        }
    }
    false
}

fn is_attacked_by_bishoplike(position: u64, enemy_bishoplike: u64, total_occupancy: u64) -> bool {
    let top = intersects(position, ROW_8);
    let bottom = intersects(position, ROW_1);
    let right = intersects(position, FILE_H);
    let left = intersects(position, FILE_A);

    if !top && !right {
        let mut target_pos = position << 9;
        loop {
            if intersects(target_pos, enemy_bishoplike) {
                return true;
            }
            if intersects(target_pos, total_occupancy | ROW_8 | FILE_H) {
                break;
            }
            target_pos <<= 9;
        }
    }

    if !top && !left {
        let mut target_pos = position << 7;
        loop {
            if intersects(target_pos, enemy_bishoplike) {
                return true;
            }
            if intersects(target_pos, total_occupancy | ROW_8 | FILE_A) {
                break;
            }
            target_pos <<= 7;
        }
    }

    if !bottom && !right {
        let mut target_pos = position >> 7;
        loop {
            if intersects(target_pos, enemy_bishoplike) {
                return true;
            }
            if intersects(target_pos, total_occupancy | ROW_1 | FILE_H) {
                break;
            }
            target_pos >>= 7;
        }
    }

    if !bottom && !left {
        let mut target_pos = position >> 9;
        loop {
            if intersects(target_pos, enemy_bishoplike) {
                return true;
            }
            if intersects(target_pos, total_occupancy | ROW_1 | FILE_A) {
                break;
            }
            target_pos >>= 9;
        }
    }
    false
}

fn is_attacked_by_white_pawns(position: u64, white_pawns: u64) -> bool {
    let targets = ((white_pawns << 9) & !FILE_A) | ((white_pawns << 7) & !FILE_H);
    intersects(position, targets)
}

fn is_attacked_by_black_pawns(position: u64, black_pawns: u64) -> bool {
    let targets = ((black_pawns >> 9) & !FILE_H) | ((black_pawns >> 7) & !FILE_A);
    intersects(position, targets)
}

fn is_attacked_by_king(position: u64, king_pos: u64) -> bool {
    let king_attack_mask = KING_ATTACK_MASK[king_pos.tzcnt() as usize];
    intersects(position, king_attack_mask)
}
