
use std::io;
use rand::Rng;

// Critical initialization data

const BISHOP_BITS : [i8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

const ROOK_BITS: [i8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

// Bitboard manipulation tools

pub fn gen_few_bit_u64() -> u64 {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
    num
}

pub fn popcnt(num: u64) -> u8 {
    let mut count : u8 = 0;
    let mut bitboard = num;
    while bitboard > 0 {
        count += 1;
        bitboard &= bitboard - 1;
    }
    count
}

// pub fn ls1b_index(num: u64) // Replaced by .trailing_zeros()

// Generating magic numbers

pub fn magic_gen(square: i8, relevant_bits: [i8; 64], attack_mask: u64) -> [u64; 64] {
    // Initialize occupancies
    let occupancies : [u64; 4096];

    // Initialize attack tables
    let attacks : [u64; 4096];

    // Init used attacks
    let used_attacks : [u64; 4096];

    // Init attack mask for current piece


    [0; 64]
}

pub fn init_magic_numbers()
{
    let mut rook_magic_numbers : [u64; 64];
    let mut bishop_magic_numbers : [u64; 64];
    // loop over 64 board squares
    for square in 0..64 {
        // init rook magic numbers
        //rook_magic_numbers[square] = magic_gen(square, ROOK_BITS[square], rook);
        //bishop_magic_numbers[square] = magic_gen(square, BISHOP_BITS[square], bishop);
    }
}

// return lookup[(occupied & mask) * magic >> shift];
