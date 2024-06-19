
use crate::board::{board::Board, types::{self, Color, Piece, PieceType}};

pub struct PlayerInfo {
    color: types::Color,
    castling_rights: bool,
    time_remaining: f64,
    increment: f64,
}

pub struct Position {
    board: Board,
    turn: types::Color,
    castling: [bool; 4], // order KQkq
    // attackBitboards: [Bitboards],
    en_pessant: u8, // Defaults to u8::max if impossible, otherwise 0-63 for en-pessantable square
}

impl Position {

    fn empty() -> Position {
        Position {
            board: Board::empty(),
            turn: Color::NONE,
            castling: [false; 4],
            en_pessant: 0,
        }
    }

    fn parse_FEN(&mut self, fen_opt: Option<&str>) -> Position {

        // Update individual piece occupation bitboards
        // let content = FEN.split(" ").collect::<Vec<&str>>();
        let mut fen: &str = match fen_opt {
            Some(fen) => fen,
            None => {
                println!("Error in FEN parsing - No FEN provided. Defaulting to starting position.");
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" // Game starting position
            },
        };

        let mut board : Board = Board::empty();

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
                _ => {
                    // Index -> 7 * color_index + piece_index
                    let idx = "KQRBNPDkqrbnpd".find(token).unwrap();
                    let color = Color::new(idx - idx % 6);
                    let piece_type = PieceType::new(idx % 6);

                    board.set_square(8 * rank + file, &Piece::new(piece_type, color));

                    file += 1
                }
            }
        }

        let turn = Color::new((color_info == "b") as usize);

        let castling_str = String::from(castling_info);
        let mut castling = [false; 4];
        if castling_str != "-" {
            let valid_tokens = ["K", "Q", "k", "q"];
            for idx in 1..4 {
                if castling_str.contains(valid_tokens[idx]) {
                    castling[idx] = true;
                }
            }
        }

        let mut en_pessant = u8::MAX;
        let ep_tokens = ep_info.chars().collect::<Vec<char>>();
        if ep_tokens[0] != '-' {
            en_pessant = 8 * ((ep_tokens[1] as u8) - b'0') + "abcdefgh".find(ep_tokens[0]).unwrap() as u8;
        }
        
        Position {
            board,
            turn,
            castling,
            en_pessant,
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
