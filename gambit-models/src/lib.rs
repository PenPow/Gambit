//! Core types and primitives for the Gambit chess engine ecosystem
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`bitboard`] | 64-bit set-of-squares representation |
//! | [`location::square`] | Individual board squares (`a1`..`h8`) |
//! | [`location::file`] | File (column) of the board (`a`..`h`) |
//! | [`location::rank`] | Rank (row) of the board (`1`..`8`) |
//! | [`piece`] | Packed piece value: type and colour in one byte |
//! | [`movement::direction`] | Ray directions for sliding piece generation |
//! | [`movement::castling`] | Castling rights and kingside/queenside encoding |
//! | [`mailbox`] | Square-indexed piece lookup table |
//! | [`moves`] | Compact 32-bit move encoding |
//! | [`error`] | Shared parse and conversion error types |
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `colored` | Coloured terminal output for [`Bitboard`](bitboard::Bitboard) and [`Mailbox`](mailbox::Mailbox) |
//!
//! # Quick start
//!
//! ```rust
//! use gambit_models::bitboard::Bitboard;
//! use gambit_models::location::file::File;
//! use gambit_models::location::rank::Rank;
//! use gambit_models::location::square::Square;
//!
//! // Construct bitboards from files, ranks, or squares
//! let e_file  = File::E.bitboard();
//! let rank_4  = Bitboard::from(Rank::Four);
//! let e4_only = e_file & rank_4;
//!
//! assert!(e4_only.contains(Square::E4));
//! assert_eq!(e4_only.into_iter().count(), 1);
//! ```

// TODO!: tests

pub mod bitboard;
pub mod error;
pub mod location;
mod macros;
pub mod mailbox;
pub mod movement;
pub mod moves;
pub mod piece;
pub mod position;
pub mod traits;
