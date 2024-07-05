
use crate::board::{bitboard::Bitboard, board::Board, types::{PieceType, Color}};

use super::mv::Move;

// Infrastructure

pub enum movegenType {
    QUIET,
    CAPTURE,
    CHECK,
    EVASION,
    NONEVASION,
    LEGAL
}

impl Board {

    // Pseudo-legal move generation

    pub fn gen_rook_moves(squarePos: u8) -> Bitboard {
        Bitboard::EMPTY
    }


    pub fn gen_bishop_moves(squarePos: u8) -> Bitboard {
        Bitboard::EMPTY
    }

    // Legal move generation

    pub fn gen_duck_moves(&self) -> Bitboard {
        let mut filled = Bitboard::EMPTY;
        for occ_board in &self.pieces {
            filled |= Bitboard(occ_board.value());
        }
        !filled
    }

    // Move Validation Section


    pub fn is_move_legal(&self, mv: Move) -> bool {
        let (from, to, promo, promoPiece) = mv.unpack();

        // Check if the move is within the board (0-63 are valid square indices)
        if from > 63 || to > 63 {
            return false; // Move is off the board, so it's invalid
        }

        // Get the piece at the 'from' square
        let piece = self.get_square(from);
        // Get the piece (if any) at the 'to' square
        let target = self.get_square(to);

        // Check if the piece belongs to the current player
        if *piece.get_color() != self.turn {
            return false; // It's not this player's turn, so the move is invalid
        }
        // Check if the target square is not occupied by a piece of the same color
        if target.get_color() == piece.get_color() {
            return false; // Can't capture your own piece, so the move is invalid
        }

        // Validate move based on piece type
        match piece.get_type() {
            &PieceType::PAWN => self.is_valid_pawn_move(from, to),
            &PieceType::KNIGHT => self.is_valid_knight_move(from, to),
            &PieceType::BISHOP => self.is_valid_bishop_move(from, to),
            &PieceType::ROOK => self.is_valid_rook_move(from, to),
            &PieceType::QUEEN => self.is_valid_queen_move(from, to),
            &PieceType::KING => self.is_valid_king_move(from, to),
            _ => false,
        }
    }


    fn is_valid_pawn_move(&self, from: u8, to: u8) -> bool {

        let piece = self.get_square(from);
        let target = self.get_square(to);

        // Determine the direction of movement based on pawn color
        let direction: i8 = if *piece.get_color() == Color::WHITE { 1 } else { -1 };

        // Calculate the difference between 'to' and 'from' squares
        let diff: i8 = (to as i8) - (from as i8);

        // Check for basic forward movement
        if diff == 8 * direction && *target.get_type() == PieceType::NONE {
            return true; // Moving forward one square to an empty square
        }

        // Check for initial two-square move

        if (from / 8 == 1 && *piece.get_color() == Color::WHITE) || (from / 8 == 6 && *piece.get_color() == Color::BLACK) {
            if diff == 16 * direction &&
                *target.get_type() == PieceType::NONE &&
                *self.get_square((from as i8 + 8 * direction) as u8).get_type() == PieceType::NONE {
                return true; // Moving forward two squares from starting position
            }
        }

        // Check for diagonal captures
        if (diff == 7 * direction || diff == 9 * direction) &&
            *target.get_type() != PieceType::NONE &&
            target.get_color() != piece.get_color() {
            return true; // Capturing diagonally
        }

        // If none of the above conditions are met, the move is invalid
        false
    }
}