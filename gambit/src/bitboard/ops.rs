use crate::location::Square;
use super::core::{Bitboard, CarryRippler};

impl Bitboard {
	/// Performs setwise [intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory))
	/// 
	/// Identical to bitwise and, except usable in const contexts
	#[doc(alias = "and")]
	#[inline(always)]
	#[must_use]
	pub const fn intersection(&self, b: Bitboard) -> Bitboard {
		Bitboard::new(self.bits() & b.bits())
	}

	/// Performs [conjunction](https://en.wikipedia.org/wiki/Logical_conjunction)
	/// 
	/// Identical to bitwise and, except usable in const contexts
	#[doc(alias = "and")]
	#[inline(always)]
	#[must_use]
	pub const fn conjunction(&self, b: Bitboard) -> Bitboard {
		self.intersection(b)
	}

	/// Performs setwise [union](https://en.wikipedia.org/wiki/Union_(set_theory))
	/// 
	/// Identical to bitwise or, except usable in const contexts
	#[doc(alias = "or")]
	#[inline(always)]
	#[must_use]
	pub const fn union(&self, b: Bitboard) -> Bitboard {
		Bitboard::new(self.bits() | b.bits())
	}

	/// Performs [disjunction](https://en.wikipedia.org/wiki/Logical_disjunction)
	/// 
	/// Identical to bitwise or, except usable in const contexts
	#[doc(alias = "or")]
	#[inline(always)]
	#[must_use]
	pub const fn disjunction(&self, b: Bitboard) -> Bitboard {
		self.union(b)
	}

	/// Returns the [complement set](https://en.wikipedia.org/wiki/Complement_(set_theory))
	/// 
	/// Identical to bitwise not, except usable in const contexts
	#[doc(alias = "not")]
	#[inline(always)]
	#[must_use]
	pub const fn complement(&self) -> Bitboard {
		Bitboard::new(!self.bits())
	}

	/// Performs [negation](https://en.wikipedia.org/wiki/Negation)
	/// 
	/// Identical to bitwise not, except usable in const contexts
	#[doc(alias = "not")]
	#[inline(always)]
	#[must_use]
	pub const fn negation(&self) -> Bitboard {
		self.complement()
	}

	/// Returns the [relative complement set](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement)
	#[doc(alias = "difference")]
	#[inline(always)]
	#[must_use]
	pub const fn relative_complement(&self, b: Bitboard) -> Bitboard {
		Bitboard::new(self.bits() & !b.bits())
	}

	/// Returns the difference between `self` and `b`
	#[inline(always)]
	#[must_use]
	pub const fn difference(&self, b: Bitboard) -> Bitboard {
		self.relative_complement(b)
	}

	/// Returns the [relative complement set](https://en.wikipedia.org/wiki/Symmetric_difference)
	/// 
	/// Identical to bitwise xor, except usable in const contexts
	#[doc(alias = "difference")]
	#[inline(always)]
	#[must_use]
	pub const fn symmetric_difference(&self, b: Bitboard) -> Bitboard {
		Bitboard::new(self.bits() ^ b.bits())
	}

	/// Performs [negation](https://en.wikipedia.org/wiki/Exclusive_or)
	/// 
	/// Identical to bitwise xor, except usable in const contexts
	#[inline(always)]
	#[must_use]
	pub const fn xor(&self, b: Bitboard) -> Bitboard {
		self.relative_complement(b)
	}
	
	/// Returns true if any bits are set
	#[inline(always)]
	#[must_use]
	pub const fn any(&self) -> bool {
		self.bits() != Bitboard::EMPTY.bits()
	}
	
	/// Returns true if no bits are set
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.bits() == Bitboard::EMPTY.bits()
	}
	
	/// Returns a bool representing whether a bitboard is a subset of `self`
	#[inline(always)]
	#[must_use]
	pub const fn is_subset_of(&self, b: Bitboard) -> bool {
		let a = *self;
	
		(a.bits() & b.bits()) == a.bits()
	}

	/// Returns a bool representing whether a bitboard is a subset of `self`
	#[inline(always)]
	#[must_use]
	pub const fn is_superset_of(&self, b: Bitboard) -> bool {
		let a = *self;
	
		(a.bits() & b.bits()) == b.bits()
	}

	/// Returns a bool representing whether `self` and a bitboard is disjoint (intersection is empty)
	#[inline(always)]
	#[must_use]
	pub const fn is_disjoint(&self, b: Bitboard) -> bool {
		let a = *self;
	
		(a.bits() & b.bits()) == Bitboard::EMPTY.bits()
	}
		
	/// Returns true if `self` contains square
	#[inline(always)]
	#[must_use]
	pub const fn contains(&self, square: Square) -> bool {
		self.intersection(Bitboard::from_square(square)).any()
	}

	/// Adds a bitboard, rank, file or square to `self`
	#[inline(always)]
	pub fn add<T: Into<Bitboard>>(&mut self, bitboard: T) {
		*self |= bitboard;
	}

	/// Toggles a bitboard, rank, file or square on `self`
	#[inline(always)]
	pub fn toggle<T: Into<Bitboard>>(&mut self, bitboard: T) {
		*self ^= bitboard;
	}

	/// Removes a bitboard, rank, file or square from `self`
	#[inline(always)]
	pub fn discard<T: Into<Bitboard>>(&mut self, bitboard: T) {
		*self &= !(bitboard.into());
	}

	/// Set a square to a boolean
    #[inline]
    pub fn set(&mut self, square: Square, value: bool) {
        if value {
			self.add(square);
		} else {
			self.discard(square);
		}
    }

	/// Removes a square from a bitboard, returing a boolean of whether the square was in the bitboard
	#[must_use = "use Bitboard::discard() if no return value needed"]
    #[inline]
    pub fn remove(&mut self, square: Square) -> bool {
        if self.contains(square) {
            self.toggle(square);
            true
        } else {
            false
        }
    }

	/// Clears the bitboard
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

	/// Pops the leading 1 off the bitboard
	#[inline]
	pub fn pop(&mut self) -> Option<Square> {
		if self.is_empty() {
            None
        } else {
            let square = Square::new(self.bits().trailing_zeros() as u8);
			*self = Bitboard::new(self.bits() & self.bits().wrapping_sub(1));

			Some(square)
        }
	}

	/// Pops the trailing 1 off the bitboard
	#[inline]
	pub fn pop_back(&mut self) -> Option<Square> {
		if self.is_empty() {
            None
        } else {
            let square = Square::new(63 - self.bits().leading_zeros() as u8);
			*self ^= Square::BITBOARDS[square as usize];

			Some(square)
        }
	}

	/// Produces an iterator over all the subsets of this bitboard
	/// 
	/// See [traversing subsets of a set](https://www.chessprogramming.org/Traversing_Subsets_of_a_Set) for more details
	#[inline]
	#[must_use]
	pub const fn carry_rippler(&self) -> CarryRippler {
		CarryRippler {
            bitboard: self.bits(),
            subset: 0,
            first: true,
        }
	}
}

impl std::ops::Not for Bitboard {
	type Output = Self;

	fn not(self) -> Self {
		Bitboard::new(!self.bits())
	}
}

impl PartialEq<u64> for Bitboard {
	fn eq(&self, bits: &u64) -> bool {
		self.bits().eq(bits)
	}
}

impl PartialEq<Bitboard> for Bitboard {
	fn eq(&self, other: &Bitboard) -> bool {
		self.bits().eq(&other.bits())
	}
}

impl Eq for Bitboard {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		let a = Bitboard::new(0b0001);
		let b = Bitboard::new(0b0010);

		assert_eq!(a + b, Bitboard::new(0b0011));
	}

	#[test]
	fn test_add_assign() {
		let mut a = Bitboard::new(0b0001);
		a += Bitboard::new(0b0010);

		assert_eq!(a, Bitboard::new(0b0011));
	}

	#[test]
	fn test_sub() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0001);

		assert_eq!(a - b, Bitboard::new(0b0010));
	}

	#[test]
	fn test_sub_assign() {
		let mut a = Bitboard::new(0b0011);
		a -= Bitboard::new(0b0001);

		assert_eq!(a, Bitboard::new(0b0010));
	}

	#[test]
	fn test_mul() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0010);

		assert_eq!(a * b, Bitboard::new(0b0110));
	}

	#[test]
	fn test_mul_assign() {
		let mut a = Bitboard::new(0b0011);
		a *= Bitboard::new(0b0010);

		assert_eq!(a, Bitboard::new(0b0110));
	}

	#[test]
	fn test_div() {
		let a = Bitboard::new(0b0110);
		let b = Bitboard::new(0b0010);

		assert_eq!(a / b, Bitboard::new(0b0011));
	}

	#[test]
	fn test_div_assign() {
		let mut a = Bitboard::new(0b0110);
		a /= Bitboard::new(0b0010);

		assert_eq!(a, Bitboard::new(0b0011));
	}

	#[test]
	fn test_rem() {
		let a = Bitboard::new(0b0110);
		let b = Bitboard::new(0b0100);

		assert_eq!(a % b, Bitboard::new(0b0010));
	}

	#[test]
	fn test_rem_assign() {
		let mut a = Bitboard::new(0b0110);
		a %= Bitboard::new(0b0100);

		assert_eq!(a, Bitboard::new(0b0010));
	}

	#[test]
	fn test_bitor() {
		let a = Bitboard::new(0b0001);
		let b = Bitboard::new(0b0010);
		
		assert_eq!(a | b, Bitboard::new(0b0011));
	}

	#[test]
	fn test_bitor_assign() {
		let mut a = Bitboard::new(0b0001);
		a |= Bitboard::new(0b0010);

		assert_eq!(a, Bitboard::new(0b0011));
	}

	#[test]
	fn test_bitand() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0010);

		assert_eq!(a & b, Bitboard::new(0b0010));
	}

	#[test]
	fn test_bitand_assign() {
		let mut a = Bitboard::new(0b0011);
		a &= Bitboard::new(0b0010);

		assert_eq!(a, Bitboard::new(0b0010));
	}

	#[test]
	fn test_bitxor() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0010);

		assert_eq!(a ^ b, Bitboard::new(0b0001));
	}

	#[test]
	fn test_bitxor_assign() {
		let mut a = Bitboard::new(0b0011);
		a ^= Bitboard::new(0b0010);

		assert_eq!(a, Bitboard::new(0b0001));
	}

	#[test]
	fn test_shl() {
		let a = Bitboard::new(0b0001);

		assert_eq!(a << 1, Bitboard::new(0b0010));
	}

	#[test]
	fn test_shl_assign() {
		let mut a = Bitboard::new(0b0001);
		a <<= 1;

		assert_eq!(a, Bitboard::new(0b0010));
	}

	#[test]
	fn test_shr() {
		let a = Bitboard::new(0b0010);

		assert_eq!(a >> 1, Bitboard::new(0b0001));
	}

	#[test]
	fn test_shr_assign() {
		let mut a = Bitboard::new(0b0010);
		a >>= 1;

		assert_eq!(a, Bitboard::new(0b0001));
	}

	#[test]
	fn test_not() {
		let a = Bitboard::new(0b0001);

		assert_eq!(!a, Bitboard::new(!0b0001));
	}

	#[test]
	fn test_intersection() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0010);

		assert_eq!(a.intersection(b), Bitboard::new(0b0010));
	}

	#[test]
	fn test_union() {
		let a = Bitboard::new(0b0001);
		let b = Bitboard::new(0b0010);

		assert_eq!(a.union(b), Bitboard::new(0b0011));
	}

	#[test]
	fn test_complement() {
		let a = Bitboard::new(0b0001);

		assert_eq!(a.complement(), Bitboard::new(!0b0001));
	}

	#[test]
	fn test_relative_complement() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0010);

		assert_eq!(a.relative_complement(b), Bitboard::new(0b0001));
	}

	#[test]
	fn test_symmetric_difference() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0010);

		assert_eq!(a.symmetric_difference(b), Bitboard::new(0b0001));
	}

	#[test]
	fn test_any() {
		let a = Bitboard::new(0b0001);
		assert!(a.any());

		let b = Bitboard::new(0b0000);
		assert!(!b.any());
	}

	#[test]
	fn test_is_empty() {
		let a = Bitboard::new(0b0000);
		assert!(a.is_empty());

		let b = Bitboard::new(0b0001);
		assert!(!b.is_empty());
	}

	#[test]
	fn test_is_subset_of() {
		let a = Bitboard::new(0b0001);
		let b = Bitboard::new(0b0011);

		assert!(a.is_subset_of(b));
		assert!(!b.is_subset_of(a));
	}

	#[test]
	fn test_is_superset_of() {
		let a = Bitboard::new(0b0011);
		let b = Bitboard::new(0b0001);

		assert!(a.is_superset_of(b));
		assert!(!b.is_superset_of(a));
	}

	#[test]
	fn test_is_disjoint() {
		let a = Bitboard::new(0b0001);
		let b = Bitboard::new(0b0010);
		assert!(a.is_disjoint(b));

		let c = Bitboard::new(0b0011);
		assert!(!a.is_disjoint(c));
	}

	#[test]
	fn test_contains() {
		let a = Bitboard::new(0b0001);
		let square = Square::new(0);
		assert!(a.contains(square));

		let square = Square::new(1);
		assert!(!a.contains(square));
	}

	#[test]
	fn test_add_method() {
		let mut a = Bitboard::new(0b0001);
		a.add(Bitboard::new(0b0010));

		assert_eq!(a, Bitboard::new(0b0011));
	}

	#[test]
	fn test_toggle() {
		let mut a = Bitboard::new(0b0001);
		a.toggle(Bitboard::new(0b0010));
		assert_eq!(a, Bitboard::new(0b0011));

		a.toggle(Bitboard::new(0b0010));
		assert_eq!(a, Bitboard::new(0b0001));
	}

	#[test]
	fn test_discard() {
		let mut a = Bitboard::new(0b0011);
		a.discard(Bitboard::new(0b0010));

		assert_eq!(a, Bitboard::new(0b0001));
	}

	#[test]
	fn test_set() {
		let mut a = Bitboard::new(0b0000);
		let square = Square::new(0);
		a.set(square, true);
		assert_eq!(a, Bitboard::new(0b0001));

		a.set(square, false);
		assert_eq!(a, Bitboard::new(0b0000));
	}

	#[test]
	fn test_remove() {
		let mut a = Bitboard::new(0b0001);
		let square = Square::new(0);

		assert!(a.remove(square));
		assert_eq!(a, Bitboard::new(0b0000));
		assert!(!a.remove(square));
	}

	#[test]
	fn test_clear() {
		let mut a = Bitboard::new(0b0011);
		a.clear();

		assert_eq!(a, Bitboard::new(0b0000));
	}

	#[test]
	fn test_pop() {
		let mut a = Bitboard::new(0b0011);

		assert_eq!(a.pop(), Some(Square::new(0)));
		assert_eq!(a, Bitboard::new(0b0010));

		assert_eq!(a.pop(), Some(Square::new(1)));
		assert_eq!(a, Bitboard::new(0b0000));

		assert_eq!(a.pop(), None);
	}

	#[test]
	fn test_pop_back() {
		let mut a = Bitboard::new(0b0011);

		assert_eq!(a.pop_back(), Some(Square::new(1)));
		assert_eq!(a, Bitboard::new(0b0001));

		assert_eq!(a.pop_back(), Some(Square::new(0)));
		assert_eq!(a, Bitboard::new(0b0000));
		
		assert_eq!(a.pop_back(), None);
	}

	#[test]
	fn test_carry_rippler() {
		let bb = Bitboard::new(0b1010);
		let subsets: Vec<Bitboard> = bb.carry_rippler().collect();

		assert_eq!(subsets, vec![Bitboard::new(0), Bitboard::new(0b0010), Bitboard::new(0b1000), Bitboard::new(0b1010)]);
	}
}
