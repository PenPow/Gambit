use std::fmt::Write;

use crate::enums::impl_enum_to_int;

/// Error thrown when parsing an invalid piece character
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsePieceError;

impl std::fmt::Display for ParsePieceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid piece character")
    }
}

impl std::error::Error for ParsePieceError {}

/// Represents a chess piece
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum PieceType {
    Pawn, // Starts at index 0 to allow for indexing arrays
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
	None = 13, // Set to 13 to allow for the offset added to black pieces
}

impl_enum_to_int!(PieceType);

impl PieceType {
	/// The total number of piece types.
	pub const COUNT: usize = 6;

	/// The total number of piece types that can be promoted to
	pub const PROMOTION_OPTION_COUNT: usize = 4;

	/// The minimum piece type based on enum value ([`PieceType::Pawn`])
	pub const MIN: PieceType = PieceType::Pawn;

	/// The maximum piece type based on enum value ([`PieceType::None`])
	pub const MAX: PieceType = PieceType::None;

	/// All piece types
	pub const ALL: [PieceType; PieceType::COUNT] = [
		PieceType::Pawn,
		PieceType::Knight,
		PieceType::Bishop,
		PieceType::Rook,
		PieceType::Queen,
		PieceType::King,
	];

	/// All possible promotion targets
	pub const PROMOTION_TARGETS: [PieceType; PieceType::PROMOTION_OPTION_COUNT] = [
		PieceType::Knight,
		PieceType::Bishop,
		PieceType::Rook,
		PieceType::Queen,
	];

	/// Creates a new [`PieceType`] from the given index.
	///
	/// # Panics
	///
	/// Panics if the index is not less than [`PieceType::MAX`].
	#[inline]
	#[must_use]
	pub const fn new(index: u8) -> PieceType {
		debug_assert!(index <= (PieceType::King as u8) || index == (PieceType::None as u8));

		unsafe { std::mem::transmute(index) }
	}

	/// Converts a [`char`] to a [`PieceType`].
	///
	/// # Errors
	/// 
	/// Returns [`Err`] if the [`char`] is invalid.
	#[inline]
	pub const fn from_char(ch: char) -> Result<PieceType, ParsePieceError> {
        match ch {
            'P' | 'p' => Ok(PieceType::Pawn),
            'N' | 'n' => Ok(PieceType::Knight),
            'B' | 'b' => Ok(PieceType::Bishop),
            'R' | 'r' => Ok(PieceType::Rook),
            'Q' | 'q' => Ok(PieceType::Queen),
            'K' | 'k' => Ok(PieceType::King),
            _ => Err(ParsePieceError),
        }
    }

	/// Converts the [`PieceType`] to its corresponding lowercase [`char`].
	#[inline]
	#[must_use]
	pub const fn as_char(self) -> char {
		match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
			PieceType::None => ' ',
        }
	}

	/// Converts the [`PieceType`] to its corresponding uppercase [`char`].
	#[inline]
	#[must_use]
	pub const fn as_uppercase_char(self) -> char {
		match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
			PieceType::None => ' ',
        }
	}
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl std::fmt::Debug for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_piece_type_from_char() {
		assert_eq!(PieceType::from_char('P').unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::from_char('N').unwrap(), PieceType::Knight);
		assert_eq!(PieceType::from_char('B').unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::from_char('R').unwrap(), PieceType::Rook);
		assert_eq!(PieceType::from_char('Q').unwrap(), PieceType::Queen);
		assert_eq!(PieceType::from_char('K').unwrap(), PieceType::King);
		assert_eq!(PieceType::from_char('p').unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::from_char('n').unwrap(), PieceType::Knight);
		assert_eq!(PieceType::from_char('b').unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::from_char('r').unwrap(), PieceType::Rook);
		assert_eq!(PieceType::from_char('q').unwrap(), PieceType::Queen);
		assert_eq!(PieceType::from_char('k').unwrap(), PieceType::King);
		assert!(PieceType::from_char('X').is_err());
	}

	#[test]
	fn test_piece_type_as_char() {
		assert_eq!(PieceType::Pawn.as_char(), 'p');
		assert_eq!(PieceType::Knight.as_char(), 'n');
		assert_eq!(PieceType::Bishop.as_char(), 'b');
		assert_eq!(PieceType::Rook.as_char(), 'r');
		assert_eq!(PieceType::Queen.as_char(), 'q');
		assert_eq!(PieceType::King.as_char(), 'k');
		assert_eq!(PieceType::None.as_char(), ' ');
	}

	#[test]
	fn test_piece_type_as_uppercase_char() {
		assert_eq!(PieceType::Pawn.as_uppercase_char(), 'P');
		assert_eq!(PieceType::Knight.as_uppercase_char(), 'N');
		assert_eq!(PieceType::Bishop.as_uppercase_char(), 'B');
		assert_eq!(PieceType::Rook.as_uppercase_char(), 'R');
		assert_eq!(PieceType::Queen.as_uppercase_char(), 'Q');
		assert_eq!(PieceType::King.as_uppercase_char(), 'K');
		assert_eq!(PieceType::None.as_uppercase_char(), ' ');
	}

	#[test]
	fn test_piece_type_from_u8() {
		assert_eq!(PieceType::try_from(0u8).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1u8).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2u8).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3u8).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4u8).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5u8).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13u8).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14u8).is_err());
	}

	#[test]
	fn test_piece_type_from_i8() {
		assert_eq!(PieceType::try_from(0i8).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1i8).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2i8).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3i8).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4i8).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5i8).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13i8).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14i8).is_err());
	}

	#[test]
	fn test_piece_type_from_u16() {
		assert_eq!(PieceType::try_from(0u16).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1u16).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2u16).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3u16).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4u16).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5u16).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13u16).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14u16).is_err());
	}

	#[test]
	fn test_piece_type_from_i16() {
		assert_eq!(PieceType::try_from(0i16).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1i16).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2i16).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3i16).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4i16).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5i16).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13i16).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14i16).is_err());
	}

	#[test]
	fn test_piece_type_from_u32() {
		assert_eq!(PieceType::try_from(0u32).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1u32).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2u32).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3u32).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4u32).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5u32).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13u32).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14u32).is_err());
	}

	#[test]
	fn test_piece_type_from_i32() {
		assert_eq!(PieceType::try_from(0i32).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1i32).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2i32).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3i32).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4i32).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5i32).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13i32).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14i32).is_err());
	}

	#[test]
	fn test_piece_type_from_u64() {
		assert_eq!(PieceType::try_from(0u64).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1u64).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2u64).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3u64).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4u64).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5u64).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13u64).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14u64).is_err());
	}

	#[test]
	fn test_piece_type_from_i64() {
		assert_eq!(PieceType::try_from(0i64).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1i64).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2i64).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3i64).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4i64).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5i64).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13i64).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14i64).is_err());
	}

	#[test]
	fn test_piece_type_from_u128() {
		assert_eq!(PieceType::try_from(0u128).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1u128).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2u128).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3u128).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4u128).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5u128).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13u128).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14u128).is_err());
	}

	#[test]
	fn test_piece_type_from_i128() {
		assert_eq!(PieceType::try_from(0i128).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1i128).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2i128).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3i128).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4i128).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5i128).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13i128).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14i128).is_err());
	}

	#[test]
	fn test_piece_type_from_usize() {
		assert_eq!(PieceType::try_from(0usize).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1usize).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2usize).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3usize).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4usize).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5usize).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13usize).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14usize).is_err());
	}

	#[test]
	fn test_piece_type_from_isize() {
		assert_eq!(PieceType::try_from(0isize).unwrap(), PieceType::Pawn);
		assert_eq!(PieceType::try_from(1isize).unwrap(), PieceType::Knight);
		assert_eq!(PieceType::try_from(2isize).unwrap(), PieceType::Bishop);
		assert_eq!(PieceType::try_from(3isize).unwrap(), PieceType::Rook);
		assert_eq!(PieceType::try_from(4isize).unwrap(), PieceType::Queen);
		assert_eq!(PieceType::try_from(5isize).unwrap(), PieceType::King);
		assert_eq!(PieceType::try_from(13isize).unwrap(), PieceType::None);
		assert!(PieceType::try_from(14isize).is_err());
	}
}