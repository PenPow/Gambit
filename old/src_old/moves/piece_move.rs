use crate::board::{piece::{Piece, PieceType}, square::Square};

type MoveShift = usize;
pub struct MoveShifts {}
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

#[derive(Clone, Copy, Debug)]
pub struct Move {
	data: usize
}

impl Move {
	pub fn new(data: usize) -> Self {
		Self {
			data
		}
	}

	pub fn piece(&self) -> Piece {
		(self.data >> MoveShifts::PIECE) & 0b111
	}

	pub fn from(&self) -> Square {
		(self.data >> MoveShifts::FROM) & 0b111111
	}

	pub fn to(&self) -> Square {
		(self.data >> MoveShifts::TO) & 0b111111
	}

	pub fn capture(&self) -> Piece {
		(self.data >> MoveShifts::CAPTURE) & 0b111
	}

	pub fn promotion(&self) -> Piece {
		(self.data >> MoveShifts::PROMOTION) & 0b111
	}

	pub fn en_passant(&self) -> bool {
		((self.data >> MoveShifts::EN_PASSANT) & 0b1) == 1
	}

	pub fn double_step(&self) -> bool {
		((self.data >> MoveShifts::DOUBLE_STEP) & 0b1) == 1
	}

	pub fn castling(&self) -> bool {
		((self.data >> MoveShifts::CASTLING) & 0b1) == 1
	}

	pub fn debug(&self) -> String {
		format!("Piece: {}, From: {}, To: {}, Capture: {}, Promotion: {}, En Passant: {}, Double Step: {}, Castling: {}", PieceType::to_str(self.piece()), self.from(), self.to(), PieceType::to_str(self.capture()), PieceType::to_str(self.promotion()), self.en_passant(), self.double_step(), self.castling())
	}
}