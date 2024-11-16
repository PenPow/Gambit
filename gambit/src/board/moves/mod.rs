//! A module containing useful structs, builders and enums to encode and construct the binary representations of moves

mod core;
mod fmt;

pub mod shifts;
pub mod builder;

pub use core::{Move, MoveUnderlyingType};