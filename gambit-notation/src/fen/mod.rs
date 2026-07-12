//! FEN (Forsyth–Edwards Notation) parsing support.
//!
//! Provides the [`FenLike`] trait, which abstracts over FEN-based parsers,
//! and the [`Fen`](crate::fen::parsers::fen::Fen) struct for parsing
//! standard six-field FEN strings into a complete [`Position`](gambit_models::position::Position).
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`error`] | FEN-specific error types |
//! | [`parsers`] | Concrete FEN parser implementations |

use crate::fen::error::FenError;
use gambit_models::location::square::Square;
use gambit_models::mailbox::Mailbox;
use gambit_models::movement::castling::rights::CastlingRights;
use gambit_models::piece::colour::Colour;
use gambit_models::position::{FullmoveNumber, HalfmoveClock, Position};

mod common;
pub mod error;
pub mod parsers;

/// A common interface for FEN-based parsers.
///
/// Types implementing `FenLike` can parse a FEN string and provide
/// access to the six standard fields.
///
/// # Examples
///
/// ```rust
/// # use gambit_notation::fen::FenLike;
/// # use gambit_notation::fen::parsers::Fen;
/// # use gambit_models::piece::colour::Colour;
/// let fen = Fen::parse("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
/// assert_eq!(fen.side_to_move(), Colour::White);
/// ```
pub trait FenLike {
    /// Parse the input string and construct the parser.
    ///
    /// Returns [`FenError`] if the input is not well-formed FEN.
    fn parse(input: &str) -> Result<Self, FenError>
    where
        Self: Sized;

    /// Returns the underlying [`Position`]
    fn position(&self) -> Position;

    /// Returns the board state as a [`Mailbox`].
    fn board(&self) -> Mailbox;

    /// Returns the colour to move next.
    fn side_to_move(&self) -> Colour;

    /// Returns the castling rights still available.
    fn castling_rights(&self) -> CastlingRights;

    /// Returns the en passant target square, if any.
    ///
    /// `None` indicates no en passant capture is possible.
    fn en_passant(&self) -> Option<Square>;

    /// Returns the halfmove clock.
    fn halfmove_clock(&self) -> HalfmoveClock;

    /// Returns the fullmove number
    fn fullmove_number(&self) -> FullmoveNumber;
}
