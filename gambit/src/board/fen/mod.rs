//! A somewhat-compliant FEN parser

#![allow(clippy::module_name_repetitions)]

mod parser;
mod error;
mod string;

pub use parser::FenParser;
pub use error::FenError;
pub use string::Fen;