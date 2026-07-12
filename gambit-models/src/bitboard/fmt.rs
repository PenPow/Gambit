use crate::bitboard::Bitboard;
use crate::location::file::File;
use crate::location::rank::Rank;
use crate::location::square::Square;
#[cfg(feature = "colored")]
use colored::Colorize;
use std::fmt;
use std::fmt::Formatter;

#[cfg(feature = "colored")]
fn format_bit(is_set: bool) -> String {
    if is_set {
        "1".green().bold().to_string()
    } else {
        "0".red().dimmed().to_string()
    }
}

#[cfg(not(feature = "colored"))]
fn format_bit(is_set: bool) -> String {
    if is_set {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  a b c d e f g h")?;

        for rank in Rank::ALL.iter().rev() {
            write!(f, "{} ", *rank as u8 + 1)?;

            for file in File::ALL {
                let square = Square::from_coordinates((file, *rank));
                write!(f, "{} ", format_bit(self.contains(square)))?;
            }

            writeln!(f, "{}", *rank as u8 + 1)?;
        }

        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, "  Bits: {}", self.0.count_ones())
    }
}

macro_rules! impl_fmt {
    ($trait:path) => {
        impl $trait for Bitboard {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <u64 as $trait>::fmt(&self.bits(), f)
            }
        }
    };
}

impl_fmt!(fmt::Binary);
impl_fmt!(fmt::LowerHex);
impl_fmt!(fmt::UpperHex);
