//! Module containing enums to represent squares, ranks and files

mod square;
mod direction;
mod rank;
mod file;

pub use self::square::Square;
pub use self::direction::Direction;
pub use self::rank::Rank;
pub use self::file::File;

/// A tuple representing a [`Rank`] and [`File`]
pub type Coordinates = (File, Rank);