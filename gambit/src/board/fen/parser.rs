use crate::{bitboard::Bitboard, location::{File, Rank, Square}, piece::{Castling, CastlingPermissions, Colour, PieceType}};
use super::{error::FenError, string::Fen};

/// A parser for [Forsyth-Edwards Notation (FEN)](https://www.chessprogramming.org/Forsyth-Edwards_Notation) strings, which represents the state of a chess game.
///
/// # Examples
///
/// ```
/// use gambit::board::fen::{FenParser, Fen};
///
/// let fen_string = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
/// let parser = FenParser::new(fen_string);
/// ```
pub struct FenParser<'a> {
	/// The [Fen] string to parse
	fen: Fen<'a>
}

impl<'a> FenParser<'a> {
	/// Create a new parser from a [Fen] string
	#[inline]
	#[must_use]
	pub const fn new(fen: Fen<'a>) -> Self {
		Self {
			fen
		}
	}

	/// Modifies the provided bitboard arrays to setup the piece placement
	/// 
	/// # Errors
	/// 
	/// Will return [`FenError::InvalidPiecePlacement`] if the piece placement will result in an invalid board
	pub fn parse_piece_placement(
		&self,
		piece_bitboards: &mut [[Bitboard; PieceType::COUNT]; Colour::COUNT],
		side_bitboards: &mut [Bitboard; Colour::COUNT]
	) -> Result<(), FenError> {
		let mut rank = Rank::R8;
		let mut file = File::A;

		let mut is_at_end = false;

		for char in self.fen.piece_placement.chars() {
			let square = Square::from_coords((file, rank));

			let colour = if char.is_ascii_uppercase() {
				Colour::White
			} else {
				Colour::Black
			};

			let mut is_piece_char = true;

			match char.to_ascii_lowercase() {
				'k' => piece_bitboards[colour as usize][PieceType::King as usize] |= square.bitboard(),
				'q' => piece_bitboards[colour as usize][PieceType::Queen as usize] |= square.bitboard(),
				'r' => piece_bitboards[colour as usize][PieceType::Rook as usize] |= square.bitboard(),
				'b' => piece_bitboards[colour as usize][PieceType::Bishop as usize] |= square.bitboard(),
				'n' => piece_bitboards[colour as usize][PieceType::Knight as usize] |= square.bitboard(),
				'p' => piece_bitboards[colour as usize][PieceType::Pawn as usize] |= square.bitboard(),

				'1'..='8' => {
					is_piece_char = false;

					if let Some(x) = char.to_digit(10) {
						if (file as u32) + x == 8 {
							is_at_end = true;
						} else {
							file = file.offset(x as i8).ok_or(FenError::InvalidPiecePlacement)?;
						}
					}
				},

				'/' => {
					is_piece_char = false;

					if !is_at_end {
						return Err(FenError::InvalidPiecePlacement)
					}

					is_at_end = false;

					rank = rank.offset(-1).ok_or(FenError::InvalidPiecePlacement)?;
					file = File::A;
				}

				_ => return Err(FenError::InvalidPiecePlacement)
			};

			if is_piece_char {
				if file == File::H {
					is_at_end = true;
				} else {
					file = file.offset(1).ok_or(FenError::InvalidPiecePlacement)?;
				};

				side_bitboards[colour as usize] |= square.bitboard();
			}
		}

		if is_at_end {
			Ok(())
		} else {
			Err(FenError::InvalidPiecePlacement)
		}
	}

	/// Parses the active colour from the [`Fen`] string
	/// 
	/// # Errors
	/// 
	/// Will return [`FenError::InvalidActiveColor`] if the active colour is invalid
	pub fn parse_active_colour(&self) -> Result<Colour, FenError> {
		match self.fen.active_colour {
			"w" => Ok(Colour::White),
			"b" => Ok(Colour::Black),
			_ => Err(FenError::InvalidActiveColor)
		}
	}

	/// Parses the castling rights from the [`Fen`] string
	/// 
	/// # Errors
	/// 
	/// Will return [`FenError::InvalidCastlingRights`] if the castling rights is invalid
	pub fn parse_castling_rights(&self) -> Result<Castling, FenError> {
		let castling = if self.fen.castling_rights.is_empty() { CastlingPermissions::NONE } else {
			let mut castling = CastlingPermissions::NONE;
			
			for char in self.fen.castling_rights.chars() {
				match char {
					'K' => castling |= CastlingPermissions::WHITE_KING,
					'Q' => castling |= CastlingPermissions::WHITE_QUEEN,
					'k' => castling |= CastlingPermissions::BLACK_KING,
					'q' => castling |= CastlingPermissions::BLACK_QUEEN,
					_ => return Err(FenError::InvalidCastlingRights)
				}
			}

			castling
		};

		Ok(Castling::new(castling))
	}

	/// Parses the en passant square from the [`Fen`] string
	/// 
	/// # Errors
	/// 
	/// Will return [`FenError::InvalidEnPassantSquare`] if the square provided is invalid
	pub fn parse_en_passant_square(&self) -> Result<Option<Square>, FenError> {
		if self.fen.en_passant_targets == "-" {
			Ok(None)
		} else {
			Square::from_algebraic_notation(self.fen.en_passant_targets).map(Some).map_err(|_| FenError::InvalidEnPassantSquare)
		}
	}

	/// Parses the half move clock from the [`Fen`] string
	/// 
	/// # Errors
	/// 
	/// Will return [`FenError::InvalidHalfmoveClock`] if the clock is not a valid [`u8`]
	pub fn parse_halfmove_clock(&self) -> Result<u8, FenError> {
		self.fen.halfmove_clock.parse::<u8>().map_err(|_| FenError::InvalidHalfmoveClock)
	}

	/// Parses the full move number from the [`Fen`] string
	/// 
	/// # Errors
	/// 
	/// Will return [`FenError::InvalidFullmoveNumber`] if the number is not a valid [`u8`]
	pub fn parse_fullmove_number(&self) -> Result<u8, FenError> {
		self.fen.fullmove_number.parse::<u8>().map_err(|_| FenError::InvalidFullmoveNumber)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_piece_placement() {
		let fen_string = Fen::STARTING_POSITION;
		let parser = FenParser::new(fen_string);

		let mut piece_bitboards = [[Bitboard::EMPTY; PieceType::COUNT]; Colour::COUNT];
		let mut side_bitboards = [Bitboard::EMPTY; Colour::COUNT];

		assert!(parser.parse_piece_placement(&mut piece_bitboards, &mut side_bitboards).is_ok());
	}

	#[test]
	fn test_parse_active_colour() {
		let fen_string = Fen::STARTING_POSITION;
		let parser = FenParser::new(fen_string);

		assert_eq!(parser.parse_active_colour().unwrap(), Colour::White);
	}

	#[test]
	fn test_parse_castling_rights() {
		let fen_string = Fen::STARTING_POSITION;
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_castling_rights().is_ok());
	}

	#[test]
	fn test_parse_en_passant_square() {
		let fen_string = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1").unwrap();
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_en_passant_square().is_ok());
	}

	#[test]
	fn test_parse_halfmove_clock() {
		let fen_string = Fen::STARTING_POSITION;
		let parser = FenParser::new(fen_string);

		assert_eq!(parser.parse_halfmove_clock().unwrap(), 0);
	}

	#[test]
	fn test_parse_fullmove_number() {
		let fen_string = Fen::STARTING_POSITION;
		let parser = FenParser::new(fen_string);

		assert_eq!(parser.parse_fullmove_number().unwrap(), 1);
	}

	#[test]
	fn test_invalid_piece_placement() {
		let fen_string: Fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN w KQkq - 0 1").unwrap();
		let parser = FenParser::new(fen_string);

		let mut piece_bitboards = [[Bitboard::EMPTY; PieceType::COUNT]; Colour::COUNT];
		let mut side_bitboards = [Bitboard::EMPTY; Colour::COUNT];

		assert!(parser.parse_piece_placement(&mut piece_bitboards, &mut side_bitboards).is_err());
	}

	#[test]
	fn test_invalid_active_colour() {
		let fen_string: Fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1").unwrap();
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_active_colour().is_err());
	}

	#[test]
	fn test_invalid_castling_rights() {
		let fen_string: Fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1").unwrap();
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_castling_rights().is_err());
	}

	#[test]
	fn test_invalid_en_passant_square() {
		let fen_string: Fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e9 0 1").unwrap();
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_en_passant_square().is_err());
	}

	#[test]
	fn test_invalid_halfmove_clock() {
		let fen_string: Fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - x 1").unwrap();
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_halfmove_clock().is_err());
	}

	#[test]
	fn test_invalid_fullmove_number() {
		let fen_string: Fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 x").unwrap();
		let parser = FenParser::new(fen_string);

		assert!(parser.parse_fullmove_number().is_err());
	}
}
