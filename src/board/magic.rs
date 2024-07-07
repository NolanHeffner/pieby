
use std::fmt;

pub struct BlackMagic {
    pub offset: usize,  // index of attack_table in array
    pub notmask: u64,  // masking relevant squares (minus outer squares)
    pub blackmagic: u64, // black magic multiplication factor
    pub shift: u8, // Number of bits in blackmagic
}

impl BlackMagic {
    pub const EMPTY : BlackMagic = BlackMagic {
        offset: 0,
        notmask: 0,
        shift: 0,
        blackmagic: 0,
    };
}

impl fmt::Display for BlackMagic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlackMagic {{\n\toffset: {:x},\n\tnotmask: {},\n\tshift: {},\n\tblackmagic: {},\n}}", self.offset, self.notmask, self.shift, self.blackmagic)
    }
}

#[cfg(test)]
mod tests {
    use super::BlackMagic;
    const TEST_BM_STRUCT : BlackMagic = BlackMagic {
        offset: 3,
        notmask: 765,
        shift: 13,
        blackmagic: 4567,
    };

    //#[test]
    fn test_display() {
        println!("{}", TEST_BM_STRUCT);
    }
}

pub const ROOK_BM : [BlackMagic; 64] = [BlackMagic::EMPTY; 64];
pub const BISHOP_BM : [BlackMagic; 64] = [BlackMagic::EMPTY; 64];

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