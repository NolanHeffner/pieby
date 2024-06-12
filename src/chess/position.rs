
use crate::board::{board::{Square, Board}, types};

struct PlayerInfo {
    color: types::Color,
    castling_rights: bool,
    time_remaining: f64,
    increment: f64,
}

struct Position {
    board: Board,
    turn: types::Color,
    castling: [bool; 4], // order KQkq
    // attackBitboards: [Bitboards],
}

impl Position {
    fn makeMove(&self, startPos: u8, endPos: u8) {
        let piece : &Square = self.board.getSquare(startPos);
        self.board.clearSquare(startPos);
        self.board.fillSquare(endPos, piece);
    }

    fn parseFEN(&mut self, FEN: &str) {
        // Update individual piece occupation bitboards
        let content = FEN.split(" ").collect::<Vec<&str>>();
        // There are 6 fields to the FEN, separated by spaces. Field 1 is piece placement
        let rows = content[0].split("/").collect::<Vec<&str>>();
        for row in rows.into_iter() {
            let tokens : Vec<char> = row.chars().collect();
            let column : u8 = 0;
            for token in tokens.iter() {
                /* let allowed_tokens : [str; 1] = ["a"];
                if !allowed_tokens.contains(&token) {
                    panic!()
                } else */ if token.is_numeric() {
                    let token_num : u8 = token.to_string().as_bytes()[0] - 48;
                    let startPos = 8 * row + column;
                    let endPos = 8 * row + column + token_num - 1;
                    self.board.setSquares(startPos, endPos, types::PieceType::NONE);
                    column += token_num;
                } else {
                    self.board.setSquare(types::Piece::new(
                        types::PieceType::PAWN,
                        match token.is_ascii_uppercase() {
                            true => types::Color::WHITE,
                            false => types::Color::BLACK
                        },
                    ));
                    column += 1;
                }
            }
            
        }
        // Field 2 is the color to move
        self.turn = match content[1] {
            "w" => Types::Color::WHITE,
            "b" => Types::Color::BLACK,
        }
        // Field 3 is the ability for either side to castle



    }

    fn parsePGN(&mut self, PGN: &str) {

    }
}
