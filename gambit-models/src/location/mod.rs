//! Types that involve locations such as ranks, files, squares and coordinates.
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`square`] | Individual board squares (`a1`..`h8`) |
//! | [`mod@file`] | File (column) of the board (`a`..`h`) |
//! | [`rank`] | Rank (row) of the board (`1`..`8`) |
//! | [`map`] | Tables allowing for indexing using the location structs |

pub mod file;
pub mod map;
pub mod rank;
pub mod square;
