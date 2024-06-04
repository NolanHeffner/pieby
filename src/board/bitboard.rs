
/*
Counting
1 2 3 4 . . .
9 10 11 12 . . .
.    .
.      .
.        .
57 58 59 . . .
*/


struct Bitboard {
    name: String,
    board: u64, // effectively 64 bits in binary
}

impl Bitboard {
    // Bitboard builder
    fn new(name: &str, board: &u64) -> Bitboard {
        Bitboard {
            name,
            board,
        }
    }

    // Bitwise board operations
    fn board_invert(&mut self) {
        self.board = !&self.board;
    }
    fn board_shift(&mut self, bits: &i8) { // positive -> shift right, negative -> shift left
        self.board >>= bits;
    }
    fn intersection(&self, other: &Bitboard) -> &Bitboard {
        self.board & other
    }
    fn union(&self, other: &Bitboard) -> &Bitboard {
        self.board | other
    }
    fn xor(&self, other: &Bitboard) -> &Bitboard {
        self.board ^ other
    }


    // Other board operations
    fn popcnt(&self) -> i32 { // Returns number of ones
        // self.board.count_ones()
    }
    fn mirrorHorizontal(&self) -> Bitboard {
        self.board ^ 7
    }
    fn mirrorVertical(&self) -> Bitboard {
        self.board ^ 56
    }

    // Bit operations
    fn set_bit(&mut self, position: u8) {
        
    }
    fn is_bit_filled(&mut self, position: u8) {
        return &self.board
    }
}

struct magicBitboard {
    //toSqBB: []Bitboard, // Currently using Go notation, need to fix
    innerBB: Bitboard,
    magic: u64,
    shift: u32
}