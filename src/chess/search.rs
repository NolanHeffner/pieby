#![allow(unused)]

use std::{
    cmp::{max, min}, 
    thread
};

use crate::{
    board::board::Board, 
    chess::{
        evaluation::evaluation, 
        movegen, 
        mv::Move, 
        transposition_table::TranspositionTable
    }
};

struct Search {
    board: Board,
    ttable: TranspositionTable,
}

impl Search {

    pub fn go(&self, board: Board, depth: u8, threads: u8) {
        // Ignores thread count for now, lazy SMP implementation will be a hassle to deal with after single-threaded search is done
        thread::scope(|s| {
            for thread_idx in 0..threads {
                s.spawn(|| {
                    // Copy into thread
                    let mut board = board.clone();
                    
                });
            }
        })
    }

    pub fn stop(&self) {

    }

    fn iterative_deepening(&mut self) {

    }

    fn minimax(board: Board, depth: u8, alpha: i16, beta: i16, is_maxim: bool) -> i16 {
        // Edge cases to return static evaluation
        let mut moves : MoveList = movegen::MoveGen::gen_moves();
        if depth == 0 || moves.len() {
            //return evaluation(&board)
            return Self::qsearch(board, moves);
            // Would normally return qsearch if depth == 0 (qsearch should return static position eval if game over)
        }

        // Movegen + move ordering
        moves.order_moves();

        if is_maxim {
            let max_eval = i16::MIN;
            for mv in moves {
                board.make_move(mv);
                let child = board;
                let eval = Self::minimax(child, depth - 1, alpha, beta, false);
                max_eval = max(alpha, eval);
            }
            return max_eval
        } else {
            let min_eval = i16::MAX;
            for mv in moves {
                board.make_move(mv);
                let child = board;
                let eval = Self::minimax(child, depth - 1, alpha, beta, true);
                min_eval = min(alpha, eval);
            }
            return min_eval
        }
    }

    // qsearch, aka Quiescence search, was I believe introduced by the Stockfish team? It essentially searches down all captures, checks, evasions, etc. until the position is quiet - it then returns the static position of the quiet positions. Aka makes sure to get all tactics. Will leave out until later versions.
    fn qsearch(board: Board, moves: Vec<Move>) -> i16 {
        evaluation(&board)
    }

    // Move ordering is really important to maximize the benefit of alpha-beta pruning. I will implement later.
    fn order_moves(board: Board, moves: Vec<Move>) -> Vec<Move> {
        // Implementation of move ordering
        moves
    }
}