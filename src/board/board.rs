

use super::{bitboard::Bitboard, types::{Color, Piece, PieceType}, zobrist};

pub struct PlayerInfo {
    color: Color,
    castling_rights: bool,
    time_remaining: f64,
    increment: f64,
}

pub struct Board {
    // a1 -> 0, h1 -> 7, a8 -> 56, h8 -> 63
    // board: [[Square; 8]; 8],
    pieces: [Bitboard; PieceType::COUNT],
    colors: [Bitboard; Color::COUNT],
    turn: Color,
    castling: [bool; 4], // order KQkq
    // attackBitboards: [Bitboards],
    en_pessant: u8, // Defaults to u8::max if impossible, otherwise 0-63 for en-pessantable square
    hash: u64,
}

impl Board {

    pub fn empty() -> Board {
        Board {
            pieces: [Bitboard::EMPTY; PieceType::COUNT],
            colors: [Bitboard::EMPTY; Color::COUNT],
            turn: Color::NONE,
            castling: [false; 4],
            en_pessant: u8::MAX,
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


    // Hashing

    pub fn generate_zobrist_hashing(&mut self) {
        self.hash = zobrist::en_pessant_key(self.en_pessant);
        for idx in 1..4 {
            if self.castling[idx] {
                self.hash ^= zobrist::castle_key(idx);
            }
        }
        if self.turn.index() == 0 {
            self.hash ^= zobrist::white_to_move_key();
        }
        // Add hashing for pieces below


    }

    // Parsing UCI communications

    pub fn parse_FEN(fen_opt: Option<&str>) -> Board {

        // Update individual piece occupation bitboards
        // let content = FEN.split(" ").collect::<Vec<&str>>();
        let mut fen: &str = match fen_opt {
            Some(fen) => fen,
            None => {
                println!("Error in FEN parsing - No FEN provided. Defaulting to starting position.");
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" // Game starting position
            },
        };

        let [board_info, color_info, castling_info, ep_info] = fen.split_whitespace().collect::<Vec<&str>>()[0..4] else {
            println!("info string invalid fen");
            return Board::empty();
        };

        let mut board : Board = Board::empty();

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
        if castling_str != "-" {
            let valid_tokens = ["K", "Q", "k", "q"];
            for idx in 1..4 {
                if castling_str.contains(valid_tokens[idx]) {
                    board.castling[idx] = true;
                }
            }
        }

        let ep_tokens = ep_info.chars().collect::<Vec<char>>();
        if ep_tokens[0] != '-' {
            board.en_pessant = 8 * ((ep_tokens[1] as u8) - b'0') + "abcdefgh".find(ep_tokens[0]).unwrap() as u8;
        }
        
        board.generate_zobrist_hashing();

        board

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
        // Call a bunch of makeMoves on a position object
        let initial_position = Board::parse_FEN(None);
        
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