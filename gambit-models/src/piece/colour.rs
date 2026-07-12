use crate::error::TryFromIntError;
use crate::location::rank::Rank;

/// The colour of a chess piece — White or Black.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::piece::colour::Colour;
/// assert_eq!(!Colour::White, Colour::Black);
/// assert_eq!(Colour::Black.other(), Colour::White);
/// assert_eq!(Colour::from('P'), Colour::White);
/// assert_eq!(Colour::from('q'), Colour::Black);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    /// Total number of colours.
    pub const COUNT: usize = 2;

    /// The first colour in index order (`White`).
    pub const MIN: Colour = Colour::White;

    /// The last colour in index order (`Black`).
    pub const MAX: Colour = Colour::Black;

    /// Both colours in index order: `[White, Black]`.
    pub const ALL: [Colour; Colour::COUNT] = [Colour::White, Colour::Black];

    /// Creates a `Colour` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 2`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `Colour` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=1`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`Colour::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 2);

        // SAFETY: index is in range 0..=1 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }

    /// Returns the opposite colour.
    ///
    /// Equivalent to the [`Not`][std::ops::Not] implementation:
    /// `colour.other() == !colour`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::piece::colour::Colour;
    /// assert_eq!(Colour::White.other(), Colour::Black);
    /// assert_eq!(Colour::Black.other(), Colour::White);
    /// ```
    #[inline]
    pub const fn other(self) -> Colour {
        unsafe { std::mem::transmute((self as u8) ^ 1) }
    }

    /// Returns the "fourth rank" from this colour's perspective.
    ///
    /// White's fourth rank is [`Rank::Four`];
    /// Black's fourth rank is [`Rank::Five`].
    #[inline]
    pub const fn fourth_rank(self) -> Rank {
        match self {
            Colour::White => Rank::Four,
            Colour::Black => Rank::Five,
        }
    }

    /// Returns the back rank for this colour — where pawns promote.
    ///
    /// White promotes on [`Rank::Eight`];
    /// Black promotes on [`Rank::One`].
    #[inline]
    pub const fn promotion_rank(self) -> Rank {
        match self {
            Colour::White => Rank::Eight,
            Colour::Black => Rank::One,
        }
    }

    /// Returns the raw `u8` index
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

impl From<Colour> for u8 {
    #[inline(always)]
    fn from(colour: Colour) -> u8 {
        colour.bits()
    }
}

impl TryFrom<u8> for Colour {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 2 {
            Ok(Colour::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}

impl From<char> for Colour {
    /// Determines colour from a FEN piece character.
    ///
    /// Uppercase ASCII letters map to [`White`][Colour::White];
    /// all other characters (including lowercase letters, digits, and
    /// punctuation) map to [`Black`][Colour::Black].
    fn from(character: char) -> Self {
        if character.is_ascii_uppercase() {
            Colour::White
        } else {
            Colour::Black
        }
    }
}

impl std::ops::Not for Colour {
    type Output = Colour;

    #[inline]
    fn not(self) -> Colour {
        self.other()
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::White => f.write_str("White"),
            Colour::Black => f.write_str("Black"),
        }
    }
}
