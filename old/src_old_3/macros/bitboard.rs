#[macro_export]
macro_rules! impl_ops {
	($struct:ident, $type:ident) => {
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

		impl_output_types!($struct);

		impl From<$type> for $struct {
			fn from(bits: $type) -> Self {
				$struct(bits)
			}
		}
		
		impl From<$struct> for $type {
			fn from(structure: $struct) -> Self {
				structure.0
			}
		}

		impl std::ops::Not for $struct {
			type Output = Self;
		
			fn not(self) -> Self {
				Self(!self.0)
			}
		}

		impl PartialEq<$type> for $struct {
			fn eq(&self, other: &$type) -> bool {
				self.0.eq(other)
			}

			fn ne(&self, other: &$type) -> bool {
				self.0.ne(other)
			}
		}

		impl PartialEq<$struct> for $struct {
			fn eq(&self, other: &$struct) -> bool {
				self.0.eq(&other.0)
			}

			fn ne(&self, other: &$struct) -> bool {
				self.0.ne(&other.0)
			}
		}

		impl Eq for $struct {}

		impl PartialOrd<$type> for $struct {
			fn partial_cmp(&self, other: &$type) -> Option<std::cmp::Ordering> {
				self.0.partial_cmp(other)
			}
		}

		impl PartialOrd<$struct> for $struct {
			fn partial_cmp(&self, other: &$struct) -> Option<std::cmp::Ordering> {
				self.0.partial_cmp(&other.0)
			}
		}

		impl Ord for $struct {
			fn cmp(&self, other: &$struct) -> std::cmp::Ordering {
				self.0.cmp(&other.0)
			}
		}
	};
}

#[macro_export]
macro_rules! impl_output_types {
	($struct: ident) => {
		impl fmt::UpperHex for $struct {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				fmt::UpperHex::fmt(&self.0, f)
			}
		}

		impl fmt::LowerHex for $struct {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				fmt::LowerHex::fmt(&self.0, f)
			}
		}

		impl fmt::Octal for $struct {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				fmt::Octal::fmt(&self.0, f)
			}
		}

		impl fmt::Binary for $struct {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				fmt::Binary::fmt(&self.0, f)
			}
		}
	};
}

#[macro_export]
macro_rules! impl_arithmetic_ops {
    ($struct:ident, $type:ident, $trait_name:ident, $assign_trait_name:ident, $method_name:ident, $assign_method_name:ident, $method:ident) => {
        impl std::ops::$trait_name for $struct {
            type Output = Self;

            fn $method_name(self, rhs: Self) -> Self::Output {
                Self::from((self.0).$method(rhs.0))
            }
        }

        impl std::ops::$assign_trait_name for $struct {
            fn $assign_method_name(&mut self, rhs: Self) {
                *self = Self::from((self.0).$method(rhs.0));
            }
        }

		impl std::ops::$trait_name<$type> for $struct {
            type Output = Self;

            fn $method_name(self, rhs: $type) -> Self::Output {
                Self::from((self.0).$method(rhs))
            }
        }

        impl std::ops::$assign_trait_name<$type> for $struct {
            fn $assign_method_name(&mut self, rhs: $type) {
                *self = Self::from((self.0).$method(rhs));
            }
        }

		impl std::ops::$trait_name<$struct> for $type {
            type Output = $struct;

            fn $method_name(self, rhs: $struct) -> Self::Output {
                $struct::from((self).$method(rhs.0))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_shift_ops {
    ($struct:ident, $trait_name:ident, $assign_trait_name:ident, $method_name:ident, $assign_method_name:ident, $method:ident) => {
		impl std::ops::$trait_name<usize> for $struct {
            type Output = Self;

            fn $method_name(self, rhs: usize) -> Self::Output {
                Self::from((self.0).$method(rhs as u32))
            }
        }

        impl std::ops::$assign_trait_name<usize> for $struct {
            fn $assign_method_name(&mut self, rhs: usize) {
                *self = Self::from((self.0).$method(rhs as u32));
            }
        }
    };
}