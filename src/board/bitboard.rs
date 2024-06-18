
/*
Counting
1 2 3 4 . . .
9 10 11 12 . . .
.    .
.      .
.        .
57 58 59 . . .
*/

pub struct Bitboard(u64); // effectively 64 bits in binary

impl Bitboard {
    pub const EMPTY : Bitboard = Bitboard(0);

    // Bitboard builder
    pub fn new(board: u64) -> Bitboard {
        Bitboard(board)
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
    /* fn popcnt(&self) -> i32 { // Returns number of ones
        // self.board.count_ones()
    } */
    pub fn mirrorHorizontal(&self) -> u64 {
        self.0 ^ 7
    }
    pub fn mirrorVertical(&self) -> u64 {
        self.0 ^ 56
    }

    // Bit operations
    pub fn set_bit(&self, position: u8, value: bool) -> u64 {
        self.0 | ((value as u64) << position)
    }
    pub fn bit_at_pos(&self, position: u8) -> u64 {
        (self.0 << position) & 0xFF
    }
}