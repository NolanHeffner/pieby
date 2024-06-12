
use crate::board::bitboard;

// Eventually need to implement operator capabilities for bitboards so no need for commands
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not};

use super::bitboard::Bitboard;

pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    DUCK,
    NONE
}

pub enum Color {
    WHITE,
    BLACK,
    YELLOW
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
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> f64 {
        match self.piece_type {
            PieceType::KING => 1000.0,
            PieceType::QUEEN => 9.0,
            PieceType::ROOK => 5.0,
            PieceType::BISHOP => 3.2,
            PieceType::KNIGHT => 3.0,
            PieceType::PAWN => 1.1,
            PieceType::DUCK => 0.0,
            PieceType::NONE => 0.0,
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