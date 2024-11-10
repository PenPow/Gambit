#[macro_export]
macro_rules! impl_bitboard_ops {
    ($trait_name:ident, $assign_trait_name:ident, $method_name:ident, $assign_method_name:ident, $method:ident) => {
        impl std::ops::$trait_name for Bitboard {
            type Output = Self;

            fn $method_name(self, rhs: Self) -> Self::Output {
                Self::from((self.0).$method(rhs.0))
            }
        }

        impl std::ops::$assign_trait_name for Bitboard {
            fn $assign_method_name(&mut self, rhs: Self) {
                *self = Self::from((self.0).$method(rhs.0));
            }
        }

		impl std::ops::$trait_name<u64> for Bitboard {
            type Output = Self;

            fn $method_name(self, rhs: u64) -> Self::Output {
                Self::from((self.0).$method(rhs))
            }
        }

        impl std::ops::$assign_trait_name<u64> for Bitboard {
            fn $assign_method_name(&mut self, rhs: u64) {
                *self = Self::from((self.0).$method(rhs));
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bitboard_shift_ops {
    ($trait_name:ident, $assign_trait_name:ident, $method_name:ident, $assign_method_name:ident, $method:ident) => {
		impl std::ops::$trait_name<usize> for Bitboard {
            type Output = Self;

            fn $method_name(self, rhs: usize) -> Self::Output {
                Self::from((self.0).$method(rhs as u32))
            }
        }

        impl std::ops::$assign_trait_name<usize> for Bitboard {
            fn $assign_method_name(&mut self, rhs: usize) {
                *self = Self::from((self.0).$method(rhs as u32));
            }
        }
    };
}