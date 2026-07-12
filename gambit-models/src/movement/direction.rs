use crate::error::TryFromIntError;
use std::fmt;

/// A compass direction on the chessboard, for use with ray generation and square offsets.
///
/// Directions map to square-index offsets on the standard 64-square board (a1=0, h8=63)
///
/// | Direction   | Offset |
/// |-------------|--------|
/// | North       | +8     |
/// | NorthEast   | +9     |
/// | East        | +1     |
/// | SouthEast   | −7     |
/// | South       | −8     |
/// | SouthWest   | −9     |
/// | West        | −1     |
/// | NorthWest   | +7     |
///
/// # Groupings
///
/// Directions are grouped into named slices for use in move generation:
///
/// - [`ORTHOGONALS`][Direction::ORTHOGONALS] / [`ROOK_RAYS`][Direction::ROOK_RAYS]
/// - [`DIAGONALS`][Direction::DIAGONALS] / [`BISHOP_RAYS`][Direction::BISHOP_RAYS]
/// - [`ALL`][Direction::ALL] / [`QUEEN_RAYS`][Direction::QUEEN_RAYS]
///
/// # Examples
///
/// ```rust
/// # use gambit_models::movement::direction::Direction;
/// assert_eq!(Direction::North.offset(), 8);
/// assert_eq!(-Direction::North, Direction::South);
/// assert!(Direction::North.is_orthogonal());
/// assert!(Direction::NorthEast.is_diagonal());
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    /// Total number of directions.
    pub const COUNT: usize = 8;

    /// Number of orthogonal directions.
    pub const ORTHOGONAL_COUNT: usize = 4;

    /// Number of diagonal directions.
    pub const DIAGONAL_COUNT: usize = 4;

    /// All 8 directions in declaration order, starting from [`North`][Direction::North], proceeding clockwise round the compass rose.
    pub const ALL: [Direction; Direction::COUNT] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    /// The 4 orthogonal directions: [`North`][Direction::North], [`East`][Direction::East], [`South`][Direction::South], [`West`][Direction::West].
    pub const ORTHOGONALS: [Direction; Direction::ORTHOGONAL_COUNT] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    /// The 4 diagonal directions: [`NorthEast`][Direction::NorthEast], [`SouthEast`][Direction::SouthEast], [`SouthWest`][Direction::SouthWest], [`NorthWest`][Direction::NorthWest].
    pub const DIAGONALS: [Direction; Direction::DIAGONAL_COUNT] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    /// Alias for [`ORTHOGONALS`][Direction::ORTHOGONALS].
    /// The directions a rook can move along.
    pub const ROOK_RAYS: [Direction; Direction::ORTHOGONAL_COUNT] = Self::ORTHOGONALS;

    /// Alias for [`DIAGONALS`][Direction::DIAGONALS].
    /// The directions a bishop can move along.
    pub const BISHOP_RAYS: [Direction; Direction::DIAGONAL_COUNT] = Self::DIAGONALS;

    /// Alias for [`ALL`][Direction::ALL].
    /// The directions a queen can move along.
    pub const QUEEN_RAYS: [Direction; Direction::COUNT] = Self::ALL;

    /// Creates a `Direction` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 8`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `Direction` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=7`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`Direction::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 8);

        // SAFETY: index is in range 0..=7 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }

    /// Creates a `Direction` from its square-index offset.
    ///
    /// Valid offsets are `±1`, `±7`, `±8`, `±9` — the eight values that correspond to one step in each compass direction on a standard 8-by-8 chess board.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not one of the 8 valid offsets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::movement::direction::Direction;
    /// assert_eq!(Direction::from_offset(8), Direction::North);
    /// assert_eq!(Direction::from_offset(-9), Direction::SouthWest);
    /// ```
    #[inline]
    pub const fn from_offset(offset: i8) -> Self {
        debug_assert!(matches!(offset, -9 | -8 | -7 | -1 | 1 | 7 | 8 | 9));

        match offset {
            -9 => Direction::SouthWest,
            -8 => Direction::South,
            -7 => Direction::SouthEast,
            -1 => Direction::West,
            1 => Direction::East,
            7 => Direction::NorthWest,
            8 => Direction::North,
            9 => Direction::NorthEast,
            _ => unreachable!(),
        }
    }

    /// Returns the square-index offset for one step in this direction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::movement::direction::Direction;
    /// assert_eq!(Direction::North.offset(), 8);
    /// assert_eq!(Direction::SouthWest.offset(), -9);
    /// ```
    #[inline]
    pub const fn offset(self) -> i8 {
        match self {
            Direction::North => 8,
            Direction::NorthEast => 9,
            Direction::East => 1,
            Direction::SouthEast => -7,
            Direction::South => -8,
            Direction::SouthWest => -9,
            Direction::West => -1,
            Direction::NorthWest => 7,
        }
    }

    /// Returns the file component of one step in this direction.
    ///
    /// - `-1` for westward directions
    /// -  `0` for north/south
    /// - `+1` for eastward directions
    #[inline]
    pub const fn file_delta(self) -> i8 {
        match self {
            Direction::North | Direction::South => 0,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::NorthWest | Direction::West | Direction::SouthWest => -1,
        }
    }

    /// Returns the rank component of one step in this direction.
    ///
    /// - `-1` for southward directions
    /// -  `0` for east/west
    /// - `+1` for northward directions
    #[inline]
    pub const fn rank_delta(self) -> i8 {
        match self {
            Direction::West | Direction::East => 0,
            Direction::NorthWest | Direction::North | Direction::NorthEast => 1,
            Direction::SouthWest | Direction::South | Direction::SouthEast => -1,
        }
    }

    /// Returns the opposite direction.
    ///
    /// `flip` is its own inverse: `d.flip().flip() == d`.
    ///
    /// Equivalent to the [`Neg`][std::ops::Neg] implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::movement::direction::Direction;
    /// assert_eq!(Direction::North.flip(), Direction::South);
    /// assert_eq!(Direction::NorthEast.flip(), Direction::SouthWest);
    /// ```
    #[inline]
    pub const fn flip(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    /// Returns `true` if this direction is orthogonal (north, east, south, or west).
    ///
    /// Orthogonal directions are the movement directions of the rook.
    /// Mutually exclusive with [`is_diagonal`][Direction::is_diagonal].
    #[inline]
    pub const fn is_orthogonal(self) -> bool {
        matches!(
            self,
            Direction::North | Direction::East | Direction::South | Direction::West
        )
    }

    /// Returns `true` if this direction is diagonal (NE, SE, SW, or NW).
    ///
    /// Diagonal directions are the movement directions of the bishop.
    /// Mutually exclusive with [`is_orthogonal`][Direction::is_orthogonal].
    #[inline]
    pub const fn is_diagonal(self) -> bool {
        matches!(
            self,
            Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest
        )
    }

    /// Returns `true` if the offset of this direction is positive.
    pub const fn is_positive(self) -> bool {
        self.offset() > 0
    }
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        self.flip()
    }
}

impl From<Direction> for i8 {
    fn from(direction: Direction) -> Self {
        direction.offset()
    }
}

impl TryFrom<i8> for Direction {
    type Error = TryFromIntError<i8>;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -9 => Ok(Direction::SouthWest),
            -8 => Ok(Direction::South),
            -7 => Ok(Direction::SouthEast),
            -1 => Ok(Direction::West),
            1 => Ok(Direction::East),
            7 => Ok(Direction::NorthWest),
            8 => Ok(Direction::North),
            9 => Ok(Direction::NorthEast),
            _ => Err(TryFromIntError(value)),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 8 {
            Ok(Direction::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => f.write_str("N"),
            Direction::NorthEast => f.write_str("NE"),
            Direction::East => f.write_str("E"),
            Direction::SouthEast => f.write_str("SE"),
            Direction::South => f.write_str("S"),
            Direction::SouthWest => f.write_str("SW"),
            Direction::West => f.write_str("W"),
            Direction::NorthWest => f.write_str("NW"),
        }
    }
}
