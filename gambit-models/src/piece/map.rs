#![allow(unused_braces)]

use crate::macros::define_map;
use crate::piece::colour::Colour;
use crate::piece::piece_type::PieceType;

define_map! {
    /// A fixed-size map from [`Colour`] to a value of type `T`.
    ///
    /// Backed by a `[T; 2]` array indexed by `colour as usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::piece::map::ColourMap;
    /// # use gambit_models::piece::colour::Colour;
    /// let mut scores: ColourMap<i32> = ColourMap::default();
    /// scores[Colour::White] = 100;
    /// scores[Colour::Black] = 95;
    /// assert_eq!(scores[Colour::White], 100);
    /// ```
    ColourMap, Colour, { Colour::COUNT }
}

define_map! {
    /// A fixed-size map from [`PieceType`] to a value of type `T`.
    ///
    /// Backed by a `[T; 6]` array indexed by `piece_type as usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::piece::map::PieceTypeMap;
    /// # use gambit_models::piece::piece_type::PieceType;
    /// let mut material: PieceTypeMap<i32> = PieceTypeMap::default();
    /// material[PieceType::Queen] = 900;
    /// material[PieceType::Rook]  = 500;
    /// assert_eq!(material[PieceType::Queen], 900);
    /// ```
    PieceTypeMap, PieceType, { PieceType::COUNT }
}
