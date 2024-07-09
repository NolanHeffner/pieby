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

use super::movegen::MoveList;

struct Search {
    board: Board,
    ttable: TranspositionTable,
}

impl Search {

    pub fn go(&self, board: Board, depth: u8, threads: u8) {
        // Ignores thread count for now, lazy SMP implementation will be a hassle to deal with after single-threaded search is done
        thread::scope(|s| {
            // Consider separating to have 1 master thread
            for thread_idx in 0..threads {
                s.spawn(|| {
                    // Copy into thread
                    let mut board = board.clone();
                    iterative_deepening(board, ...)
                });
            }
        })
    }

    pub fn stop(&self) {

    }

    fn iterative_deepening(board: &mut Board, is_white: bool) {
        let mut depth = 0;
        let stopped = false;
        let mut eval = 0;
        while !stopped {
            eval = Self::alphabeta(board, depth, is_white);
            depth += 1;
        }
    }

    fn alphabeta(board: &mut Board, depth: u8, is_white: bool) -> i16 {
        minimax(board, depth, ...., is_white)
    }

    fn minimax(board: &mut Board, depth: u8, alpha: i16, beta: i16, is_maxim: bool) -> i16 {
        // Edge cases to return static evaluation
        let mut moves : MoveList = board.gen_moves();
        if depth == 0 || moves.len() == 0 {
            //return evaluation(&board)
            return Self::qsearch(board, moves);
            // Would normally return qsearch if depth == 0 (qsearch should return static position eval if game over)
        }

        // Movegen + move ordering
        moves = Self::order_moves(board, moves);

        let is_maxim = board.
        if is_maxim {
            let mut max_eval = i16::MIN;
            for mv in moves {
                board.make_move(mv);
                let eval = Self::minimax(board, depth - 1, alpha, beta, false);
                max_eval = max(alpha, eval);
                board.unmake_move(mv);
            }
            return max_eval
        } else {
            let mut min_eval = i16::MAX;
            for mv in moves {
                board.make_move(mv);
                let eval = Self::minimax(board, depth - 1, alpha, beta, true);
                min_eval = min(alpha, eval);
                board.unmake_move(mv);
            }
            return min_eval
        }
    }

    // qsearch, aka Quiescence search, was I believe introduced by the Stockfish team? It essentially searches down all captures, checks, evasions, etc. until the position is quiet - it then returns the static position of the quiet positions. Aka makes sure to get all tactics. Will leave out until later versions.
    fn qsearch(board: &Board, moves: MoveList) -> i16 {
        evaluation(&board)
    }

    // Move ordering is really important to maximize the benefit of alpha-beta pruning. I will implement later.
    fn order_moves(board: &Board, moves: MoveList) -> MoveList {
        // Implementation of move ordering
        moves
    }
}