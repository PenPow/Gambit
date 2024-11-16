//! Module containing the shifts needed to encode a move

use std::ops::{Shl, Shr};
use super::MoveUnderlyingType;

/// The different shifts that are used in a [`super::Move`] to encode the data.
#[allow(clippy::module_name_repetitions)]
pub enum MoveShifts {
	/// The offset for the [`crate::piece::PieceType`]
	Piece = 0,
	
	/// The offset for the [`crate::location::Square`] that the piece moved from
	From = 3,
	
	/// The offset for the [`crate::location::Square`] that the piece moved to
	To = 9,

	/// The offset for the [`crate::piece::PieceType`] that the piece captured
	Capture = 15,
	
	/// The offset for the [`crate::piece::PieceType`] that the piece promoted to
	Promotion = 18,

	/// The offset for the [`bool`] representing whether it is an en passant attack
	EnPassant = 21,
	
	/// The offset for the [`bool`] representing whether it is an double step
	DoubleStep = 22,

	/// The offset for the [`bool`] representing whether the move involves castling
	Castling = 23,
}

impl MoveShifts {
	/// Converts the shift to a [`u8`]
	#[inline(always)]
	#[must_use]
	pub const fn as_u8(self) -> u8 {
		self as u8
	}
}

impl Shl<MoveShifts> for MoveUnderlyingType {
	type Output = MoveUnderlyingType;

	fn shl(self, rhs: MoveShifts) -> Self::Output {
		self << rhs.as_u8()
	}
}

impl Shr<MoveShifts> for MoveUnderlyingType {
	type Output = MoveUnderlyingType;

	fn shr(self, rhs: MoveShifts) -> Self::Output {
		self >> rhs.as_u8()
	}
}