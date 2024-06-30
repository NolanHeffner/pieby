
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

// Generate attack boards

fn rook_attacks(block: u8, rank: u8, file: u8) -> Bitboard {
    let mask = prune(rook_mask(rank, file), rank, file);
    Bitboard::EMPTY
}

// Generating magic numbers

pub fn magic_gen(rank: u8, file: u8, is_bishop: bool, attempts: u64) -> BlackMagic {
    let occ_mask = rook_mask(rank, file).value();
    let rel_bits = popcnt(occ_mask); // Counts number of relevant bits in mask
    let arr_size = 1 << rel_bits;

    // Initialize occupancies and attack tables

    let mut occupancies : [u64; 4096];
    let mut attacks : [u64; 4096];

    for i in 0..(1 << rel_bits) {
        // attacks[i] = if is_bishop {bmask())}
    }

    // Init used attacks
    let mut used_attacks : [u64; 4096];

    // Init attack mask for current piece

    let unmask = !occ_mask;
    for _ in 0..attempts {
        // if count_1s((mask * magic) & 0xFF00000000000000) < 6 {continue};
        let candidate_magic: u64 = gen_few_bit_u64();
        let mut fail = false;

        // Test magic against each attack pattern
        for _ in 0..(1 << rel_bits) {
            let shift = 64 - popcnt(unmask);
        }
    }

    /* let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?; */

    println!("Failed to find magic number for bishop on square {} in {} attempts.", 8 * rank + file, attempts);
    panic!();

    BlackMagic::EMPTY
}

pub fn init_magic_numbers() -> (Vec::<BlackMagic>, Vec<BlackMagic>) {

    let mut bm_rook_table = Vec::<BlackMagic>::new();
    let mut bm_bishop_table = Vec::<BlackMagic>::new();

    let attempts = 1_000_000;
    let (mut rank, mut file) : (u8, u8) = (0, 0);
    while rank < 8 {
        while file < 8 {
            bm_rook_table.push(magic_gen(rank, file, false, attempts));
            bm_bishop_table.push(magic_gen(rank, file, false, attempts));
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