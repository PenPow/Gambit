use std::fmt;
use super::Move;

impl Move {
	pub(super) fn as_str(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Move Data: {self:#b}\n")?;

		let piece = self.piece();
		writeln!(f, "Piece: {} ({:#b})", piece.as_str(), piece)?;

		let from = self.from();
		writeln!(f, "From: {}", from.as_str())?;

		let to = self.to();
		writeln!(f, "To: {}", to.as_str())?;

		let capture = self.capture();
		writeln!(f, "Capture: {} ({:#b})", capture.as_str(), capture)?;

		let promotion = self.promotion();
		writeln!(f, "Promotion: {} ({:#b})", promotion.as_str(), promotion)?;

		let en_passant = self.en_passant();
		writeln!(f, "En Passant: {en_passant}")?;

		let double_step = self.double_step();
		writeln!(f, "Double Step: {double_step}")?;

		let castling = self.castling();
		writeln!(f, "Castling: {castling}")?;

		Ok(())
	}
}

impl fmt::UpperHex for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::UpperHex::fmt(&self.bits(), f)
	}
}

impl fmt::LowerHex for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::LowerHex::fmt(&self.bits(), f)
	}
}

impl fmt::Octal for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Octal::fmt(&self.bits(), f)
	}
}

impl fmt::Binary for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&self.bits(), f)
	}
}

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