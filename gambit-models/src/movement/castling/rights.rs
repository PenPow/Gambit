use crate::error::{ParseCastlingRightsError, TryFromIntError};
use crate::location::square::Square;
use crate::movement::castling::side::CastlingSide;
use crate::piece::colour::Colour;
use bitflags::bitflags;
use std::fmt;
use std::str::FromStr;

bitflags! {
    /// The castling rights currently available in a chess position.
    ///
    /// Represented as a 4-bit flags value packed into a `u8`:
    ///
    /// | Bit | Meaning         | FEN character |
    /// |-----|-----------------|---------------|
    /// | 0   | White kingside  | `K`           |
    /// | 1   | White queenside | `Q`           |
    /// | 2   | Black kingside  | `k`           |
    /// | 3   | Black queenside | `q`           |
    ///
    /// # FEN representation
    ///
    /// [`Display`][std::fmt::Display] outputs standard FEN castling notation.
    /// [`FromStr`] parses the same format, including `"-"` for no rights:
    ///
    /// ```rust
    /// # use gambit_models::movement::castling::rights::CastlingRights;
    /// # use std::str::FromStr;
    /// assert_eq!(CastlingRights::ALL.to_string(), "KQkq");
    /// assert_eq!(CastlingRights::NONE.to_string(), "-");
    /// assert_eq!("Kq".parse::<CastlingRights>().unwrap(),
    ///     CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_QUEENSIDE);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_KINGSIDE = 0b0100;
        const BLACK_QUEENSIDE = 0b1000;

        const WHITE = Self::WHITE_KINGSIDE.bits() | Self::WHITE_QUEENSIDE.bits();
        const BLACK = Self::BLACK_KINGSIDE.bits() | Self::BLACK_QUEENSIDE.bits();
        const KINGSIDE = Self::WHITE_KINGSIDE.bits() | Self::BLACK_KINGSIDE.bits();
        const QUEENSIDE = Self::WHITE_QUEENSIDE.bits() | Self::BLACK_QUEENSIDE.bits();

        const ALL = 0b1111;
        const NONE = 0b0000;
    }
}

impl CastlingRights {
    pub const RIGHTS: [Self; 4] = [
        Self::WHITE_KINGSIDE,
        Self::WHITE_QUEENSIDE,
        Self::BLACK_KINGSIDE,
        Self::BLACK_QUEENSIDE,
    ];

    /// Returns `true` if `colour` retains the right to castle on `side`.
    #[inline]
    pub const fn has(self, colour: Colour, side: CastlingSide) -> bool {
        self.intersects(match (colour, side) {
            (Colour::White, CastlingSide::Kingside) => CastlingRights::WHITE_KINGSIDE,
            (Colour::White, CastlingSide::Queenside) => CastlingRights::WHITE_QUEENSIDE,
            (Colour::Black, CastlingSide::Kingside) => CastlingRights::BLACK_KINGSIDE,
            (Colour::Black, CastlingSide::Queenside) => CastlingRights::BLACK_QUEENSIDE,
        })
    }

    /// Returns `true` if `colour` retains castling rights on either side.
    #[inline]
    pub const fn has_any_for(self, colour: Colour) -> bool {
        self.intersects(match colour {
            Colour::White => CastlingRights::WHITE,
            Colour::Black => CastlingRights::BLACK,
        })
    }

    /// Returns the castling rights masked to `colour` only.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::movement::castling::rights::CastlingRights;
    /// # use gambit_models::piece::colour::Colour;
    /// let rights = CastlingRights::ALL;
    /// assert_eq!(rights.for_colour(Colour::White), CastlingRights::WHITE);
    /// ```
    #[inline]
    pub const fn for_colour(self, colour: Colour) -> CastlingRights {
        self.intersection(match colour {
            Colour::White => CastlingRights::WHITE,
            Colour::Black => CastlingRights::BLACK,
        })
    }

    /// Returns the castling rights masked to `side` only.
    #[inline]
    pub const fn for_side(self, side: CastlingSide) -> CastlingRights {
        self.intersection(match side {
            CastlingSide::Kingside => CastlingRights::KINGSIDE,
            CastlingSide::Queenside => CastlingRights::QUEENSIDE,
        })
    }

    /// Returns new rights with all rights for `colour` removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::movement::castling::rights::CastlingRights;
    /// # use gambit_models::piece::colour::Colour;
    /// let rights = CastlingRights::ALL.remove_colour(Colour::White);
    /// assert_eq!(rights, CastlingRights::BLACK);
    /// ```
    #[inline]
    pub const fn remove_colour(self, colour: Colour) -> CastlingRights {
        self.difference(match colour {
            Colour::White => CastlingRights::WHITE,
            Colour::Black => CastlingRights::BLACK,
        })
    }

    /// Returns new rights with all rights for `side` removed.
    #[inline]
    pub const fn remove_side(self, side: CastlingSide) -> CastlingRights {
        self.difference(match side {
            CastlingSide::Kingside => CastlingRights::KINGSIDE,
            CastlingSide::Queenside => CastlingRights::QUEENSIDE,
        })
    }

    /// Returns new rights with the specific `colour`+`side` right removed.
    #[inline]
    pub const fn remove_right(self, side: CastlingSide, colour: Colour) -> CastlingRights {
        self.difference(side.rights(colour))
    }

    pub fn updated_by(self, from: Square, to: Square) -> Self {
        self.difference(from.revoked_castling_rights())
            .difference(to.revoked_castling_rights())
    }
}

impl From<CastlingRights> for u8 {
    #[inline]
    fn from(rights: CastlingRights) -> u8 {
        rights.bits()
    }
}

impl TryFrom<u8> for CastlingRights {
    type Error = TryFromIntError<u8>;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if bits <= 0b1111 {
            Ok(CastlingRights::from_bits_truncate(bits))
        } else {
            Err(TryFromIntError(bits))
        }
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("-");
        }
        if self.contains(CastlingRights::WHITE_KINGSIDE) {
            f.write_str("K")?;
        }
        if self.contains(CastlingRights::WHITE_QUEENSIDE) {
            f.write_str("Q")?;
        }
        if self.contains(CastlingRights::BLACK_KINGSIDE) {
            f.write_str("k")?;
        }
        if self.contains(CastlingRights::BLACK_QUEENSIDE) {
            f.write_str("q")?;
        }
        Ok(())
    }
}

impl FromStr for CastlingRights {
    type Err = ParseCastlingRightsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseCastlingRightsError::Empty);
        }

        if s == "-" {
            return Ok(CastlingRights::NONE);
        }

        let mut rights = CastlingRights::NONE;

        for character in s.chars() {
            let flag = match character {
                'K' => CastlingRights::WHITE_KINGSIDE,
                'Q' => CastlingRights::WHITE_QUEENSIDE,
                'k' => CastlingRights::BLACK_KINGSIDE,
                'q' => CastlingRights::BLACK_QUEENSIDE,
                _ => return Err(ParseCastlingRightsError::InvalidChar(character)),
            };

            if rights.contains(flag) {
                return Err(ParseCastlingRightsError::DuplicateFlag(character));
            }

            rights |= flag;
        }

        Ok(rights)
    }
}
