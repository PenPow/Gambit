#![allow(clippy::unreadable_literal)]

use std::fmt;
use colored::Colorize;
use super::Bitboard;

impl Bitboard {
	pub(super) fn as_str(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		const LAST_BIT: u64 = 63;

		writeln!(f)?;
		
		for rank in 0..8 {
			for file in (0..8).rev() {
				let mask = 1u64 << (LAST_BIT - (rank * 8) - file);

				let char = if self.bits() & mask != 0 {
					if cfg!(debug_assertions) {
						"1".green().to_string()
					} else {
						"1".to_string()
					}
				} else if cfg!(debug_assertions) {
					"0".red().to_string()
				} else {
					"0".to_string()
				};

				write!(f, "{char} ")?;
			}

			writeln!(f)?;
		}

		Ok(())
	}
}

impl fmt::UpperHex for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::UpperHex::fmt(&self.bits(), f)
	}
}

impl fmt::LowerHex for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::LowerHex::fmt(&self.bits(), f)
	}
}

impl fmt::Octal for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Octal::fmt(&self.bits(), f)
	}
}

impl fmt::Binary for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&self.bits(), f)
	}
}

impl fmt::Display for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

impl fmt::Debug for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_str(f)
	}
}

#[cfg(test)]
mod tests {
    use super::Bitboard;

	#[test]
	fn test_upper_hex() {
		let bitboard = Bitboard::new(0x1234ABCD);

		assert_eq!(format!("{bitboard:X}"), "1234ABCD");
	}

	#[test]
	fn test_lower_hex() {
		let bitboard = Bitboard::new(0x1234abcd);

		assert_eq!(format!("{bitboard:x}"), "1234abcd");
	}

	#[test]
	fn test_octal() {
		let bitboard = Bitboard::new(0o1234567);

		assert_eq!(format!("{bitboard:o}"), "1234567");
	}

	#[test]
	fn test_binary() {
		let bitboard = Bitboard::new(0b101010);

		assert_eq!(format!("{bitboard:b}"), "101010");
	}

}
