use std::collections::HashMap;
use crate::zobrist::Zobrist;

pub struct TranspositionTable {
    table: HashMap<u64, i32>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
        }
    }

    pub fn store(&mut self, zobrist_hash: u64, eval: i32) {
        self.table.insert(zobrist_hash, eval);
    }

    pub fn retrieve(&self, zobrist_hash: u64) -> Option<i32> {
        self.table.get(&zobrist_hash).copied()
    }
}
