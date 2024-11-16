use super::error::FenError;

/// Represents a [FEN (Forsyth-Edwards Notation)](https://www.chessprogramming.org/Forsyth-Edwards_Notation) for a chess board.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fen<'a> {
	pub(crate) piece_placement: &'a str,
	pub(crate) active_colour: &'a str,
	pub(crate) castling_rights: &'a str,
	pub(crate) en_passant_targets: &'a str,
	pub(crate) halfmove_clock: &'a str,
	pub(crate) fullmove_number: &'a str,
}

type FenParts<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str);

impl<'a> Fen<'a> {
	/// The [`Fen`] for the starting position
	pub const STARTING_POSITION: Fen<'static> = Fen::from_list(("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"));

	/// The [`Fen`] for an empty chessboard
	pub const EMPTY: Fen<'static> = Fen::from_list(("8/8/8/8/8/8/8/8", "w", "-", "-", "0", "1"));

	/// Create a new [`Fen`] from a string, validating the format
	/// 
	/// # Errors
	/// 
	/// Will return [`Err`] when the [`Fen`]'s format is incorrect. Does not parse and validate the internal sections - instead use [`crate::board::fen::FenParser`]
	pub fn new(fen: &'a str) -> Result<Fen<'a>, FenError> {
		let mut split = fen
			.split([' ', '_'])
			.filter(|s| !s.is_empty());

		let piece_placement: &str = split.next().ok_or(FenError::InvalidFormat)?;
		let active_colour: &str  = split.next().ok_or(FenError::InvalidFormat)?;
		let castling_rights: &str  = split.next().ok_or(FenError::InvalidFormat)?;
		let en_passant_targets: &str  = split.next().ok_or(FenError::InvalidFormat)?;
		let halfmove_clock: &str  = split.next().ok_or(FenError::InvalidFormat)?;
		let fullmove_number: &str  = split.next().ok_or(FenError::InvalidFormat)?;

		let fen = Self {
			piece_placement,
			active_colour,
			castling_rights,
			en_passant_targets,
			halfmove_clock,
			fullmove_number
		};

		Ok(fen)
	}

	/// Converts from a tuple containing each part into a [`Fen`] object
	#[inline]
	#[must_use]
	pub const fn from_list(fen: FenParts<'a>) -> Fen<'a> {
		Self {
			piece_placement: fen.0,
			active_colour: fen.1,
			castling_rights: fen.2,
			en_passant_targets: fen.3,
			halfmove_clock: fen.4,
			fullmove_number: fen.5
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_starting_position() {
		let fen = Fen::STARTING_POSITION;

		assert_eq!(fen.piece_placement, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
		assert_eq!(fen.active_colour, "w");
		assert_eq!(fen.castling_rights, "KQkq");
		assert_eq!(fen.en_passant_targets, "-");
		assert_eq!(fen.halfmove_clock, "0");
		assert_eq!(fen.fullmove_number, "1");
	}

	#[test]
	fn test_empty_board() {
		let fen = Fen::EMPTY;

		assert_eq!(fen.piece_placement, "8/8/8/8/8/8/8/8");
		assert_eq!(fen.active_colour, "w");
		assert_eq!(fen.castling_rights, "-");
		assert_eq!(fen.en_passant_targets, "-");
		assert_eq!(fen.halfmove_clock, "0");
		assert_eq!(fen.fullmove_number, "1");
	}

	#[test]
	fn test_custom_fen() {
		let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 1 2";
		let fen = Fen::new(fen_str).unwrap();

		assert_eq!(fen.piece_placement, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
		assert_eq!(fen.active_colour, "b");
		assert_eq!(fen.castling_rights, "KQkq");
		assert_eq!(fen.en_passant_targets, "-");
		assert_eq!(fen.halfmove_clock, "1");
		assert_eq!(fen.fullmove_number, "2");
	}

	#[test]
	fn test_invalid_fen_format() {
		let fen_str = "invalid_fen_string";
		let fen = Fen::new(fen_str);

		assert!(fen.is_err());
	}
}
