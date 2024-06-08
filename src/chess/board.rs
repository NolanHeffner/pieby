use crate::chess::Position;
use crate::board::Types;

struct Board {
    board: [[Square; 8]; 8],
}

impl Board {
    fn parseFEN(&mut self) {
        // Update individual piece occupation bitboards
        
    }

    fn parsePGN() {

    }

    fn getSquare(&self, squarePos: u8) -> &Square {
        let row : u8 = squarePos % 8;
        &self.board[row][squarePos - 8 * row]
    }

    fn clearSquare(&mut self, squarePos: u8) {
        let row : u8 = squarePos % 8;
        self.board[row][squarePos - 8 * row].setPiece(None);
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
