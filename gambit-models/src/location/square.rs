use crate::bitboard::Bitboard;
use crate::error::{ParseSquareError, TryFromIntError};
use crate::location::file::File;
use crate::location::map::square::SquareMap;
use crate::location::rank::Rank;
use crate::macros::define_squares;
use crate::movement::castling::rights::CastlingRights;
use crate::traits::IntoBitboard;
use std::str::FromStr;

/// A type alias for a `(File, Rank)` coordinate pair.
///
/// ```rust
/// # use gambit_models::location::square::{Square, Coordinates};
/// # use gambit_models::location::file::File;
/// # use gambit_models::location::rank::Rank;
/// let coords: Coordinates = (File::E, Rank::Four);
/// assert_eq!(Square::from_coordinates(coords), Square::E4);
/// ```
pub type Coordinates = (File, Rank);

/// A square on the chessboard, indexed 0 (`a1`) to 63 (`h8`).
///
/// # Index Layout
///
/// ```text
///  56 57 58 59 60 61 62 63   ← rank 8 (a8..h8)
///  48 49 50 51 52 53 54 55   ← rank 7
///   .  .  .  .  .  .  .  .
///   0  1  2  3  4  5  6  7   ← rank 1 (a1..h1)
/// ```
///
/// # Ordering
///
/// The derived [`Ord`] follows index order: `a1 < b1 < ... < h1 < a2 < ... < h8`.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::location::square::Square;
/// # use gambit_models::location::file::File;
/// # use gambit_models::location::rank::Rank;
/// let e4 = Square::E4;
/// assert_eq!(e4.file(), File::E);
/// assert_eq!(e4.rank(), Rank::Four);
/// assert_eq!(e4.to_string(), "e4");
/// assert_eq!(e4.flip_rank(), Square::E5);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Square(u8);

impl Square {
    /// Total number of squares on the board.
    pub const COUNT: usize = 64;

    /// The lowest-indexed square (`a1`, index 0).
    pub const MIN: Square = Square::A1;

    /// The highest-indexed square (`h8`, index 63).
    pub const MAX: Square = Square::H8;

    /// All 64 squares in index order (a1, b1, ..., h1, a2, ..., h8).
    pub const ALL: [Square; Square::COUNT] = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ];

    /// Precomputed single-square bitboards for every square.
    ///
    /// Non-const contexts should prefer using [`Self::MAP`].
    pub const BITBOARDS: [Bitboard; Square::COUNT] = {
        let mut squares = [Bitboard::EMPTY; Square::COUNT];

        let mut square = 0;
        while square < Square::COUNT {
            squares[square] = Bitboard::from_square(Square::from_index(square as u8));
            square += 1;
        }

        squares
    };

    /// A precomputed [`SquareMap`] containing each square bitboard.
    pub const MAP: SquareMap<Bitboard> = SquareMap::from_array(Self::BITBOARDS);

    /// Creates a `Square` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 64`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `Square` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=63`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`Square::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 64);

        Self(index)
    }

    /// Creates a `Square` from a file and rank pair.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::square::Square;
    /// # use gambit_models::location::file::File;
    /// # use gambit_models::location::rank::Rank;
    /// assert_eq!(Square::from_coordinates((File::E, Rank::Four)), Square::E4);
    /// assert_eq!(Square::from_coordinates((File::A, Rank::One)),  Square::A1);
    /// ```
    #[inline]
    pub const fn from_coordinates((file, rank): Coordinates) -> Self {
        let index = file.bits() | (rank.bits() << 3);

        // SAFETY: As (file, rank) are valid, the above calculation only yields values in the range
        // 0..=63 making this safe to call
        unsafe { Self::from_index_unchecked(index) }
    }

    /// Returns the rank (row) of this square.
    #[inline]
    pub const fn rank(self) -> Rank {
        // SAFETY: Any valid square can be transmuted to a valid rank
        unsafe { Rank::from_index_unchecked(self.bits() >> 3) }
    }

    /// Returns the file (column) of this square.
    #[inline]
    pub const fn file(self) -> File {
        // SAFETY: Any valid square can be transmuted to a valid file
        unsafe { File::from_index_unchecked(self.bits() & 7) }
    }

    /// Returns the `(File, Rank)` coordinates of this square.
    ///
    /// The inverse of [`from_coordinates`][Square::from_coordinates].
    #[inline]
    pub const fn coordinates(self) -> Coordinates {
        (self.file(), self.rank())
    }

    /// Returns the square reflected horizontally (same file, opposite rank).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::square::Square;
    /// assert_eq!(Square::E1.flip_rank(), Square::E8);
    /// assert_eq!(Square::A1.flip_rank(), Square::A8);
    /// ```
    #[inline]
    pub const fn flip_rank(self) -> Square {
        Square(self.0 ^ 56)
    }

    /// Returns the square reflected vertically (opposite file, same rank).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::square::Square;
    /// assert_eq!(Square::A1.flip_file(), Square::H1);
    /// assert_eq!(Square::E4.flip_file(), Square::D4);
    /// ```
    #[inline]
    pub const fn flip_file(self) -> Square {
        Square(self.0 ^ 7)
    }

    pub const fn revoked_castling_rights(self) -> CastlingRights {
        match self {
            Square::A1 => CastlingRights::WHITE_QUEENSIDE,
            Square::E1 => CastlingRights::WHITE,
            Square::H1 => CastlingRights::WHITE_KINGSIDE,
            Square::A8 => CastlingRights::BLACK_QUEENSIDE,
            Square::E8 => CastlingRights::BLACK,
            Square::H8 => CastlingRights::BLACK_KINGSIDE,
            _ => CastlingRights::NONE,
        }
    }

    /// Returns the raw `u8` index of this square (`a1` = 0, `h8` = 63).
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Returns the precomputed single-square bitboard for this square.
    #[inline(always)]
    pub const fn bitboard(self) -> Bitboard {
        Self::BITBOARDS[self.bits() as usize]
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank) = self.coordinates();

        write!(f, "{}{}", file.as_char(), rank.as_char())
    }
}

impl From<Square> for u8 {
    #[inline(always)]
    fn from(square: Square) -> u8 {
        square.bits()
    }
}

impl From<Coordinates> for Square {
    #[inline(always)]
    fn from(coordinates: Coordinates) -> Square {
        Square::from_coordinates(coordinates)
    }
}

impl From<Square> for Coordinates {
    #[inline(always)]
    fn from(square: Square) -> Coordinates {
        square.coordinates()
    }
}

impl IntoBitboard for Square {
    #[inline]
    fn into_bitboard(self) -> Bitboard {
        self.bitboard()
    }
}

impl TryFrom<u8> for Square {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 64 {
            // SAFETY: condition above validates precondition
            Ok(unsafe { Square::from_index_unchecked(index) })
        } else {
            Err(TryFromIntError(index))
        }
    }
}

impl FromStr for Square {
    type Err = ParseSquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let file_char = chars
            .next()
            .ok_or(ParseSquareError::InvalidLength(s.len()))?;
        let rank_char = chars
            .next()
            .ok_or(ParseSquareError::InvalidLength(s.len()))?;
        if chars.next().is_some() {
            return Err(ParseSquareError::InvalidLength(s.len()));
        }

        let file =
            File::try_from(file_char).map_err(|_| ParseSquareError::InvalidFile(file_char))?;
        let rank =
            Rank::try_from(rank_char).map_err(|_| ParseSquareError::InvalidRank(rank_char))?;
        Ok(Square::from_coordinates((file, rank)))
    }
}

define_squares! {
    A1 = 0,  B1 = 1,  C1 = 2,  D1 = 3,  E1 = 4,  F1 = 5,  G1 = 6,  H1 = 7,
    A2 = 8,  B2 = 9,  C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}
