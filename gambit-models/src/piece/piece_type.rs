use crate::error::{TryFromCharError, TryFromIntError};
use std::fmt::Write;

// One of the six chess piece types, independent of colour.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::piece::piece_type::PieceType;
/// assert_eq!(PieceType::Queen.as_char(), 'q');
/// assert!(PieceType::Rook.is_sliding());
/// assert!(!PieceType::Knight.is_sliding());
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Total number of piece types.
    pub const COUNT: usize = 6;

    /// Number of legal pawn promotion targets (Knight, Bishop, Rook, Queen).
    pub const PROMOTION_OPTION_COUNT: usize = 4;

    /// The lowest-indexed piece type (`Pawn`).
    pub const MIN: PieceType = PieceType::Pawn;

    /// The highest-indexed piece type (`King`).
    pub const MAX: PieceType = PieceType::King;

    /// All six piece types in index order.
    pub const ALL: [PieceType; PieceType::COUNT] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];

    /// The four legal pawn promotion targets.
    pub const PROMOTION_TARGETS: [PieceType; PieceType::PROMOTION_OPTION_COUNT] = [
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
    ];

    /// Creates a `PieceType` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 6`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `PieceType` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=5`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`PieceType::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 6);

        // SAFETY: index is in range 0..=5 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }

    /// Returns the lowercase FEN/UCI character for this piece type.
    ///
    /// | Type   | Char |
    /// |--------|------|
    /// | Pawn   | `p`  |
    /// | Knight | `n`  |
    /// | Bishop | `b`  |
    /// | Rook   | `r`  |
    /// | Queen  | `q`  |
    /// | King   | `k`  |
    ///
    /// For the colour-sensitive FEN character (uppercase for White,
    /// lowercase for Black), use [`Piece::as_char`][super::Piece::as_char].
    #[inline]
    pub const fn as_char(self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

    /// Parses a FEN/UCI piece character, accepting both cases.
    ///
    /// `'p'` and `'P'` both map to [`Pawn`][PieceType::Pawn], and so on.
    /// Returns `None` for any character that is not a recognised piece letter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::piece::piece_type::PieceType;
    /// assert_eq!(PieceType::from_char('Q'), Some(PieceType::Queen));
    /// assert_eq!(PieceType::from_char('q'), Some(PieceType::Queen));
    /// assert_eq!(PieceType::from_char('x'), None);
    /// ```
    #[inline]
    pub const fn from_char(character: char) -> Option<Self> {
        match character.to_ascii_lowercase() {
            'p' => Some(PieceType::Pawn),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            'r' => Some(PieceType::Rook),
            'q' => Some(PieceType::Queen),
            'k' => Some(PieceType::King),
            _ => None,
        }
    }

    /// Returns `true` if this piece type moves along rays (bishop, rook, or queen).
    #[inline]
    pub const fn is_sliding(self) -> bool {
        matches!(self, PieceType::Bishop | PieceType::Rook | PieceType::Queen)
    }

    /// Returns the raw `u8` index of this piece type.
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

impl From<PieceType> for u8 {
    #[inline(always)]
    fn from(piece_type: PieceType) -> u8 {
        piece_type.bits()
    }
}

impl TryFrom<u8> for PieceType {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 6 {
            Ok(PieceType::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}

impl TryFrom<char> for PieceType {
    type Error = TryFromCharError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        PieceType::from_char(character).ok_or(TryFromCharError(character))
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}
