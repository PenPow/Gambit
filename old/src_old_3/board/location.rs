use std::ops::RangeInclusive;
use crate::{dbg_assert_file_in_range, dbg_assert_rank_in_range, dbg_assert_side_in_range, dbg_assert_square_in_range};
use super::{bitboard::Bitboard, piece::{Side, Sides}};

pub type Direction = i8;
pub struct Directions;
impl Directions {
	pub const NORTH: Direction = 8;
	pub const NORTH_EAST: Direction = 9;
	pub const EAST: Direction = 1;
	pub const SOUTH_EAST: Direction = -7;
	pub const SOUTH: Direction = -8;
	pub const SOUTH_WEST: Direction = -9;
	pub const WEST: Direction = -1;
	pub const NORTH_WEST: Direction = 7;
	
	pub const NO_MOVEMENT: Direction = 0;

	pub const COUNT: usize = 8;

	pub const ALL: [Direction; Self::COUNT] = [
		Directions::NORTH,
		Directions::NORTH_EAST,
		Directions::EAST,
		Directions::SOUTH_EAST,
		Directions::SOUTH,
		Directions::SOUTH_WEST,
		Directions::WEST,
		Directions::NORTH_WEST,
	];

	pub const PAWN_CAPTURE_DIRECTIONS: [[Direction; 2]; Sides::COUNT] = [
		[
			Directions::NORTH_EAST,
			Directions::NORTH_WEST,
		],
		[
			Directions::SOUTH_EAST,
			Directions::SOUTH_WEST,
		]
	];
}

// TODO: Add tests
pub struct KnightJumps;
impl KnightJumps {
	pub const LONG_NORTH_WEST: Direction = 15;
	pub const SHORT_NORTH_WEST: Direction = 6;
	pub const LONG_NORTH_EAST: Direction = 17;
	pub const SHORT_NORTH_EAST: Direction = 10;
	pub const LONG_SOUTH_WEST: Direction = -17;
	pub const SHORT_SOUTH_WEST: Direction = -10;
	pub const LONG_SOUTH_EAST: Direction = -15;
	pub const SHORT_SOUTH_EAST: Direction = -6;
	
	pub const NO_MOVEMENT: Direction = 0;

	pub const COUNT: usize = 8;

	pub const ALL: [Direction; Self::COUNT] = [
		KnightJumps::LONG_NORTH_WEST,
		KnightJumps::SHORT_NORTH_WEST,
		KnightJumps::LONG_NORTH_EAST,
		KnightJumps::SHORT_NORTH_EAST,
		KnightJumps::LONG_SOUTH_WEST,
		KnightJumps::SHORT_SOUTH_WEST,
		KnightJumps::LONG_SOUTH_EAST,
		KnightJumps::SHORT_SOUTH_EAST,
	];
}

pub type File = usize;
pub struct Files;
impl Files {
	pub const A: File = 0;
	pub const B: File = 1;
	pub const C: File = 2;
	pub const D: File = 3;
	pub const E: File = 4;
	pub const F: File = 5;
	pub const G: File = 6;
	pub const H: File = 7;

	pub const COUNT: usize = 8;

	pub const ALL_FILES: RangeInclusive<usize> = Files::A..=Files::H;

	pub const fn as_str(file: File) -> &'static str {
		dbg_assert_file_in_range!(file);

		match file {
			Files::A => "A",
			Files::B => "B",
			Files::C => "C",
			Files::D => "D",
			Files::E => "E",
			Files::F => "F",
			Files::G => "G",
			Files::H => "H",
			_ => unreachable!()
		}
	}

	// TODO: Add tests
	pub const fn distance(lhs: File, rhs: File) -> usize {
		lhs.abs_diff(rhs)
	}
}

pub const FILE_BITBOARDS: [Bitboard; Files::COUNT] = {
	let mut files = [Bitboard::EMPTY; Files::COUNT];

	let mut file: File = 0;

	while file < Files::COUNT { // for is not stable in const functions yet
		files[file] = Bitboard::from_file(file);
		file += 1;
	}

	files
};

pub type Rank = usize;
pub struct Ranks;
impl Ranks {
	pub const R1: Rank = 0;
	pub const R2: Rank = 1;
	pub const R3: Rank = 2;
	pub const R4: Rank = 3;
	pub const R5: Rank = 4;
	pub const R6: Rank = 5;
	pub const R7: Rank = 6;
	pub const R8: Rank = 7;

	pub const COUNT: usize = 8;

	pub const ALL_RANKS: RangeInclusive<usize> = Ranks::R1..=Ranks::R8;

	// TODO: Add tests
	pub fn square_is_on_rank(square: Square, rank: Rank) -> bool {
		dbg_assert_square_in_range!(square);
		dbg_assert_rank_in_range!(rank);

		let start = rank * 8;
		let end = start + 7;

		(start..=end).contains(&square)
	}

	pub const fn get_fourth_rank(side: Side) -> Rank {
		dbg_assert_side_in_range!(side);

		if side == Sides::WHITE {
			Ranks::R4
		} else {
			Ranks::R5
		}
	}

	pub const fn get_promotion_rank(side: Side) -> Rank {
		dbg_assert_side_in_range!(side);

		if side == Sides::WHITE {
			Ranks::R8
		} else {
			Ranks::R1
		}
	}

	pub const fn as_str(rank: Rank) -> &'static str {
		dbg_assert_rank_in_range!(rank);

		match rank {
			Ranks::R1 => "1",
			Ranks::R2 => "2",
			Ranks::R3 => "3",
			Ranks::R4 => "4",
			Ranks::R5 => "5",
			Ranks::R6 => "6",
			Ranks::R7 => "7",
			Ranks::R8 => "8",
			_ => unreachable!()
		}
	}

	// TODO: Add tests
	pub const fn distance(lhs: Rank, rhs: Rank) -> usize {
		lhs.abs_diff(rhs)
	}
}

pub const RANK_BITBOARDS: [Bitboard; Ranks::COUNT] = {
	let mut ranks = [Bitboard::EMPTY; Ranks::COUNT];

	let mut rank: Rank = 0;

	while rank < Ranks::COUNT { // for is not stable in const functions yet
		ranks[rank] = Bitboard::from_rank(rank);
		rank += 1;
	}

	ranks
};

pub type Square = usize;
pub struct Squares;
impl Squares {
	pub const A1: Square = 0;
	pub const B1: Square = 1;
	pub const C1: Square = 2;
	pub const D1: Square = 3;
	pub const E1: Square = 4;
	pub const F1: Square = 5;
	pub const G1: Square = 6;
	pub const H1: Square = 7;

	pub const A2: Square = 8;
	pub const B2: Square = 9;
	pub const C2: Square = 10;
	pub const D2: Square = 11;
	pub const E2: Square = 12;
	pub const F2: Square = 13;
	pub const G2: Square = 14;
	pub const H2: Square = 15;

	pub const A3: Square = 16;
	pub const B3: Square = 17;
	pub const C3: Square = 18;
	pub const D3: Square = 19;
	pub const E3: Square = 20;
	pub const F3: Square = 21;
	pub const G3: Square = 22;
	pub const H3: Square = 23;

	pub const A4: Square = 24;
	pub const B4: Square = 25;
	pub const C4: Square = 26;
	pub const D4: Square = 27;
	pub const E4: Square = 28;
	pub const F4: Square = 29;
	pub const G4: Square = 30;
	pub const H4: Square = 31;

	pub const A5: Square = 32;
	pub const B5: Square = 33;
	pub const C5: Square = 34;
	pub const D5: Square = 35;
	pub const E5: Square = 36;
	pub const F5: Square = 37;
	pub const G5: Square = 38;
	pub const H5: Square = 39;

	pub const A6: Square = 40;
	pub const B6: Square = 41;
	pub const C6: Square = 42;
	pub const D6: Square = 43;
	pub const E6: Square = 44;
	pub const F6: Square = 45;
	pub const G6: Square = 46;
	pub const H6: Square = 47;

	pub const A7: Square = 48;
	pub const B7: Square = 49;
	pub const C7: Square = 50;
	pub const D7: Square = 51;
	pub const E7: Square = 52;
	pub const F7: Square = 53;
	pub const G7: Square = 54;
	pub const H7: Square = 55;

	pub const A8: Square = 56;
	pub const B8: Square = 57;
	pub const C8: Square = 58;
	pub const D8: Square = 59;
	pub const E8: Square = 60;
	pub const F8: Square = 61;
	pub const G8: Square = 62;
	pub const H8: Square = 63;

	pub const ALL: RangeInclusive<Square> = Squares::A1..=Squares::H8;
	pub const COUNT: usize = 64;

	// TODO: Add tests
	pub const fn get_rank(square: Square) -> Rank {
		dbg_assert_square_in_range!(square);

		(square / 8) as Rank
	}

	// TODO: Add tests
	pub const fn get_file(square: Square) -> File {
		dbg_assert_square_in_range!(square);

		(square % 8) as File
	}

	// TODO: Add tests
	pub const fn get_coordinates(square: Square) -> Location {
		dbg_assert_square_in_range!(square);

		(Squares::get_rank(square), Squares::get_file(square))
	}

	// TODO: Add tests to ensure translating by direction works
	pub const fn translate(square: Square, direction: Direction) -> Square {
		dbg_assert_square_in_range!(square);

		let new_square = ((square as i8) + (direction as i8)) as Square;
		dbg_assert_square_in_range!(new_square);

		new_square
	}

	// TODO: Add tests
	pub const fn distance(lhs: Square, rhs: Square) -> usize {
		let file_distance = Files::distance(Squares::get_file(lhs), Squares::get_file(rhs));
		let rank_distance = Ranks::distance(Squares::get_rank(lhs), Squares::get_rank(rhs));
		
		if file_distance > rank_distance { file_distance } else { rank_distance }
	}

	pub fn from_algebraic_notation(notation: &str) -> Square {
		assert!(notation.len() == 2);

		match notation {
			"a1" => Squares::A1,
			"b1" => Squares::B1,
			"c1" => Squares::C1,
			"d1" => Squares::D1,
			"e1" => Squares::E1,
			"f1" => Squares::F1,
			"g1" => Squares::G1,
			"h1" => Squares::H1,

			"a2" => Squares::A2,
			"b2" => Squares::B2,
			"c2" => Squares::C2,
			"d2" => Squares::D2,
			"e2" => Squares::E2,
			"f2" => Squares::F2,
			"g2" => Squares::G2,
			"h2" => Squares::H2,

			"a3" => Squares::A3,
			"b3" => Squares::B3,
			"c3" => Squares::C3,
			"d3" => Squares::D3,
			"e3" => Squares::E3,
			"f3" => Squares::F3,
			"g3" => Squares::G3,
			"h3" => Squares::H3,

			"a4" => Squares::A4,
			"b4" => Squares::B4,
			"c4" => Squares::C4,
			"d4" => Squares::D4,
			"e4" => Squares::E4,
			"f4" => Squares::F4,
			"g4" => Squares::G4,
			"h4" => Squares::H4,

			"a5" => Squares::A5,
			"b5" => Squares::B5,
			"c5" => Squares::C5,
			"d5" => Squares::D5,
			"e5" => Squares::E5,
			"f5" => Squares::F5,
			"g5" => Squares::G5,
			"h5" => Squares::H5,

			"a6" => Squares::A6,
			"b6" => Squares::B6,
			"c6" => Squares::C6,
			"d6" => Squares::D6,
			"e6" => Squares::E6,
			"f6" => Squares::F6,
			"g6" => Squares::G6,
			"h6" => Squares::H6,

			"a7" => Squares::A7,
			"b7" => Squares::B7,
			"c7" => Squares::C7,
			"d7" => Squares::D7,
			"e7" => Squares::E7,
			"f7" => Squares::F7,
			"g7" => Squares::G7,
			"h7" => Squares::H7,

			"a8" => Squares::A8,
			"b8" => Squares::B8,
			"c8" => Squares::C8,
			"d8" => Squares::D8,
			"e8" => Squares::E8,
			"f8" => Squares::F8,
			"g8" => Squares::G8,
			"h8" => Squares::H8,
			
			_ => panic!("Invalid square")
		}
	}

	pub fn to_algebraic_notation(square: Square) -> &'static str {
		dbg_assert_square_in_range!(square);
	
		match square {
			Squares::A1 => "a1",
			Squares::B1 => "b1",
			Squares::C1 => "c1",
			Squares::D1 => "d1",
			Squares::E1 => "e1",
			Squares::F1 => "f1",
			Squares::G1 => "g1",
			Squares::H1 => "h1",
			
			Squares::A2 => "a2",
			Squares::B2 => "b2",
			Squares::C2 => "c2",
			Squares::D2 => "d2",
			Squares::E2 => "e2",
			Squares::F2 => "f2",
			Squares::G2 => "g2",
			Squares::H2 => "h2",
			
			Squares::A3 => "a3",
			Squares::B3 => "b3",
			Squares::C3 => "c3",
			Squares::D3 => "d3",
			Squares::E3 => "e3",
			Squares::F3 => "f3",
			Squares::G3 => "g3",
			Squares::H3 => "h3",
			
			Squares::A4 => "a4",
			Squares::B4 => "b4",
			Squares::C4 => "c4",
			Squares::D4 => "d4",
			Squares::E4 => "e4",
			Squares::F4 => "f4",
			Squares::G4 => "g4",
			Squares::H4 => "h4",
			
			Squares::A5 => "a5",
			Squares::B5 => "b5",
			Squares::C5 => "c5",
			Squares::D5 => "d5",
			Squares::E5 => "e5",
			Squares::F5 => "f5",
			Squares::G5 => "g5",
			Squares::H5 => "h5",
			
			Squares::A6 => "a6",
			Squares::B6 => "b6",
			Squares::C6 => "c6",
			Squares::D6 => "d6",
			Squares::E6 => "e6",
			Squares::F6 => "f6",
			Squares::G6 => "g6",
			Squares::H6 => "h6",
			
			Squares::A7 => "a7",
			Squares::B7 => "b7",
			Squares::C7 => "c7",
			Squares::D7 => "d7",
			Squares::E7 => "e7",
			Squares::F7 => "f7",
			Squares::G7 => "g7",
			Squares::H7 => "h7",
			
			Squares::A8 => "a8",
			Squares::B8 => "b8",
			Squares::C8 => "c8",
			Squares::D8 => "d8",
			Squares::E8 => "e8",
			Squares::F8 => "f8",
			Squares::G8 => "g8",
			Squares::H8 => "h8",
			
			_ => panic!("Invalid square"),
		}
	}

	pub fn as_str(square: Square) -> String {
		dbg_assert_square_in_range!(square);
	
		let (rank, file) = Squares::get_coordinates(square);
	
		format!("{}{}", Files::as_str(file), Ranks::as_str(rank))
	}
}

pub const SQUARE_BITBOARDS: [Bitboard; Squares::COUNT] = {
	let mut squares = [Bitboard::EMPTY; Squares::COUNT];

	let mut square: Square = 0;

	while square < Squares::COUNT { // for is not stable in const functions yet
		squares[square] = Bitboard::from_square(square);
		square += 1;
	}

	squares
};

pub type Location = (Rank, File);

// TODO: Add more tests
#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn valid_square_translation_offsets() {
		let square = Squares::D4;

		assert_eq!(Squares::translate(square, Directions::NORTH), Squares::D5);
		assert_eq!(Squares::translate(square, Directions::NORTH_EAST), Squares::E5);
		assert_eq!(Squares::translate(square, Directions::EAST), Squares::E4);
		assert_eq!(Squares::translate(square, Directions::SOUTH_EAST), Squares::E3);
		assert_eq!(Squares::translate(square, Directions::SOUTH), Squares::D3);
		assert_eq!(Squares::translate(square, Directions::SOUTH_WEST), Squares::C3);
		assert_eq!(Squares::translate(square, Directions::WEST), Squares::C4);
		assert_eq!(Squares::translate(square, Directions::NORTH_WEST), Squares::C5);
		assert_eq!(Squares::translate(square, Directions::NO_MOVEMENT), Squares::D4);
	}

	#[test]
	#[should_panic]
	fn invalid_translation_panics_a1_west() {
		let square = Squares::A1;

		Squares::translate(square, Directions::WEST);
	}
	
	#[test]
	fn valid_knight_jump_translation_offsets() {
		let square = Squares::D4;

		assert_eq!(Squares::translate(square, KnightJumps::LONG_NORTH_WEST), Squares::C6);
		assert_eq!(Squares::translate(square, KnightJumps::SHORT_NORTH_WEST), Squares::B5);
		assert_eq!(Squares::translate(square, KnightJumps::LONG_NORTH_EAST), Squares::E6);
		assert_eq!(Squares::translate(square, KnightJumps::SHORT_NORTH_EAST), Squares::F5);
		assert_eq!(Squares::translate(square, KnightJumps::LONG_SOUTH_WEST), Squares::C2);
		assert_eq!(Squares::translate(square, KnightJumps::SHORT_SOUTH_WEST), Squares::B3);
		assert_eq!(Squares::translate(square, KnightJumps::LONG_SOUTH_EAST), Squares::E2);
		assert_eq!(Squares::translate(square, KnightJumps::SHORT_SOUTH_EAST), Squares::F3);
		assert_eq!(Squares::translate(square, KnightJumps::NO_MOVEMENT), Squares::D4);
	}
}