//! Fixed-size maps indexed by location types.
//!
//! Provides array-backed map types for [`File`][crate::location::file::File], [`Rank`][crate::location::rank::Rank], and [`Square`][crate::location::square::Square],
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`square`] | Map indexed by [`Square`][crate::location::square::Square] |
//! | [`rank`] | Map indexed by [`Rank`][crate::location::rank::Rank] |
//! | [`mod@file`] | Map indexed by [`File`][crate::location::file::File] |

pub mod file;
pub mod rank;
pub mod square;
