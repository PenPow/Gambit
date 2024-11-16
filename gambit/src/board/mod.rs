//! A module containing structures to represent the board and the state of it

pub mod fen;

mod core;
mod fmt;

pub use core::{Board, State};