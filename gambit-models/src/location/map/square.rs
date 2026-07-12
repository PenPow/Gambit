#![allow(unused_braces)]

use crate::location::square::Square;
use crate::macros::define_map;

define_map! {
    /// A fixed-size map from [`Square`] to a value of type `T`.
    ///
    /// Backed by a `[T; 64]` array indexed by `square as usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::map::square::SquareMap;
    /// # use gambit_models::location::square::Square;
    /// let mut map: SquareMap<i32> = SquareMap::default();
    /// map[Square::A3] = 900;
    /// map[Square::G5]  = 500;
    /// assert_eq!(map[Square::A3], 900);
    /// ```
    SquareMap, Square, { Square::COUNT }
}
