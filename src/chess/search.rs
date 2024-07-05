
use std::cmp::{max, min};
use crate::board::{board::Board, types::PieceType};
use super::{evaluation::evaluation, movegen, mv::Move, transposition_table::TranspositionTable};

struct Search {
    board: Board,
    ttable: TranspositionTable,
}

impl Search {

    pub fn go(&self, starting_pos: Board, depth: u8) {
        
    }

    pub fn stop(&self) {

    }

    fn minimax(board: Board, depth: u8, alpha: i16, beta: i16, is_maxim: bool) -> i16 {
        // Edge cases to return static evaluation
        let mut moves = movegen.gen_legal_moves();
        if depth == 0 || board.is_game_over() {
            return evaluation(&board)
            // Would normally return qsearch if depth == 0 (qsearch should return static position eval if game over)
        }

        // Movegen + move ordering
        moves.order_moves();

        if is_maxim {
            let max_eval = i16::MIN;
            for mv in moves {
                let child = ...
                eval = minimax(child, depth - 1, alpha, beta, false);
                max_eval = max(alpha, eval)
            }
            return max_eval
        } else {
            let min_eval = i16::MAX;
            for mv in moves {
                let child = ...
                eval = minimax(child, depth - 1, alpha, beta, true);
                max_eval = min(alpha, eval)
            }
            return min_eval
        }
    }

    // qsearch, aka Quiescence search, was I believe introduced by the Stockfish team? It essentially searches down all captures, checks, evasions, etc. until the position is quiet - it then returns the static position of the quiet positions. Aka makes sure to get all tactics. Will leave out until later versions.
    fn qsearch() -> i16 {
        0
    }

    // Move ordering is really important to maximize the benefit of alpha-beta pruning. I will implement later.
    fn order_moves(moves: Vec<Move>, position: &Board) -> Vec<Move> {
        Vec::from([Move::new(0, 0, false, PieceType::KNIGHT)]) // Just to shut up return type error
        // Implementation of move ordering
    }


    
}