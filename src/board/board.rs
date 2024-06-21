

use crate::board::types::{Color, PieceType};
use super::{bitboard::Bitboard, types::Piece};

pub struct Board {
    // a1 -> 0, h1 -> 7, a8 -> 56, h8 -> 63
    // board: [[Square; 8]; 8],
    pieces: [Bitboard; PieceType::COUNT],
    colors: [Bitboard; Color::COUNT],
    hash: u64,
}

impl Board {

    pub fn empty() -> Board {
        Board {
            pieces: [Bitboard::EMPTY; PieceType::COUNT],
            colors: [Bitboard::EMPTY; Color::COUNT],
            hash: 0,
        }
    }

    pub fn make_move(&mut self, start_pos: u8, end_pos: u8) {
        //let piece : &Piece = self.getSquare(startPos).get_piece();
        //self.getSquare(endPos).set_piece(piece);
        //self.clearSquare(startPos);
        for board in &self.pieces {
            if board.bit_at_pos(start_pos) == 1 {
                if self.is_move_legal() {
                    board.set_bit(start_pos, false);
                    board.set_bit(end_pos, true);
                }
            }
        }
    }

    pub fn is_move_legal(&self) -> bool {
        true
    }
    
    pub fn set_square(&mut self, square_pos: u8, piece: &Piece) {
        // Need to write extra code to ensure that we overwrite previous occupancies
        self.pieces[piece.get_type().index()].set_bit(square_pos, true);
        self.colors[piece.get_color().index()].set_bit(square_pos, true);
    }

    pub fn get_square(&self, square_pos: u8) -> Piece {
        let mut piece_type : PieceType = PieceType::NONE;
        for idx in 1..PieceType::COUNT {
            if self.pieces[idx].bit_at_pos(square_pos) == 1 {
                piece_type = PieceType::new(idx);
            }
        }

        let mut piece_color : Color = Color::NONE;
        for idx in 1..Color::COUNT {
            if self.colors[idx].bit_at_pos(square_pos) == 1 {
                piece_color = Color::new(idx);
            }
        }

        Piece::new(piece_type, piece_color)
    }

    pub fn get_rank(rank: u8) -> Bitboard {
        if rank > 7 {return Bitboard::new(0)}
        let shift = 8 * rank - 8;
        // print_bitboard((0xFF as u64) << shift)
        Bitboard::new((0xFF as u64) << shift)
    }
    
    pub fn get_file(file: u8) -> Bitboard {
        if file > 7 {return Bitboard::new(0)}
        Bitboard::new(0x0101010101010101 << (file - 1))
    }
}

/*
pub struct Square {
    piece: Piece,
    color: Color,
}

impl Square {
    pub fn set_piece(&mut self, newPiece: Piece) {
        self.piece = newPiece;
    }

    pub fn get_piece(&self) -> Piece {
        self.piece
    }

    pub fn get_ASCII(&self) -> &str {
        
        symbols[self.piece.index()]
    }
}
*/