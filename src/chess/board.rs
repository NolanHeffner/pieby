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

    fn setSquares(&mut self, squarePos: &Vec<u8>, pieces: Types::Piece) {
        for idx in 1..len(squarePos) {
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
}
