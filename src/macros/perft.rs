#[macro_export]
macro_rules! generate_perft_tests {
	($([$name:ident, $desc:expr, $fen:expr, [$($expected:expr),*]]),*) => {
		$(
			#[test]
			fn $name() {
				const FEN: &'static str = $fen;
				const DEPTH: usize = count!($($expected),*);

				const EXPECTED: [u64; DEPTH] = [$($expected),*];

				println!("Test Case: {}", $desc);

				let perft_result = perft(FEN, DEPTH as u8);
				let mut should_fail = false;

				for (i, (expected, actual)) in EXPECTED.iter().zip(perft_result).enumerate() {
					let expected = *expected;
					let depth = i + 1;

					if expected != actual {
						if !should_fail {
							should_fail = true;
		
							eprintln!("\n== PERFT TEST FAILURE ==")
						}

						eprintln!("Perft({depth}): Expected {expected}, recieved {actual}");
					}
				}

				if should_fail {
					panic!("\nPerft test failed")
				}
			}
		)*
	};
}

#[macro_export]
macro_rules! count {
    () => { 0 };
    ($head:expr $(, $tail:expr)*) => { 1 + count!($($tail),*) };
}