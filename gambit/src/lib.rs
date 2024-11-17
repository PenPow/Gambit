#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs, rustdoc::unescaped_backticks)]
#![allow(clippy::inline_always, clippy::cast_possible_truncation)]

#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]

/// The current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod bitboard;
pub mod location;
pub mod piece;
pub mod board;

mod enums;