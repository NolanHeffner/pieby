
use crate::board::{board::Board, types::{self, Color, PieceType}};

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

    fn empty() -> Position {
        Position {
            board: Board::empty(),
            turn: Color::NONE,
            castling: [false; 4],
        }
    }

    fn parse_FEN(&mut self, fen: &str) -> Position {

        // Update individual piece occupation bitboards
        // let content = FEN.split(" ").collect::<Vec<&str>>();

        let board : Board = Board::empty();

        let [board_info, color_info, castling_info, ep_info] = fen.split_whitespace().collect::<Vec<&str>>()[0..4] else {
            println!("info string invalid fen");
            return Position::empty();
        };

        let mut tokens: Vec<char> = board_info.chars().collect::<Vec<char>>();
        tokens.retain(|&x| x != ' ');

        let mut rank: u8 = 7;
        let mut file: u8 = 0;
        
        for token in tokens {
            match token {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    file += (token as u8) - b'0' // Need to check this actually works
                }
                '_' => {
                    board.set_square(8 * rank + file, PieceType::new("".find())); // Need to fix
                }
            }
        }

        let turn = Color::NONE;
        let castling = [false; 4];

        Position {
            board,
            turn,
            castling
        }

        // There are 6 fields to the FEN, separated by spaces. Field 1 is piece placement
        // let rows = content[0].split("/").collect::<Vec<&str>>();
        /* 
        for (index, row) in rows.iter().enumerate() {
            let tokens : Vec<char> = row.chars().collect();
            let column : u8 = 0;
            for token in tokens.iter() {
                /* let allowed_tokens : [str; 1] = ["a"];
                if !allowed_tokens.contains(&token) {
                    panic!()
                } else */ 
                let start_pos = 8 * row + column;
                if token.is_numeric() {
                    let token_num : usize = (token.to_string().as_bytes()[0] - 48) as usize;
                    let endPos = 8 * row + column + token_num - 1;
                    self.board.setSquares(start_pos, endPos, types::PieceType::NONE);
                    column += token_num;
                } else {
                    self.board.setSquare(start_pos as u8, &types::Piece::new(
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
        } */
        // Field 3 is the ability for either side to castle



    }

    fn parsePGN(&mut self, PGN: &str) {

    }
}
