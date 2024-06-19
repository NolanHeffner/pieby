
use crate::board::bitboard;

// Eventually need to implement operator capabilities for bitboards so no need for commands
// use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not};

use super::bitboard::Bitboard;

pub struct PieceType(u8);

impl PieceType {
    pub const KING: PieceType = PieceType(0);
    pub const QUEEN: PieceType = PieceType(1);
    pub const ROOK: PieceType = PieceType(2);
    pub const BISHOP: PieceType = PieceType(3);
    pub const KNIGHT: PieceType = PieceType(4);
    pub const PAWN: PieceType = PieceType(5);
    pub const DUCK: PieceType = PieceType(6);
    pub const NONE: PieceType = PieceType(7);

    pub const COUNT: usize = 7;

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn new(idx: usize) -> PieceType {
        PieceType(idx.try_into().unwrap())
    }
}

pub struct Color(u8);

impl Color {
    pub const WHITE : Color = Color(0);
    pub const BLACK : Color = Color(1);
    pub const YELLOW : Color = Color(2);
    pub const NONE : Color = Color(3);

    pub const COUNT: usize = 3;

    pub fn new(idx: usize) -> Color {
        match idx {
            0 => Color::WHITE,
            1 => Color::BLACK,
            2 => Color::YELLOW,
            _ => Color::NONE,
        }
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    /*
    pub fn index(&self) -> usize {
        *self as usize
    }
    */
}

pub struct Piece {
    piece_type: PieceType,
    color: Color,
    attack_bitboard: bitboard::Bitboard,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece {
            piece_type,
            color,
            attack_bitboard: Bitboard::new(0), // or some default value
        }
    }

    pub fn get_type(&self) -> &PieceType {
        &self.piece_type
    } // Eventually replace with trait to enable "as" functionality for u8?

    pub fn index(&self) -> usize { // Indexes non-NONE pieces to usizes 1 through 14
        let idx = 7 * self.color.index() - self.piece_type.index();
        idx as usize + PieceType::COUNT
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> f64 {
        match self.piece_type.0 {
            KING => 1000.0,
            QUEEN => 9.0,
            ROOK => 5.0,
            BISHOP => 3.2,
            KNIGHT => 3.0,
            PAWN => 1.1,
            DUCK => 0.0,
            NONE => 0.0,
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