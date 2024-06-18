

use crate::board::types::{Color, PieceType};
use super::{bitboard::Bitboard, types::Piece};

pub struct Board {
    // a1 -> 1, h1 -> 8, a8 -> 57, h8 -> 64
    // board: [[Square; 8]; 8],
    pieces: [Bitboard; PieceType::COUNT],
    colors: [Bitboard; Color::COUNT],
}

impl Board {

    pub fn empty() -> Board {
        Board {
            pieces: [Bitboard::EMPTY; PieceType::COUNT],
            colors: [Bitboard::EMPTY; Color::COUNT],
        }
    }

    pub fn make_move(&mut self, startPos: u8, endPos: u8) {
        //let piece : &Piece = self.getSquare(startPos).get_piece();
        //self.getSquare(endPos).set_piece(piece);
        //self.clearSquare(startPos);
        for board in &self.pieces {
            if (board.bit_at_pos(startPos) == 1) {
                if self.is_move_legal() {
                    board.set_bit(startPos, false);
                    board.set_bit(endPos, true);
                }
            }
        }
    }

    pub fn is_move_legal(&self) -> bool {
        true
    }
    
    pub fn set_square(&mut self, squarePos: u8, piece: &Piece) {
        // Need to write extra code to ensure that we overwrite previous occupancies
        self.pieces[piece.get_type().index()].set_bit(squarePos, true);
        self.colors[piece.get_color().index()].set_bit(squarePos, true);
    }

    pub fn get_square(&self, squarePos: u8) -> Piece {
        let mut piece_type : PieceType = PieceType::NONE;
        for idx in 1..PieceType::COUNT {
            if self.pieces[idx].bit_at_pos(squarePos) == 1 {
                piece_type = PieceType::new(idx);
            }
        }

        let mut piece_color : Color = Color::NONE;
        for idx in 1..Color::COUNT {
            if self.colors[idx].bit_at_pos(squarePos) == 1 {
                piece_color = Color::new(idx);
            }
        }

        Piece::new(piece_type, piece_color)
    }

    /*
    pub fn setSquare(&mut self, squarePos: u8, piece: &Piece) {
        let row : usize = (squarePos as usize) % 8;
        self.board[row][squarePos as usize - 8 * row].set_piece(&piece);
    }

    pub fn setSquares(&mut self, startPos: u8, endPos: u8, pieces: Piece) {
        for idx in startPos..endPos {
            self.setSquare(idx, &pieces)
        }
    }

    pub fn getSquare(&mut self, squarePos: u8) -> &mut Square {
        let row : u8 = squarePos % 8;
        &mut self.board[row as usize][(squarePos - 8 * row) as usize]
    }

    pub fn clearSquare(&mut self, squarePos: u8) {
        self.setSquare(squarePos, Piece {});
    }
    */
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