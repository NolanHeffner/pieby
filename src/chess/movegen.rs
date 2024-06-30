
use crate::board::{
    bitboard::Bitboard, 
    board::Board
};

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