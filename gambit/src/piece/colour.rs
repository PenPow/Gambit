use std::{fmt::Write, ops::RangeInclusive, str::FromStr};
use crate::{enums::impl_enum_to_int, location::{Direction, Rank}};

/// Error thrown when parsing an invalid colour character
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseColourError;

impl std::fmt::Display for ParseColourError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid Colour Name")
    }
}

impl std::error::Error for ParseColourError {}

/// Represents each colour on a chessboard
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Colour {
	White,
	Black
}

impl_enum_to_int!(Colour);

impl Colour {
	/// The total number of colours.
	pub const COUNT: usize = 2;

	/// The lowest side by enum value (white)
	pub const MIN: Colour = Colour::White;

	/// The highest side by enum value (black)
	pub const MAX: Colour = Colour::Black;

	/// A range inclusive of both colours
	pub const ALL: RangeInclusive<Colour> = Colour::White..=Colour::Black;

	/// Creates a new [`Colour`] from an index (0 or 1).
	///
	/// # Panics
	///
	/// Panics if the index is not in the range 0 or 1.
	#[inline]
	#[must_use]
	pub const fn new(index: u8) -> Colour {
		debug_assert!(index < 2);

		unsafe { std::mem::transmute(index) }
	}

	/// Converts a [`char`] ('w' or 'b') to a [`Colour`].
	/// 
	/// # Errors
	///
	/// Will return [`Err`] if the colour is invalid
	#[inline]
	pub const fn from_char(char: char) -> Result<Colour, ParseColourError> {
		match char {
			'w' => Ok(Colour::White),
			'b' => Ok(Colour::Black),
			_ => Err(ParseColourError)
		}
	}

	/// Converts the [`Colour`] to its corresponding lowercase [`char`] ('w' or 'b').
	#[inline]
	#[must_use]
	pub const fn as_char(self) -> char {
		match self {
			Colour::White => 'w',
			Colour::Black => 'b'
		}
	}

	/// Returns the other [`Colour`], usable in a const context
	#[inline]
	#[must_use]
	pub const fn other(self) -> Colour {
		match self {
			Colour::White => Colour::Black,
			Colour::Black => Colour::White
		}
	}

	/// Returns the [`Direction`] of movement based off of the colour
	#[inline]
	#[must_use]
	pub const fn movement_direction(self) -> Direction {
		match self {
			Colour::White => Direction::North,
			Colour::Black => Direction::South
		}
	}

	/// Returns the fourth [`Rank`] relative to the current [`Colour`]
	#[inline]
	#[must_use]
	pub const fn fourth_rank(self) -> Rank {
		match self {
			Colour::White => Rank::R4,
			Colour::Black => Rank::R5
		}
	}

	/// Returns the promotion [`Rank`] relative to the current [`Colour`]
	#[inline]
	#[must_use]
	pub const fn promotion_rank(self) -> Rank {
		match self {
			Colour::White => Rank::R8,
			Colour::Black => Rank::R1
		}
	}
}

impl std::fmt::Display for Colour {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.as_char())
	}
}

impl std::fmt::Debug for Colour {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.as_char())
	}
}

impl std::ops::Not for Colour {
	type Output = Colour;

    #[inline]
    fn not(self) -> Colour {
        self.other()
    }
}

impl std::ops::BitXor<bool> for Colour {
	type Output = Colour;

    #[inline]
    fn bitxor(self, rhs: bool) -> Self::Output {
		Colour::new((self as u8) ^ u8::from(rhs))
	}
}

impl std::ops::BitXorAssign<bool> for Colour {
    fn bitxor_assign(&mut self, rhs: bool) {
		*self = Colour::new((*self as u8) ^ u8::from(rhs));
	}
}

impl FromStr for Colour {
    type Err = ParseColourError;

	fn from_str(s: &str) -> Result<Colour, ParseColourError> {
		if s.len() == 1 {
			Colour::from_char(s.chars().next().unwrap())
		} else {
			Err(ParseColourError)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_colour_from_u8() {
		assert_eq!(Colour::new(0u8), Colour::White);
		assert_eq!(Colour::new(1u8), Colour::Black);
		assert!(Colour::try_from(2u8).is_err());
	}

	#[test]
	fn test_colour_from_char() {
		assert_eq!(Colour::from_char('w'), Ok(Colour::White));
		assert_eq!(Colour::from_char('b'), Ok(Colour::Black));
		assert!(Colour::from_char('x').is_err());
	}

	#[test]
	fn test_colour_as_char() {
		assert_eq!(Colour::White.as_char(), 'w');
		assert_eq!(Colour::Black.as_char(), 'b');
	}

	#[test]
	fn test_colour_other() {
		assert_eq!(Colour::White.other(), Colour::Black);
		assert_eq!(Colour::Black.other(), Colour::White);
	}

	#[test]
	fn test_colour_display() {
		assert_eq!(format!("{}", Colour::White), "w");
		assert_eq!(format!("{}", Colour::Black), "b");
	}

	#[test]
	fn test_colour_debug() {
		assert_eq!(format!("{:?}", Colour::White), "w");
		assert_eq!(format!("{:?}", Colour::Black), "b");
	}

	#[test]
	fn test_colour_not() {
		assert_eq!(!Colour::White, Colour::Black);
		assert_eq!(!Colour::Black, Colour::White);
	}

	#[test]
	fn test_colour_bitxor() {
		assert_eq!(Colour::White ^ true, Colour::Black);
		assert_eq!(Colour::Black ^ true, Colour::White);
		assert_eq!(Colour::White ^ false, Colour::White);
		assert_eq!(Colour::Black ^ false, Colour::Black);
	}

	#[test]
	fn test_colour_bitxor_assign() {
		let mut colour = Colour::White;
		
		colour ^= true;
		assert_eq!(colour, Colour::Black);

		colour ^= false;
		assert_eq!(colour, Colour::Black);
		
		colour ^= true;
		assert_eq!(colour, Colour::White);
	}

	#[test]
	fn test_colour_from_str() {
		assert_eq!(Colour::from_str("w"), Ok(Colour::White));
		assert_eq!(Colour::from_str("b"), Ok(Colour::Black));
		assert!(Colour::from_str("x").is_err());
		assert!(Colour::from_str("white").is_err());
	}

	
	#[test]
	fn test_colour_movement_direction() {
		assert_eq!(Colour::White.movement_direction(), Direction::North);
		assert_eq!(Colour::Black.movement_direction(), Direction::South);
	}

	#[test]
	fn test_colour_fourth_rank() {
		assert_eq!(Colour::White.fourth_rank(), Rank::R4);
		assert_eq!(Colour::Black.fourth_rank(), Rank::R5);
	}

	#[test]
	fn test_colour_promotion_rank() {
		assert_eq!(Colour::White.promotion_rank(), Rank::R8);
		assert_eq!(Colour::Black.promotion_rank(), Rank::R1);
	}
}
