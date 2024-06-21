
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

/* Black magic implementation reference

U64 attack_table[88507]; // 692 KiB for published black magics by Volker Annuss (new size 88316 aka 690 KiB) 

struct SBlackMagic {
   U64* ptr;  // pointer to attack_table for each particular square
   U64 notmask;  // to mask relevant squares of both lines (no outer squares)
   U64 blackmagic; // black magic 64-bit factor
};

SBlackMagic mBishopTbl[64];
SBlackMagic mRookTbl[64];

U64 rookAttacks(U64 occ, enumSquare sq) {
   U64* aptr = mRookTbl[sq].ptr;
   occ      |= mRookTbl[sq].notmask;
   occ      *= mRookTbl[sq].blackmagic;
   occ     >>= 64-12;
   return aptr[occ];
}
*/