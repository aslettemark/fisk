use bitintr::Popcnt;

use fisk::constants::*;

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
fn knight_attack_table() {
    test_table_n_attacks(0, 2, &KNIGHT_ATTACK_LIST);
    test_table_n_attacks(63, 2, &KNIGHT_ATTACK_LIST);
    test_table_n_attacks(27, 8, &KNIGHT_ATTACK_LIST);
    test_table_n_attacks(4, 4, &KNIGHT_ATTACK_LIST);
}

#[test]
fn king_attack_table() {
    test_table_n_attacks(0, 3, &KING_ATTACK);
    test_table_n_attacks(63, 3, &KING_ATTACK);
    test_table_n_attacks(1, 5, &KING_ATTACK);
    test_table_n_attacks(9, 8, &KING_ATTACK);
}

#[test]
fn rook_attacks() {
    for i in 0..64usize {
        let ra = RANK_ATTACK[i];
        let fa = FILE_ATTACK[i];
        assert_eq!(ra.popcnt(), 7);
        assert_eq!(fa.popcnt(), 7);
        assert_eq!((ra | fa).popcnt(), 14); // No overlap
    }
}
