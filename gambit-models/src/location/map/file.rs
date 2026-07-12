#![allow(unused_braces)]

use crate::location::file::File;
use crate::macros::define_map;

define_map! {
    /// A fixed-size map from [`File`] to a value of type `T`.
    ///
    /// Backed by a `[T; 8]` array indexed by `file as usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::map::file::FileMap;
    /// # use gambit_models::location::file::File;
    /// let mut map: FileMap<i32> = FileMap::default();
    /// map[File::G] = 900;
    /// map[File::B]  = 500;
    /// assert_eq!(map[File::G], 900);
    /// ```
    FileMap, File, { File::COUNT }
}
