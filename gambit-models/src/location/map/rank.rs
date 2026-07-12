#![allow(unused_braces)]

use crate::location::rank::Rank;
use crate::macros::define_map;

define_map! {
    /// A fixed-size map from [`Rank`] to a value of type `T`.
    ///
    /// Backed by a `[T; 8]` array indexed by `rank as usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::map::rank::RankMap;
    /// # use gambit_models::location::rank::Rank;
    /// let mut map: RankMap<i32> = RankMap::default();
    /// map[Rank::Five] = 900;
    /// map[Rank::Seven]  = 500;
    /// assert_eq!(map[Rank::Five], 900);
    /// ```
    RankMap, Rank, { Rank::COUNT }
}
