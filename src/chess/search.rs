
use std::cmp::{max, min};
use crate::board::board::Board;
use super::{evaluation::evaluation, movegen, transposition_table::TranspositionTable};

struct Search {
    board: Board,
    ttable: TranspositionTable,
}

// Need to implement check for this later
const GAME_OVER : bool = false;

impl Search {

    pub fn minimax(board: Board, depth: u8, alpha: i16, beta: i16, is_maxim: bool) -> i16 {
        // Edge cases to return static evaluation
        if depth == 0 || GAME_OVER {
            return evaluation(&board)
        }

        if is_maxim {
            let max_eval = i16::MIN;
            for mv in movegen.gen_legal_moves() {
                let child = ...
                eval = minimax(child, depth - 1, alpha, beta, false);
                max_eval = max(alpha, eval)
            }
            return max_eval
        } else {
            let min_eval = i16::MAX;
            for mv in movegen.gen_legal_moves() {
                let child = ...
                eval = minimax(child, depth - 1, alpha, beta, true);
                max_eval = min(alpha, eval)
            }
            return min_eval
        }
    }

    pub fn go(starting_pos: Board) {

    }

    
}