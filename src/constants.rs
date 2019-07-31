pub const ROW_1: u64 = 0xFF;
pub const ROW_2: u64 = ROW_1 << 8;
pub const ROW_3: u64 = ROW_2 << 8;
pub const ROW_4: u64 = ROW_3 << 8;
pub const ROW_5: u64 = ROW_4 << 8;
pub const ROW_6: u64 = ROW_5 << 8;
pub const ROW_7: u64 = ROW_6 << 8;
pub const ROW_8: u64 = ROW_7 << 8;
pub const ROWS: [u64; 8] = [
    ROW_1,
    ROW_2,
    ROW_3,
    ROW_4,
    ROW_5,
    ROW_6,
    ROW_7,
    ROW_8
];

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;
pub const FILES: [u64; 8] = [
    FILE_A,
    FILE_B,
    FILE_C,
    FILE_D,
    FILE_E,
    FILE_F,
    FILE_G,
    FILE_H
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

pub const KNIGHT_ATTACK: [[u64; 8]; 64] = generate_knight_attacks();

pub const fn get_row_mask(row: usize) -> u64 {
    0xFF << (row as u64) * 8
}

const fn generate_knight_attacks() -> [[u64; 8]; 64] {
    let mut attacks: [[u64; 8]; 64] = [[0; 8]; 64];
    /*for i in 0..64 { // TODO uncomment when iterating in const fn is stable
        attacks[i] = get_knight_attacks(1 << (i as u64));
    }*/

    // Y I K E S
    attacks[0] = get_knight_attacks(1 << (0 as u64));
    attacks[1] = get_knight_attacks(1 << (1 as u64));
    attacks[2] = get_knight_attacks(1 << (2 as u64));
    attacks[3] = get_knight_attacks(1 << (3 as u64));
    attacks[4] = get_knight_attacks(1 << (4 as u64));
    attacks[5] = get_knight_attacks(1 << (5 as u64));
    attacks[6] = get_knight_attacks(1 << (6 as u64));
    attacks[7] = get_knight_attacks(1 << (7 as u64));
    attacks[8] = get_knight_attacks(1 << (8 as u64));
    attacks[9] = get_knight_attacks(1 << (9 as u64));
    attacks[10] = get_knight_attacks(1 << (10 as u64));
    attacks[11] = get_knight_attacks(1 << (11 as u64));
    attacks[12] = get_knight_attacks(1 << (12 as u64));
    attacks[13] = get_knight_attacks(1 << (13 as u64));
    attacks[14] = get_knight_attacks(1 << (14 as u64));
    attacks[15] = get_knight_attacks(1 << (15 as u64));

    attacks[16] = get_knight_attacks(1 << (16 as u64));
    attacks[17] = get_knight_attacks(1 << (17 as u64));
    attacks[18] = get_knight_attacks(1 << (18 as u64));
    attacks[19] = get_knight_attacks(1 << (19 as u64));
    attacks[20] = get_knight_attacks(1 << (20 as u64));
    attacks[21] = get_knight_attacks(1 << (21 as u64));
    attacks[22] = get_knight_attacks(1 << (22 as u64));
    attacks[23] = get_knight_attacks(1 << (23 as u64));
    attacks[24] = get_knight_attacks(1 << (24 as u64));
    attacks[25] = get_knight_attacks(1 << (25 as u64));
    attacks[26] = get_knight_attacks(1 << (26 as u64));
    attacks[27] = get_knight_attacks(1 << (27 as u64));
    attacks[28] = get_knight_attacks(1 << (28 as u64));
    attacks[29] = get_knight_attacks(1 << (29 as u64));
    attacks[30] = get_knight_attacks(1 << (30 as u64));
    attacks[31] = get_knight_attacks(1 << (31 as u64));

    attacks[32] = get_knight_attacks(1 << (32 as u64));
    attacks[33] = get_knight_attacks(1 << (33 as u64));
    attacks[34] = get_knight_attacks(1 << (34 as u64));
    attacks[35] = get_knight_attacks(1 << (35 as u64));
    attacks[36] = get_knight_attacks(1 << (36 as u64));
    attacks[37] = get_knight_attacks(1 << (37 as u64));
    attacks[38] = get_knight_attacks(1 << (38 as u64));
    attacks[39] = get_knight_attacks(1 << (39 as u64));
    attacks[40] = get_knight_attacks(1 << (40 as u64));
    attacks[41] = get_knight_attacks(1 << (41 as u64));
    attacks[42] = get_knight_attacks(1 << (42 as u64));
    attacks[43] = get_knight_attacks(1 << (43 as u64));
    attacks[44] = get_knight_attacks(1 << (44 as u64));
    attacks[45] = get_knight_attacks(1 << (45 as u64));
    attacks[46] = get_knight_attacks(1 << (46 as u64));
    attacks[47] = get_knight_attacks(1 << (47 as u64));

    attacks[48] = get_knight_attacks(1 << (48 as u64));
    attacks[49] = get_knight_attacks(1 << (49 as u64));
    attacks[50] = get_knight_attacks(1 << (50 as u64));
    attacks[51] = get_knight_attacks(1 << (51 as u64));
    attacks[52] = get_knight_attacks(1 << (52 as u64));
    attacks[53] = get_knight_attacks(1 << (53 as u64));
    attacks[54] = get_knight_attacks(1 << (54 as u64));
    attacks[55] = get_knight_attacks(1 << (55 as u64));
    attacks[56] = get_knight_attacks(1 << (56 as u64));
    attacks[57] = get_knight_attacks(1 << (57 as u64));
    attacks[58] = get_knight_attacks(1 << (58 as u64));
    attacks[59] = get_knight_attacks(1 << (59 as u64));
    attacks[60] = get_knight_attacks(1 << (60 as u64));
    attacks[61] = get_knight_attacks(1 << (61 as u64));
    attacks[62] = get_knight_attacks(1 << (62 as u64));
    attacks[63] = get_knight_attacks(1 << (63 as u64));

    return attacks;
}

const fn get_knight_attacks(pos: u64) -> [u64; 8] {
    //https://www.chessprogramming.org/Knight_Pattern
    // TODO

    [0; 8]
}
