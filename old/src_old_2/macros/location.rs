#[macro_export]
macro_rules! dbg_assert_square_in_range {
	($square: ident) => {
		#[cfg(debug_assertions)]
		{
			assert!($square < 64usize, "Square must be in range 0-63 (A1-H8)")
		}
	};
}

#[macro_export]
macro_rules! dbg_assert_file_in_range {
	($file: ident) => {
		#[cfg(debug_assertions)]
		{
			assert!($file < 8, "File must be in range 0-7 (A-H)")
		}
	};
}

#[macro_export]
macro_rules! dbg_assert_rank_in_range {
	($rank: ident) => {
		#[cfg(debug_assertions)]
		{
			assert!($rank < 8, "File must be in range 0-7 (R1-R8)")
		}
	};
}