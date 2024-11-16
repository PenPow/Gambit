use std::{fmt::Write, ops::RangeInclusive};
use crate::{bitboard::Bitboard, enums::impl_enum_to_int};

/// Error thrown when parsing an invalid file
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseFileError;

impl std::fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid file")
    }
}

impl std::error::Error for ParseFileError {}

/// Represents a file (column) on a chessboard, ranging from [`File::A`] to [`File::H`].
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum File {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
}

impl_enum_to_int!(File);

impl File {
	/// The total number of files.
	pub const COUNT: usize = 8;

	/// The minimum file ([`File::A`]).
	pub const MIN: File = File::A;

	/// The maximum file ([`File::H`]).
	pub const MAX: File = File::H;

	/// A range inclusive of all files from [`File::A`] to [`File::H`].
	pub const ALL: RangeInclusive<File> = File::MIN..=File::MAX;

	/// An array of bitboards representing each [`File`].
	pub const BITBOARDS: [Bitboard; File::COUNT] = {
		let mut files = [Bitboard::EMPTY; File::COUNT];

		let mut file = 0;
		while file < File::COUNT {
			files[file] = Bitboard::new(0x0101_0101_0101_0101 << (file as u8));
			file += 1;
		}

		files
	};

	/// Creates a new [`File`] from a u8.
	///
	/// # Panics
	///
	/// Panics if the index is not in the range [`File::MIN`] to [`File::MAX`].
	#[inline]
	#[must_use]
	pub const fn new(index: u8) -> File {
		debug_assert!(index <= (File::MAX as u8));

		unsafe { std::mem::transmute(index) }
	}

	/// Converts a [`char`] ('a'-'h') to a [`File`].
	///
	/// # Errors
	/// 
	/// Returns [`Err`] if the [`char`] is not in the range 'a'-'h'.
	#[inline]
	pub fn from_char(char: char) -> Result<File, ParseFileError> {
		if ('a'..='h').contains(&char) {
			Ok(File::new((char as u8) - b'a'))
		} else {
			Err(ParseFileError)
		}
	}

	/// Converts the [`File`] to its corresponding lowercase [`char`] ('a'-'h').
	#[inline]
	#[must_use]
	pub const fn as_char(self) -> char {
		(b'a' + (self as u8)) as char
	}

	/// Converts the [`File`] to its corresponding uppercase [`char`] ('A'-'H').
	#[inline]
	#[must_use]
	pub const fn as_uppercase_char(self) -> char {
		(b'A' + (self as u8)) as char
	}

	/// Calculates the distance between two files.
	#[inline]
	#[must_use]
	pub const fn distance(self, rhs: File) -> u8 {
		(self as u8).abs_diff(rhs as u8)
	}

	/// Offsets the [`File`] by a given amount.
	///
	/// Returns [`None`] if the resulting file is out of bounds.
	#[inline]
	#[must_use]
	pub fn offset(self, rhs: i8) -> Option<File> {
		i8::from(self).checked_add(rhs).and_then(|index| index.try_into().ok())
	}
}

impl std::fmt::Display for File {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.as_uppercase_char())
	}
}

impl std::fmt::Debug for File {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.as_uppercase_char())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_file_new() {
		assert_eq!(File::new(0), File::A);
		assert_eq!(File::new(7), File::H);
	}

	#[test]
	#[should_panic(expected = "assertion failed: index <= (File::MAX as u8)")]
	fn test_file_new_out_of_bounds() {
		let _ = File::new(8);
	}

	#[test]
	fn test_file_from_char() {
		assert_eq!(File::from_char('a'), Ok(File::A));
		assert_eq!(File::from_char('h'), Ok(File::H));
		assert_eq!(File::from_char('i'), Err(ParseFileError));
	}

	#[test]
	fn test_file_as_char() {
		assert_eq!(File::A.as_char(), 'a');
		assert_eq!(File::H.as_char(), 'h');
	}

	#[test]
	fn test_file_as_uppercase_char() {
		assert_eq!(File::A.as_uppercase_char(), 'A');
		assert_eq!(File::H.as_uppercase_char(), 'H');
	}

	#[test]
	fn test_file_distance() {
		assert_eq!(File::A.distance(File::H), 7);
		assert_eq!(File::D.distance(File::B), 2);
	}

	#[test]
	fn test_file_offset() {
		assert_eq!(File::A.offset(1), Some(File::B));
		assert_eq!(File::H.offset(-1), Some(File::G));
		assert_eq!(File::A.offset(-1), None);
		assert_eq!(File::H.offset(1), None);
	}

	#[test]
	fn test_file_bitboards() {
		assert_eq!(File::BITBOARDS[0], Bitboard::new(0x0101_0101_0101_0101));
		assert_eq!(File::BITBOARDS[7], Bitboard::new(0x8080_8080_8080_8080));
	}
}
