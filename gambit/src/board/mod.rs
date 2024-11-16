//! A module containing structures to represent the board and the state of it

pub mod fen;
pub mod moves;

mod core;
mod fmt;

pub use core::{Board, State};