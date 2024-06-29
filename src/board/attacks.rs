#![allow(unused)]

use std::i128::MAX;
use crate::board::bitboard::Bitboard;
use super::bitboard::{get_file, get_rank};

const PAWN_ATTACKS: [[Bitboard; 64]; 2] = init_pawn_attacks();
const KNIGHT_ATTACKS: [Bitboard; 64] = init_knight_attacks();
const KING_ATTACKS: [Bitboard; 64] = init_king_attacks();

const SLIDING_ATTACKS: [Bitboard; 88507] = init_sliding_attacks();

const NOT_H_FILE : u64 = 0x7F7F7F7F7F7F7F7F;
const NOT_A_FILE : u64 = 0xFEFEFEFEFEFEFEFE;
const NOT_RANK_0 : u64 = 0xFFFFFFFFFFFFFF00;
const NOT_RANK_7 : u64 = 0x00FFFFFFFFFFFFFF;

const fn gen_sliding_attacks(square: u8, occ_bb: Bitboard) -> Bitboard {
    Bitboard::EMPTY
}

const fn init_sliding_attacks() -> [Bitboard; 88507] {
    let mut sliding_attacks = [Bitboard::EMPTY; 88507];
    let mut square = 0;
    while square < 64 {
        square += 1;
    }
    [Bitboard::EMPTY; 88507]
}

const fn init_pawn_attacks() -> [[Bitboard; 64]; 2] {
    let mut attacks : [[Bitboard; 64]; 2] = [[Bitboard::EMPTY; 64], [Bitboard::EMPTY; 64]];

    let mut square : usize = 0;
    while square < 64 {
        let sq_bit : u64 = 1 << square;

        attacks[0][square] = Bitboard(((sq_bit & NOT_A_FILE) << 7) | ((sq_bit & NOT_H_FILE) << 9));
        attacks[1][square] = Bitboard(((sq_bit & NOT_A_FILE) >> 9) | ((sq_bit & NOT_H_FILE) >> 7));

        square += 1;
    }
    attacks
}

const fn init_knight_attacks() -> [Bitboard; 64] {
    let mut knight_attacks = [Bitboard::EMPTY; 64];
    let mut square : i32 = 0;
    while square < 64 {
        let mut attack : u64 = 0xa1100110a;
        let top_mask = if square > 46 {!0u64 >> (square - 47)} else {u64::MAX};
        let file_mask : u64 = if (square % 8) > 3 {0xFCFCFCFCFCFCFCFC} else {0x3F3F3F3F3F3F3F3F};
        attack = file_mask & (attack & top_mask).wrapping_shl((18 - square).wrapping_neg() as u32);
        knight_attacks[square as usize] = Bitboard(attack & file_mask);
        square += 1;
    }
    knight_attacks
}

const fn init_king_attacks() -> [Bitboard; 64] {
    let mut king_attacks = [Bitboard::EMPTY; 64];
    let mut square : usize = 0;
    while square < 64 {
        let sq_bit : u64 = 1 << square;
        let mut attack = sq_bit | ((sq_bit & NOT_H_FILE) << 1) | ((sq_bit & NOT_A_FILE) >> 1);
        attack = attack | ((attack & NOT_RANK_0) >> 8) | ((attack & NOT_RANK_7) << 8);
        king_attacks[square] = Bitboard(attack & !sq_bit);
        square += 1;
    }
    king_attacks
}

#[cfg(test)]
mod tests {
    use crate::board::{attacks::{init_knight_attacks, init_pawn_attacks, init_king_attacks}, bitboard::Bitboard};

    //#[test]
    fn print_knight_attacks() {
        let attacks = init_knight_attacks();
        for idx in 0..64 {
            let attack = Bitboard(attacks[idx].value());
            println!("Square {}\n{}\n", idx, attack);
        }
    }

    //#[test]
    fn print_pawn_attacks() {
        let attacks = init_pawn_attacks();
        for idx in 0..64 {
            let mut attack = Bitboard(attacks[0][idx].value());
            println!("Square {}\n{}\n", idx, attack);
            attack = Bitboard(attacks[1][idx].value());
            println!("Square {}\n{}\n", idx, attack);
        }
    }

    #[test]
    fn print_king_attacks() {
        let attacks = init_king_attacks();
        for idx in 0..64 {
            let attack = Bitboard(attacks[idx].value());
            println!("Square {}\n{}\n", idx, attack);
        }
    }
}