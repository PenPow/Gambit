use std::collections::HashMap;
use colored::Colorize;

use crate::board::zobrist::ZobristKey;

const SIZE_IN_MB: usize = 16384;
const SIZE_IN_BYTES: usize = SIZE_IN_MB * (1024 ^ 2);

const BYTES_PER_ENTRY: usize = std::mem::size_of::<TTEntry>();

const MAX_ENTRIES: usize = SIZE_IN_BYTES / BYTES_PER_ENTRY;

pub struct TTEntry {
	pub hash: ZobristKey,
	pub nodes: u64,
}

pub struct TranspositionTable {
	table: HashMap<ZobristKey, TTEntry>,
	size: usize,
}

impl TranspositionTable {
	pub fn new() -> Self {
		Self {
			table: HashMap::with_capacity(MAX_ENTRIES),
			size: MAX_ENTRIES
		}
	}

	pub fn clear(&mut self) {
		eprintln!("{}", "Clearing transposition table".yellow());

		self.table.clear()
	}

	pub fn get(&mut self, key: ZobristKey) -> Option<&TTEntry> {
		self.table.get(&key)
	}

	pub fn insert(&mut self, key: ZobristKey, value: TTEntry) {
		if self.table.len() >= self.size {
			self.clear()
		}

		self.table.insert(key, value);
	}
}