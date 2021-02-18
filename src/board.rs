use bitintr::*;

use crate::board::Color::{Black, Empty, White};
use crate::board::PieceKind::*;
use crate::constants::*;

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

    pub fn toggle_white_to_move(&mut self) {
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
        let knight_squares_to_check = get_knight_target_mask(king_pos);

        if intersects(knight_squares_to_check, enemy_knights) {
            return true;
        }
        if is_in_rooklike_check(king_pos, enemy_rooklike, total_occupancy) {
            return true;
        }
        if is_in_bishoplike_check(king_pos, enemy_bishoplike, total_occupancy) {
            return true;
        }

        if white {
            let black_pawns = self.bitboard.black_pawns;
            let targets = ((black_pawns >> 9) & !FILE_H) | ((black_pawns >> 7) & !FILE_A);
            if intersects(king_pos, targets) {
                return true;
            }
        } else {
            let white_pawns = self.bitboard.white_pawns;
            let targets = ((white_pawns << 9) & !FILE_A) | ((white_pawns << 7) & !FILE_H);
            if intersects(king_pos, targets) {
                return true;
            }
        }

        false
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

fn is_in_rooklike_check(king_pos: u64, enemy_rooklike: u64, total_occupancy: u64) -> bool {
    if !intersects(king_pos, FILE_H) {
        let mut target_pos = king_pos << 1;
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
    if !intersects(king_pos, FILE_A) {
        let mut target_pos = king_pos >> 1;
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
    if !intersects(king_pos, ROW_8) {
        let mut target_pos = king_pos << 8;
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
    if !intersects(king_pos, ROW_1) {
        let mut target_pos = king_pos >> 8;
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

fn is_in_bishoplike_check(king_pos: u64, enemy_bishoplike: u64, total_occupancy: u64) -> bool {
    let top = intersects(king_pos, ROW_8);
    let bottom = intersects(king_pos, ROW_1);
    let right = intersects(king_pos, FILE_H);
    let left = intersects(king_pos, FILE_A);

    if !top && !right {
        let mut target_pos = king_pos << 9;
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
        let mut target_pos = king_pos << 7;
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
        let mut target_pos = king_pos >> 7;
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
        let mut target_pos = king_pos >> 9;
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
