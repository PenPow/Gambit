//! Types representing an individual move in a game
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`Move`] | An encoded move, stored in the bits of a u32 |
//! | [`MoveBuilder`] | A builder to help construct a move object without having to perform manual bitops |
//! | [`MoveKind`] | An enum representing the different kinds of move that can be made |

pub mod builder;
pub mod kind;

use crate::error::TryFromIntError;
use crate::location::square::Square;
use crate::moves::builder::MoveBuilder;
use crate::moves::kind::MoveKind;
use crate::piece::piece_type::PieceType;

/// A compact, 32-bit encoding of a single chess move.
///
/// All information about a move is packed into the lower 24 bits of a
/// `u32`. The upper 8 bits are unused and ignored during construction
/// and decoding.
///
/// # Bit layout
///
/// ```text
/// Bits  0– 5  from square        (6 bits, 0–63)
/// Bits  6–11  to square          (6 bits, 0–63)
/// Bits 12–14  move kind          (3 bits, MoveKind 0–7)
/// Bits 15–17  moving piece type  (3 bits, PieceType 0–5)
/// Bits 18–20  captured piece     (3 bits, PieceType 0–5 or null value 7 = None)
/// Bits 21–23  promotion piece    (3 bits, PieceType 0–5 or null value 7 = None)
/// Bits 24–31  unused
/// ```
///
/// # Construction
///
/// Use [`MoveBuilder`]'s named constructors — do not construct `Move`
/// values directly from raw integers except via [`TryFrom<u32>`]:
///
/// ```rust
/// # use gambit_models::moves::builder::MoveBuilder;
/// # use gambit_models::piece::piece_type::PieceType;
/// # use gambit_models::location::square::Square;
/// let mv = MoveBuilder::quiet(Square::E2, Square::E4, PieceType::Pawn).build();
/// assert_eq!(mv.from(), Square::E2);
/// assert_eq!(mv.to(), Square::E4);
/// ```
///
/// # Display
///
/// [`Display`][std::fmt::Display] outputs [UCI long algebraic notation](https://www.chessprogramming.org/UCI):
/// `"e2e4"`, `"e7e8q"`, `"e1g1"`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Move(u32);

impl Move {
    const FROM_SHIFT: u32 = 0;
    const TO_SHIFT: u32 = 6;
    const KIND_SHIFT: u32 = 12;
    const PIECE_SHIFT: u32 = 15;
    const CAPTURE_SHIFT: u32 = 18;
    const PROMOTION_SHIFT: u32 = 21;

    const FROM_MASK: u32 = 0b111111 << Self::FROM_SHIFT;
    const TO_MASK: u32 = 0b111111 << Self::TO_SHIFT;
    const KIND_MASK: u32 = 0b111 << Self::KIND_SHIFT;
    const PIECE_MASK: u32 = 0b111 << Self::PIECE_SHIFT;
    const CAPTURE_MASK: u32 = 0b111 << Self::CAPTURE_SHIFT;
    const PROMOTION_MASK: u32 = 0b111 << Self::PROMOTION_SHIFT;

    /// The null move represents an invalid move.
    ///
    /// The null move has the from square equal the to square. It contains invalid values for all
    /// other fields.
    ///
    /// You can detect the null move using the [`is_null`][Move::is_null] function.
    ///
    /// You should *not* apply a null move to the board, nor attempt to decode any data from it.
    ///
    /// ```rust
    /// # use gambit_models::moves::Move;
    /// assert!(Move::NULL.is_null());
    /// ```
    pub const NULL: Move = Move(7 << Self::CAPTURE_SHIFT | 7 << Self::PROMOTION_SHIFT);

    /// Returns the square the moving piece starts on.
    #[inline]
    pub const fn from(self) -> Square {
        Square::from_index(((self.0 & Self::FROM_MASK) >> Self::FROM_SHIFT) as u8)
    }

    /// Returns the square the moving piece ends on.
    #[inline]
    pub const fn to(self) -> Square {
        Square::from_index(((self.0 & Self::TO_MASK) >> Self::TO_SHIFT) as u8)
    }

    /// Returns the [`MoveKind`], encoding the special properties of this move.
    #[inline]
    pub const fn kind(self) -> MoveKind {
        MoveKind::from_index(((self.0 & Self::KIND_MASK) >> Self::KIND_SHIFT) as u8)
    }

    /// Returns the type of the moving piece.
    #[inline]
    pub const fn piece(self) -> PieceType {
        PieceType::from_index(((self.0 & Self::PIECE_MASK) >> Self::PIECE_SHIFT) as u8)
    }

    /// Returns the type of the captured piece, or `None` for non-captures.
    ///
    /// For en passant, returns `Some(PieceType::Pawn)`.
    #[inline]
    pub const fn captured(self) -> Option<PieceType> {
        let bits = ((self.0 & Self::CAPTURE_MASK) >> Self::CAPTURE_SHIFT) as u8;
        if bits == 7 {
            None
        } else {
            Some(PieceType::from_index(bits))
        }
    }

    /// Returns the piece type promoted to, or `None` for non-promotions.
    ///
    /// Only set for [`MoveKind::Promotion`] and [`MoveKind::PromotionCapture`].
    #[inline]
    pub const fn promotion(self) -> Option<PieceType> {
        let bits = ((self.0 & Self::PROMOTION_MASK) >> Self::PROMOTION_SHIFT) as u8;
        if bits == 7 {
            None
        } else {
            Some(PieceType::from_index(bits))
        }
    }

    /// Returns `true` if this is the [`NULL`][Move::NULL] move.
    #[inline]
    pub const fn is_null(self) -> bool {
        self.from().bits() == self.to().bits()
    }

    /// Returns `true` if this move captures an opponent piece.
    ///
    /// Matches [`Capture`][MoveKind::Capture], [`EnPassant`][MoveKind::EnPassant], and
    /// [`PromotionCapture`][MoveKind::PromotionCapture].
    #[inline]
    pub const fn is_capture(self) -> bool {
        matches!(
            self.kind(),
            MoveKind::Capture | MoveKind::EnPassant | MoveKind::PromotionCapture
        )
    }

    /// Returns `true` if a pawn promotes on this move.
    ///
    /// Matches [`Promotion`][MoveKind::Promotion] and [`PromotionCapture`][MoveKind::PromotionCapture].
    #[inline]
    pub const fn is_promotion(self) -> bool {
        matches!(
            self.kind(),
            MoveKind::Promotion | MoveKind::PromotionCapture
        )
    }

    /// Returns `true` if this is a castling move.
    #[inline]
    pub const fn is_castling(self) -> bool {
        matches!(
            self.kind(),
            MoveKind::KingsideCastle | MoveKind::QueensideCastle
        )
    }

    /// Returns `true` if this is a quiet (non-capture, non-special) move.
    #[inline]
    pub const fn is_quiet(self) -> bool {
        matches!(self.kind(), MoveKind::Quiet)
    }

    /// Returns `true` if this is an en passant capture.
    #[inline]
    pub const fn is_en_passant(self) -> bool {
        matches!(self.kind(), MoveKind::EnPassant)
    }

    /// Returns `true` if this is a double pawn push.
    #[inline]
    pub const fn is_double_pawn_push(self) -> bool {
        matches!(self.kind(), MoveKind::DoublePawnPush)
    }

    /// Returns the raw 32-bit packed representation.
    ///
    /// Only the lower 24 bits are meaningful.
    #[inline(always)]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl Default for Move {
    fn default() -> Self {
        Move::NULL
    }
}

impl From<Move> for u32 {
    #[inline]
    fn from(mv: Move) -> u32 {
        mv.0
    }
}

impl TryFrom<u32> for Move {
    type Error = TryFromIntError<u32>;

    fn try_from(bit_input: u32) -> Result<Self, Self::Error> {
        let bits = bit_input & 0x00FFFFFF;

        let piece = (bits & Self::PIECE_MASK) >> Self::PIECE_SHIFT;
        let capture = (bits & Self::CAPTURE_MASK) >> Self::CAPTURE_SHIFT;
        let promotion = (bits & Self::PROMOTION_MASK) >> Self::PROMOTION_SHIFT;

        if piece > 5 || capture == 6 || promotion == 6 {
            return Err(TryFromIntError(bit_input));
        }

        Ok(Move(bits))
    }
}

impl From<MoveBuilder> for Move {
    #[inline]
    fn from(builder: MoveBuilder) -> Move {
        builder.build()
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("from", &self.from())
            .field("to", &self.to())
            .field("kind", &self.kind())
            .field("piece", &self.piece())
            .field("captured", &self.captured())
            .field("promotion", &self.promotion())
            .finish()
    }
}

// UCI format
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from(), self.to())?;

        if let Some(promotion) = self.promotion() {
            write!(f, "{}", promotion)?;
        }

        Ok(())
    }
}

impl std::fmt::Binary for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#034b}", self.0)
    }
}

impl std::fmt::LowerHex for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl std::fmt::UpperHex for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}
