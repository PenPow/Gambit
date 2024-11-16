use arrayvec::ArrayVec;
use crate::{bitboard::Bitboard, location::Square, piece::{Castling, Colour, PieceType}};
use super::fen::{Fen, FenError, FenParser};

/// A struct containing the current game state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
	/// The active [`Colour`]
	pub active_colour: Colour,

	/// The number of halfmoves
	pub halfmove_clock: u8,

	/// The number of fullmoves
	pub fullmove_number: u8,

	/// The possible en-passant square, if any
	pub en_passant_square: Option<Square>,

	/// The ability to castle, stored in the bits of [`Castling`]
	pub castling_availability: Castling,
}

/// Represents the game board
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
	/// The current [`State`] of the board
	pub state: State,

	/// An [`ArrayVec`] storing the previous [`State`] of the board for use in make-unmake
	pub history: ArrayVec<State, 5949>,

	/// An 2D-array containing a [`Bitboard`] for each [`PieceType`] for each [`Colour`]
	pub piece_bitboards: [[Bitboard; PieceType::COUNT]; Colour::COUNT],

	/// An array containing a [`Bitboard`] for each [`Colour`]
	pub side_bitboards: [Bitboard; Colour::COUNT],
}

impl Board {
	/// Creates a new [`Board`] instance from the starting position.
	#[inline]
	#[must_use]
	pub fn from_start_pos() -> Self {
		unsafe { Board::from_fen(Fen::STARTING_POSITION).unwrap_unchecked() }
	}

	/// Creates a new [`Board`] instance from the given [`Fen`].
	///
	/// # Errors
	///
	/// This function will return a [`FenError`] if the provided [`Fen`] is invalid.
	///
	/// # Examples
	///
	/// ```rust
	/// use gambit::board::{Board, fen::Fen};
	///
	/// let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
	/// let board = Board::from_fen(fen).unwrap();
	/// ```
	pub fn from_fen(fen: Fen) -> Result<Self, FenError> {
		let parser = FenParser::new(fen);

		let mut board = Self {
			state: State {
				active_colour: parser.parse_active_colour()?,
				castling_availability: parser.parse_castling_rights()?,
				en_passant_square: parser.parse_en_passant_square()?,
				halfmove_clock: parser.parse_halfmove_clock()?,
				fullmove_number: parser.parse_fullmove_number()?,
			},
			history: ArrayVec::new(),
			
			piece_bitboards: [[Bitboard::EMPTY; PieceType::COUNT]; Colour::COUNT],
			side_bitboards: [Bitboard::EMPTY; Colour::COUNT],
		};

		parser.parse_piece_placement(&mut board.piece_bitboards, &mut board.side_bitboards)?;

		Ok(board)
	}

	/// Returns a [`Bitboard`] representing the occupancy of all pieces on the board.
	///
	/// # Examples
	///
	/// ```
	/// use gambit::board::Board;
	/// 
	/// let board = Board::from_start_pos();
	/// let occupancy = board.occupancy();
	/// ```
	#[inline]
	#[must_use]
	pub fn occupancy(&self) -> Bitboard {
		self.side_bitboards[Colour::White as usize] | self.side_bitboards[Colour::Black as usize]
	}
}

impl Default for Board {
	fn default() -> Self {
		Board::from_start_pos()
	}
}

#[cfg(test)]
mod tests {
	use crate::location::Rank;

	use super::*;

	#[test]
	fn test_from_start_pos() {
		let board = Board::from_start_pos();

		assert_eq!(board.state.active_colour, Colour::White);
		assert_eq!(board.state.halfmove_clock, 0);
		assert_eq!(board.state.fullmove_number, 1);
		assert_eq!(board.occupancy().bits().count_ones(), 32);
	}

	#[test]
	fn test_from_fen_starting_position() {
		let board = Board::from_start_pos();

		assert_eq!(board.state.active_colour, Colour::White);
		assert_eq!(board.state.halfmove_clock, 0);
		assert_eq!(board.state.fullmove_number, 1);
		assert_eq!(board.occupancy().bits().count_ones(), 32);
	}

	#[test]
	fn test_from_fen_custom_position() {
		let fen_str = "r1bqkbnr/pppppppp/n7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
		let fen = Fen::new(fen_str).unwrap();
		let board = Board::from_fen(fen).unwrap();

		assert_eq!(board.state.active_colour, Colour::White);
		assert!(board.piece_bitboards[Colour::Black as usize][PieceType::Knight as usize].contains(Square::A6));
		assert_eq!(board.occupancy().bits().count_ones(), 32);
	}

	#[test]
	fn test_occupancy() {
		let board = Board::from_start_pos();
		let occupancy = board.occupancy();

		for square in 0..64 {
			let square = Square::new(square);
			let expected = matches!(square.rank(), Rank::R1 | Rank::R2 | Rank::R7 | Rank::R8);

			assert_eq!(occupancy.contains(square), expected);
		}
	}

	#[test]
	fn test_default_impl() {
		let board_default = Board::default();
		let board_start_pos = Board::from_start_pos();

		assert_eq!(board_default, board_start_pos);
	}
}
