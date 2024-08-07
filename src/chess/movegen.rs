
use arrayvec::ArrayVec;

use crate::board::{attacks, bitboard::{get_file, Bitboard}, board::Board, types::{Color, PieceType}};

use super::mv::Move;

// Infrastructure

pub struct MoveList(ArrayVec<Move, 256>);

impl MoveList {
    pub fn new() -> Self {
        MoveList(ArrayVec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    } 

    // Given a piece on square "from", and a list of legal positions represented via a bitboard "to", it appends all legal moves to MoveList
    pub fn readMoves(&mut self, from: u8, mut to: u64, promo: bool) {
        while to != 0 {
            let to_current = to.trailing_zeros() as u8;
            if promo {
                for promo_piece in PieceType::PROMOTABLE {
                    self.0.push(Move::new(from, to_current, true, promo_piece));
                }
            } else {
                self.0.push(Move::new(from, to_current, false, PieceType::NONE));
            }            
            to &= to - 1;
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = arrayvec::IntoIter<Self::Item, 256>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone)]
pub struct MoveGenInfo {
    checkers: Bitboard,
    threats: Bitboard,
    pins: Bitboard,
}

impl MoveGenInfo {
    pub const EMPTY : MoveGenInfo = MoveGenInfo {
        checkers: Bitboard::EMPTY,
        threats: Bitboard::EMPTY,
        pins: Bitboard::EMPTY,
    };
}

impl Board {

    // Calculate checks, threats, and pins

    pub fn init_movegen(self) -> Self {
        self
    }

    // Legal move generation

    pub fn gen_moves(&self) -> MoveList {
        let mut mv_list : MoveList = MoveList::new();
        for sq in 0..64 {
            match self.get_square(sq).get_type() {
                &PieceType::PAWN => self.gen_pawn_moves(&mut mv_list, sq),
                &PieceType::KNIGHT => self.gen_knight_moves(&mut mv_list, sq),
                &PieceType::BISHOP => self.gen_bishop_moves(&mut mv_list, sq),
                &PieceType::ROOK => self.gen_rook_moves(&mut mv_list, sq),
                &PieceType::QUEEN => self.gen_queen_moves(&mut mv_list, sq),
                &PieceType::KING => self.gen_king_moves(&mut mv_list, sq),
                _ => ()
            }
        }
        mv_list
    }
    
    pub fn gen_pawn_moves(&self, mv_list: &mut MoveList, squarePos: u8) {
        let turn = self.turn.index();
        
        // Gen legal moves
        let attacks = attacks::PAWN_ATTACKS[turn][squarePos as usize];
        let opponent = self.colors[1 - turn];
        let legal_attacks = opponent & attacks;

        // Gen legal forward moves (non captures)
        let file = (squarePos - squarePos % 8) / 8;
        // Masking as below is unnecessary, as you should NEVER have a pawn on the 1st or 8th rank. This is kept for to make the engine more robust to errors during initial troubleshooting.
        let masked_pos : u64 = (1 << squarePos) & 0x00FFFFFFFFFFFF00;
        let mut forward : u64;
        if turn == 0 {
            forward = masked_pos << 8;
            if file == 1 {
                forward |= masked_pos << 16;
            }
        } else {
            forward = masked_pos >> 8;
            if file == 6 {
                forward |= masked_pos >> 16;
            }
        }
        let legal_forward = forward;

        mv_list.readMoves(squarePos, legal_attacks.0 | legal_forward, (file == 1) | (file == 6));
    }

    pub fn gen_knight_moves(&self, mv_list: &mut MoveList, square_pos: u8) {
        let turn = self.turn.index();
        let attacks = attacks::KNIGHT_ATTACKS[square_pos as usize];
        let opponent = self.colors[1 - turn];
        mv_list.readMoves(square_pos, (opponent & attacks).0, false);
    }

    pub fn gen_rook_moves(&self, mv_list: &mut MoveList, square_pos: u8) {
        let turn = self.turn.index();
        let opponent = self.colors[1 - turn];
        let attacks = attacks::sliding_attack(square_pos, self.occupied(), false);
        mv_list.readMoves(square_pos, (opponent & attacks).0, false);
    }

    pub fn gen_bishop_moves(&self, mv_list: &mut MoveList, square_pos: u8) {
        let turn = self.turn.index();
        let opponent = self.colors[1 - turn];
        let attacks = attacks::sliding_attack(square_pos, self.occupied(), true);
        mv_list.readMoves(square_pos, (opponent & attacks).0, false);
    }

    pub fn gen_queen_moves(&self, mv_list: &mut MoveList, square_pos: u8) {
        let turn = self.turn.index();
        let attacks = attacks::sliding_attack(square_pos, self.occupied(), true) |
                                attacks::sliding_attack(square_pos, self.occupied(), false);
        let opponent = self.colors[1 - turn];
        mv_list.readMoves(square_pos, (opponent & attacks).0, false);
    }

    pub fn gen_king_moves(&self, mv_list: &mut MoveList, square_pos: u8) {
        let turn = self.turn.index();
        let attacks = attacks::KING_ATTACKS[square_pos as usize];
        let opponent = self.colors[1 - turn];
        mv_list.readMoves(square_pos, (opponent & attacks).0, false);
    }

    pub fn gen_duck_moves(&self) -> Bitboard {
        let mut filled = Bitboard::EMPTY;
        for occ_board in &self.pieces {
            filled |= Bitboard(occ_board.value());
        }
        !filled
    }

    pub fn gen_en_pessant(&self) {
        
    }

    // Fill in movegen fields

    pub fn init_checks(&mut self) {

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

    fn is_valid_bishop_move(&self, from: u8, to: u8) -> bool {true}
    fn is_valid_knight_move(&self, from: u8, to: u8) -> bool {true}
    fn is_valid_rook_move(&self, from: u8, to: u8) -> bool {true}
    fn is_valid_queen_move(&self, from: u8, to: u8) -> bool {true}
    fn is_valid_king_move(&self, from: u8, to: u8) -> bool {true}
}