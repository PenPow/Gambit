use crate::bitboard::Bitboard;
use crate::error::{TryFromCharError, TryFromIntError};
use crate::location::map::rank::RankMap;
use crate::traits::IntoBitboard;
use std::fmt::Write;

/// A rank of the chessboard, from `1` to `8`.
///
/// # Ordering
///
/// The derived [`Ord`] follows board convention: `One < Two < ... < Eight`.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::location::rank::Rank;
/// assert_eq!(Rank::Four.as_char(), '4');
/// assert_eq!(Rank::One.distance(Rank::Eight), 7);
/// assert_eq!(Rank::Three.offset(3), Some(Rank::Six));
/// ```
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    /// Total number of ranks on the board.
    pub const COUNT: usize = 8;

    /// The lowest rank (`One`).
    pub const MIN: Rank = Rank::One;

    /// The highest rank (`Eight`).
    pub const MAX: Rank = Rank::Eight;

    /// All 8 ranks in order from `One` to `Eight`.
    pub const ALL: [Rank; Rank::COUNT] = [
        Rank::One,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
    ];

    /// Precomputed bitboard mask for each rank.
    ///
    /// Non-const contexts should prefer using [`Self::MAP`].
    pub const BITBOARDS: [Bitboard; Rank::COUNT] = {
        let mut ranks = [Bitboard::EMPTY; Rank::COUNT];

        let mut rank = 0;
        while rank < Rank::COUNT {
            // for is not stable in const functions yet
            ranks[rank] = Bitboard::from_rank(Rank::from_index(rank as u8));
            rank += 1;
        }

        ranks
    };

    /// A precomputed [`RankMap`] containing each square bitboard.
    pub const MAP: RankMap<Bitboard> = RankMap::from_array(Self::BITBOARDS);

    /// Creates a `Rank` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 8`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `Rank` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=7`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`Rank::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 8);

        // SAFETY: index is in range 0..=7 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }

    /// Returns the digit for this rank (`'1'`..=`'8'`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::rank::Rank;
    /// assert_eq!(Rank::One.as_char(), '1');
    /// assert_eq!(Rank::Four.as_char(), '4');
    /// assert_eq!(Rank::Eight.as_char(), '8');
    /// ```
    #[inline]
    pub const fn as_char(self) -> char {
        (b'1' + (self as u8)) as char
    }

    /// Returns the absolute distance in rank between `self` and `rhs`.
    ///
    /// Always returns a value in `0..=7`. Order does not matter:
    /// `a.distance(b) == b.distance(a)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::rank::Rank;
    /// assert_eq!(Rank::One.distance(Rank::Eight), 7);
    /// assert_eq!(Rank::Five.distance(Rank::Three), 2);
    /// assert_eq!(Rank::Four.distance(Rank::Four), 0);
    /// ```
    #[inline]
    pub const fn distance(self, rhs: Rank) -> u8 {
        (self as u8).abs_diff(rhs as u8)
    }

    /// Returns the rank `rhs` steps away, or `None` if off the board.
    ///
    /// Positive `rhs` moves toward `Eight`; negative moves toward `One`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::rank::Rank;
    /// assert_eq!(Rank::Five.offset(2),  Some(Rank::Seven));
    /// assert_eq!(Rank::Five.offset(-4), Some(Rank::One));
    /// assert_eq!(Rank::Eight.offset(1),  None);
    /// assert_eq!(Rank::One.offset(-1), None);
    /// ```
    #[inline]
    pub const fn offset(self, rhs: i8) -> Option<Rank> {
        let index = self as i8 + rhs;

        #[allow(clippy::manual_range_contains)]
        if index < 0 || index > 7 {
            return None;
        }

        // SAFETY: Check above validates index
        Some(unsafe { Self::from_index_unchecked(index as u8) })
    }

    /// Returns the raw `u8` index of this rank (`One` = 0, `Eight` = 7).
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }

    /// Returns the precomputed bitboard mask for this rank.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::rank::Rank;
    /// assert_eq!(Rank::One.bitboard(), Bitboard::from_rank(Rank::One));
    /// assert_eq!(Rank::One.bitboard().bits(), 0x0000_0000_0000_00FF);
    /// ```
    #[inline(always)]
    pub const fn bitboard(self) -> Bitboard {
        Self::BITBOARDS[self as usize]
    }
}

impl From<Rank> for u8 {
    #[inline(always)]
    fn from(rank: Rank) -> u8 {
        rank.bits()
    }
}

impl IntoBitboard for Rank {
    #[inline]
    fn into_bitboard(self) -> Bitboard {
        self.bitboard()
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl TryFrom<char> for Rank {
    type Error = TryFromCharError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '1'..='8' => Ok(Rank::from_index(character as u8 - b'1')),
            _ => Err(TryFromCharError(character)),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 8 {
            Ok(Rank::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}
