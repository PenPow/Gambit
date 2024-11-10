use std::{fmt::Write, ops::RangeInclusive, str::FromStr};
use crate::{bitboard::Bitboard, enums::{impl_enum_arithmetic_ops, impl_enum_to_int}};
use super::{file::File, rank::Rank, Coordinates, Direction};

/// Error thrown when parsing an invalid square name
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseSquareError;

impl std::fmt::Display for ParseSquareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid Square Name")
    }
}

impl std::error::Error for ParseSquareError {}

/// Represents a square on a chessboard.
///
/// Provides various methods to create and manipulate squares, as well as constants
/// for common square-related values.
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl_enum_to_int!(Square);
impl_enum_arithmetic_ops!(Square);

impl Square {
	/// The total number of squares on a chessboard.
	pub const COUNT: usize = 64;

	/// The minimum square ([`Square::A1`]) on a chessboard.
	pub const MIN: Square = Square::A1;

	/// The maximum square ([`Square::H8`]) on a chessboard.
	pub const MAX: Square = Square::H8;

	/// A range that includes all squares on a chessboard, from [`Square::A1`] to [`Square::H8`].
	pub const ALL: RangeInclusive<Square> = Square::MIN..=Square::MAX;

	/// An array containing a bitboard for each square.
	pub const BITBOARDS: [Bitboard; Square::COUNT] = {
		let mut squares = [Bitboard::EMPTY; Square::COUNT];

		let mut square = 0;
		while square < Square::COUNT {
			squares[square] = Bitboard::from_square(Square::new(square as u8));
			square += 1;
		}

		squares
	};

	/// Creates a new [`Square`] from the given index.
	///
	/// # Panics
	///
	/// Panics if the index is not less than [`Square::MAX`].
	#[inline]
	#[must_use]
	pub const fn new(index: u8) -> Square {
		debug_assert!(index <= (Square::MAX as u8));

		unsafe { std::mem::transmute(index) }
	}

	/// Creates a [`Square`] from the given file and rank coordinates.
	#[inline]
	#[must_use]
	pub const fn from_coords((file, rank): Coordinates) -> Square {
		let index = file as u8 | ((rank as u8) << 3);

		Self::new(index)
	}

	/// Parses a [`Square`] from algebraic notation.
	///
	/// # Errors
	///
	/// Will return [`Err`] if the notation is invalid
	#[inline]
	pub fn from_algebraic_notation(notation: &str) -> Result<Square, ParseSquareError> {
		if notation.len() == 2 {
			let mut chars = notation.chars();
			match (File::from_char(unsafe { chars.nth(0).unwrap_unchecked() }), Rank::from_char(unsafe { chars.nth(0).unwrap_unchecked() })) {
				(Ok(file), Ok(rank)) => Ok(Square::from_coords((file, rank))),
				_ => Err(ParseSquareError)
			}
		} else { Err(ParseSquareError) }
	}

	/// Converts a [`Square`] to algebraic notation.
	#[must_use]
	pub fn as_str(self) -> String {
		let (file, rank) = self.as_coords();

		format!("{}{}", file.as_uppercase_char(), rank.as_char())
	}

	/// Returns the [`Rank`] of the square.
	#[inline]
	#[must_use]
	pub const fn rank(self) -> Rank {
		Rank::new((self as u8) >> 3)
	}

	/// Returns the [`File`] of the square.
	#[inline]
	#[must_use]
	pub const fn file(self) -> File {
		File::new((self as u8) & 7)
	}

	/// Returns the [`Coordinates`] of the square.
	#[inline]
	#[must_use]
	pub const fn as_coords(self) -> Coordinates {
		(self.file(), self.rank())
	}

	/// Returns a new square offset by the given amount, or [`None`] if the result is out of bounds.
	#[inline]
	#[must_use]
	pub fn offset(self, rhs: i8) -> Option<Square> {
		i8::from(self).checked_add(rhs).and_then(|index| index.try_into().ok())
	}

	/// Translates the square in the given direction, returning the new square or [`None`] if out of bounds.
	#[inline]
	#[must_use]
	pub fn translate(self, direction: Direction) -> Option<Square> {
		self.offset(direction.offset())
	}

	/// Returns the distance between this square and another square.
	///
	/// The distance is the maximum of the file and rank distances.
	#[must_use]
	pub fn distance(self, other: Square) -> u8 {
		std::cmp::max(self.file().distance(other.file()), self.rank().distance(other.rank()))
	}

	/// Returns the bitboard associated with the square
	#[must_use]
	pub const fn bitboard(self) -> Bitboard {
		debug_assert!(self as usize <= Self::COUNT);
		
		Self::BITBOARDS[self as usize]
	} 
}

/// Converts [`Coordinates`] into a [`Square`].
///
/// ```
/// use gambit::location::{Coordinates, Square, Rank, File};
/// 
/// let coords: Coordinates = (File::A, Rank::R2);
/// let square: Square = coords.into();
/// assert_eq!(square, Square::A2)
/// ```
impl From<Coordinates> for Square {
	#[inline]
	fn from(coords: Coordinates) -> Self {
		Square::from_coords(coords)
	}
}

impl FromStr for Square {
	type Err = ParseSquareError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Square::from_algebraic_notation(s)
	}
}

impl std::fmt::Display for Square {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.file().as_uppercase_char())?;
        f.write_char(self.rank().as_char())
    }
}

impl std::fmt::Debug for Square {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.file().as_uppercase_char())?;
        f.write_char(self.rank().as_char())
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_square_new() {
		assert_eq!(Square::new(0), Square::A1);
		assert_eq!(Square::new(63), Square::H8);
	}

	#[test]
	#[should_panic(expected = "assertion failed: index <= (Square::MAX as u8)")]
	fn test_square_new_out_of_bounds() {
		let _ = Square::new(64);
	}

	#[test]
	fn test_square_from_coords() {
		assert_eq!(Square::from_coords((File::A, Rank::R1)), Square::A1);
		assert_eq!(Square::from_coords((File::H, Rank::R8)), Square::H8);
	}

	#[test]
	fn test_square_from_algebraic_notation() {
		assert_eq!(Square::from_algebraic_notation("a1").unwrap(), Square::A1);
		assert_eq!(Square::from_algebraic_notation("h8").unwrap(), Square::H8);
		assert!(Square::from_algebraic_notation("i9").is_err());
	}

	#[test]
	fn test_square_rank() {
		assert_eq!(Square::A1.rank(), Rank::R1);
		assert_eq!(Square::H8.rank(), Rank::R8);
	}

	#[test]
	fn test_square_file() {
		assert_eq!(Square::A1.file(), File::A);
		assert_eq!(Square::H8.file(), File::H);
	}

	#[test]
	fn test_square_as_coords() {
		assert_eq!(Square::A1.as_coords(), (File::A, Rank::R1));
		assert_eq!(Square::H8.as_coords(), (File::H, Rank::R8));
	}

	#[test]
	fn test_square_offset() {
		assert_eq!(Square::A1.offset(1), Some(Square::B1));
		assert_eq!(Square::H8.offset(-1), Some(Square::G8));
		assert_eq!(Square::A1.offset(-1), None);
	}

	#[test]
	fn test_square_translate() {
		assert_eq!(Square::A1.translate(Direction::North), Some(Square::A2));
		assert_eq!(Square::H8.translate(Direction::South), Some(Square::H7));
		assert_eq!(Square::A1.translate(Direction::West), None);
	}

	#[test]
	fn test_square_distance() {
		assert_eq!(Square::A1.distance(Square::H8), 7);
		assert_eq!(Square::A1.distance(Square::A1), 0);
	}

	#[test]
	fn test_square_from_u8() {
		assert_eq!(Square::try_from(0u8).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63u8).unwrap(), Square::H8);
		assert!(Square::try_from(64u8).is_err());
	}

	#[test]
	fn test_square_from_i8() {
		assert_eq!(Square::try_from(0i8).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63i8).unwrap(), Square::H8);
		assert!(Square::try_from(64i8).is_err());
	}

	#[test]
	fn test_square_from_u16() {
		assert_eq!(Square::try_from(0u16).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63u16).unwrap(), Square::H8);
		assert!(Square::try_from(64u16).is_err());
	}

	#[test]
	fn test_square_from_i16() {
		assert_eq!(Square::try_from(0i16).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63i16).unwrap(), Square::H8);
		assert!(Square::try_from(64i16).is_err());
	}

	#[test]
	fn test_square_from_u32() {
		assert_eq!(Square::try_from(0u32).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63u32).unwrap(), Square::H8);
		assert!(Square::try_from(64u32).is_err());
	}

	#[test]
	fn test_square_from_i32() {
		assert_eq!(Square::try_from(0i32).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63i32).unwrap(), Square::H8);
		assert!(Square::try_from(64i32).is_err());
	}

	#[test]
	fn test_square_from_u64() {
		assert_eq!(Square::try_from(0u64).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63u64).unwrap(), Square::H8);
		assert!(Square::try_from(64u64).is_err());
	}

	#[test]
	fn test_square_from_i64() {
		assert_eq!(Square::try_from(0i64).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63i64).unwrap(), Square::H8);
		assert!(Square::try_from(64i64).is_err());
	}

	#[test]
	fn test_square_from_u128() {
		assert_eq!(Square::try_from(0u128).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63u128).unwrap(), Square::H8);
		assert!(Square::try_from(64u128).is_err());
	}

	#[test]
	fn test_square_from_i128() {
		assert_eq!(Square::try_from(0i128).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63i128).unwrap(), Square::H8);
		assert!(Square::try_from(64i128).is_err());
	}

	#[test]
	fn test_square_from_usize() {
		assert_eq!(Square::try_from(0usize).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63usize).unwrap(), Square::H8);
		assert!(Square::try_from(64usize).is_err());
	}

	#[test]
	fn test_square_from_isize() {
		assert_eq!(Square::try_from(0isize).unwrap(), Square::A1);
		assert_eq!(Square::try_from(63isize).unwrap(), Square::H8);
		assert!(Square::try_from(64isize).is_err());
	}
}