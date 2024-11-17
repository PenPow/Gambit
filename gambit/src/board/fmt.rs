use std::fmt;
use crate::piece::{Colour, PieceType};

use super::Board;

impl Board {
	pub(super) fn as_str(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f)?;

		writeln!(f, "White Kings \n{}", self.piece_bitboards[Colour::White as usize][PieceType::King as usize])?;
		writeln!(f, "White Queens \n{}", self.piece_bitboards[Colour::White as usize][PieceType::Queen as usize])?;
		writeln!(f, "White Rooks \n{}", self.piece_bitboards[Colour::White as usize][PieceType::Rook as usize])?;
		writeln!(f, "White Bishops \n{}", self.piece_bitboards[Colour::White as usize][PieceType::Bishop as usize])?;
		writeln!(f, "White Knights \n{}", self.piece_bitboards[Colour::White as usize][PieceType::Knight as usize])?;
		writeln!(f, "White Pawns \n{}", self.piece_bitboards[Colour::White as usize][PieceType::Pawn as usize])?;

		writeln!(f, "Black Kings \n{}", self.piece_bitboards[Colour::Black as usize][PieceType::King as usize])?;
		writeln!(f, "Black Queens \n{}", self.piece_bitboards[Colour::Black as usize][PieceType::Queen as usize])?;
		writeln!(f, "Black Rooks \n{}", self.piece_bitboards[Colour::Black as usize][PieceType::Rook as usize])?;
		writeln!(f, "Black Bishops \n{}", self.piece_bitboards[Colour::Black as usize][PieceType::Bishop as usize])?;
		writeln!(f, "Black Knights \n{}", self.piece_bitboards[Colour::Black as usize][PieceType::Knight as usize])?;
		writeln!(f, "Black Pawns \n{}", self.piece_bitboards[Colour::Black as usize][PieceType::Pawn as usize])?;

		writeln!(f, "Combined \n{}", self.occupancy())?;

		writeln!(f, "Zobrist Key: {}", self.state.zobrist_key)?;
		writeln!(f, "Side to Move: {}", self.state.active_colour.as_char())?;
		writeln!(f, "En Passant Square: {}", if let Some(square) = self.state.en_passant_square { square.as_str() } else { String::from("-") })?;
		writeln!(f, "Castling Availability: {:b} (qkQK)", self.state.castling_availability)?;
		writeln!(f, "Half Move Clock: {}", self.state.halfmove_clock)?;
		writeln!(f, "Full Move Timer: {}", self.state.fullmove_number)?;		

		Ok(())
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

impl fmt::Debug for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}