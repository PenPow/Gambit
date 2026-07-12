//! Trait definitions for usage within the gambit-models crate.

use crate::bitboard::Bitboard;

/// Conversion into a [`Bitboard`]
///
/// The trait is the RHS type parameter for all bitwise operator
/// overloads on [`Bitboard`]:
///
/// ```rust
/// # use gambit_models::bitboard::Bitboard;
/// # use gambit_models::location::file::File;
/// # use gambit_models::location::rank::Rank;
/// # use gambit_models::location::square::Square;
/// // All of these use IntoBitboard under the hood:
/// let by_file   = Bitboard::EMPTY | File::E;
/// let by_rank   = Bitboard::EMPTY | Rank::Four;
/// let by_square = Bitboard::EMPTY | Square::E4;
/// let by_bb     = Bitboard::EMPTY | Bitboard::UNIVERSE;
/// ```
pub trait IntoBitboard {
    /// Converts `self` into its canonical [`Bitboard`] representation.
    fn into_bitboard(self) -> Bitboard;
}
