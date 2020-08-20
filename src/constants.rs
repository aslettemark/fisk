use std::cmp::{max, min};

pub const ROW_1: u64 = 0xFF;
pub const ROW_2: u64 = ROW_1 << 8;
pub const ROW_3: u64 = ROW_2 << 8;
pub const ROW_4: u64 = ROW_3 << 8;
pub const ROW_5: u64 = ROW_4 << 8;
pub const ROW_6: u64 = ROW_5 << 8;
pub const ROW_7: u64 = ROW_6 << 8;
pub const ROW_8: u64 = ROW_7 << 8;
pub const ROWS: [u64; 8] = [ROW_1, ROW_2, ROW_3, ROW_4, ROW_5, ROW_6, ROW_7, ROW_8];

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;
pub const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

pub const EMPTY_SQUARE: u8 = 0;

pub const WHITE_PAWN: u8 = 1;
pub const WHITE_BISHOP: u8 = 1 << 1;
pub const WHITE_KNIGHT: u8 = 1 << 2;
pub const WHITE_ROOK: u8 = 1 << 3;
pub const WHITE_QUEEN: u8 = 1 << 4;
pub const WHITE_KING: u8 = 1 << 5;

pub const BLACK_BIT: u8 = 1 << 7;

pub const BLACK_PAWN: u8 = WHITE_PAWN | BLACK_BIT;
pub const BLACK_BISHOP: u8 = WHITE_BISHOP | BLACK_BIT;
pub const BLACK_KNIGHT: u8 = WHITE_KNIGHT | BLACK_BIT;
pub const BLACK_ROOK: u8 = WHITE_ROOK | BLACK_BIT;
pub const BLACK_QUEEN: u8 = WHITE_QUEEN | BLACK_BIT;
pub const BLACK_KING: u8 = WHITE_KING | BLACK_BIT;

lazy_static! {
    pub static ref KNIGHT_ATTACK: [[u64; 8]; 64] = generate_knight_attacks();
    pub static ref FILE_ATTACK: [u64; 64] = generate_file_attacks();
    pub static ref RANK_ATTACK: [u64; 64] = generate_rank_attacks();
    pub static ref KING_ATTACK: [[u64; 8]; 64] = generate_king_attacks();
}

fn generate_king_attacks() -> [[u64; 8]; 64] {
    let mut attacks: [[u64; 8]; 64] = [[0; 8]; 64];
    for (i, attack) in attacks.iter_mut().enumerate() {
        *attack = get_king_attacks(i as u64);
    }

    attacks
}

fn get_king_attacks(trailing: u64) -> [u64; 8] {
    let file_index = (trailing % 8) as isize;
    let mut file_mask: u64 = 0;
    for i in max(file_index - 1, 0)..min(file_index + 1, 7) + 1 {
        file_mask |= FILES[i as usize];
    }

    let rank_index = (trailing / 8) as isize;
    let mut rank_mask: u64 = 0;
    for i in max(rank_index - 1, 0)..min(rank_index + 1, 7) + 1 {
        rank_mask |= ROWS[i as usize];
    }

    let mask = file_mask & rank_mask;
    let attack_offsets: [isize; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
    let mut attacks: [u64; 8] = [0; 8];
    for (i, offset) in attack_offsets.iter().enumerate() {
        let bit = trailing as isize + offset;
        if bit >= 0 && bit < 64 {
            attacks[i] = (1 << (bit as u64)) & mask;
        }
    }
    attacks
}

fn generate_file_attacks() -> [u64; 64] {
    let mut attacks = [0 as u64; 64];
    for (i, attack) in attacks.iter_mut().enumerate() {
        let file_index = i % 8;
        *attack = FILES[file_index] & !(1 << i as u64); //zero out the occupied position
    }
    attacks
}

fn generate_rank_attacks() -> [u64; 64] {
    let mut attacks = [0 as u64; 64];
    for (i, attack) in attacks.iter_mut().enumerate() {
        let rank_index = i / 8;
        *attack = ROWS[rank_index] & !(1 << i as u64); //zero out the occupied position
    }
    attacks
}

fn generate_knight_attacks() -> [[u64; 8]; 64] {
    let mut attacks: [[u64; 8]; 64] = [[0; 8]; 64];
    for (i, attack) in attacks.iter_mut().enumerate() {
        *attack = get_knight_attacks(i as u64);
    }

    attacks
}

fn get_knight_attacks(trailing: u64) -> [u64; 8] {
    // https://www.chessprogramming.org/Knight_Pattern
    let file_index = (trailing % 8) as isize;
    let mut file_mask: u64 = 0;
    for i in max(file_index - 2, 0)..min(file_index + 2, 7) + 1 {
        file_mask |= FILES[i as usize];
    }

    let rank_index = (trailing / 8) as isize;
    let mut rank_mask: u64 = 0;
    for i in max(rank_index - 2, 0)..min(rank_index + 2, 7) + 1 {
        rank_mask |= ROWS[i as usize];
    }

    let mask = file_mask & rank_mask;
    let attack_offsets: [isize; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let mut attacks: [u64; 8] = [0; 8];
    for (i, offset) in attack_offsets.iter().enumerate() {
        let bit = trailing as isize + offset;
        if bit >= 0 && bit < 64 {
            attacks[i] = (1 << (bit as u64)) & mask;
        }
    }
    attacks
}

#[cfg(test)]
mod tests {
    use bitintr::*;

    use super::*;

    fn test_table_n_attacks(trailing: usize, n_attacks: u64, table: &[[u64; 8]; 64]) {
        let attacks = table[trailing];
        let mut n = 0;
        for a in attacks.iter() {
            if *a != 0 {
                n += 1;
            }
        }
        assert_eq!(n, n_attacks);
    }

    #[test]
    fn test_knight_attack_table() {
        test_table_n_attacks(0, 2, &KNIGHT_ATTACK);
        test_table_n_attacks(63, 2, &KNIGHT_ATTACK);
        test_table_n_attacks(27, 8, &KNIGHT_ATTACK);
        test_table_n_attacks(4, 4, &KNIGHT_ATTACK);
    }

    #[test]
    fn test_king_attack_table() {
        test_table_n_attacks(0, 3, &KING_ATTACK);
        test_table_n_attacks(63, 3, &KING_ATTACK);
        test_table_n_attacks(1, 5, &KING_ATTACK);
        test_table_n_attacks(9, 8, &KING_ATTACK);
    }

    #[test]
    fn test_rook_attacks() {
        for i in 0..64usize {
            let ra = RANK_ATTACK[i];
            let fa = FILE_ATTACK[i];
            assert_eq!(ra.popcnt(), 7);
            assert_eq!(fa.popcnt(), 7);
            assert_eq!((ra | fa).popcnt(), 14); // No overlap
        }
    }
}
