use crate::bitboard::Bitboard;
use crate::location::square::Square;
use std::iter::FusedIterator;

impl Bitboard {
    /// Creates a [`CarryRippler`] iterator for the current bitboard mask.
    ///
    /// See [`CarryRippler`] for the full details and usage examples.
    #[inline]
    pub const fn carry_rippler(self) -> CarryRippler {
        CarryRippler {
            bitboard: self.bits(),
            subset: 0,
            first: true,
        }
    }
}

/// An iterator over the set squares of a [`Bitboard`] from `a1` to `h8`.
///
/// Obtained via `bitboard.into_iter()`. This iterator is:
///
/// - [`ExactSizeIterator`] - [`len`][ExactSizeIterator::len] is available without consuming the iterator
/// - [`DoubleEndedIterator`] - [`next_back`][DoubleEndedIterator::next_back] yields squares from `h8` down to `a1`.
/// - [`FusedIterator`] - safe to call [`next`][Iterator::next] after exhaustion.
/// - [`Copy`] - you can snapshot the iterator mid-traversal.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::bitboard::Bitboard;
/// # use gambit_models::location::rank::Rank;
/// # use gambit_models::location::square::Square;
/// let bb = Rank::Four.bitboard();
/// let mut iter = bb.into_iter();
///
/// assert_eq!(iter.len(), 8);
/// assert_eq!(iter.next_back(), Some(Square::H4));
/// assert_eq!(iter.len(), 7);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BitboardIterator(Bitboard);

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIterator;

    fn into_iter(self) -> BitboardIterator {
        BitboardIterator(self)
    }
}

impl Iterator for BitboardIterator {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        self.0.pop()
    }

    #[inline]
    fn count(self) -> usize {
        self.0.0.count_ones() as usize
    }

    #[inline]
    fn last(self) -> Option<Square> {
        if self.0.is_empty() {
            None
        } else {
            Some(Square::from_index(63 - self.0.0.leading_zeros() as u8))
        }
    }
}

impl ExactSizeIterator for BitboardIterator {
    #[inline]
    fn len(&self) -> usize {
        self.0.0.count_ones() as usize
    }
}

impl FusedIterator for BitboardIterator {}

impl DoubleEndedIterator for BitboardIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Square> {
        self.0.pop_back()
    }
}

/// An iterator over all subsets of a [`Bitboard`] mask, in carry-rippler order.
///
/// This implements the [Carry-Rippler](https://www.chessprogramming.org/Traversing_Subsets_of_a_Set#All_Subsets_of_any_Set) method,
/// exhaustively enumerating all `2^n` subsets of an `n`-bit mask using two operations per step.
///
/// This is used extensively with [magic bitboard](https://www.chessprogramming.org/Magic_Bitboards) when generating attack tables.
///
/// # Guarantees
///
/// - The first subset yielded is always [`Bitboard::EMPTY`].
/// - The last subset yielded is always the full mask itself.
/// - Every subset of the mask is yielded exactly once.
/// - The total number of subsets is `2^popcount(mask)`
///
/// # Examples
///
/// ```rust
/// # use gambit_models::bitboard::Bitboard;
/// # use gambit_models::location::rank::Rank;
/// let mask = Bitboard::from(Rank::One);
/// let count = mask.carry_rippler().count();
/// // Rank::One has 8 squares, so there are 2^8 = 256 subsets
/// assert_eq!(count, 256);
/// ```
///
/// # Note
///
/// For masks with many bits, the number of subsets grows exponentially.
/// A full-board mask (`UNIVERSE`, 64 bits) would require `2^64` iterations —
/// always use carry-rippler on *masked* occupancy boards, not the full board.
#[derive(Debug, Clone, Copy)]
pub struct CarryRippler {
    bitboard: u64,
    subset: u64,
    first: bool,
}

impl Iterator for CarryRippler {
    type Item = Bitboard;

    #[inline]
    fn next(&mut self) -> Option<Bitboard> {
        let subset = self.subset;
        if subset != 0 || self.first {
            self.first = false;

            self.subset = self.subset.wrapping_sub(self.bitboard) & self.bitboard;
            Some(Bitboard(subset))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.subset != 0 || self.first;
        let lower = remaining as usize;
        let upper = 1_usize.checked_shl(self.bitboard.count_ones());

        (lower, upper)
    }

    #[inline]
    fn last(self) -> Option<Bitboard> {
        if self.subset != 0 || self.first {
            Some(Bitboard(self.bitboard))
        } else {
            None
        }
    }
}

impl FusedIterator for CarryRippler {}
