#[macro_export]
macro_rules! if_debug {
	($expr:expr) => {
		#[cfg(debug_assertions)]
		{
			$expr
		}
	}
}