//! Types representing pieces and their colours
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`colour`] | Individual piece colours |
//! | [`piece_type`] | Individual piece types |
//! | [`map`] | Maps allowing indexing of generic types using colours and piece types |

pub mod colour;
pub mod map;
pub mod piece_type;

use crate::error::{TryFromCharError, TryFromIntError};
use crate::piece::colour::Colour;
use crate::piece::piece_type::PieceType;
use std::fmt::Write;

/// A compact one-byte encoding of a chess piece, or the absence of one.
///
/// The four lower bits encode the piece:
///
/// ```text
/// Bit  3    : colour  (0 = White, 1 = Black)
/// Bits 2–0  : piece type (0 = Pawn … 5 = King)
/// Value 0b00001111 (15) : NONE piece
/// ```
///
/// All other bit patterns are invalid
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece(u8);

impl Piece {
    pub const WHITE_PAWN: Piece = Piece::new(PieceType::Pawn, Colour::White);
    pub const WHITE_KNIGHT: Piece = Piece::new(PieceType::Knight, Colour::White);
    pub const WHITE_BISHOP: Piece = Piece::new(PieceType::Bishop, Colour::White);
    pub const WHITE_ROOK: Piece = Piece::new(PieceType::Rook, Colour::White);
    pub const WHITE_QUEEN: Piece = Piece::new(PieceType::Queen, Colour::White);
    pub const WHITE_KING: Piece = Piece::new(PieceType::King, Colour::White);
    pub const BLACK_PAWN: Piece = Piece::new(PieceType::Pawn, Colour::Black);
    pub const BLACK_KNIGHT: Piece = Piece::new(PieceType::Knight, Colour::Black);
    pub const BLACK_BISHOP: Piece = Piece::new(PieceType::Bishop, Colour::Black);
    pub const BLACK_ROOK: Piece = Piece::new(PieceType::Rook, Colour::Black);
    pub const BLACK_QUEEN: Piece = Piece::new(PieceType::Queen, Colour::Black);
    pub const BLACK_KING: Piece = Piece::new(PieceType::King, Colour::Black);

    /// A null piece - not a real piece.
    ///
    /// [`is_none`][Piece::is_none] returns `true` for this value.
    /// [`piece_type`][Piece::piece_type] and [`colour`][Piece::colour]
    /// both return `None`.
    pub const NONE: Piece = Piece(15);

    const COLOUR_MASK: u8 = 0b1000;

    const PIECE_TYPE_MASK: u8 = 0b0111;

    /// Packs a piece type and colour into a `Piece`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::piece::{Piece, colour::Colour, piece_type::PieceType};
    /// let piece = Piece::new(PieceType::Queen, Colour::Black);
    /// assert_eq!(piece, Piece::BLACK_QUEEN);
    /// ```
    #[inline(always)]
    pub const fn new(piece_type: PieceType, colour: Colour) -> Self {
        Piece(piece_type as u8 | ((colour as u8) << 3))
    }

    /// Creates a `Piece` from a raw bit pattern.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `bits` is not a valid piece encoding.
    /// Valid values are `0–5` (White pieces), `8–13` (Black pieces),
    /// and `15` ([`NONE`][Piece::NONE]).
    ///
    /// In release builds, passing an invalid value is undefined behaviour.
    /// Prefer [`TryFrom<u8>`] for input that may be out of range.
    #[inline(always)]
    pub const fn from_bits(bits: u8) -> Self {
        debug_assert!(bits <= 15 && (bits == 15 || (bits & 0b0111) < 6));

        // SAFETY: bits are asserted to be valid
        unsafe { Piece::from_bits_unchecked(bits) }
    }

    /// Creates a `Piece` from a raw bit pattern without any validation.
    ///
    /// # Safety
    ///
    /// This function is only safe to call when the caller has manually validated the bits.
    ///
    /// UB can be obtained if bits is invalid. Valid values for bits are:
    /// - `0-5` (White pieces)
    /// - `8-13` (Black pieces)
    /// - `15` ([`NONE`][Piece::NONE])
    #[inline(always)]
    const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        Piece(bits)
    }

    /// Returns the [`PieceType`], or `None` if this is [`NONE`][Piece::NONE].
    #[inline]
    pub const fn piece_type(self) -> Option<PieceType> {
        if self.is_none() {
            None
        } else {
            Some(match self.bits() & Piece::PIECE_TYPE_MASK {
                0 => PieceType::Pawn,
                1 => PieceType::Knight,
                2 => PieceType::Bishop,
                3 => PieceType::Rook,
                4 => PieceType::Queen,
                5 => PieceType::King,
                _ => unreachable!(),
            })
        }
    }

    /// Returns the [`Colour`], or `None` if this is [`NONE`][Piece::NONE].
    #[inline]
    pub const fn colour(self) -> Option<Colour> {
        if self.is_none() {
            None
        } else {
            Some(if self.bits() & Piece::COLOUR_MASK == 0 {
                Colour::White
            } else {
                Colour::Black
            })
        }
    }

    /// Returns `true` if this piece is [`NONE`][Piece::NONE]
    #[inline]
    pub const fn is_none(self) -> bool {
        self.bits() == Piece::NONE.bits()
    }

    /// Returns `true` if this is a real piece (is not [`NONE`][Piece::NONE]).
    #[inline]
    pub const fn is_some(self) -> bool {
        !self.is_none()
    }

    /// Returns `true` if this piece slides along rays (Bishop, Rook, or Queen).
    ///
    /// Returns `false` for [`NONE`][Piece::NONE].
    ///
    /// See also [`PieceType::is_sliding`].
    #[inline]
    pub const fn is_sliding(self) -> bool {
        match self.piece_type() {
            Some(piece_type) => piece_type.is_sliding(),
            None => false,
        }
    }

    /// Returns a copy of this piece with the colour flipped.
    ///
    /// Returns `self` unchanged if this is [`NONE`][Piece::NONE].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::piece::Piece;
    /// assert_eq!(Piece::WHITE_PAWN.flip_colour(), Piece::BLACK_PAWN);
    /// assert_eq!(Piece::NONE.flip_colour(), Piece::NONE);
    /// ```
    #[inline]
    pub const fn flip_colour(self) -> Piece {
        if self.is_none() {
            self
        } else {
            Piece(self.bits() ^ Piece::COLOUR_MASK)
        }
    }

    /// Returns a copy of this piece with the colour set to `colour`.
    ///
    /// Returns `self` unchanged if this is [`NONE`][Piece::NONE].
    #[inline]
    pub const fn with_colour(self, colour: Colour) -> Piece {
        if self.is_none() {
            self
        } else {
            let bits = (self.bits() & !Self::COLOUR_MASK) | ((colour as u8) << 3);

            Piece(bits)
        }
    }

    /// Returns the FEN character for this piece.
    ///
    /// Uppercase for White pieces, lowercase for Black pieces.
    /// Returns `'?'` for [`NONE`][Piece::NONE].
    ///
    /// | Piece        | Char |
    /// |--------------|------|
    /// | White King   | `K`  |
    /// | White Queen  | `Q`  |
    /// | Black Queen  | `q`  |
    /// | ...          | ...  |
    /// | None         | `?`  |
    #[inline]
    pub const fn as_char(self) -> char {
        if self.is_none() {
            '?'
        } else {
            let character = self.piece_type().unwrap().as_char();
            match self.colour().unwrap() {
                Colour::White => character.to_ascii_uppercase(),
                Colour::Black => character,
            }
        }
    }

    /// Returns the raw 4-bit packed representation.
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self.0
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::NONE
    }
}

impl From<Piece> for u8 {
    #[inline(always)]
    fn from(piece: Piece) -> u8 {
        piece.bits()
    }
}

impl TryFrom<u8> for Piece {
    type Error = TryFromIntError<u8>;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if bits <= 15 && (bits == 15 || (bits & 0b0111) < 6) {
            // SAFETY: Condition above validates bits
            Ok(unsafe { Piece::from_bits_unchecked(bits) })
        } else {
            Err(TryFromIntError(bits))
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = TryFromCharError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        let piece_type = PieceType::try_from(character)?;
        let colour = Colour::from(character);

        Ok(Piece::new(piece_type, colour))
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}
