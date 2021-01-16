use crate::constants::*;

/// Bit overview:
/// 0: white to move
/// 1: black queenside castling availability
/// 2: black kingside castling
/// 3: white queenside castling
/// 4: white kingside castling
/// 5-7: unused
/// 8-15: reserved (en passant file, one-hot/popcnt encoded)
/// 16-31: halfmove_clock
/// 32-47: fullmove_counter
#[derive(Copy, Clone, Debug)]
struct Flags(u64);

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub en_passant: u64,
    pub bitboard: BitBoard,
    pub piece_positions: [u64; 32],
    pub piece_kinds: [u8; 32],
    flags: Flags,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard {
    // Little-Endian Rank-File (LERF) https://www.chessprogramming.org/Square_Mapping_Considerations
    // bit 0 is a1, bit 7 is h1, bit 63 is h8
    // TODO: Consider having array(s) of u64 to reduce branching when checking side, ie pawns[white]
    pub white_pawns: u64,
    pub white_queen: u64,
    pub white_king: u64,
    pub white_rooks: u64,
    pub white_bishops: u64,
    pub white_knights: u64,

    pub black_pawns: u64,
    pub black_queen: u64,
    pub black_king: u64,
    pub black_rooks: u64,
    pub black_bishops: u64,
    pub black_knights: u64,
}

impl BitBoard {
    #[inline]
    pub const fn empty() -> BitBoard {
        BitBoard {
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
        }
    }

    #[inline]
    pub fn white_coverage(&self) -> u64 {
        self.white_king
            | self.white_queen
            | self.white_knights
            | self.white_rooks
            | self.white_bishops
            | self.white_pawns
    }

    #[inline]
    pub fn black_coverage(&self) -> u64 {
        self.black_king
            | self.black_queen
            | self.black_knights
            | self.black_rooks
            | self.black_bishops
            | self.black_pawns
    }

    #[inline]
    pub fn coverage(&self) -> u64 {
        // Might be a silly premature opt to not trust the codegen for doing white_cover | black_cover
        self.white_king
            | self.white_queen
            | self.white_knights
            | self.white_rooks
            | self.white_bishops
            | self.white_pawns
            | self.black_king
            | self.black_queen
            | self.black_knights
            | self.black_rooks
            | self.black_bishops
            | self.black_pawns
    }

    #[inline]
    pub fn unset_white_piece(&mut self, capture_pos: u64) {
        self.white_pawns &= !capture_pos;
        self.white_bishops &= !capture_pos;
        self.white_rooks &= !capture_pos;
        self.white_knights &= !capture_pos;
        self.white_queen &= !capture_pos;
        self.white_king &= !capture_pos;
    }

    #[inline]
    pub fn unset_black_piece(&mut self, capture_pos: u64) {
        self.black_pawns &= !capture_pos;
        self.black_bishops &= !capture_pos;
        self.black_rooks &= !capture_pos;
        self.black_knights &= !capture_pos;
        self.black_queen &= !capture_pos;
        self.black_king &= !capture_pos;
    }
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
    Empty,
}

pub fn is_kind_white(kind: u8) -> bool {
    if kind == EMPTY_SQUARE {
        return false;
    }
    if (kind & BLACK_BIT) != 0 {
        return false;
    }
    true
}

impl Board {
    // Create default chess board
    pub fn new(
        bitboard: BitBoard,
        piece_positions: [u64; 32],
        piece_kinds: [u8; 32],
        halfmove_clock: u16,
        fullmove_counter: u16,
        en_passant: u64,
        white_to_move: bool,
        castling_availability: u8, // 0b0000KQkq
    ) -> Board {
        let mut board = Board {
            en_passant,
            bitboard,
            piece_positions,
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

        board
    }

    pub fn print(&self) {
        println!(" A B C D E F G H");

        for (i, row) in ROWS.iter().enumerate() {
            print!("{}", 7 - i + 1);

            for file in &FILES {
                let pos = row & file;
                let c = self.piece_representation(self.kind_at(pos));

                print!("{} ", c);
            }

            println!();
        }

        println!(" A B C D E F G H");

        /*for p in self.pieces.iter() {
            println!("{:#066b}: {}", p.position, self.piece_representation(p.kind));
        }*/
    }

    fn piece_representation(&self, kind: u8) -> char {
        match kind {
            WHITE_PAWN => '♙',
            WHITE_BISHOP => '♗',
            WHITE_KNIGHT => '♘',
            WHITE_ROOK => '♖',
            WHITE_QUEEN => '♕',
            WHITE_KING => '♔',
            BLACK_PAWN => '♟',
            BLACK_BISHOP => '♝',
            BLACK_KNIGHT => '♞',
            BLACK_ROOK => '♜',
            BLACK_QUEEN => '♛',
            BLACK_KING => '♚',
            _ => ' ',
        }
    }

    pub fn kind_at(&self, pos: u64) -> u8 {
        //TODO: There should be a faster way to do this
        // Idea 1: Try merging bitboards, binary search? (possibly large overhead)
        // Idea 2: profile, sort cases, special case for empty?
        let b = &self.bitboard;
        if pos & b.white_pawns != 0 {
            return WHITE_PAWN;
        }
        if pos & b.black_pawns != 0 {
            return BLACK_PAWN;
        }

        if pos & b.white_bishops != 0 {
            return WHITE_BISHOP;
        }
        if pos & b.white_knights != 0 {
            return WHITE_KNIGHT;
        }
        if pos & b.white_rooks != 0 {
            return WHITE_ROOK;
        }
        if pos & b.black_bishops != 0 {
            return BLACK_BISHOP;
        }
        if pos & b.black_knights != 0 {
            return BLACK_KNIGHT;
        }
        if pos & b.black_rooks != 0 {
            return BLACK_ROOK;
        }

        if pos & b.white_queen != 0 {
            return WHITE_QUEEN;
        }
        if pos & b.white_king != 0 {
            return WHITE_KING;
        }

        if pos & b.black_queen != 0 {
            return BLACK_QUEEN;
        }
        if pos & b.black_king != 0 {
            return BLACK_KING;
        }

        EMPTY_SQUARE
    }

    pub fn white_to_move(self) -> bool {
        self.flags.get_bit(0)
    }

    pub fn toggle_white_to_move(&mut self) {
        self.flags.toggle_bit(0);
    }

    pub fn disqualify_white_castling(&mut self) {
        self.flags.0 &= !(0b11000);
    }

    pub fn disqualify_black_castling(&mut self) {
        self.flags.0 &= !(0b00110);
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
}

impl Default for Board {
    fn default() -> Self {
        let mut ps = [(0u8, 0u64); 32];

        for i in 0usize..8 {
            let iu = i as u64;
            ps[i] = (WHITE_PAWN, ROW_2 & (FILE_A << iu));
            ps[i + 8] = (BLACK_PAWN, ROW_7 & (FILE_A << iu));
        }

        ps[16] = (WHITE_KING, ROW_1 & FILE_E);
        ps[17] = (WHITE_QUEEN, ROW_1 & FILE_D);
        ps[18] = (WHITE_BISHOP, ROW_1 & FILE_C);
        ps[19] = (WHITE_BISHOP, ROW_1 & FILE_F);
        ps[20] = (WHITE_KNIGHT, ROW_1 & FILE_B);
        ps[21] = (WHITE_KNIGHT, ROW_1 & FILE_G);
        ps[22] = (WHITE_ROOK, ROW_1 & FILE_A);
        ps[23] = (WHITE_ROOK, ROW_1 & FILE_H);

        ps[24] = (BLACK_KING, ROW_8 & FILE_E);
        ps[25] = (BLACK_QUEEN, ROW_8 & FILE_D);
        ps[26] = (BLACK_BISHOP, ROW_8 & FILE_C);
        ps[27] = (BLACK_BISHOP, ROW_8 & FILE_F);
        ps[28] = (BLACK_KNIGHT, ROW_8 & FILE_B);
        ps[29] = (BLACK_KNIGHT, ROW_8 & FILE_G);
        ps[30] = (BLACK_ROOK, ROW_8 & FILE_A);
        ps[31] = (BLACK_ROOK, ROW_8 & FILE_H);

        let piece_positions = ps.map(|(_, p)| p);
        let piece_kinds = ps.map(|(k, _)| k);

        Board::new(
            BitBoard {
                white_pawns: ROW_2,
                white_queen: ROW_1 & FILE_D,
                white_king: ROW_1 & FILE_E,
                white_rooks: ROW_1 & (FILE_A | FILE_H),
                white_bishops: ROW_1 & (FILE_C | FILE_F),
                white_knights: ROW_1 & (FILE_B | FILE_G),
                black_pawns: ROW_7,
                black_queen: ROW_8 & FILE_D,
                black_king: ROW_8 & FILE_E,
                black_rooks: ROW_8 & (FILE_A | FILE_H),
                black_bishops: ROW_8 & (FILE_C | FILE_F),
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
    fn get_bit(&self, i: usize) -> bool {
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
