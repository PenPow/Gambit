//! Implementations of a [mailbox](https://www.chessprogramming.org/Mailbox) and its iterators
//! allowing for the representation of a chess board as an array.

use crate::location::file::File;
use crate::location::map::square::SquareMap;
use crate::location::rank::Rank;
use crate::location::square::Square;
use crate::piece::Piece;
#[cfg(feature = "colored")]
use crate::piece::colour::Colour;
#[cfg(feature = "colored")]
use colored::Colorize;
use std::fmt;
use std::fmt::Formatter;
use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

/// A square-indexed array of [`Piece`] values representing board occupancy.
///
/// Backed by `[Piece; 64]`, where index `n` corresponds to [`Square`] with
/// index `n`. Empty squares hold [`Piece::NONE`].
///
/// # Indexing
///
/// Indexing by [`Square`] is provided via [`Index`] and [`IndexMut`]:
///
/// ```rust
/// # use gambit_models::mailbox::Mailbox;
/// # use gambit_models::piece::Piece;
/// # use gambit_models::location::square::Square;
/// let mut mailbox = Mailbox::default();
/// mailbox[Square::E4] = Piece::WHITE_PAWN;
/// assert_eq!(mailbox[Square::E4], Piece::WHITE_PAWN);
/// assert_eq!(mailbox[Square::E5], Piece::NONE);
/// ```
///
/// # Iteration
///
/// Two iterators are available:
/// - [`iter`][Mailbox::iter] / [`IntoIterator`] — yields all 64 `(Square, Piece)` pairs,
///   including empty squares.
/// - [`iter_pieces`][Mailbox::iter_pieces] — yields only occupied squares, skipping
///   [`Piece::NONE`].
///
/// # Display
///
/// [`Display`][std::fmt::Display] prints an 8×8 ASCII board from rank 8 down to rank 1.
/// With the `colored` feature, white pieces are bright white and black pieces are bright blue.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Mailbox(SquareMap<Piece>);

impl Mailbox {
    /// The standard chess starting position.
    ///
    /// ```rust
    /// # use gambit_models::mailbox::Mailbox;
    /// # use gambit_models::piece::Piece;
    /// # use gambit_models::location::square::Square;
    /// assert_eq!(Mailbox::STARTING_POSITION[Square::E1], Piece::WHITE_KING);
    /// assert_eq!(Mailbox::STARTING_POSITION[Square::E8], Piece::BLACK_KING);
    /// assert_eq!(Mailbox::STARTING_POSITION[Square::E4], Piece::NONE);
    /// ```
    pub const STARTING_POSITION: Mailbox = {
        let mut array = [Piece::NONE; Square::COUNT];

        array[Square::A1.bits() as usize] = Piece::WHITE_ROOK;
        array[Square::B1.bits() as usize] = Piece::WHITE_KNIGHT;
        array[Square::C1.bits() as usize] = Piece::WHITE_BISHOP;
        array[Square::D1.bits() as usize] = Piece::WHITE_QUEEN;
        array[Square::E1.bits() as usize] = Piece::WHITE_KING;
        array[Square::F1.bits() as usize] = Piece::WHITE_BISHOP;
        array[Square::G1.bits() as usize] = Piece::WHITE_KNIGHT;
        array[Square::H1.bits() as usize] = Piece::WHITE_ROOK;

        array[Square::A2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::B2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::C2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::D2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::E2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::F2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::G2.bits() as usize] = Piece::WHITE_PAWN;
        array[Square::H2.bits() as usize] = Piece::WHITE_PAWN;

        array[Square::A8.bits() as usize] = Piece::BLACK_ROOK;
        array[Square::B8.bits() as usize] = Piece::BLACK_KNIGHT;
        array[Square::C8.bits() as usize] = Piece::BLACK_BISHOP;
        array[Square::D8.bits() as usize] = Piece::BLACK_QUEEN;
        array[Square::E8.bits() as usize] = Piece::BLACK_KING;
        array[Square::F8.bits() as usize] = Piece::BLACK_BISHOP;
        array[Square::G8.bits() as usize] = Piece::BLACK_KNIGHT;
        array[Square::H8.bits() as usize] = Piece::BLACK_ROOK;

        array[Square::A7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::B7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::C7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::D7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::E7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::F7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::G7.bits() as usize] = Piece::BLACK_PAWN;
        array[Square::H7.bits() as usize] = Piece::BLACK_PAWN;

        Mailbox(SquareMap::from_array(array))
    };

    /// Wraps an existing [`SquareMap<Piece>`] as a `Mailbox`.
    ///
    /// This is a low-level constructor; prefer [`Mailbox::default`] or
    /// [`Mailbox::empty`] unless you already have a [`SquareMap<Piece>`].
    pub const fn new(map: SquareMap<Piece>) -> Self {
        Self(map)
    }

    /// Returns a `Mailbox` with every square set to [`Piece::NONE`].
    ///
    /// ```rust
    /// # use gambit_models::mailbox::Mailbox;
    /// # use gambit_models::piece::Piece;
    /// # use gambit_models::location::square::Square;
    /// let board = Mailbox::empty();
    /// assert_eq!(board[Square::E4], Piece::NONE);
    /// assert_eq!(board.iter_pieces().count(), 0);
    /// ```
    pub const fn empty() -> Self {
        Self(SquareMap::filled(Piece::NONE))
    }

    /// Returns an iterator over all 64 squares, including empty ones.
    ///
    /// Yields `(Square, Piece)` from `a1` to `h8`. For occupied squares
    /// only, use [`iter_pieces`][Mailbox::iter_pieces].
    pub const fn iter(&self) -> MailboxIterator {
        MailboxIterator {
            mailbox: *self,
            index: 0,
        }
    }

    /// Returns an iterator over only the occupied squares.
    ///
    /// Skips squares containing [`Piece::NONE`].
    pub const fn iter_pieces(&self) -> MailboxPieceIterator {
        MailboxPieceIterator {
            mailbox: *self,
            index: 0,
        }
    }
}

impl Index<Square> for Mailbox {
    type Output = Piece;

    fn index(&self, square: Square) -> &Piece {
        &self.0[square]
    }
}

impl IndexMut<Square> for Mailbox {
    fn index_mut(&mut self, square: Square) -> &mut Piece {
        &mut self.0[square]
    }
}

impl From<[Piece; Square::COUNT]> for Mailbox {
    fn from(pieces: [Piece; Square::COUNT]) -> Mailbox {
        Mailbox(SquareMap::from(pieces))
    }
}

impl From<SquareMap<Piece>> for Mailbox {
    fn from(pieces: SquareMap<Piece>) -> Mailbox {
        Mailbox(pieces)
    }
}

impl From<Mailbox> for [Piece; Square::COUNT] {
    fn from(mailbox: Mailbox) -> [Piece; Square::COUNT] {
        mailbox.0.into()
    }
}

impl From<Mailbox> for SquareMap<Piece> {
    fn from(mailbox: Mailbox) -> SquareMap<Piece> {
        mailbox.0
    }
}

impl IntoIterator for Mailbox {
    type Item = (Square, Piece);
    type IntoIter = MailboxIterator;

    fn into_iter(self) -> MailboxIterator {
        MailboxIterator {
            mailbox: self,
            index: 0,
        }
    }
}

impl IntoIterator for &Mailbox {
    type Item = (Square, Piece);
    type IntoIter = MailboxIterator;

    fn into_iter(self) -> MailboxIterator {
        MailboxIterator {
            mailbox: *self,
            index: 0,
        }
    }
}

/// Iterator over all 64 squares of a [`Mailbox`], including empty ones.
///
/// Obtained via [`Mailbox::iter`] or [`IntoIterator`].
/// Implements [`ExactSizeIterator`] and [`FusedIterator`].
#[derive(Debug, Clone, Copy)]
pub struct MailboxIterator {
    mailbox: Mailbox,
    index: u8,
}

impl Iterator for MailboxIterator {
    type Item = (Square, Piece);

    fn next(&mut self) -> Option<(Square, Piece)> {
        if self.index >= 64 {
            return None;
        }

        let square = Square::from_index(self.index);
        let piece = self.mailbox[square];

        self.index += 1;

        Some((square, piece))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (64 - self.index) as usize;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for MailboxIterator {
    #[inline]
    fn len(&self) -> usize {
        64 - self.index as usize
    }
}

impl FusedIterator for MailboxIterator {}

/// Iterator over only the occupied squares of a [`Mailbox`].
///
/// Obtained via [`Mailbox::iter_pieces`]. Skips [`Piece::NONE`] squares.
#[derive(Debug, Clone, Copy)]
pub struct MailboxPieceIterator {
    mailbox: Mailbox,
    index: u8,
}

impl Iterator for MailboxPieceIterator {
    type Item = (Square, Piece);

    fn next(&mut self) -> Option<(Square, Piece)> {
        while self.index < 64 {
            let square = Square::from_index(self.index);
            let piece = self.mailbox[square];

            self.index += 1;

            if piece.is_some() {
                return Some((square, piece));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (64 - self.index) as usize;

        (0, Some(remaining))
    }
}

impl FusedIterator for MailboxPieceIterator {}

#[cfg(feature = "colored")]
fn piece_to_symbol(piece: Piece) -> String {
    if piece.is_none() {
        return ".".dimmed().to_string();
    }

    let ch = piece.as_char().to_string();

    match piece.colour() {
        Some(Colour::White) => ch.bright_white().bold().to_string(),
        Some(Colour::Black) => ch.bright_blue().bold().to_string(),
        None => ch,
    }
}

#[cfg(not(feature = "colored"))]
fn piece_to_symbol(piece: Piece) -> String {
    if piece.is_none() {
        ".".to_string()
    } else {
        piece.as_char().to_string()
    }
}

impl fmt::Display for Mailbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  a b c d e f g h")?;

        for rank in Rank::ALL.iter().rev() {
            write!(f, "{} ", *rank as u8 + 1)?;

            for file in File::ALL {
                let square = Square::from_coordinates((file, *rank));
                let piece = self[square];

                let symbol = piece_to_symbol(piece);

                write!(f, "{} ", symbol)?;
            }

            writeln!(f, "{}", *rank as u8 + 1)?;
        }

        writeln!(f, "  a b c d e f g h")
    }
}
