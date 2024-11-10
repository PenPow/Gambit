macro_rules! impl_ops {
	($struct:ty, $type:ty) => {
		use std::ops::{BitOr, BitAnd, BitXor, Rem};
		use super::macros::{impl_arithmetic_ops, impl_shift_ops};

		impl_arithmetic_ops!($struct, $type, Add, AddAssign, add, add_assign, wrapping_add);
		impl_arithmetic_ops!($struct, $type, Sub, SubAssign, sub, sub_assign, wrapping_sub);
		impl_arithmetic_ops!($struct, $type, Mul, MulAssign, mul, mul_assign, wrapping_mul);
		impl_arithmetic_ops!($struct, $type, Div, DivAssign, div, div_assign, wrapping_div);

		impl_arithmetic_ops!($struct, $type, Rem, RemAssign, rem, rem_assign, rem);

		impl_arithmetic_ops!($struct, $type, BitOr, BitOrAssign, bitor, bitor_assign, bitor);
		impl_arithmetic_ops!($struct, $type, BitAnd, BitAndAssign, bitand, bitand_assign, bitand);
		impl_arithmetic_ops!($struct, $type, BitXor, BitXorAssign, bitxor, bitxor_assign, bitxor);

		impl_shift_ops!($struct, Shl, ShlAssign, shl, shl_assign, wrapping_shl);
		impl_shift_ops!($struct, Shr, ShrAssign, shr, shr_assign, wrapping_shr);
	};
}

pub(crate) use impl_ops;

macro_rules! impl_arithmetic_ops {
    ($struct:ty, $type:ty, $trait_name:ident, $assign_trait_name:ident, $method_name:ident, $assign_method_name:ident, $method:ident) => {
        impl<T> std::ops::$trait_name<T> for $struct
		where
			T: Into<$struct>
		{
            type Output = Self;

			#[inline]
            fn $method_name(self, rhs: T) -> Self::Output {
				let Bitboard(rhs) = rhs.into();

                Self::from((self.0).$method(rhs))
            }
        }

        impl<T> std::ops::$assign_trait_name<T> for $struct
		where
			T: Into<$struct>
		{
			#[inline]
            fn $assign_method_name(&mut self, rhs: T) {
				let Bitboard(rhs) = rhs.into();

                *self = Self::from((self.0).$method(rhs));
            }
        }

		impl std::ops::$trait_name<$struct> for $type {
            type Output = $struct;

			#[inline]
            fn $method_name(self, rhs: $struct) -> Self::Output {
                <$struct>::from((self).$method(rhs.0))
            }
        }
    };
}

pub(crate) use impl_arithmetic_ops;

macro_rules! impl_shift_ops {
    ($struct:ty, $trait_name:ident, $assign_trait_name:ident, $method_name:ident, $assign_method_name:ident, $method:ident) => {
		impl<T> std::ops::$trait_name<T> for $struct
		where
			T: Into<$struct>
		{
            type Output = Self;

			#[inline]
            fn $method_name(self, rhs: T) -> Self::Output {
				let Bitboard(rhs) = rhs.into();

                Self::from((self.0).$method(rhs as u32))
            }
        }

        impl<T> std::ops::$assign_trait_name<T> for $struct
		where
			T: Into<$struct>
		{
			#[inline]
            fn $assign_method_name(&mut self, rhs: T) {
				let Bitboard(rhs) = rhs.into();

                *self = Self::from((self.0).$method(rhs as u32));
            }
        }
    };
}

pub(crate) use impl_shift_ops;