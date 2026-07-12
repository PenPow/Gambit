//! Notation parsers for the Gambit chess engine ecosystem
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`fen`] | FEN (Forsyth–Edwards Notation) parser |
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `fen` | Enable the FEN parser module |

// TODO!: tests

#[cfg(feature = "fen")]
pub mod fen;

#[cfg(feature = "lan")]
pub mod algebraic;

#[cfg(feature = "lan")]
pub use algebraic::lan;
