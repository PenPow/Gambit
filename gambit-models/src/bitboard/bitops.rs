use crate::bitboard::Bitboard;
use crate::traits::IntoBitboard;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

impl Not for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn not(self) -> Bitboard {
        Bitboard::new(!self.bits())
    }
}

impl<T: IntoBitboard> BitAnd<T> for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitand(self, rhs: T) -> Bitboard {
        Bitboard::new(self.bits() & rhs.into_bitboard().bits())
    }
}

impl<T: IntoBitboard> BitAndAssign<T> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: T) {
        *self = Bitboard::new(self.bits() & rhs.into_bitboard().bits())
    }
}

impl<T: IntoBitboard> BitOr<T> for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitor(self, rhs: T) -> Bitboard {
        Bitboard::new(self.bits() | rhs.into_bitboard().bits())
    }
}

impl<T: IntoBitboard> BitOrAssign<T> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: T) {
        *self = Bitboard::new(self.bits() | rhs.into_bitboard().bits())
    }
}

impl<T: IntoBitboard> BitXor<T> for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitxor(self, rhs: T) -> Bitboard {
        Bitboard::new(self.bits() ^ rhs.into_bitboard().bits())
    }
}

impl<T: IntoBitboard> BitXorAssign<T> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: T) {
        *self = Bitboard::new(self.bits() ^ rhs.into_bitboard().bits())
    }
}

impl Shl<u32> for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn shl(self, rhs: u32) -> Bitboard {
        Bitboard::new(self.bits() << rhs)
    }
}

impl Shr<u32> for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn shr(self, rhs: u32) -> Bitboard {
        Bitboard::new(self.bits() >> rhs)
    }
}
