use std::{fmt, ops::{BitAnd, BitOr, BitXor, Rem}};
use colored::Colorize;
use crate::{dbg_assert_file_in_range, dbg_assert_rank_in_range, dbg_assert_square_in_range, impl_arithmetic_ops, impl_shift_ops, impl_output_types, impl_ops};
use super::location::{File, Rank, Square};

#[derive(Copy, Clone, Hash)]
pub struct Bitboard(pub u64);
impl Bitboard {
	pub const EMPTY: Bitboard = Bitboard(0);
	pub const UNIVERSE: Bitboard = Bitboard(u64::MAX);

	pub fn is_subset_of(&self, b: Bitboard) -> bool {
		let a = *self;

		(a & b) == a
	}

	pub const fn from_square(square: Square) -> Self {
		dbg_assert_square_in_range!(square);

		Bitboard(1 << square)
	}

	pub const fn from_file(file: File) -> Self {
		dbg_assert_file_in_range!(file);

		const FILE_A: u64 = 0x0101_0101_0101_0101;
		Bitboard(FILE_A << file)
	}

	pub const fn from_rank(rank: Rank) -> Self {
		dbg_assert_rank_in_range!(rank);

		Bitboard(0xFF << (rank * 8))
	}

	fn as_str(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		const LAST_BIT: u64 = 63;

		writeln!(f)?;
		
		for rank in 0..8 {
			for file in (0..8).rev() {
				let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
				let char = if self.0 & mask != 0 { "1".green() } else { "0".red() };
				write!(f, "{char} ")?;
			}

			writeln!(f)?;
		}

		Ok(())
	}
}

impl_ops!(Bitboard, u64);

impl Default for Bitboard {
	fn default() -> Self {
		Bitboard::EMPTY
	}
}

impl fmt::Display for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

impl fmt::Debug for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

#[cfg(test)]
mod tests {
	use crate::board::location::{Files, Ranks, Squares};
	use super::*;

	#[test]
	fn add_bitboard_to_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let expected = Bitboard(0b1100);

		assert_eq!(a + b, expected);
	}

	#[test]
	fn add_bitboard_to_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let expected = Bitboard(0b1100);

		assert_eq!(a + b, expected);
	}

	#[test]
	fn sub_bitboard_from_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let expected = Bitboard(0b1000);

		assert_eq!(a - b, expected);
	}

	#[test]
	fn sub_u64_from_bitboard() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let expected = Bitboard(0b1000);
		
		assert_eq!(a - b, expected);
	}

	#[test]
	fn mul_bitboard_by_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let expected = Bitboard(0b10100);

		assert_eq!(a * b, expected);
	}

	#[test]
	fn mul_bitboard_by_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let expected = Bitboard(0b10100);
		
		assert_eq!(a * b, expected);
	}

	#[test]
	fn div_bitboard_by_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let expected = Bitboard(0b0101);

		assert_eq!(a / b, expected);
	}

	#[test]
	fn div_bitboard_by_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let expected = Bitboard(0b0101);
		
		assert_eq!(a / b, expected);
	}

	#[test]
	fn rem_bitboard_by_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0011);
		let expected = Bitboard(0b1);

		assert_eq!(a % b, expected);
	}

	#[test]
	fn rem_bitboard_by_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0011;
		let expected = Bitboard(0b0001);

		assert_eq!(a % b, expected);
	}

	#[test]
	fn bitor_with_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0110);
		let expected = Bitboard(0b1110);

		assert_eq!(a | b, expected);
	}

	#[test]
	fn bitor_with_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0110;
		let expected = Bitboard(0b1110);

		assert_eq!(a | b, expected);
	}

	#[test]
	fn bitand_with_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let expected = Bitboard(0b0010);

		assert_eq!(a & b, expected);
	}

	#[test]
	fn bitand_with_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let expected = Bitboard(0b0010);

		assert_eq!(a & b, expected);
	}

	#[test]
	fn bitxor_with_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let expected = Bitboard(0b1000);

		assert_eq!(a ^ b, expected);
	}

	#[test]
	fn bitxor_with_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let expected = Bitboard(0b1000);

		assert_eq!(a ^ b, expected);
	}

	#[test]
	fn shl() {
		let a = Bitboard(0b1010);
		let shift = 2;
		let expected = Bitboard(0b101000);

		assert_eq!(a << shift, expected);
	}

	#[test]
	fn shr() {
		let a = Bitboard(0b1010);
		let shift = 2;
		let expected = Bitboard(0b10);

		assert_eq!(a >> shift, expected);
	}

	#[test]
	fn not() {
		let a = Bitboard(0b1010);
		let expected = Bitboard(0b1111111111111111111111111111111111111111111111111111111111110101);

		assert_eq!(!a, expected);
	}

	#[test]
	fn eq_to_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b1010);

		assert_eq!(a, b);
	}

	#[test]
	fn eq_to_u64() {
		let a = Bitboard(0b1010);
		let b = 0b1010;

		assert_eq!(a, b);
	}

	#[test]
	fn ne_to_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);

		assert_ne!(a, b);
	}

	#[test]
	fn ne_to_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;

		assert_ne!(a, b);
	}
	
	#[test]
	fn partial_ord_bitboard_bitboard() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let c = Bitboard(0b1101);

		assert_eq!(a.partial_cmp(&a), Some(std::cmp::Ordering::Equal));
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Greater));
		assert_eq!(a.partial_cmp(&c), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn partial_ord_bitboard_u64() {
		let a = Bitboard(0b1010);
		let b = 0b0010;
		let c = 0b1101;

		assert_eq!(a.partial_cmp(&a), Some(std::cmp::Ordering::Equal));
		assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Greater));
		assert_eq!(a.partial_cmp(&c), Some(std::cmp::Ordering::Less));
	}

	#[test]
	fn ord() {
		let a = Bitboard(0b1010);
		let b = Bitboard(0b0010);
		let c = Bitboard(0b1101);

		assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
		assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
		assert_eq!(a.cmp(&c), std::cmp::Ordering::Less);
	}

	#[test]
	fn from_square() {
		assert_eq!(Bitboard::from_square(Squares::A1), Bitboard(1));
		assert_eq!(Bitboard::from_square(Squares::H8), Bitboard(0x8000000000000000));
	}

	#[test]
	fn from_file() {
		assert_eq!(Bitboard::from_file(Files::A), Bitboard(0x0101_0101_0101_0101));
		assert_eq!(Bitboard::from_file(Files::H), Bitboard(0x8080_8080_8080_8080));
	}

	#[test]
	fn from_rank() {
		assert_eq!(Bitboard::from_rank(Ranks::R1), Bitboard(0xFF));
		assert_eq!(Bitboard::from_rank(Ranks::R8), Bitboard(0xFF_00_00_00_00_00_00_00));
	}

	#[test]
	fn is_subset_of() {
		todo!()
	}
}
