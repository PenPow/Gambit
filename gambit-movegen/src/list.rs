use gambit_models::moves::Move;
use std::cmp;

#[derive(Debug, Clone)]
pub struct MoveList {
    moves: [Move; MoveList::CAPACITY],
    len: usize,
}

impl MoveList {
    pub const CAPACITY: usize = 256;

    #[inline]
    pub const fn new() -> Self {
        Self {
            moves: [Move::NULL; MoveList::CAPACITY],
            len: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.len < Self::CAPACITY, "move list overflow");

        self.moves[self.len] = mv;
        self.len += 1;
    }

    /// # Safety
    ///
    /// This function can lead to UB if you attempt to push more moves than the [`CAPACITY`][MoveList::CAPACITY].
    ///
    /// This cannot happen in normal chess play.
    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, mv: Move) {
        // SAFETY: len is always < 256 in any reachable position
        unsafe {
            *self.moves.get_unchecked_mut(self.len) = mv;
        }
        self.len += 1;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<Move> {
        if let mv = self.moves[index]
            && !mv.is_null()
        {
            return Some(mv);
        }

        None
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> Move {
        debug_assert!(index < self.len, "get_unchecked index out of bounds");
        let mv = unsafe { *self.moves.get_unchecked(index) };
        debug_assert!(!mv.is_null(), "get_unchecked returned NULL move");

        mv
    }

    #[inline]
    pub fn last(&self) -> Move {
        debug_assert!(
            !self.is_empty(),
            "cannot get last element of empty move list"
        );

        self.moves[self.len - 1]
    }

    #[inline]
    pub fn pop(&mut self) -> Move {
        debug_assert!(!self.is_empty(), "cannot pop from empty move list");

        let mv = self.moves[self.len - 1];
        self.len -= 1;

        mv
    }

    // removes item at index, swaps last to index
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> Move {
        debug_assert!(index < self.len, "swap_remove index out of bounds");

        let mv = self.moves[index];

        self.moves[index] = self.moves[self.len - 1];
        self.len -= 1;

        mv
    }

    #[inline]
    pub fn truncate(&mut self, n: usize) {
        if n < self.len {
            self.len = n
        }
    }

    #[inline]
    pub fn sort_by<F: FnMut(&Move, &Move) -> cmp::Ordering>(&mut self, cmp: F) {
        self.as_mut_slice().sort_by(cmp);
    }

    #[inline]
    pub fn partition<F: FnMut(&Move) -> bool>(&mut self, mut predicate: F) -> usize {
        let slice = self.as_mut_slice();

        let mut i = 0;

        for j in 0..slice.len() {
            if predicate(&slice[j]) {
                if i != j {
                    slice.swap(i, j);
                }

                i += 1;
            }
        }

        i
    }

    #[inline]
    pub fn swap_at(&mut self, i: usize, j: usize) {
        debug_assert!(i < self.len && j < self.len, "swap_at index out of bounds");
        self.moves.swap(i, j);
    }

    #[inline]
    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.len]
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Move] {
        &mut self.moves[..self.len]
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Move> {
        self.as_slice().iter()
    }
}

impl Default for MoveList {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a MoveList {
    type Item = &'a Move;
    type IntoIter = std::slice::Iter<'a, Move>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
