use std::{error::Error, fmt};

/// Represents errors that can occur when parsing a [FEN (Forsyth-Edwards Notation)](https://www.chessprogramming.org/Forsyth-Edwards_Notation) string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum FenError {
	/// The FEN string has an invalid format.
    InvalidFormat,
	/// The piece placement section of the FEN string is invalid.
    InvalidPiecePlacement,
	/// The active color section of the FEN string is invalid.
    InvalidActiveColor,
	/// The castling rights section of the FEN string is invalid.
    InvalidCastlingRights,
	/// The en passant square section of the FEN string is invalid.
    InvalidEnPassantSquare,
	/// The halfmove clock section of the FEN string is invalid.
    InvalidHalfmoveClock,
	/// The fullmove number section of the FEN string is invalid.
    InvalidFullmoveNumber,
}

impl Error for FenError {}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::InvalidFormat => write!(f, "Invalid FEN format"),
            FenError::InvalidPiecePlacement => write!(f, "Invalid piece placement in FEN"),
            FenError::InvalidActiveColor => write!(f, "Invalid active color in FEN"),
            FenError::InvalidCastlingRights => write!(f, "Invalid castling rights in FEN"),
            FenError::InvalidEnPassantSquare => write!(f, "Invalid en passant square in FEN"),
            FenError::InvalidHalfmoveClock => write!(f, "Invalid halfmove clock in FEN"),
            FenError::InvalidFullmoveNumber => write!(f, "Invalid fullmove number in FEN"),
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_invalid_format_display() {
		let error = FenError::InvalidFormat;

		assert_eq!(format!("{error}"), "Invalid FEN format");
	}

	#[test]
	fn test_invalid_piece_placement_display() {
		let error = FenError::InvalidPiecePlacement;

		assert_eq!(format!("{error}"), "Invalid piece placement in FEN");
	}

	#[test]
	fn test_invalid_active_color_display() {
		let error = FenError::InvalidActiveColor;

		assert_eq!(format!("{error}"), "Invalid active color in FEN");
	}

	#[test]
	fn test_invalid_castling_rights_display() {
		let error = FenError::InvalidCastlingRights;

		assert_eq!(format!("{error}"), "Invalid castling rights in FEN");
	}

	#[test]
	fn test_invalid_en_passant_square_display() {
		let error = FenError::InvalidEnPassantSquare;

		assert_eq!(format!("{error}"), "Invalid en passant square in FEN");
	}

	#[test]
	fn test_invalid_halfmove_clock_display() {
		let error = FenError::InvalidHalfmoveClock;

		assert_eq!(format!("{error}"), "Invalid halfmove clock in FEN");
	}

	#[test]
	fn test_invalid_fullmove_number_display() {
		let error = FenError::InvalidFullmoveNumber;
		
		assert_eq!(format!("{error}"), "Invalid fullmove number in FEN");
	}
}
