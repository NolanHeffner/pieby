
mod board;

use crate::chess::Position;
use crate::board::Types;

struct Board {
    board: [[Square; 8]; 8],
}

impl Board {
    fn setSquare(&mut self, squarePos: u8, piece: Types::Piece) {
        let row : u8 = squarePos % 8;
        self.board[row][squarePos - 8 * row].setPiece(piece);
    }

    fn setSquare(&mut self, squarePos: u8, piece: Types::Piece) {
        let row : u8 = squarePos % 8;
        self.board[row][squarePos - 8 * row].setPiece(piece);
    }

    fn setSquares(&mut self, startPos: u8, endPos: u8, pieces: Types::Piece) {
        for idx in startPos..endPos {
            self.setSquare(squarePos.get(idx), pieces)
        }
    }

    fn getSquare(&self, squarePos: u8) -> &Square {
        let row : u8 = squarePos % 8;
        &self.board[row][squarePos - 8 * row]
    }

    fn clearSquare(&mut self, squarePos: u8) {
        self.setSquare(squarePos, Types::Piece::NONE);
    }
}

struct Square {
    piece: Option<Types::Piece>,
    color: Types::Color,
}

impl Square {
    fn setPiece(&mut self, newPiece : Piece) {
        self.piece = newPiece;
    }

    fn getPiece(&self) -> &Piece {
        &self.piece
    }

    fn get_ASCII(&self) -> &str {
        let symbols : [&str; 14] = ["♔", "♕", "♖", "♗", "♘", "♙", "🦆", "♚", "♛", "♜", "♝", "♞", "♟︎", "🦆"];
        let index : u8 = match self.piece {
            KING => 0,
            QUEEN => 1,
            ROOK => 2,
            BISHOP => 3,
            KNIGHT => 4,
            PAWN => 5,
            DUCK => 6,
        };
        index += match self.color {
            WHITE => 7,
            BLACK => 0,
            YELLOW => 0,
        };
        symbols[index]
    }
}
