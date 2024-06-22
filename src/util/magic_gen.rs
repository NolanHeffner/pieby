
use rand::Rng;

use crate::board::{magic, bitboard::Bitboard};

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

pub fn get_rank(rank: u8) -> Bitboard {
    // if rank > 7 {return Bitboard::new(0)}
    let shift = 8 * rank;
    // print_bitboard((0xFF as u64) << shift)
    Bitboard::new((0xFF as u64) << shift)
}

pub fn get_file(file: u8) -> Bitboard {
    Bitboard::new(0x0101010101010101 << file)
}

// pub fn ls1b_index(num: u64) // Replaced by .trailing_zeros()

// Generate attack bitboards

pub fn gen_rook_attacks(rank: u8, file: u8) -> Bitboard {
    if rank > 7 || file > 7 {
        println!("Error. Rank and file index from 0 to 7.");
        return Bitboard::new(0)
    }
    let square = 8 * rank + file;
    (get_rank(rank) | get_file(file)) - Bitboard::new(1 << square)
}

pub fn gen_bishop_attacks(rank: u8, file: u8) -> Bitboard {
    if rank > 7 || file > 7 {
        println!("Error. Rank and file index from 0 to 7.");
        return Bitboard::new(0)
    }
    let square = 8 * rank + file;
    let square_board : u64 = 1 << square;
    let mut ret = square_board;
    for h in 0..8 {
        if 0 <= (rank + h - file) && (rank + h - file) <= 7 {
            ret += 1 << (square - 9 * (file - h))
        }
        if 0 <= (rank - h + file) && (rank - h + file) <= 7  {
            ret += 1 << (square + 7 * (file - h))
        }
    }
    Bitboard::new(ret - 2 * square_board)
}

pub fn rmask(rank: u8, file: u8) -> Bitboard {
    let mut mask = Bitboard::EMPTY;
    if rank != 1 {mask |= get_rank(1);}
    if rank != 7 {mask |= get_rank(7);}
    if file != 1 {mask |= get_file(1);}
    if file != 7 {mask |= get_file(7);}
    gen_rook_attacks(rank, file) & !mask
}

pub fn bmask(rank: u8, file: u8) -> Bitboard {
    let mut mask = Bitboard::EMPTY;
    if rank != 1 {mask |= get_rank(1);}
    if rank != 7 {mask |= get_rank(7);}
    if file != 1 {mask |= get_file(1);}
    if file != 7 {mask |= get_file(7);}
    gen_bishop_attacks(rank, file) & !mask
}

// Generating magic numbers

pub fn magic_gen(square: u8, relevant_bits: [u8; 64], attack_mask: u64) -> [u64; 64] {
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
