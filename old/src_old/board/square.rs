use std::ops::RangeInclusive;

use super::{bitboard::Bitboard, piece::{Side, Sides}};

pub type File = usize;
pub struct Files {}
impl Files {
	pub const A: File = 0;
	pub const B: File = 1;
	pub const C: File = 2;
	pub const D: File = 3;
	pub const E: File = 4;
	pub const F: File = 5;
	pub const G: File = 6;
	pub const H: File = 7;

	pub const ALL_FILES_RANGE: RangeInclusive<usize> = Files::A..=Files::H;
}

pub const FILE_BITBOARD_LOOKUP: [Bitboard; 8] = generate_files_bitboard();

const fn generate_files_bitboard() -> [Bitboard; 8] {
	let mut files = [Bitboard(0); 8];

	let mut i = 0usize;

	while i < 8usize { // for is not stable in const functions yet
		files[i] = Bitboard(0x0101_0101_0101_0101 << i);
		i += 1;
	}

	files
}

pub type Rank = usize;
pub struct Ranks {}
impl Ranks {
	pub const R1: Rank = 0;
	pub const R2: Rank = 1;
	pub const R3: Rank = 2;
	pub const R4: Rank = 3;
	pub const R5: Rank = 4;
	pub const R6: Rank = 5;
	pub const R7: Rank = 6;
	pub const R8: Rank = 7;

	pub const ALL_RANKS_RANGE: RangeInclusive<usize> = Ranks::R1..=Ranks::R8;

	pub fn fourth_rank(side: Side) -> Rank {
		if side == Sides::WHITE {
			Ranks::R4
		} else {
			Ranks::R5
		}
	}

	pub fn promotion_rank(side: Side) -> Rank {
		if side == Sides::WHITE {
			Ranks::R8
		} else {
			Ranks::R1
		}
	}
}
pub const RANK_BITBOARD_LOOKUP: [Bitboard; 8] = generate_rank_bitboard();

const fn generate_rank_bitboard() -> [Bitboard; 8] {
	let mut ranks = [Bitboard(0); 8];

	let mut i = 0usize;

	while i < 8usize { // for is not stable in const functions yet
		ranks[i] = Bitboard(0xFF << (i * 8));
		i += 1;
	}

	ranks
}

pub type Square = usize;
pub struct Squares {}
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

	pub const ALL_SQUARES_RANGE: RangeInclusive<usize> = Squares::A1..=Squares::H8;

	pub fn on_rank(square: Square, rank: Rank) -> bool {
		let start = rank * 8;
		let end = start + 7;

		(start..=end).contains(&square)
	}

	pub fn get_location(square: Square) -> (Rank, File) {
		let rank = (square / 8) as Rank;
		let file = (square % 8) as File;

		(rank, file)
	}
}

pub const SQUARE_BITBOARD_LOOKUP: [Bitboard; 64] = generate_squares_bitboard();

const fn generate_squares_bitboard() -> [Bitboard; 64] {
	let mut squares = [Bitboard(0); 64];

	let mut i = 0usize;

	while i < 64usize { // for is not stable in const functions yet
		squares[i] = Bitboard(1u64 << i);
		i += 1;
	}

	squares
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn squares_have_correct_values() {
		assert_eq!(Squares::A1, 0);
		assert_eq!(Squares::H1, 7);
		assert_eq!(Squares::A2, 8);
		assert_eq!(Squares::H8, 63);
	}

	#[test]
	fn can_parse_square_from_string() {
		todo!()
	}

	#[test]
	fn can_parse_square_into_number() {
		todo!()
	}

	#[test]
	fn should_panic_on_parsing_invalid_square() {
		todo!()
	}
}