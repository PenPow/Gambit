//! Module containing useful enums to represent pieces
#![allow(clippy::module_name_repetitions)]

mod castling;
mod colour;
mod piece_type;

use std::fmt::Write;

pub use castling::{Castling, CastlingPermissions};
pub use colour::Colour;
pub use piece_type::PieceType;

/// Represents a chess piece with a specific colour and type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Piece {
	/// The colour of this piece
	pub colour: Colour,
	/// The actual type of the piece
	pub piece_type: PieceType
}

impl Piece {
	/// Creates a new `Piece` with the given colour and piece type.
	/// # Examples
	/// ```
	/// use gambit::piece::{Piece, Colour, PieceType};
	/// 
	/// let piece = Piece::new((Colour::White, PieceType::King));
	/// ```
	#[inline]
	#[must_use]
	pub const fn new((colour, piece_type): (Colour, PieceType)) -> Piece {
		Piece {
			colour,
			piece_type
		}
	}

	/// Creates a new `Piece` from an index
	/// 
	/// # Examples
	/// ```
	/// use gambit::piece::{Piece, Colour, PieceType};
	/// 
	/// let piece1 = Piece::new((Colour::Black, PieceType::Bishop));
	/// let piece2 = Piece::from(8);
	/// assert_eq!(piece1, piece2)
	/// ```
	#[inline]
	#[must_use]
	pub const fn from(index: u8) -> Piece {
		debug_assert!(index <= (PieceType::None as u8));

		if index > (PieceType::King as u8) {
			Piece {
				colour: Colour::Black,
				piece_type: PieceType::new(index - (PieceType::COUNT as u8))
			}
		} else {
			Piece {
				colour: Colour::White,
				piece_type: PieceType::new(index)
			}
		}
	}

	/// Converts this piece to a character, capitalizing it based upon colour
	#[inline]
	#[must_use]
	pub const fn as_char(&self) -> char {
		match self.colour {
			Colour::White => self.piece_type.as_uppercase_char(),
			Colour::Black => self.piece_type.as_char(),
		}
	}

	/// Converts this piece to a u8 to represent as a move
	#[inline]
	#[must_use]
	pub const fn index(&self) -> u8 {
		if matches!(self.piece_type, PieceType::None) {
			PieceType::None as u8
		} else {
			let offset = match self.colour {
				Colour::White => 0,
				Colour::Black => PieceType::COUNT as u8,
			};
	
			(self.piece_type as u8) + offset
		}
		
	}
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}