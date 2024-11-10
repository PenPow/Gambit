use std::ops::RangeInclusive;

use crate::enums::{impl_signed_enum_to_int, impl_enum_arithmetic_ops_internal};

/// Represents the 8 different directions a piece can move in, based off of the compose rose.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i8)]
pub enum Direction {
	North = 8,
	NorthEast = 9,
	East = 1,
	SouthEast = -7,
	South = -8,
	SouthWest = -9,
	West = -1,
	NorthWest = 7,
	NoMovement = 0,
}

impl_signed_enum_to_int!(Direction);
impl_enum_arithmetic_ops_internal!(Direction, u8);
impl_enum_arithmetic_ops_internal!(Direction, i8);

impl_enum_arithmetic_ops_internal!(Direction, u16);
impl_enum_arithmetic_ops_internal!(Direction, i16);

impl_enum_arithmetic_ops_internal!(Direction, u32);
impl_enum_arithmetic_ops_internal!(Direction, i32);

impl_enum_arithmetic_ops_internal!(Direction, u64);
impl_enum_arithmetic_ops_internal!(Direction, i64);

impl_enum_arithmetic_ops_internal!(Direction, u128);
impl_enum_arithmetic_ops_internal!(Direction, i128);

impl_enum_arithmetic_ops_internal!(Direction, usize);
impl_enum_arithmetic_ops_internal!(Direction, isize);

impl Direction {
	/// The total number of directions to move.
	pub const COUNT: usize = 9;

	/// The furthest back you can move ([`Direction::SouthWest`])
	pub const MIN: Direction = Direction::SouthWest;

	/// The furthest forward you can move ([`Direction::NorthEast`])
	pub const MAX: Direction = Direction::NorthEast;

	/// A range inclusive of all directions
	pub const ALL: RangeInclusive<Direction> = Direction::MIN..=Direction::MAX;

	/// Creates a new [`Direction`] from an i8.
	///
	/// # Panics
	///
	/// Panics if the index is not in the range [`Direction::MIN`] to [`Direction::MAX`].
	#[inline]
	#[must_use]
	pub const fn new(index: i8) -> Direction {
		debug_assert!(index <= (Direction::MAX as i8) && index >= (Direction::MIN as i8));

		unsafe { std::mem::transmute(index) }
	}

	/// Returns the offset value associated with the [`Direction`], can be used in const contexts
	/// 
	/// ```
	/// use gambit::location::Direction;
	///
	/// let north_offset = Direction::North.offset();
	/// assert_eq!(north_offset, 8);
	/// ```
	#[inline(always)]
	#[must_use]
    pub const fn offset(self) -> i8 {
		self as i8
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_direction_offsets() {
		assert_eq!(Direction::North.offset(), 8);
		assert_eq!(Direction::NorthEast.offset(), 9);
		assert_eq!(Direction::East.offset(), 1);
		assert_eq!(Direction::SouthEast.offset(), -7);
		assert_eq!(Direction::South.offset(), -8);
		assert_eq!(Direction::SouthWest.offset(), -9);
		assert_eq!(Direction::West.offset(), -1);
		assert_eq!(Direction::NorthWest.offset(), 7);
		assert_eq!(Direction::NoMovement.offset(), 0);
	}

	#[test]
	fn test_direction_enum_values() {
		assert_eq!(Direction::North as i8, 8);
		assert_eq!(Direction::NorthEast as i8, 9);
		assert_eq!(Direction::East as i8, 1);
		assert_eq!(Direction::SouthEast as i8, -7);
		assert_eq!(Direction::South as i8, -8);
		assert_eq!(Direction::SouthWest as i8, -9);
		assert_eq!(Direction::West as i8, -1);
		assert_eq!(Direction::NorthWest as i8, 7);
		assert_eq!(Direction::NoMovement as i8, 0);
	}
}
