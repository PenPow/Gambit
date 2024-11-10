use std::fmt;
use bitflags::bitflags;

/// A struct to represent the castling permissions in a game
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Castling(
	/// The bits containing the castling permission, you shouldn't have to touch this
	u8
);

impl Castling {
	/// Creates a new [`Castling`] struct from the given [`CastlingPermissions`].
	///
	/// # Panics
	///
	/// Panics if the index is not less than [`CastlingPermissions::ALL`].
	#[inline]
	#[must_use]
	pub const fn new(permissions: CastlingPermissions) -> Self {
		Self(permissions.bits())
	}

	/// Creates a new [`Castling`] struct from the given permission bits.
	///
	/// # Panics
	///
	/// Panics if the index is not less than [`CastlingPermissions::ALL`].
	#[inline]
	#[must_use]
	pub const fn from(bits: u8) -> Self {
		debug_assert!(bits <= CastlingPermissions::ALL.bits());

		Self(bits)
	}

	/// Checks if the given [`CastlingPermissions`] are present.
	///
	/// # Examples
	///
	/// ```
	/// use gambit::piece::{Castling, CastlingPermissions};
	/// 
	/// let castling = Castling::from(6u8);
	/// 
	/// assert!(castling.has(CastlingPermissions::WHITE_QUEEN));
	/// assert!(castling.has(CastlingPermissions::BLACK_KING));
	/// assert!(!castling.has(CastlingPermissions::WHITE_KING));
	/// ```
	#[inline]
	#[must_use]
	pub const fn has(self, permissions: CastlingPermissions) -> bool {
		(self.0 & permissions.bits()) != 0
	}

	/// Sets a [`CastlingPermissions`] flag to true
	///
	/// # Examples
	///
	/// ```
	/// use gambit::piece::{Castling, CastlingPermissions};
	/// 
	/// let mut castling = Castling::from(0u8);
	/// assert!(!castling.has(CastlingPermissions::WHITE_KING));
	/// 
	/// castling.set(CastlingPermissions::WHITE_KING);
	/// assert!(castling.has(CastlingPermissions::WHITE_KING));
	/// ```
	#[inline]
	pub fn set(&mut self, permissions: CastlingPermissions) {
		self.0 |= permissions.bits();
	}

	/// Sets a [`CastlingPermissions`] flag to false
	///
	/// # Examples
	///
	/// ```
	/// use gambit::piece::{Castling, CastlingPermissions};
	/// 
	/// let mut castling = Castling::from(1u8);
	/// assert!(castling.has(CastlingPermissions::WHITE_KING));
	/// 
	/// castling.remove(CastlingPermissions::WHITE_KING);
	/// assert!(!castling.has(CastlingPermissions::WHITE_KING));
	/// ```
	#[inline]
	pub fn remove(&mut self, permissions: CastlingPermissions) {
		self.0 &= !permissions.bits();
	}

	/// Converts a [`Castling`] into its raw [`u8`]
	///
	/// # Examples
	///
	/// ```
	/// use gambit::piece::{Castling, CastlingPermissions};
	/// 
	/// let castling = Castling::from(6u8);
	/// assert_eq!(castling.bits(), (CastlingPermissions::WHITE_QUEEN | CastlingPermissions::BLACK_KING).bits());
	/// ```
	#[inline]
	#[must_use]
	pub fn bits(&self) -> u8 {
		self.0
	}
}

impl Default for Castling {
	fn default() -> Self {
		Castling::from(CastlingPermissions::ALL.bits())
	}
}

impl fmt::UpperHex for Castling {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::UpperHex::fmt(&self.0, f)
	}
}

impl fmt::LowerHex for Castling {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::LowerHex::fmt(&self.0, f)
	}
}

impl fmt::Octal for Castling {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Octal::fmt(&self.0, f)
	}
}

impl fmt::Binary for Castling {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&self.0, f)
	}
}

impl fmt::Display for Castling {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&self.0, f)
	}
}

impl fmt::Debug for Castling {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&self.0, f)
	}
}

bitflags! {
	/// Represents the different shifts for different castling amounts
	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
	pub struct CastlingPermissions: u8 {
		/// The value for castling on the side of a white king
		const WHITE_KING = 1;
		
		/// The value for castling on the side of a white queen
		const WHITE_QUEEN = 2;
		
		/// The value for castling on the side of a black king
		const BLACK_KING = 4;
		
		/// The value for castling on the side of a black queen
		const BLACK_QUEEN = 8;

		/// The value for castling on either side, for either colour
		const ALL = Self::WHITE_KING.bits() | Self::WHITE_QUEEN.bits() | Self::BLACK_KING.bits() | Self::BLACK_QUEEN.bits();
		
		/// The value for no castling being possible
		const NONE = 0;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_castling() {
		let castling = Castling::new(CastlingPermissions::WHITE_KING);

		assert_eq!(castling.0, CastlingPermissions::WHITE_KING.bits());
	}

	#[test]
	fn test_from_bits() {
		let castling = Castling::from(1);

		assert_eq!(castling.0, 1);
	}

	#[test]
	fn test_has_permission() {
		let castling = Castling::new(CastlingPermissions::WHITE_KING);

		assert!(castling.has(CastlingPermissions::WHITE_KING));
		assert!(!castling.has(CastlingPermissions::WHITE_QUEEN));
	}

	#[test]
	fn test_set_permission() {
		let mut castling = Castling::new(CastlingPermissions::WHITE_KING);
		castling.set(CastlingPermissions::WHITE_QUEEN);

		assert!(castling.has(CastlingPermissions::WHITE_QUEEN));
	}

	#[test]
	fn test_remove_permission() {
		let mut castling = Castling::new(CastlingPermissions::WHITE_KING | CastlingPermissions::WHITE_QUEEN);
		castling.remove(CastlingPermissions::WHITE_QUEEN);

		assert!(!castling.has(CastlingPermissions::WHITE_QUEEN));
		assert!(castling.has(CastlingPermissions::WHITE_KING));
	}

	#[test]
	fn test_default() {
		let castling = Castling::default();
		
		assert_eq!(castling.0, CastlingPermissions::ALL.bits());
	}
}
