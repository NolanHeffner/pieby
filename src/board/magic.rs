
use crate::board::bitboard::Bitboard;

struct MagicBitboard {
    //toSqBB: []Bitboard, // Currently using Go notation, need to fix
    innerBB: Bitboard,
    magic: u64,
    shift: u32
}

pub struct Magic {
    pub mask: u64,
    pub factor: u64,
    pub shift: usize,
}

