use crate::error::TryFromIntError;

/// The specific kind of chess move
///
/// Every [`Move`][crate::moves::Move] has exactly one `MoveKind`.
/// The kind determines which fields are meaningful and which special rules
/// apply when the move is made on a board.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::moves::kind::MoveKind;
/// assert!(MoveKind::Capture.is_capture());
/// assert!(MoveKind::PromotionCapture.is_capture());
/// assert!(MoveKind::PromotionCapture.is_promotion());
/// assert!(!MoveKind::Quiet.is_capture());
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveKind {
    /// A non-capturing move with no special properties.
    Quiet,
    /// A pawn advances two squares from its starting rank.
    DoublePawnPush,
    /// The king castles to the kingside (short castling).
    KingsideCastle,
    /// The king castles to the queenside (long castling).
    QueensideCastle,
    /// A piece captures an opponent piece on the destination square.
    Capture,
    /// A pawn captures an adjacent pawn that just made a double push.
    EnPassant,
    /// A pawn reaches the back rank without capturing.
    Promotion,
    /// A pawn reaches the back rank while capturing.
    PromotionCapture,
}

impl MoveKind {
    /// Total number of move kinds.
    pub const COUNT: usize = 8;

    /// All move kinds in index order.
    pub const ALL: [MoveKind; MoveKind::COUNT] = [
        MoveKind::Quiet,
        MoveKind::DoublePawnPush,
        MoveKind::KingsideCastle,
        MoveKind::QueensideCastle,
        MoveKind::Capture,
        MoveKind::EnPassant,
        MoveKind::Promotion,
        MoveKind::PromotionCapture,
    ];

    /// Creates a `MoveKind` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 8`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `MoveKind` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=7`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`MoveKind::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 8);

        // SAFETY: index is in range 0..=7 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }

    /// Returns `true` if this kind involves capturing an opponent piece.
    ///
    /// Matches [`Capture`][MoveKind::Capture], [`EnPassant`][MoveKind::EnPassant], and
    /// [`PromotionCapture`][MoveKind::PromotionCapture].
    #[inline]
    pub const fn is_capture(self) -> bool {
        matches!(
            self,
            MoveKind::Capture | MoveKind::EnPassant | MoveKind::PromotionCapture
        )
    }

    /// Returns `true` if this kind involves pawn promotion.
    #[inline]
    pub const fn is_promotion(self) -> bool {
        matches!(self, MoveKind::Promotion | MoveKind::PromotionCapture)
    }

    /// Returns `true` if this kind is a castling move.
    #[inline]
    pub const fn is_castling(self) -> bool {
        matches!(self, MoveKind::KingsideCastle | MoveKind::QueensideCastle)
    }

    /// Returns `true` if this is a quiet move (no capture, no special rule).
    #[inline]
    pub const fn is_quiet(self) -> bool {
        matches!(self, MoveKind::Quiet)
    }

    /// Returns `true` if this is an en passant capture.
    #[inline]
    pub const fn is_en_passant(self) -> bool {
        matches!(self, MoveKind::EnPassant)
    }

    /// Returns `true` if this is a double pawn push.
    #[inline]
    pub const fn is_double_pawn_push(self) -> bool {
        matches!(self, MoveKind::DoublePawnPush)
    }

    /// Returns the raw `u8` index of this kind.
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

impl From<MoveKind> for u8 {
    #[inline]
    fn from(kind: MoveKind) -> u8 {
        kind.bits()
    }
}

impl TryFrom<u8> for MoveKind {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 8 {
            Ok(MoveKind::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}

impl std::fmt::Display for MoveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveKind::Quiet => f.write_str("Quiet"),
            MoveKind::DoublePawnPush => f.write_str("Double Pawn Push"),
            MoveKind::KingsideCastle => f.write_str("Kingside Castle"),
            MoveKind::QueensideCastle => f.write_str("Queenside Castle"),
            MoveKind::Capture => f.write_str("Capture"),
            MoveKind::EnPassant => f.write_str("En Passant"),
            MoveKind::Promotion => f.write_str("Promotion"),
            MoveKind::PromotionCapture => f.write_str("Promotion Capture"),
        }
    }
}
