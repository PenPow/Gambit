use std::{fmt::Write, ops::RangeInclusive};
use crate::{bitboard::Bitboard, enums::impl_enum_to_int};

/// Error thrown when parsing an invalid rank
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseRankError;

impl std::fmt::Display for ParseRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid rank")
    }
}

impl std::error::Error for ParseRankError {}

/// Represents a rank (row) on a chessboard, ranging from [`Rank::R1`] to [`Rank::R8`].
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl_enum_to_int!(Rank);

impl Rank {
	/// The total number of ranks.
	pub const COUNT: usize = 8;

	/// The minimum rank ([`Rank::R1`]).
	pub const MIN: Rank = Rank::R1;
	
	/// The maximum rank ([`Rank::R8`]).
	pub const MAX: Rank = Rank::R8;

	/// A range inclusive of all ranks from [`Rank::R1`] to [`Rank::R8`].
	pub const ALL: RangeInclusive<Rank> = Rank::MIN..=Rank::MAX;

	/// An array of bitboards representing each [`Rank`].
	pub const BITBOARDS: [Bitboard; Rank::COUNT] = {
		let mut ranks = [Bitboard::EMPTY; Rank::COUNT];

		let mut rank = 0;
		while rank < Rank::COUNT { // for is not stable in const functions yet
			ranks[rank] = Bitboard::new(0xFF  << ((rank as u8) << 3));
			rank += 1;
		}

		ranks
	};

	/// Creates a new [`Rank`] from a u8.
	///
	/// # Panics
	///
	/// Panics if the index is not in the range [`Rank::MIN`] to [`Rank::MAX`].
	#[inline]
	#[must_use]
	pub const fn new(index: u8) -> Rank {
		debug_assert!(index <= (Rank::MAX as u8));

		unsafe { std::mem::transmute(index) }
	}

	/// Converts a [`char`] ('1'-'8') to a [`Rank`].
	///
	/// # Errors
	/// 
	/// Returns [`Err`] if the [`char`] is not in the range '1'-'8'.
	#[inline]
	pub fn from_char(char: char) -> Result<Rank, ParseRankError> {
		if ('1'..='8').contains(&char) {
			Ok(Rank::new((char as u8) - b'1'))
		} else {
			Err(ParseRankError)
		}
	}

	/// Converts the [`Rank`] to its number as a [`char`].
	#[inline]
	#[must_use]
	pub const fn as_char(self) -> char {
		(b'1' + (self as u8)) as char
	}

	/// Calculates the distance between two ranks.
	#[inline]
	#[must_use]
	pub const fn distance(self, rhs: Rank) -> u8 {
		(self as u8).abs_diff(rhs as u8)
	}

	/// Offsets the [`Rank`] by a given amount.
	///
	/// Returns [`None`] if the resulting file is out of bounds.
	#[inline]
	#[must_use]
	pub fn offset(self, rhs: i8) -> Option<Rank> {
		i8::from(self).checked_add(rhs).and_then(|index| index.try_into().ok())
	}
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rank_new() {
		assert_eq!(Rank::new(0), Rank::R1);
		assert_eq!(Rank::new(7), Rank::R8);
	}

	#[test]
	#[should_panic(expected = "assertion failed: index <= (Rank::MAX as u8)")]
	fn test_rank_new_out_of_bounds() {
		let _ = Rank::new(8);
	}

	#[test]
	fn test_rank_from_char() {
		assert_eq!(Rank::from_char('1'), Ok(Rank::R1));
		assert_eq!(Rank::from_char('8'), Ok(Rank::R8));
		assert_eq!(Rank::from_char('9'), Err(ParseRankError));
		assert_eq!(Rank::from_char('0'), Err(ParseRankError));
	}

	#[test]
	fn test_rank_as_char() {
		assert_eq!(Rank::R1.as_char(), '1');
		assert_eq!(Rank::R8.as_char(), '8');
	}

	#[test]
	fn test_rank_distance() {
		assert_eq!(Rank::R1.distance(Rank::R8), 7);
		assert_eq!(Rank::R8.distance(Rank::R1), 7);
		assert_eq!(Rank::R4.distance(Rank::R4), 0);
	}

	#[test]
	fn test_rank_offset() {
		assert_eq!(Rank::R1.offset(1), Some(Rank::R2));
		assert_eq!(Rank::R8.offset(-1), Some(Rank::R7));
		assert_eq!(Rank::R1.offset(-1), None);
		assert_eq!(Rank::R8.offset(1), None);
	}

	#[test]
	fn test_rank_bitboards() {
		assert_eq!(Rank::BITBOARDS[0], Bitboard::new(0xFF));
		assert_eq!(Rank::BITBOARDS[7], Bitboard::new(0xFF << 56));
	}
}
