
use crate::chess::position;
use crate::board::types::{Piece, Color, PieceType};

pub struct Board {
    board: [[Square; 8]; 8],
}

impl Board {
    pub fn setSquare(&mut self, squarePos: u8, piece: Piece) {
        let row : u8 = squarePos % 8;
        self.board[row][squarePos - 8 * row].setPiece(piece);
    }

    pub fn setSquares(&mut self, startPos: u8, endPos: u8, pieces: Piece) {
        for idx in startPos..endPos {
            self.setSquare(idx, &pieces)
        }
    }

    pub fn getSquare(&self, squarePos: u8) -> &Square {
        let row : u8 = squarePos % 8;
        &self.board[row as usize][(squarePos - 8 * row) as usize]
    }

    pub fn clearSquare(&mut self, squarePos: u8) {
        self.setSquare(squarePos, Piece {});
    }
}

pub struct Square {
    piece: Piece,
    color: Color,
}

impl Square {
    pub fn set_piece(&mut self, newPiece: Piece) {
        self.piece = newPiece;
    }

    pub fn get_piece(&self) -> &Piece {
        &self.piece
    }

    pub fn get_ASCII(&self) -> &str {
        let symbols : [&str; 16] = ["♔", "♕", "♖", "♗", "♘", "♙", "🦆", " ", "♚", "♛", "♜", "♝", "♞", "♟︎", "🦆", " "];
        let mut index : u8 = match self.piece.get_type() {
            PieceType::KING => 0,
            PieceType::QUEEN => 1,
            PieceType::ROOK => 2,
            PieceType::BISHOP => 3,
            PieceType::KNIGHT => 4,
            PieceType::PAWN => 5,
            PieceType::DUCK => 6,
            PieceType::NONE => 7,
        };
        index += match self.piece.get_color() {
            Color::WHITE => 7,
            Color::BLACK => 0,
            Color::YELLOW => 0,
        };
        symbols[index as usize]
    }
}
