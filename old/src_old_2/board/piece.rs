use crate::{dbg_assert_piece_in_range, dbg_assert_side_in_range};

use super::location::{Direction, Directions};

pub type Side = usize;
pub struct Sides;
impl Sides {
	pub const WHITE: Side = 0;
	pub const BLACK: Side = 1;

	pub const COUNT: usize = 2;

	pub const fn get_pawn_movement_direction(side: Side) -> Direction {
		dbg_assert_side_in_range!(side);

		if side == Sides::WHITE {
			Directions::NORTH
		} else {
			Directions::SOUTH
		}
	}

	pub const fn as_str(side: Side) -> &'static str {
		dbg_assert_side_in_range!(side);
		
		match side {
			Self::WHITE => "White",
			Self::BLACK => "Black",
			_ => unreachable!()
		}
	}

	pub const fn as_char(side: Side) -> char {
		dbg_assert_side_in_range!(side);
		
		match side {
			Self::WHITE => 'W',
			Self::BLACK => 'B',
			_ => unreachable!()
		}
	}
}

pub type Piece = usize;
pub struct Pieces;
impl Pieces {
	pub const PAWN: Piece = 0;
	pub const KNIGHT: Piece = 1;
	pub const BISHOP: Piece = 2;
	pub const ROOK: Piece = 3;
	pub const QUEEN: Piece = 4;
	pub const KING: Piece = 5;
	pub const NONE: Piece = 6; // Set to 6 so the others can be used to index arrays

	pub const COUNT: usize = 6;
	pub const PROMOTION_OPTION_COUNT: usize = 4;

	pub const ALL: [Piece; Self::COUNT] = [
		Pieces::PAWN,
		Pieces::KNIGHT,
		Pieces::BISHOP,
		Pieces::ROOK,
		Pieces::QUEEN,
		Pieces::KING,
	];

	pub const PROMOTION_TARGETS: [Piece; Self::PROMOTION_OPTION_COUNT] = [
		Pieces::KNIGHT,
		Pieces::BISHOP,
		Pieces::ROOK,
		Pieces::QUEEN,
	];

	pub const fn as_str(piece: Piece) -> &'static str {
		dbg_assert_piece_in_range!(piece);
		
		match piece {
			Self::PAWN => "Pawn",
			Self::KNIGHT => "Knight",
			Self::BISHOP => "Bishop",
			Self::ROOK => "Rook",
			Self::QUEEN => "Queen",
			Self::KING => "King",
			Self::NONE => "None",
			_ => unreachable!()
		}
	}

	pub const fn as_char(piece: Piece, side: Side) -> char {
		dbg_assert_piece_in_range!(piece);
		
		let mut char = match piece {
			Self::PAWN => 'P',
			Self::KNIGHT => 'N',
			Self::BISHOP => 'B',
			Self::ROOK => 'R',
			Self::QUEEN => 'Q',
			Self::KING => 'K',
			Self::NONE => '!',
			_ => unreachable!()
		};

		if side == Sides::BLACK {
			char = char.to_ascii_lowercase();
		}

		char
	}
}