
use crate::board::{bitboard::Bitboard, board::Board,
   types::{Piece, PieceType, Color} attacks::*};


// Pseudo-legal move generation


pub fn gen_rook_moves(squarePos: u8) -> Bitboard {
   Bitboard::EMPTY
}


pub fn gen_bishop_moves(squarePos: u8) -> Bitboard {
   Bitboard::EMPTY
}


// Legal move generation


pub fn gen_duck_moves(board: &Board) -> Bitboard {
   let mut filled = Bitboard::EMPTY;
   for occ_board in &board.pieces {
       filled |= Bitboard(occ_board.value());
   }
   !filled
}




// Move Validation Section
pub fn is_move_legal(board: &Board, from: u8, to: u8) -> bool {
   // Get the piece at the 'from' square
   let piece = board.get_square(from);
   // Get the piece (if any) at the 'to' square
   let target = board.get_square(to);


   // Check if the move is within the board (0-63 are valid square indices)
   if from > 63 || to > 63 {
       return false; // Move is off the board, so it's invalid
   }


   // Check if the piece belongs to the current player
   if piece.get_color() != board.turn {
       return false; // It's not this player's turn, so the move is invalid
   }
 // Check if the target square is not occupied by a piece of the same color
   if target.get_color() == piece.get_color() {
       return false; // Can't capture your own piece, so the move is invalid
   }


   // Validate move based on piece type
   match piece.get_type() {
       PieceType::PAWN => is_valid_pawn_move(board, from, to),
       PieceType::KNIGHT => is_valid_knight_move(board, from, to),
       PieceType::BISHOP => is_valid_bishop_move(board, from, to),
       PieceType::ROOK => is_valid_rook_move(board, from, to),
       PieceType::QUEEN => is_valid_queen_move(board, from, to),
       PieceType::KING => is_valid_king_move(board, from, to),
       _ => false,
   }
}


fn is_valid_pawn_move(board: &Board, from: u8, to: u8) -> bool {
   let piece = board.get_square(from);
   let target = board.get_square(to);


   // Determine the direction of movement based on pawn color
   let direction: i8 = if piece.get_color() == Color::WHITE { 1 } else { -1 };


   // Calculate the difference between 'to' and 'from' squares
   let diff: i8 = (to as i8) - (from as i8);


   // Check for basic forward movement
   if diff == 8 * direction && target.get_type() == PieceType::NONE {
       return true; // Moving forward one square to an empty square
   }


   // Check for initial two-square move
   if (from / 8 == 1 && piece.get_color() == Color::WHITE) ||
      (from / 8 == 6 && piece.get_color() == Color::BLACK) {
       if diff == 16 * direction &&
          target.get_type() == PieceType::NONE &&
          board.get_square((from as i8 + 8 * direction) as u8).get_type() == PieceType::NONE {
           return true; // Moving forward two squares from starting position
       }
}


   // Check for diagonal captures
   if (diff == 7 * direction || diff == 9 * direction) &&
      target.get_type() != PieceType::NONE &&
      target.get_color() != piece.get_color() {
       return true; // Capturing diagonally
   }


   // If none of the above conditions are met, the move is invalid
   false
}
