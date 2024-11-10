use std::{fmt, ops::{Rem, BitOr, BitAnd, BitXor}};
use colored::Colorize;

use crate::{impl_bitboard_ops, impl_bitboard_shift_ops};

#[derive(Copy, Clone, Default, Hash, Debug)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl From<u64> for Bitboard {
	fn from(bits: u64) -> Self {
		Bitboard(bits)
	}
}

impl From<Bitboard> for u64 {
	fn from(bitboard: Bitboard) -> Self {
		bitboard.0
	}
}

impl_bitboard_ops!(Add, AddAssign, add, add_assign, wrapping_add);
impl_bitboard_ops!(Sub, SubAssign, sub, sub_assign, wrapping_sub);
impl_bitboard_ops!(Mul, MulAssign, mul, mul_assign, wrapping_mul);
impl_bitboard_ops!(Div, DivAssign, div, div_assign, wrapping_div);

impl_bitboard_ops!(Rem, RemAssign, rem, rem_assign, rem);

impl_bitboard_ops!(BitOr, BitOrAssign, bitor, bitor_assign, bitor);
impl_bitboard_ops!(BitAnd, BitAndAssign, bitand, bitand_assign, bitand);
impl_bitboard_ops!(BitXor, BitXorAssign, bitxor, bitxor_assign, bitxor);

impl_bitboard_shift_ops!(Shl, ShlAssign, shl, shl_assign, wrapping_shl);
impl_bitboard_shift_ops!(Shr, ShrAssign, shr, shr_assign, wrapping_shr);

impl std::ops::Not for Bitboard {
	type Output = Self;

	fn not(self) -> Self {
		Self(!self.0)
	}
}

impl PartialEq<u64> for Bitboard {
	fn eq(&self, other: &u64) -> bool {
		self.0.eq(other)
	}

	fn ne(&self, other: &u64) -> bool {
		self.0.ne(other)
	}
}

impl PartialEq<Bitboard> for Bitboard {
	fn eq(&self, other: &Bitboard) -> bool {
		self.0.eq(&other.0)
	}

	fn ne(&self, other: &Bitboard) -> bool {
		self.0.ne(&other.0)
	}
}

impl PartialOrd<u64> for Bitboard {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(other)
	}
}

impl PartialOrd<Bitboard> for Bitboard {
	fn partial_cmp(&self, other: &Bitboard) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}

impl fmt::Display for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		const LAST_BIT: u64 = 63;
		
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty_bitboard_eq_0() {
		todo!()
	}

	#[test]
	fn bitboard_add() {
		todo!()
	}

	#[test]
	fn bitboard_add_wraps() {
		todo!()
	}

	#[test]
	fn bitboard_subtract() {
		todo!()
	}

	#[test]
	fn bitboard_subtract_wraps() {
		todo!()
	}

	#[test]
	fn bitboard_multiply() {
		todo!()
	}

	#[test]
	fn bitboard_multiply_wraps() {
		todo!()
	}

	#[test]
	fn bitboard_divide() {
		todo!()
	}

	#[test]
	fn bitboard_divide_wraps() {
		todo!()
	}

	#[test]
	fn bitboard_remainder() {
		todo!()
	}

	#[test]
	fn bitboard_shl() {
		todo!()
	}

	#[test]
	fn bitboard_shl_wraps() {
		todo!()
	}

	#[test]
	fn bitboard_shr() {
		todo!()
	}

	#[test]
	fn bitboard_shr_wraps() {
		todo!()
	}
}