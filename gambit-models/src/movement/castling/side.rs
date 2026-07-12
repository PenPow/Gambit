use crate::bitboard::Bitboard;
use crate::error::TryFromIntError;
use crate::location::square::Square;
use crate::movement::castling::rights::CastlingRights;
use crate::piece::colour::Colour;
use std::fmt;

/// Which side of the board a castling move is on.
///
/// Encodes the two possible castling directions for either colour.
/// Use [`CastlingSide::Kingside`] for short castling (king moves to g-file)
/// and [`CastlingSide::Queenside`] for long castling (king moves to c-file).
///
/// # Examples
///
/// ```rust
/// # use gambit_models::movement::castling::side::CastlingSide;
/// # use gambit_models::piece::colour::Colour;
/// # use gambit_models::location::square::Square;
/// assert_eq!(CastlingSide::Kingside.king_to(Colour::White), Square::G1);
/// assert_eq!(!CastlingSide::Kingside, CastlingSide::Queenside);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

impl CastlingSide {
    /// Total number of castling sides.
    pub const COUNT: usize = 2;

    /// Both castling sides in declaration order.
    pub const ALL: [CastlingSide; CastlingSide::COUNT] =
        [CastlingSide::Kingside, CastlingSide::Queenside];

    /// Creates a `CastlingSide` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 2`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `CastlingSide` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=1`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`CastlingSide::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 2);

        // SAFETY: index is in range 0..=1 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }
    /// The square the king starts on before castling.
    ///
    /// Always `e1` for White and `e8` for Black, regardless of side.
    #[inline]
    pub const fn king_from(self, colour: Colour) -> Square {
        match colour {
            Colour::White => Square::E1,
            Colour::Black => Square::E8,
        }
    }

    /// The square the king occupies after castling.
    ///
    /// | Side      | White | Black |
    /// |-----------|-------|-------|
    /// | Kingside  | g1    | g8    |
    /// | Queenside | c1    | c8    |
    #[inline]
    pub const fn king_to(self, colour: Colour) -> Square {
        match (self, colour) {
            (CastlingSide::Kingside, Colour::White) => Square::G1,
            (CastlingSide::Queenside, Colour::White) => Square::C1,
            (CastlingSide::Kingside, Colour::Black) => Square::G8,
            (CastlingSide::Queenside, Colour::Black) => Square::C8,
        }
    }

    /// The square the rook starts on before castling.
    ///
    /// | Side      | White | Black |
    /// |-----------|-------|-------|
    /// | Kingside  | h1    | h8    |
    /// | Queenside | a1    | a8    |
    #[inline]
    pub const fn rook_from(self, colour: Colour) -> Square {
        match (self, colour) {
            (CastlingSide::Kingside, Colour::White) => Square::H1,
            (CastlingSide::Queenside, Colour::White) => Square::A1,
            (CastlingSide::Kingside, Colour::Black) => Square::H8,
            (CastlingSide::Queenside, Colour::Black) => Square::A8,
        }
    }

    /// The square the rook occupies after castling.
    ///
    /// | Side      | White | Black |
    /// |-----------|-------|-------|
    /// | Kingside  | f1    | f8    |
    /// | Queenside | d1    | d8    |
    #[inline]
    pub const fn rook_to(self, colour: Colour) -> Square {
        match (self, colour) {
            (CastlingSide::Kingside, Colour::White) => Square::F1,
            (CastlingSide::Queenside, Colour::White) => Square::D1,
            (CastlingSide::Kingside, Colour::Black) => Square::F8,
            (CastlingSide::Queenside, Colour::Black) => Square::D8,
        }
    }

    /// The squares that must be unoccupied for castling to be legal.
    ///
    /// These are all squares strictly between the king and the rook.
    /// On the queenside this includes `b1`/`b8`, which the king does
    /// not pass through but which must be empty.
    ///
    /// See also [`safe_squares`][CastlingSide::safe_squares], which is a
    /// strict subset of this on the queenside.
    #[inline]
    pub const fn empty_squares(self, colour: Colour) -> Bitboard {
        match (self, colour) {
            // F1, G1
            (CastlingSide::Kingside, Colour::White) => Bitboard::new(0x0000000000000060),
            // B1, C1, D1
            (CastlingSide::Queenside, Colour::White) => Bitboard::new(0x000000000000000E),
            // F8, G8
            (CastlingSide::Kingside, Colour::Black) => Bitboard::new(0x6000000000000000),
            // B8, C8, D8
            (CastlingSide::Queenside, Colour::Black) => Bitboard::new(0x0E00000000000000),
        }
    }

    /// The squares the king passes through that must not be attacked.
    ///
    /// Includes the king's starting square, any intermediate square, and
    /// the king's destination square. On the queenside this does **not**
    /// include `b1`/`b8` (that square is in [`empty_squares`][CastlingSide::empty_squares] but the
    /// king does not traverse it).
    #[inline]
    pub const fn safe_squares(self, colour: Colour) -> Bitboard {
        match (self, colour) {
            // E1, F1, G1
            (CastlingSide::Kingside, Colour::White) => Bitboard::new(0x0000000000000070),
            // C1, D1, E1
            (CastlingSide::Queenside, Colour::White) => Bitboard::new(0x000000000000001C),
            // E8, F8, G8
            (CastlingSide::Kingside, Colour::Black) => Bitboard::new(0x7000000000000000),
            // C8, D8, E8
            (CastlingSide::Queenside, Colour::Black) => Bitboard::new(0x1C00000000000000),
        }
    }

    /// Returns the [`CastlingRights`] flag corresponding to this side and colour.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::movement::castling::side::CastlingSide;
    /// # use gambit_models::movement::castling::rights::CastlingRights;
    /// # use gambit_models::piece::colour::Colour;
    /// assert_eq!(
    ///     CastlingSide::Kingside.rights(Colour::White),
    ///     CastlingRights::WHITE_KINGSIDE,
    /// );
    /// ```
    #[inline]
    pub const fn rights(self, colour: Colour) -> CastlingRights {
        match (self, colour) {
            (CastlingSide::Kingside, Colour::White) => CastlingRights::WHITE_KINGSIDE,
            (CastlingSide::Queenside, Colour::White) => CastlingRights::WHITE_QUEENSIDE,
            (CastlingSide::Kingside, Colour::Black) => CastlingRights::BLACK_KINGSIDE,
            (CastlingSide::Queenside, Colour::Black) => CastlingRights::BLACK_QUEENSIDE,
        }
    }

    /// Returns the raw index (`0` for Kingside, `1` for Queenside).
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

impl From<CastlingSide> for u8 {
    #[inline]
    fn from(side: CastlingSide) -> u8 {
        side.bits()
    }
}

impl TryFrom<u8> for CastlingSide {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 2 {
            Ok(CastlingSide::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}

impl std::ops::Not for CastlingSide {
    type Output = CastlingSide;

    #[inline]
    fn not(self) -> CastlingSide {
        match self {
            CastlingSide::Kingside => CastlingSide::Queenside,
            CastlingSide::Queenside => CastlingSide::Kingside,
        }
    }
}

impl fmt::Display for CastlingSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastlingSide::Kingside => f.write_str("Kingside"),
            CastlingSide::Queenside => f.write_str("Queenside"),
        }
    }
}
