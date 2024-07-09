
use rand::{Error, Rng};

use crate::board::{
    magic::BlackMagic, 
    bitboard::{Bitboard, get_file, get_rank}
};

// Initialization data
// Note as of 6/27: Arrays denote number of relevant bits - replaced by popcnt(occ_mask)

/* 
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
 */

// Bitboard manipulation tools

fn gen_few_bit_u64() -> u64 {
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

// Generate mask bitboards

pub fn rook_mask(rank: u8, file: u8) -> Bitboard {
    if rank > 7 || file > 7 {
        println!("Error. Rank and file index from 0 to 7.");
        return Bitboard::EMPTY
    }
    let square = 8 * rank + file;
    (get_rank(rank) | get_file(file)) - Bitboard(1 << square)
}

pub fn bishop_mask(rank: u8, file: u8) -> Bitboard {
    let (irank, ifile) = (rank as i8, file as i8);
    if rank > 7 || file > 7 {
        println!("Error. Rank and file index from 0 to 7.");
        return Bitboard::EMPTY
    }
    let square = 8 * irank + ifile;
    let square_board : u64 = 1 << square;
    let mut ret : u64 = square_board;
    for h in 0..8 {
        if (ifile <= (irank + h)) && ((irank + h) <= (7 + ifile)) {
            ret += 1 << (square - 9 * (ifile - h))
        }
        if (h <= (irank + ifile)) && ((irank + ifile) <= (7 + h)) {
            ret += 1 << (square + 7 * (ifile - h))
        }
    }
    Bitboard(ret as u64 - 2 * square_board)
}

fn prune(board: Bitboard, rank: u8, file: u8) -> Bitboard {
    let mut mask = Bitboard::EMPTY;
    if rank != 1 {mask |= get_rank(1);}
    if rank != 7 {mask |= get_rank(7);}
    if file != 1 {mask |= get_file(1);}
    if file != 7 {mask |= get_file(7);}
    board & !mask
}

// Carry-Rippler Trick
// Iterates over all subsets of a mask u64 to produce all blocking configurations

struct CarryRippler {
    set: u64,
    subset: u64,
}

impl CarryRippler {
    fn new(set: u64) -> Self {
        CarryRippler {
            set,
            subset: 0,
        }
    }
}

impl Iterator for CarryRippler {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.subset.wrapping_sub(self.set) & self.set)
    }
}

// Generate occupancy boards

fn gen_occs(mask: fn(u8, u8) -> Bitboard, rank: u8, file: u8) -> [u64; 4096] {
    let mask = prune(mask(rank, file), rank, file);
    let iter = CarryRippler::new(*mask);
    let mut attacks = [0; 4096];

    for subset in iter {
        attacks[subset as usize] = subset;
    }
    attacks
}

// Generate attack boards

fn gen_rook_attack(sq: u8, block: u64) -> u64 {
    let mut result = 0u64;
    let rank : u8 = sq / 8;
    let file : u8 = sq % 8;

    

    return result;
}
  
fn gen_bishop_attack(sq: u8, block: u64) -> u64 {
    let mut result = 0u64;
    let rank : u8 = sq / 8;
    let file : u8 = sq % 8;
    
    return result;
}

fn gen_attacks(sq: u8, occ: [u64; 4096], is_bishop: bool) -> [u64; 4096] {
    let mut attacks = [0; 4096];
    for idx in 0..4096 {
        let block = occ[idx];
        attacks[idx] = if is_bishop {gen_bishop_attack(sq, block)} else {gen_rook_attack(sq, block)};
    }
    attacks
}

// Hashing used_attack indices
// Credit to Chess Programming Wiki "Looking for Magics" page for transform() hashing function

fn transform(block: u64, magic: u64, bits: u8) -> u64 {
    (block * magic) ^ ((block >> 32) * (magic >> 32)) >> 32
}

// Generating magic numbers

pub fn magic_gen(rank: u8, file: u8, is_bishop: bool, attempts: u64) -> Option<BlackMagic> {
    let occ_mask = rook_mask(rank, file).value();
    let rel_bits = popcnt(occ_mask); // Counts number of relevant bits in mask
    let arr_size = 1 << rel_bits;

    // Initialize occupancies and attack tables

    let occupancies : [u64; 4096] = gen_occs(if is_bishop {bishop_mask} else {rook_mask}, rank, file);
    let attacks : [u64; 4096] = gen_attacks(8 * rank + file, occupancies, is_bishop);

    // Init used attacks
    let mut used_attacks : [u64; 4096] = [0; 4096];

    // Init attack mask for current piece

    let notmask = !occ_mask;
    let shift = 64 - popcnt(notmask);
    for _ in 0..attempts {
        // if count_1s((mask * magic) & 0xFF00000000000000) < 6 {continue};
        let candidate_magic = gen_few_bit_u64();

        // Test magic against each attack pattern

        let mut fail = false;
        used_attacks.fill(0);

        for (i, &occ) in occupancies.iter().enumerate() {
            let index = transform(occ, candidate_magic, rel_bits) as usize;
            if used_attacks[index] == 0 || used_attacks[index] == attacks[i] {
                used_attacks[index] = attacks[i];
            } else {
                fail = true;
                continue;
            }
        }

        if !fail {
            return Some(BlackMagic {
                offset: 0,
                notmask,
                blackmagic: candidate_magic,
                shift,
            })
        }
    }

    return None;
    /* let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?; */
    println!("Failed to find magic number for bishop on square {} in {} attempts.", 8 * rank + file, attempts);
    panic!();
    
}

pub fn init_magic_numbers() -> (Vec::<BlackMagic>, Vec<BlackMagic>) {

    let mut bm_rook_table = Vec::<BlackMagic>::new();
    let mut bm_bishop_table = Vec::<BlackMagic>::new();

    let attempts = 1_000_000;
    let (mut rank, mut file) : (u8, u8) = (0, 0);
    while rank < 8 {
        while file < 8 {
            bm_rook_table.push(magic_gen(rank, file, false, attempts).unwrap_or_else(|| {
                println!("Error calculating rook magic number at rank {rank} and file {file}.");
                BlackMagic::EMPTY
            }));
            bm_bishop_table.push(magic_gen(rank, file, true, attempts).unwrap_or_else(|| {
                println!("Error calculating bishop magic number at rank {rank} and file {file}.");
                BlackMagic::EMPTY
            }));
            file += 1;
        }
        rank += 1;
    }

    (bm_rook_table, bm_bishop_table)
}

// Mockup function
/* pub fn get_attack_table_index(occ: Bitboard, square: u8, bishop: bool) -> Bitboard {
    let bm : BlackMagic = if bishop {bmBishopTbl[sq]} else {bmRookTbl[sq]};
    let mut attacks = occ.value();
    Bitboard(((attacks | bm.notmask) * bm.blackmagic) >> bm.shift)
} */

// return lookup[(occupied & mask) * magic >> shift];

#[cfg(test)]
mod tests {
    use crate::{board::bitboard::Bitboard, util::magic_gen::init_magic_numbers};

    use super::{bishop_mask};

    //#[test]
    fn print_bishop_attack() {
        Bitboard::print_bitboard(bishop_mask(4, 3));
    }

    //#[test]
    fn print_magics() {
        let (rook_magics, bishop_magics) = init_magic_numbers();

        // Rook magics
        println!("Rook magics:\n\npub const ROOK_BLACK_MAGICS: [BlackMagic; 64] = [\n");
        for magic in rook_magics.into_iter() {
            println!("{}\n", magic);
        }
        println!("];");

        // Bishop magics
        println!("Bishop magics:\n\npub const BISHOP_BLACK_MAGICS: [BlackMagic; 64] = [\n");
        for magic in bishop_magics.into_iter() {
            println!("{}\n", magic);
        }
        println!("];");

    }
}