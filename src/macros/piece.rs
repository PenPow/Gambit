#[macro_export]
macro_rules! dbg_assert_side_in_range {
	($side: ident) => {
		#[cfg(debug_assertions)]
		{
			assert!($side < 2, "Side must be between 0 and 1")
		}
	};
}

#[macro_export]
macro_rules! dbg_assert_piece_in_range {
	($side: ident) => {
		#[cfg(debug_assertions)]
		{
			assert!($side < 7, "Piece must be between 0 and 7")
		}
	};
}