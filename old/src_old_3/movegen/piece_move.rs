use crate::{board::{location::{Square, Squares}, piece::{Piece, Pieces}}, dbg_assert_piece_in_range, dbg_assert_square_in_range, impl_output_types};
use std::fmt;

type MoveShift = u8;
pub struct MoveShifts;
impl MoveShifts {
	pub const PIECE: MoveShift = 0;
	pub const FROM: MoveShift = 3;
	pub const TO: MoveShift = 9;
	pub const CAPTURE: MoveShift = 15;
	pub const PROMOTION: MoveShift = 18;
	pub const EN_PASSANT: MoveShift = 21;
	pub const DOUBLE_STEP: MoveShift = 23;
	pub const CASTLING: MoveShift = 24;
}

pub type MoveType = usize;
#[derive(Clone, Copy)]
pub struct Move(pub MoveType);
impl Move {
	pub const NULL: Move = Move::new(Pieces::NONE);

	pub const fn new(data: MoveType) -> Self {
		Self(data)
	}

	pub fn piece(&self) -> Piece {
		(self.0 >> MoveShifts::PIECE) & 0b111
	}

	pub fn from(&self) -> Square {
		(self.0 >> MoveShifts::FROM) & 0b111111
	}

	pub fn to(&self) -> Square {
		(self.0 >> MoveShifts::TO) & 0b111111
	}

	pub fn capture(&self) -> Piece {
		(self.0 >> MoveShifts::CAPTURE) & 0b111
	}

	pub fn promotion(&self) -> Piece {
		(self.0 >> MoveShifts::PROMOTION) & 0b111
	}

	pub fn en_passant(&self) -> bool {
		((self.0 >> MoveShifts::EN_PASSANT) & 0b1) == 1
	}

	pub fn double_step(&self) -> bool {
		((self.0 >> MoveShifts::DOUBLE_STEP) & 0b1) == 1
	}

	pub fn castling(&self) -> bool {
		((self.0 >> MoveShifts::CASTLING) & 0b1) == 1
	}

	fn as_str(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Move Data: {:#b}\n", self)?;

		let piece = self.piece();
		writeln!(f, "Piece: {} ({:#b})", Pieces::as_str(piece), piece)?;

		let from = self.from();
		writeln!(f, "From: {} ({:#b})", Squares::as_str(from), from)?;

		let to = self.to();
		writeln!(f, "To: {} ({:#b})", Squares::as_str(to), to)?;

		let capture = self.capture();
		writeln!(f, "Capture: {} ({:#b})", Pieces::as_str(capture), capture)?;

		let promotion = self.promotion();
		writeln!(f, "Promotion: {} ({:#b})", Pieces::as_str(promotion), promotion)?;

		let en_passant = self.en_passant();
		writeln!(f, "En Passant: {}", en_passant)?;

		let double_step = self.double_step();
		writeln!(f, "Double Step: {}", double_step)?;

		let castling = self.castling();
		writeln!(f, "Castling: {}", castling)?;

		Ok(())
	}
}

impl_output_types!(Move);

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

impl fmt::Debug for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

#[derive(Clone, Copy)]
pub struct MoveBuilder {
	data: MoveType,
	has_called_capture: bool,
	has_called_promotion: bool
}

impl MoveBuilder {
	pub const fn piece(piece: Piece) -> MoveBuilder {
		dbg_assert_piece_in_range!(piece);

		Self {
			data: piece,
			has_called_capture: false,
			has_called_promotion: false
		}
	}

	pub fn from(&mut self, square: Square) -> &mut Self {
		dbg_assert_square_in_range!(square);

		self.data |= square << MoveShifts::FROM;
		self
	}

	pub fn to(&mut self, square: Square) -> &mut Self {
		dbg_assert_square_in_range!(square);

		self.data |= square << MoveShifts::TO;
		self
	}

	pub fn capture(&mut self, piece: Piece) -> &mut Self {
		dbg_assert_piece_in_range!(piece);

		self.has_called_capture = true;

		self.data |= piece << MoveShifts::CAPTURE;
		self
	}

	pub fn promotion(&mut self, piece: Piece) -> &mut Self {
		dbg_assert_piece_in_range!(piece);

		self.has_called_promotion = true;

		self.data |= piece << MoveShifts::PROMOTION;
		self
	}

	pub fn en_passant(&mut self) -> &mut Self {
		self.data |= 1 << MoveShifts::EN_PASSANT;
		self
	}

	pub fn double_step(&mut self) -> &mut Self {
		self.data |= 1 << MoveShifts::DOUBLE_STEP;
		self
	}

	pub fn castling(&mut self) -> &mut Self {
		self.data |= 1 << MoveShifts::CASTLING;
		self
	}

	pub fn to_move(&mut self) -> Move {
		if !self.has_called_capture {
			self.data |= Pieces::NONE << MoveShifts::CAPTURE
		}

		if !self.has_called_promotion {
			self.data |= Pieces::NONE << MoveShifts::PROMOTION
		}

		Move::new(self.data)
	}
}