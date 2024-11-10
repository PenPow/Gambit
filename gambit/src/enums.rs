macro_rules! impl_enum_to_int {
	($enum:ty) => {
		use $crate::enums::{impl_enum_to_int_internal};

		impl_enum_to_int_internal!($enum, u8);
		impl_enum_to_int_internal!($enum, i8);

		impl_enum_to_int_internal!($enum, u16);
		impl_enum_to_int_internal!($enum, i16);

		impl_enum_to_int_internal!($enum, u32);
		impl_enum_to_int_internal!($enum, i32);

		impl_enum_to_int_internal!($enum, u64);
		impl_enum_to_int_internal!($enum, i64);

		impl_enum_to_int_internal!($enum, u128);
		impl_enum_to_int_internal!($enum, i128);

		impl_enum_to_int_internal!($enum, usize);
		impl_enum_to_int_internal!($enum, isize);
	}
}

pub(crate) use impl_enum_to_int;

macro_rules! impl_signed_enum_to_int {
	($enum:ty) => {
		use $crate::enums::{impl_signed_enum_to_int_internal};

		impl_signed_enum_to_int_internal!($enum, u8);
		impl_signed_enum_to_int_internal!($enum, i8);

		impl_signed_enum_to_int_internal!($enum, u16);
		impl_signed_enum_to_int_internal!($enum, i16);

		impl_signed_enum_to_int_internal!($enum, u32);
		impl_signed_enum_to_int_internal!($enum, i32);

		impl_signed_enum_to_int_internal!($enum, u64);
		impl_signed_enum_to_int_internal!($enum, i64);

		impl_signed_enum_to_int_internal!($enum, u128);
		impl_signed_enum_to_int_internal!($enum, i128);

		impl_signed_enum_to_int_internal!($enum, usize);
		impl_signed_enum_to_int_internal!($enum, isize);
	}
}

pub(crate) use impl_signed_enum_to_int;

macro_rules! impl_enum_to_int_internal {
	($enum:ty, $num:ty) => {
		impl From<$enum> for $num {
			#[inline]
			fn from(value: $enum) -> $num {
				value as $num
			}
		}

		impl TryFrom<$num> for $enum {
			type Error = std::num::TryFromIntError;
		
			#[inline]
			fn try_from(value: $num) -> Result<$enum, Self::Error> {
				if ((<$enum>::MIN as $num)..=(<$enum>::MAX as $num)).contains(&value) {
					#[allow(clippy::cast_sign_loss)]
					Ok(<$enum>::new(value as u8))
				} else {
					Err(unsafe { u32::try_from(u64::MAX).unwrap_err_unchecked() })
				}
			}
		}
	}
}

pub(crate) use impl_enum_to_int_internal;

macro_rules! impl_signed_enum_to_int_internal {
	($enum:ty, $num:ty) => {
		impl From<$enum> for $num {
			#[inline]
			fn from(value: $enum) -> $num {
				value as $num
			}
		}

		impl TryFrom<$num> for $enum {
			type Error = std::num::TryFromIntError;
		
			#[inline]
			fn try_from(value: $num) -> Result<$enum, Self::Error> {
				if ((<$enum>::MIN as $num)..=(<$enum>::MAX as $num)).contains(&value) {
					#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
					Ok(<$enum>::new(value as i8))
				} else {
					Err(unsafe { u32::try_from(u64::MAX).unwrap_err_unchecked() })
				}
			}
		}
	}
}

pub(crate) use impl_signed_enum_to_int_internal;

macro_rules! impl_enum_arithmetic_ops {
	($enum:ty) => {
		use $crate::enums::impl_enum_arithmetic_ops_internal;

		impl std::ops::Add<$enum> for $enum {
			type Output = Bitboard;
			
			#[inline]
			fn add(self, other: $enum) -> Self::Output {
				Self::Output::from(self) + Self::Output::from(other)
			}
		}

		impl std::ops::Sub<$enum> for $enum {
			type Output = Bitboard;
			
			#[inline]
			fn sub(self, other: Square) -> Self::Output {
				Self::Output::from(self) - Self::Output::from(other)
			}
		}

		impl_enum_arithmetic_ops_internal!($enum, u8);
		impl_enum_arithmetic_ops_internal!($enum, i8);

		impl_enum_arithmetic_ops_internal!($enum, u16);
		impl_enum_arithmetic_ops_internal!($enum, i16);

		impl_enum_arithmetic_ops_internal!($enum, u32);
		impl_enum_arithmetic_ops_internal!($enum, i32);

		impl_enum_arithmetic_ops_internal!($enum, u64);
		impl_enum_arithmetic_ops_internal!($enum, i64);

		impl_enum_arithmetic_ops_internal!($enum, u128);
		impl_enum_arithmetic_ops_internal!($enum, i128);

		impl_enum_arithmetic_ops_internal!($enum, usize);
		impl_enum_arithmetic_ops_internal!($enum, isize);
	}
}

pub(crate) use impl_enum_arithmetic_ops;

macro_rules! impl_enum_arithmetic_ops_internal {
	($enum:ty, $num:ty) => {
		impl std::ops::Add<$num> for $enum {
			type Output = Result<$enum, std::num::TryFromIntError>;
			
			#[inline]
			fn add(self, other: $num) -> Self::Output {
				<$enum>::try_from((self as $num) + other)
			}
		}

		impl std::ops::Sub<$num> for $enum {
			type Output = Result<$enum, std::num::TryFromIntError>;
			
			#[inline]
			fn sub(self, other: $num) -> Self::Output {
				<$enum>::try_from((self as $num) - other)
			}
		}
	}
}

pub(crate) use impl_enum_arithmetic_ops_internal;