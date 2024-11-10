use std::iter::FusedIterator;
use crate::location::{File, Rank, Square};
use super::macros::impl_ops;

/// A struct to store a [bitboard](https://www.chessprogramming.org/Bitboards), an ordered set of 64 elements, each representing a square.
/// 
/// Defines useful functions to work with bitboards, and overloads operators to help ease of use
/// 
/// ```
/// use gambit::bitboard::Bitboard;
/// 
/// let bb = Bitboard::EMPTY;
/// assert_eq!(bb, 0);
/// 
/// let bb = Bitboard::UNIVERSE;
/// assert_eq!(bb, u64::MAX);
/// ```
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Clone, Copy, Hash)]
pub struct Bitboard(
	pub(super) u64
);

impl_ops!(Bitboard, u64);

impl Bitboard {
	/// An empty bitboard, where all squares are 0
	pub const EMPTY: Bitboard = Bitboard(0);

	/// The universe bitboard, where all squares are 1
	pub const UNIVERSE: Bitboard = Bitboard(u64::MAX);

	/// Create a bitboard from a u64
	#[must_use]
	pub const fn new(bits: u64) -> Bitboard {
		Bitboard(bits)
	}

	/// A bitboard with containing single [`Square`]
	#[must_use]
	pub const fn from_square(square: Square) -> Bitboard {
		Bitboard(1 << (square as u8))
	}

	/// Returns the underlying bits used to store the bitboard as a [`u64`]
	#[inline(always)]
	#[must_use]
	pub const fn bits(self) -> u64 {
		self.0
	}
}


/// Converts a [`u64`] value into a [`Bitboard`].
///
/// ```
/// use gambit::bitboard::Bitboard;
/// 
/// let bitboard = Bitboard::from(0b1010u64);
/// ```
impl From<u64> for Bitboard {
	fn from(bits: u64) -> Self {
		Self(bits)
	}
}

/// Converts a [`Bitboard`] into a [`u64`].
///
/// This implementation allows a `Bitboard` to be converted directly into a `u64`
/// by extracting the inner `u64` value from the `Bitboard`.
/// 
/// ```
/// use gambit::bitboard::Bitboard;
/// 
/// let bitboard = Bitboard::new(42);
/// let value: u64 = bitboard.into();
/// 
/// assert_eq!(value, 42);
/// ```
impl From<Bitboard> for u64 {
	fn from(bb: Bitboard) -> Self {
		bb.0
	}
}

/// Converts a [`Square`] into a [`Bitboard`].
///
/// ```
/// use gambit::{bitboard::Bitboard, location::Square};
/// 
/// let square = Square::C4;
/// let bitboard: Bitboard = square.into();
/// ```
impl From<Square> for Bitboard {
	fn from(square: Square) -> Self {
		Bitboard::from_square(square)
	}
}

/// Converts a [`Rank`] into a [`Bitboard`] where all the bits corresponding to the given rank are set to 1.
///
/// 
/// ```
/// use gambit::{bitboard::Bitboard, location::Rank};
/// 
/// let rank = Rank::R1;
/// let bitboard: Bitboard = rank.into();
/// 
/// assert_eq!(bitboard, Bitboard::new(0xFF << (rank as u8 * 8)));
/// ```
impl From<Rank> for Bitboard {
	fn from(rank: Rank) -> Self {
		Bitboard::new(0xFF  << ((rank as u8) << 3))
	}
}

/// Converts a [`File`] into a [`Bitboard`] where all the bits corresponding to the given file are set to 1.
/// 
/// ```
/// use gambit::{bitboard::Bitboard, location::File};
/// 
/// let file = File::A;
/// let bitboard: Bitboard = file.into();
/// 
/// assert_eq!(bitboard, Bitboard::new(0x0101_0101_0101_0101 << (file as u8)));
/// ```
impl From<File> for Bitboard {
	fn from(file: File) -> Self {
		Bitboard::new(0x0101_0101_0101_0101 << (file as u8))
	}
}

/// Returns an empty [`Bitboard`] instance.
/// 
/// ```
/// use gambit::bitboard::Bitboard;
/// 
/// let bitboard = Bitboard::default();
/// assert_eq!(bitboard, Bitboard::EMPTY);
/// ```
impl Default for Bitboard {
	fn default() -> Self {
		Bitboard::EMPTY
	}
}

impl IntoIterator for Bitboard {
	type Item = Square;
	type IntoIter = BitboardIterator;

	fn into_iter(self) -> Self::IntoIter {
		BitboardIterator(self)
	}
}

/// Iterator over the squares of a [`Bitboard`].
#[derive(Debug, Clone)]
pub struct BitboardIterator(Bitboard);

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
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.0.count_ones() as usize;
        (len, Some(len))
    }

    #[inline]
    fn last(self) -> Option<Square> {
        if self.0.is_empty() {
            None
        } else {
            Some(Square::new(63 - self.0.0.leading_zeros() as u8))
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

/// Iterator over the subsets of a [`Bitboard`].
///
/// See [`Bitboard::carry_rippler()`].
#[derive(Debug, Clone)]
pub struct CarryRippler {
    pub(crate) bitboard: u64,
    pub(crate) subset: u64,
    pub(crate) first: bool,
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
    fn last(self) -> Option<Bitboard> {
        if self.subset != 0 || self.first {
            Some(Bitboard(self.bitboard))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, 1_usize.checked_shl(self.bitboard.count_ones()))
    }
}

impl FusedIterator for CarryRippler {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bitboard_empty() {
		let bb = Bitboard::EMPTY;

		assert_eq!(bb.0, 0);
	}

	#[test]
	fn test_bitboard_universe() {
		let bb = Bitboard::UNIVERSE;

		assert_eq!(bb.0, u64::MAX);
	}

	#[test]
	fn test_is_subset_of() {
		let bb1 = Bitboard(0b0011);
		let bb2 = Bitboard(0b0111);

		assert!(bb1.is_subset_of(bb2));
		assert!(!bb2.is_subset_of(bb1));
	}

	#[test]
	fn test_from_u64() {
		let bb = Bitboard::from(0b1010);

		assert_eq!(bb.0, 0b1010);
	}

	#[test]
	fn test_into_u64() {
		let bb = Bitboard(0b1010);
		let bits: u64 = bb.into();

		assert_eq!(bits, 0b1010);
	}

	#[test]
	fn test_default() {
		let bb: Bitboard = Bitboard::default();

		assert_eq!(bb, Bitboard::EMPTY);
	}

	#[test]
	fn test_from_square() {
		let square = Square::C4;
		let bb = Bitboard::from_square(square);

		assert_eq!(bb.0, 1 << (square as u8));
	}

	#[test]
	fn test_from_rank() {
		let rank = Rank::R1;
		let bb: Bitboard = rank.into();

		assert_eq!(bb, Bitboard::new(0xFF << (rank as u8 * 8)));
	}

	#[test]
	fn test_from_file() {
		let file = File::A;
		let bb: Bitboard = file.into();

		assert_eq!(bb, Bitboard::new(0x0101_0101_0101_0101 << (file as u8)));
	}

	#[test]
	fn test_bitboard_iterator() {
		let bb = Bitboard(0b1010);
		let squares: Vec<Square> = bb.into_iter().collect();
		
		assert_eq!(squares, vec![Square::B1, Square::D1]);
	}
}
