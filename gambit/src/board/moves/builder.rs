//! Module containing a [`MoveBuilder`], used to build a move from the higher-level enum variants

use crate::{location::Square, piece::PieceType};
use super::{shifts::MoveShifts, Move, MoveUnderlyingType};

/// A builder for constructing chess moves
#[allow(clippy::module_name_repetitions, clippy::struct_excessive_bools)]
pub struct MoveBuilder {
	data: MoveUnderlyingType,
	
	piece_set: bool,
	from_set: bool,
	to_set: bool,
	capture_set: bool,
	promotion_set: bool,
}

impl MoveBuilder {
	/// Creates a new [`MoveBuilder`]
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self {
			data: 0,

			piece_set: false,
			from_set: false,
			to_set: false,
			capture_set: false,
			promotion_set: false
		}
	}

	/// Set the [`PieceType`] of the move
	pub fn piece(&mut self, piece: PieceType) -> &mut Self {
		let piece = if piece == PieceType::None {
			Move::NULL_PIECE
		} else {
			MoveUnderlyingType::from(piece.bits())
		};

		self.piece_set = true;
		self.data |= piece << MoveShifts::Piece.as_u8();

		self
	}

	/// Set the [`Square`] that the piece has moved from
	pub fn from(&mut self, square: Square) -> &mut Self {
		self.data |= (square as MoveUnderlyingType) << MoveShifts::From.as_u8();
		self.from_set = true;

		self
	}

	/// Set the [`Square`] that the piece has moved to
	pub fn to(&mut self, square: Square) -> &mut Self {
		self.data |= (square as MoveUnderlyingType) << MoveShifts::To.as_u8();
		self.to_set = true;

		self
	}

	/// Set the [`PieceType`] of the captured piece
	pub fn capture(&mut self, piece: PieceType) -> &mut Self {
		let piece = if piece == PieceType::None {
			Move::NULL_PIECE
		} else {
			MoveUnderlyingType::from(piece.bits())
		};

		self.data |= piece << MoveShifts::Capture.as_u8();
		self.capture_set = true;

		self
	}

	/// Set the [`PieceType`] that the pawn promotes to
	pub fn promotion(&mut self, piece: PieceType) -> &mut Self {
		let piece = if piece == PieceType::None {
			Move::NULL_PIECE
		} else {
			MoveUnderlyingType::from(piece.bits())
		};

		self.data |= piece << MoveShifts::Promotion.as_u8();
		self.promotion_set = true;

		self
	}

	/// Set whether this move is an en passant attack
	#[inline]
	pub fn en_passant(&mut self, value: bool) -> &mut Self {
		self.data |= MoveUnderlyingType::from(value) << MoveShifts::EnPassant.as_u8();

		self
	}

	/// Set whether this move is a double step
	#[inline]
	pub fn double_step(&mut self, value: bool) -> &mut Self {
		self.data |= MoveUnderlyingType::from(value) << MoveShifts::DoubleStep.as_u8();

		self
	}

	/// Set whether this move involves castling
	#[inline]
	pub fn castling(&mut self, value: bool) -> &mut Self {
		self.data |= MoveUnderlyingType::from(value) << MoveShifts::Castling.as_u8();

		self
	}

	/// Converts the data into a binary encoded [`Move`]
	/// 
	/// # Panics
	/// 
	/// This may panic in builds where `cfg!(debug_assertions)` is true, where the 3 required attributes (piece, from, to) are not set
	#[must_use]
	pub fn to_move(&mut self) -> Move {
		assert!(!(cfg!(debug_assertions) && (!self.piece_set || !self.from_set || !self.to_set)), "Invalid piece constructed, missing one of: piece, from or to sq");

		if !self.capture_set {
			self.data |= Move::NULL_PIECE << MoveShifts::Capture.as_u8();
		}

		if !self.promotion_set {
			self.data |= Move::NULL_PIECE << MoveShifts::Promotion.as_u8();
		}

		Move::new(self.data)
	}
}

impl Default for MoveBuilder {
	fn default() -> Self {
		MoveBuilder::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_move_builder_basic_move() {
		let mut builder = MoveBuilder::new();
		
		builder
			.piece(PieceType::Knight)
			.from(Square::B1)
			.to(Square::C3);
		
		let mv = builder.to_move();

		assert_eq!(mv.piece(), PieceType::Knight);
		assert_eq!(mv.from(), Square::B1);
		assert_eq!(mv.to(), Square::C3);
	}

	#[test]
	fn test_move_builder_capture_move() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::Bishop)
			.from(Square::C1)
			.to(Square::G5)
			.capture(PieceType::Knight);
		
		let mv = builder.to_move();

		assert_eq!(mv.piece(), PieceType::Bishop);
		assert_eq!(mv.from(), Square::C1);
		assert_eq!(mv.to(), Square::G5);
		assert_eq!(mv.capture(), PieceType::Knight);
	}

	#[test]
	fn test_move_builder_promotion_move() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::Pawn)
			.from(Square::E7)
			.to(Square::E8)
			.promotion(PieceType::Queen);
		
		let mv = builder.to_move();

		assert_eq!(mv.piece(), PieceType::Pawn);
		assert_eq!(mv.from(), Square::E7);
		assert_eq!(mv.to(), Square::E8);
		assert_eq!(mv.promotion(), PieceType::Queen);
	}

	#[test]
	fn test_move_builder_en_passant_move() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::Pawn)
			.from(Square::E5)
			.to(Square::D6)
			.en_passant(true);

		let mv = builder.to_move();

		assert_eq!(mv.piece(), PieceType::Pawn);
		assert_eq!(mv.from(), Square::E5);
		assert_eq!(mv.to(), Square::D6);
		assert!(mv.en_passant());
	}

	#[test]
	fn test_move_builder_castling_move() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::King)
			.from(Square::E1)
			.to(Square::G1)
			.castling(true);
		
		let mv = builder.to_move();

		assert_eq!(mv.piece(), PieceType::King);
		assert_eq!(mv.from(), Square::E1);
		assert_eq!(mv.to(), Square::G1);
		assert!(mv.castling());
	}

	#[test]
	fn test_move_builder_double_step_move() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::Pawn)
			.from(Square::D2)
			.to(Square::D4)
			.double_step(true);
		
		let mv = builder.to_move();

		assert_eq!(mv.piece(), PieceType::Pawn);
		assert_eq!(mv.from(), Square::D2);
		assert_eq!(mv.to(), Square::D4);
		assert!(mv.double_step());
	}

	#[test]
	#[should_panic(expected = "Invalid piece constructed")]
	fn test_move_builder_missing_piece() {
		let mut builder = MoveBuilder::new();
		
		builder.from(Square::E2)
			.to(Square::E4);

		let _ = builder.to_move();
	}

	#[test]
	#[should_panic(expected = "Invalid piece constructed")]
	fn test_move_builder_missing_from() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::Pawn)
			.to(Square::E4);
		
		let _ = builder.to_move();
	}

	#[test]
	#[should_panic(expected = "Invalid piece constructed")]
	fn test_move_builder_missing_to() {
		let mut builder = MoveBuilder::new();
		
		builder.piece(PieceType::Pawn)
			.from(Square::E2);
		
		let _ = builder.to_move();
	}
}
