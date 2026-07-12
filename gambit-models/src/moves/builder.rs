use crate::location::square::Square;
use crate::movement::castling::side::CastlingSide;
use crate::moves::Move;
use crate::moves::kind::MoveKind;
use crate::piece::piece_type::PieceType;

/// A constructor for [`Move`] values.
///
/// Each named constructor creates a fully-specified move of a particular
/// kind. Call [`build`][MoveBuilder::build] to obtain the compact [`Move`].
///
/// ```rust
/// # use gambit_models::moves::builder::MoveBuilder;
/// # use gambit_models::piece::piece_type::PieceType;
/// # use gambit_models::location::square::Square;
/// let mv = MoveBuilder::capture(
///     Square::E4, Square::D5,
///     PieceType::Pawn, PieceType::Knight,
/// ).build();
/// assert!(mv.is_capture());
/// assert_eq!(mv.captured(), Some(PieceType::Knight));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MoveBuilder {
    from: Square,
    to: Square,
    kind: MoveKind,
    piece: PieceType,
    captured: Option<PieceType>,
    promotion: Option<PieceType>,
}

impl MoveBuilder {
    /// A non-capturing, non-special move.
    pub const fn quiet(from: Square, to: Square, piece: PieceType) -> Self {
        Self {
            from,
            to,
            piece,
            kind: MoveKind::Quiet,
            captured: None,
            promotion: None,
        }
    }

    /// A pawn advancing two squares from its starting rank.
    pub const fn double_pawn_push(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            kind: MoveKind::DoublePawnPush,
            captured: None,
            promotion: None,
        }
    }

    /// A move that captures an opponent piece on the destination square.
    pub const fn capture(from: Square, to: Square, piece: PieceType, captured: PieceType) -> Self {
        Self {
            from,
            to,
            piece,
            kind: MoveKind::Capture,
            captured: Some(captured),
            promotion: None,
        }
    }

    /// A pawn capturing en passant.
    pub const fn en_passant(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            kind: MoveKind::EnPassant,
            captured: Some(PieceType::Pawn),
            promotion: None,
        }
    }

    /// A castling move.
    ///
    /// `from` and `to` are the king's squares.
    pub const fn castle(from: Square, to: Square, side: CastlingSide) -> Self {
        Self {
            from,
            to,
            piece: PieceType::King,
            kind: match side {
                CastlingSide::Kingside => MoveKind::KingsideCastle,
                CastlingSide::Queenside => MoveKind::QueensideCastle,
            },
            captured: None,
            promotion: None,
        }
    }

    /// A pawn reaching the back rank and promoting without capturing.
    ///
    /// `promotion` must be one of [`PieceType::PROMOTION_TARGETS`].
    /// Other promotions are illegal but are not validated.
    pub const fn promotion(from: Square, to: Square, promotion: PieceType) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            kind: MoveKind::Promotion,
            captured: None,
            promotion: Some(promotion),
        }
    }

    /// A pawn reaching the back rank while capturing, then promoting.
    ///
    /// `promotion` must be one of [`PieceType::PROMOTION_TARGETS`].
    /// Other promotions are illegal but are not validated.
    pub const fn promotion_capture(
        from: Square,
        to: Square,
        promotion: PieceType,
        captured: PieceType,
    ) -> Self {
        Self {
            from,
            to,
            piece: PieceType::Pawn,
            kind: MoveKind::PromotionCapture,
            captured: Some(captured),
            promotion: Some(promotion),
        }
    }

    /// Packs all fields into a [`Move`].
    #[inline]
    pub const fn build(self) -> Move {
        Move(
            (self.from.bits() as u32) << Move::FROM_SHIFT
                | (self.to.bits() as u32) << Move::TO_SHIFT
                | (self.kind as u32) << Move::KIND_SHIFT
                | (self.piece.bits() as u32) << Move::PIECE_SHIFT
                | (match self.captured {
                    Some(piece) => piece.bits() as u32,
                    None => 7u32,
                }) << Move::CAPTURE_SHIFT
                | (match self.promotion {
                    Some(piece) => piece.bits() as u32,
                    None => 7u32,
                }) << Move::PROMOTION_SHIFT,
        )
    }
}
