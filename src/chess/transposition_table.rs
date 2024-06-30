#![allow(unused)]

use dashmap::DashMap;
use std::sync::Arc;
use crate::chess::mv::Move;

// Type definitions

type ZobristHash = u64;

#[derive(Clone, Copy)]
enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
pub struct TTableEntry {
    key: u64, // Zobrist hash key
    eval: i16,
    pos_move: Option<Move>,
    node_type: NodeType,
    depth: u8,
}

// Transposition table implementation

pub struct TranspositionTable(Arc<DashMap<ZobristHash, TTableEntry>>);

impl TranspositionTable {
    pub fn empty() -> Self {
        TranspositionTable(Arc::new(DashMap::default()))
    }

    pub fn new(dashmap: DashMap<ZobristHash, TTableEntry>) -> Self {
        TranspositionTable(Arc::new(dashmap))
    }

    pub fn insert(&self, hash: ZobristHash, entry: TTableEntry) {
        self.0.insert(hash, entry);
    }

    pub fn probe(&self, hash: ZobristHash) -> TTableEntry {
        self.0.get(&hash).map(|entry| *entry.value()).expect("Transposition table lookup failed.")
    }

    // Note from Nolan: Need to verify if this does what I think it does
    fn lookup(&self, hash: ZobristHash, depth: u8, alpha: i16, beta: i16) -> Option<(i16, Option<Move>)> {
        let entry = self.probe(hash);
        if entry.depth >= depth {
            return match entry.node_type {
                NodeType::Exact => return Some((entry.eval, entry.pos_move)),
                NodeType::LowerBound if entry.eval >= beta => return Some((entry.eval, entry.pos_move)),
                NodeType::UpperBound if entry.eval <= alpha => return Some((entry.eval, entry.pos_move)),
                _ => None
            }
        }
        None
    }
}