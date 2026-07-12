// TODO:
// Determine whether this is providing an ELO boost or not

#[derive(Debug, Clone)]
pub struct RepetitionTable {
    entries: [RepetitionEntry; RepetitionTable::CAPACITY],
    len: usize,
}

#[derive(Debug, Clone, Copy)]
struct RepetitionEntry {
    hash: u64,
    count: u8,
}

impl RepetitionTable {
    const CAPACITY: usize = 4096;

    pub fn new() -> Self {
        Self {
            entries: [RepetitionEntry { hash: 0, count: 0 }; RepetitionTable::CAPACITY],
            len: 0,
        }
    }

    pub fn from_history(hashes: impl IntoIterator<Item = u64>) -> Self {
        let mut table = Self::new();
        table.fill(hashes);

        table
    }

    pub fn fill(&mut self, hashes: impl IntoIterator<Item = u64>) {
        for hash in hashes {
            self.push(hash)
        }
    }

    pub fn push(&mut self, hash: u64) {
        for entry in &mut self.entries[..self.len] {
            if entry.hash == hash {
                entry.count += 1;
                return;
            }
        }

        if self.len < self.entries.len() {
            self.entries[self.len] = RepetitionEntry { hash, count: 1 };
            self.len += 1;
        }
    }

    pub fn pop(&mut self, hash: u64) {
        for entry in &mut self.entries[..self.len] {
            if entry.hash == hash {
                entry.count -= 1;
                if entry.count == 0 {
                    self.entries[self.len - 1] = RepetitionEntry { hash: 0, count: 0 };
                    self.len -= 1;
                }
                return;
            }
        }
    }

    #[inline]
    pub fn is_repeated(&self, hash: u64) -> bool {
        self.entries[..self.len]
            .iter()
            .any(|entry| entry.hash == hash && entry.count > 1)
    }
}
