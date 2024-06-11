
mod types;

enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    DUCK,
    NONE
}

enum Color {
    WHITE,
    BLACK,
    YELLOW
}

struct Piece {
    piece_type: PieceType,
    color: Color,
    attack_bitboard: Bitboard,
}

impl Piece {
    fn value(&self) -> f64 {
        match self.piece_type {
            KING => 1000,
            QUEEN => 9,
            ROOK => 5,
            BISHOP => 3.2,
            KNIGHT => 3,
            PAWN => 1.1,
            DUCK => 0,
            NONE => 0,
        }
    }

/*     fn get_ASCII(&self) -> &str {
        letter = match self.piece_type {
            KING => "K",
            QUEEN => "Q",
            ROOK => "R",
            BISHOP => "B",
            KNIGHT => "N",
            PAWN => "P",
            DUCK => "D",
            NONE => " ",
        };
        if self.color == Color::BLACK {
            return letter.to_ascii_lowercase();
        } else {
            return letter;
        }
    } */
}