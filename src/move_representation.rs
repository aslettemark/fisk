use std::fmt;

use crate::constants::SQUARE_NAME;

/// Bit overview:
/// 0-5: from (tzcnt)
/// 6-11: to (tzcnt)
/// Special bits as https://www.chessprogramming.org/Encoding_Moves#From-To_Based
/// 12: special 0
/// 13: special 1
/// 14: capture
/// 15: promotion
#[derive(Copy, Clone, Debug)]
pub struct Move {
    repr: u16,
}
// TODO from_kind

impl Move {
    #[inline]
    pub fn new(from: u8, to: u8, capture: bool, special_bits: u8) -> Move {
        let capture_bit = if capture { 0x4000u16 } else { 0 };
        Move {
            repr: (from as u16) | ((to as u16) << 6) | capture_bit | ((special_bits as u16) << 12),
        }
    }

    pub fn from(&self) -> u8 {
        (self.repr & 0x3F) as u8
    }

    pub fn to(&self) -> u8 {
        ((self.repr >> 6) & 0x3F) as u8
    }

    pub fn flags_nibble(&self) -> u8 {
        ((self.repr >> 12) & 0xF) as u8
    }

    pub fn is_promotion(&self) -> bool {
        (self.repr & (1 << 15)) != 0
    }

    pub fn is_capture(&self) -> bool {
        (self.repr & (1 << 14)) != 0
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let capture = self.is_capture();
        let promotion = self.is_promotion();
        if promotion {
            let promo = match self.flags_nibble() & 0b11 {
                0 => 'K',
                1 => 'B',
                2 => 'R',
                3 => 'Q',
                _ => unreachable!(),
            };
            if capture {
                write!(
                    f,
                    "{}x{}={}",
                    SQUARE_NAME[self.from() as usize],
                    SQUARE_NAME[self.to() as usize],
                    promo
                )
            } else {
                write!(
                    f,
                    "{}{}={}",
                    SQUARE_NAME[self.from() as usize],
                    SQUARE_NAME[self.to() as usize],
                    promo
                )
            }
        } else if capture {
            write!(
                f,
                "{}x{}",
                SQUARE_NAME[self.from() as usize],
                SQUARE_NAME[self.to() as usize]
            )
        } else {
            write!(
                f,
                "{}{}",
                SQUARE_NAME[self.from() as usize],
                SQUARE_NAME[self.to() as usize]
            )
        }
    }
}
