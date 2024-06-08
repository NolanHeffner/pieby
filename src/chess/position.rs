use crate::board::Board;
use crate::board::Types;

struct PlayerInfo {
    color: Types::Color,
    castling_rights: bool,
    time_remaining: f64,
    increment: f64,
}

struct Position {
    board: Board::Board,
    turn: Types::Color,
    castling: [bool; 2],
    // attackBitboards: [Bitboards],
}

impl Position {
    fn makeMove(&self, startPos: u8, endPos: u8) {
        let piece : &Square = self.board.getSquare(startPos);
        self.board.clearSquare(startPos);
        self.board.fillSquare(endPos, piece);
    }
}
