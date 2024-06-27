#![allow(unused)]

use crate::board::bitboard::Bitboard;

use super::bitboard::get_rank;

const PAWN_ATTACKS: [[Bitboard; 64]; 2] = init_pawn_attacks();
const KNIGHT_ATTACKS: [Bitboard; 64] = init_knight_attacks();
const KING_ATTACKS: [Bitboard; 64] = init_king_attacks();

const SLIDING_ATTACKS: [Bitboard; 88507] = init_sliding_attacks();

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
    [[Bitboard::EMPTY; 64], [Bitboard::EMPTY; 64]]
}

const fn init_knight_attacks() -> [Bitboard; 64] {
    let mut knight_attacks = [Bitboard::EMPTY; 64];
    for square in 0..64 {
        let knight_bit: u64 = 1 << square;
        let knight_attack = ((knight_bit >> 6) | (knight_bit >> 10)) & get_rank(square % 8 + 1);
    }

    [Bitboard::EMPTY; 64]
}

const fn init_king_attacks() -> [Bitboard; 64] {
    [Bitboard::EMPTY; 64]
}