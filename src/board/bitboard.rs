#![allow(unused)]

use std::{
    fmt, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Not, Sub}
};

pub struct Bitboard(pub u64); // effectively 64 bits in binary

impl Bitboard {
    pub const EMPTY : Bitboard = Bitboard(0);

    // to u64

    pub fn value(&self) -> u64 {
        self.0
    }

    // Bitwise board operations

    pub fn board_invert(&mut self) {
        self.0 = !self.0;
    }

    pub fn board_shift(&mut self, bits: &i8) { // positive -> shift right, negative -> shift left
        self.0 >>= bits;
    }

    pub fn intersection(&self, other: &Bitboard) -> u64 {
        self.0 & other.0
    }

    pub fn union(&self, other: &Bitboard) -> u64 {
        self.0 | other.0
    }

    pub fn xor(&self, other: &Bitboard) -> u64 {
        self.0 ^ other.0
    }

    // Other board operations

    pub fn mirror_horizontal(&mut self) {
        self.0 ^= 7;
        // Does not work
    }

    pub fn mirror_vertical(&mut self) {
        self.0 ^= 56;
        // Does not work
    }

    // Bit operations

    pub fn set_bit(&self, position: u8, value: bool) -> u64 {
        self.0 | ((value as u64) << position)
    }

    pub fn bit_at_pos(&self, position: u8) -> u64 {
        (self.0 >> position) & 1
    }

    // Util functions

    pub fn print_bitboard(bitboard: Bitboard) {
        println!("\n");
        let mut rank = 8;
        while rank > 0 {
            rank -= 1;
            let mut row = String::from("");
            let mut file = 0;
            while file < 8 {
                let square : u8 = 8 * rank + file;
                row = format!("{}{:b} ", row, (bitboard.0 >> square) & 1);
                file += 1;
            }
            println!("{}\n", row);
        }
    }
}

impl Deref for Bitboard {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Sub for Bitboard {
    type Output = Bitboard;
    fn sub(self, rhs: Bitboard) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("\n");
        let mut rank = 8;
        while rank > 0 {
            rank -= 1;
            let mut file = 0;
            while file < 8 {
                let square : u8 = 8 * rank + file;
                res = format!("{}{:b} ", res, (self.0 >> (square)) & 1);
                file += 1;
            }
            res += "\n"
        }
        write!(f, "{}", res)
    }
}

// Utility functions

pub fn get_rank(rank: u8) -> Bitboard {
    // if rank > 7 {return Bitboard::new(0)}
    let shift = 8 * rank;
    // print_bitboard((0xFF as u64) << shift)
    Bitboard((0xFF as u64) << shift)
}

pub fn get_file(file: u8) -> Bitboard {
    Bitboard(0x0101010101010101 << file)
}