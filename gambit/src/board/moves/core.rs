use crate::{location::Square, piece::PieceType};
use super::shifts::MoveShifts;

/// The underlying number type that the move is stored as
pub type MoveUnderlyingType = u32;

/// Represents a move in the game.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(MoveUnderlyingType);

impl Move {
	/// A [`MoveUnderlyingType`] which represents `(2^3) - 1`, our null piece.
	/// 
	/// This differs from [`PieceType::None`] as that one takes into account black vs white pieces.
	pub(super) const NULL_PIECE: MoveUnderlyingType = 7;

	/// An invalid or null move
	pub const NULL: Move = Move::new(0b0001_1111_1000_0000_0000_0111);

	/// Creates a new [`Move`] from the bits
	///
	/// You should never need to use this directly, instead use a [`super::builder::MoveBuilder`] and call [`super::builder::MoveBuilder::to_move()`]
	#[inline]
	#[must_use]
	pub const fn new(data: MoveUnderlyingType) -> Self {
		Self(data)
	}

	/// Obtain the [`PieceType`] of the piece moved
	#[must_use]
	pub const fn piece(&self) -> PieceType {
		let piece = (self.bits() >> MoveShifts::Piece.as_u8()) & 0b111;

		if piece == Self::NULL_PIECE {
			PieceType::None
		} else {
			PieceType::new(piece as u8)
		}
	}

	/// Obtain the [`Square`] the piece moved from
	#[must_use]
	pub const fn from(&self) -> Square {
		let square_index = (self.bits() >> MoveShifts::From.as_u8()) & 0b11_1111;

		Square::new(square_index as u8)
	}

	/// Obtain the [`Square`] the piece moved to
	#[must_use]
	pub const fn to(&self) -> Square {
		let square_index = (self.bits() >> MoveShifts::To.as_u8()) & 0b11_1111;

		Square::new(square_index as u8)
	}

	/// Obtain the [`PieceType`] captured
	#[must_use]
	pub const fn capture(&self) -> PieceType {
		let piece = (self.bits() >> MoveShifts::Capture.as_u8()) & 0b111;

		if piece == Self::NULL_PIECE {
			PieceType::None
		} else {
			PieceType::new(piece as u8)
		}
	}

	/// Obtain the [`PieceType`] promoted to
	#[must_use]
	pub const fn promotion(&self) -> PieceType {
		let piece = (self.bits() >> MoveShifts::Promotion.as_u8()) & 0b111;

		if piece == Self::NULL_PIECE {
			PieceType::None
		} else {
			PieceType::new(piece as u8)
		}
	}

	/// Obtain the [`bool`] representing whether this was an en passant attack
	#[must_use]
	pub const fn en_passant(&self) -> bool {
		let en_passant = (self.bits() >> MoveShifts::EnPassant.as_u8()) & 0b1;

		en_passant == 1
	}

	/// Obtain the [`bool`] representing whether this was a double step move
	#[must_use]
	pub const fn double_step(&self) -> bool {
		let double_step = (self.bits() >> MoveShifts::DoubleStep.as_u8()) & 0b1;

		double_step == 1
	}

	/// Obtain the [`bool`] representing whether this move involved castling
	#[must_use]
	pub const fn castling(&self) -> bool {
		let castling = (self.bits() >> MoveShifts::Castling.as_u8()) & 0b1;

		castling == 1
	}

	/// Check whether this move is equal to [`Move::NULL`]
	#[must_use]
	pub const fn is_null(&self) -> bool {
		self.from() as u8 == self.to() as u8
	}

	/// Obtain the bits representing the move
	#[must_use]
	pub const fn bits(&self) -> MoveUnderlyingType {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_move_new() {
		let data = 0;
		let m = Move::new(data);
		assert_eq!(m.bits(), data);
	}

	#[test]
	fn test_move_piece() {
		let data = (PieceType::Knight as u32) << MoveShifts::Piece.as_u8();
		let m = Move::new(data);
		assert_eq!(m.piece(), PieceType::Knight);
	}

	#[test]
	fn test_move_null_piece() {
		let data = Move::NULL_PIECE << MoveShifts::Piece.as_u8();
		let m = Move::new(data);
		assert_eq!(m.piece(), PieceType::None);
	}

	#[test]
	fn test_move_from() {
		let data = (Square::new(12) as u32) << MoveShifts::From.as_u8();
		let m = Move::new(data);
		assert_eq!(m.from(), Square::new(12));
	}

	#[test]
	fn test_move_to() {
		let data = (Square::new(28) as u32) << MoveShifts::To.as_u8();
		let m = Move::new(data);
		assert_eq!(m.to(), Square::new(28));
	}

	#[test]
	fn test_move_capture() {
		let data = (PieceType::Bishop as u32) << MoveShifts::Capture.as_u8();
		let m = Move::new(data);
		assert_eq!(m.capture(), PieceType::Bishop);
	}

	#[test]
	fn test_move_promotion() {
		let data = (PieceType::Queen as u32) << MoveShifts::Promotion.as_u8();
		let m = Move::new(data);
		assert_eq!(m.promotion(), PieceType::Queen);
	}

	#[test]
	fn test_move_en_passant() {
		let data = 1 << MoveShifts::EnPassant.as_u8();
		let m = Move::new(data);
		assert!(m.en_passant());
	}

	#[test]
	fn test_move_double_step() {
		let data = 1 << MoveShifts::DoubleStep.as_u8();
		let m = Move::new(data);
		assert!(m.double_step());
	}

	#[test]
	fn test_move_castling() {
		let data = 1 << MoveShifts::Castling.as_u8();
		let m = Move::new(data);
		assert!(m.castling());
	}

	#[test]
	fn test_move_combined_flags() {
		let data = ((PieceType::Pawn as u32) << MoveShifts::Piece.as_u8())
			| ((Square::new(12) as u32) << MoveShifts::From.as_u8())
			| ((Square::new(20) as u32) << MoveShifts::To.as_u8())
			| (1 << MoveShifts::DoubleStep.as_u8());
		let m = Move::new(data);
		assert_eq!(m.piece(), PieceType::Pawn);
		assert_eq!(m.from(), Square::new(12));
		assert_eq!(m.to(), Square::new(20));
		assert!(m.double_step());
	}

	#[test]
	fn test_move_null() {
		let m = Move::NULL;
		assert_eq!(m.piece(), PieceType::None);
		assert_eq!(m.from(), Square::A1);
		assert_eq!(m.to(), Square::A1);
		assert_eq!(m.capture(), PieceType::None);
		assert_eq!(m.promotion(), PieceType::None);
		assert!(!m.en_passant());
		assert!(!m.double_step());
		assert!(!m.castling());
	}

	#[test]
	fn test_move_is_null() {
		let null_move = Move::NULL;
		assert!(null_move.is_null());
		
		let data = ((PieceType::Pawn as u32) << MoveShifts::Piece.as_u8())
			| ((Square::new(12) as u32) << MoveShifts::From.as_u8())
			| ((Square::new(20) as u32) << MoveShifts::To.as_u8());

		let non_null_move = Move::new(data);
		assert!(!non_null_move.is_null());
	}
}
