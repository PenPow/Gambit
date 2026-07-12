use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

// TODO:
// Determine whether this is providing an ELO boost or not

#[derive(Default, Copy, Clone, Debug)]
pub struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        panic!("IdentityHasher only supports u64 keys via write_u64")
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

type IdentityHashMap = HashMap<u64, u8, BuildHasherDefault<IdentityHasher>>;

#[derive(Debug, Default, Clone)]
pub struct RepetitionTable {
    map: IdentityHashMap,
}

impl RepetitionTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_history(hashes: impl IntoIterator<Item = u64>) -> Self {
        let mut table = Self::default();
        table.fill(hashes);

        table
    }

    pub fn fill(&mut self, hashes: impl IntoIterator<Item = u64>) {
        for hash in hashes {
            self.push(hash)
        }
    }

    pub fn push(&mut self, hash: u64) {
        *self.map.entry(hash).or_insert(0) += 1;
    }

    pub fn pop(&mut self, hash: u64) {
        if let Some(count) = self.map.get_mut(&hash) {
            *count -= 1;
            if *count == 0 {
                self.map.remove(&hash);
            }
        }
    }

    #[inline]
    pub fn is_repeated(&self, hash: u64) -> bool {
        self.map.get(&hash).is_some_and(|&count| count > 1)
    }
}
