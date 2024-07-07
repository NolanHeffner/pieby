
use std::fmt;
use crate::board::types::PieceType;

/*
Structure of move object:
- First 6 bits represent "from" coords, 2^6 = 64
- Second 6 bits represent "to" coords, 2^6 = 64

- List of special movetypes:
    - Castling
        - Kingside
        - Queenside
    - En pessant
    - Capture
    - Promotion

- 2 bits for flagging special moves
    - 1st bit flags castling
    - 2nd bit flags ep
    - 
- 2 bits to specify extra information
- 

*/

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

const FROM_MASK : u16 = 0x3f;
const TO_MASK : u16 = 0xfc0;
const PIECE_MASK : u16 = 111 << 13;

impl Move {

    pub fn new(from: u8, to: u8, promo: bool, promo_piece: PieceType) -> Self {
        let bits : u16 = (from as u16)
                        | ((to as u16)<< 6)
                        | ((promo as u16) << 12)
                        | ((promo_piece.id() as u16) << 13);
        Move(bits)
    }

    pub fn from(&self) -> u8 {
        (self.0 & FROM_MASK) as u8
    }

    pub fn to(&self) -> u8 {
        ((self.0 & TO_MASK) >> 6) as u8
    }

    pub fn promo(&self) -> bool {
        (self.0 >> 12) & 1 == 1
    }

    pub fn piece(&self) -> PieceType {
        match self.promo() {
            false => PieceType::NONE,
            true => PieceType::new((self.0 >> 13).into()),
        }
    }

    pub fn unpack(&self) -> (u8, u8, bool, PieceType) {
        (self.from(), self.to(), self.promo(), self.piece())
    }
}

impl fmt::Display for Move {
    // Move only implements critical information at the moment - a full description would require a u32, which would beef up the transposition table. Hence, we're not going to display full algebraic notation when a Move is passed thru to output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sq_ascii = |sq: u8| -> String {
            let file = (sq % 8) as usize;
            format!("{}{}", &"abcdefgh"[(file - 1)..(file)], (sq - sq % 8) / 8)
        };

        write!(f, "{}{}", sq_ascii(self.from()), sq_ascii(self.to()))
    }
}