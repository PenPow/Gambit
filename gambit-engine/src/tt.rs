use crate::search::MATE_VALUE;
use gambit_models::moves::Move;

pub const MATE_THRESHOLD: i32 = MATE_VALUE - 128;

const TT_SIZE_MB: u64 = 64;
const TT_CAPACITY: usize = {
    let bytes = TT_SIZE_MB * 1_048_576;
    let entry_size = size_of::<TTEntry>() as u64;
    let capacity = (bytes / entry_size) as usize;
    capacity.next_power_of_two()
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TTEntry {
    pub depth: u16,
    pub hash: u64,
    pub score: i32,
    pub best_move: Move,
    pub entry_type: EntryType,
    pub generation: u8,
}

impl Default for TTEntry {
    fn default() -> Self {
        Self {
            depth: 0,
            hash: 0,
            score: 0,
            generation: 0,
            best_move: Move::NULL,
            entry_type: EntryType::Exact,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Exact,
    UpperBound,
    LowerBound,
}

pub struct TranspositionTable {
    table: Box<[TTEntry]>,
    mask: u64,
    generation: u8,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: vec![TTEntry::default(); TT_CAPACITY].into_boxed_slice(),
            mask: (TT_CAPACITY - 1) as u64,
            generation: 0,
        }
    }

    #[inline]
    pub fn lookup(
        &self,
        hash: u64,
        depth: u16,
        ply: u16,
        alpha: i32,
        beta: i32,
    ) -> Option<(i32, Move)> {
        let entry = &self.table[(hash & self.mask) as usize];

        if entry.hash != hash {
            return None;
        }

        if entry.depth < depth {
            return None;
        }

        let score = {
            let ply = ply as i32;

            if entry.score >= MATE_THRESHOLD {
                entry.score - ply
            } else if entry.score <= -MATE_THRESHOLD {
                entry.score + ply
            } else {
                entry.score
            }
        };

        match entry.entry_type {
            EntryType::Exact => Some((score, entry.best_move)),
            EntryType::UpperBound => {
                if score <= alpha {
                    Some((score, entry.best_move))
                } else {
                    None
                }
            }
            EntryType::LowerBound => {
                if score >= beta {
                    Some((score, entry.best_move))
                } else {
                    None
                }
            }
        }
    }

    #[inline]
    pub fn probe_move(&self, hash: u64) -> Option<Move> {
        let entry = &self.table[(hash & self.mask) as usize];
        if entry.hash == hash && entry.best_move != Move::NULL {
            Some(entry.best_move)
        } else {
            None
        }
    }

    #[inline]
    pub fn store(
        &mut self,
        hash: u64,
        depth: u16,
        ply: u16,
        score: i32,
        best_move: Move,
        entry_type: EntryType,
    ) {
        let index = (hash & self.mask) as usize;
        let entry = &mut self.table[index];

        let stored_score = {
            let ply = ply as i32;

            if score >= MATE_THRESHOLD {
                score + ply
            } else if score <= -MATE_THRESHOLD {
                score - ply
            } else {
                score
            }
        };

        if entry.hash == hash {
            if depth < entry.depth {
                return;
            }

            if entry.entry_type != EntryType::Exact
                && entry_type != EntryType::Exact
                && entry.entry_type == entry_type
            {
                match entry_type {
                    EntryType::UpperBound if stored_score >= entry.score => return,
                    EntryType::LowerBound if stored_score <= entry.score => return,
                    _ => {}
                }
            }
        } else {
            if entry.generation == self.generation && entry.depth > depth {
                return;
            }
        }

        entry.hash = hash;
        entry.score = stored_score;
        entry.depth = depth;
        entry.generation = self.generation;
        entry.entry_type = entry_type;
        entry.best_move = best_move;
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = TTEntry::default();
        }

        self.generation = 0
    }

    pub fn new_search(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    pub fn hashfull(&self) -> u32 {
        const SAMPLE_SIZE: usize = 1000;
        let sample_size = SAMPLE_SIZE.min(self.table.len());

        let filled = self.table[..sample_size]
            .iter()
            .filter(|entry| entry.hash != 0)
            .count();

        ((filled * 1000) / sample_size) as u32
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}
