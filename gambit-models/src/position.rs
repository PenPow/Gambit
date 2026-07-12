use crate::location::square::Square;
use crate::mailbox::Mailbox;
use crate::movement::castling::rights::CastlingRights;
use crate::piece::colour::Colour;

/// The number of plies since the last capture or pawn push.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HalfmoveClock(pub u8);

impl HalfmoveClock {
    pub const ZERO: HalfmoveClock = HalfmoveClock(0);

    pub const fn new(clock: u8) -> Self {
        Self(clock)
    }

    pub const fn is_fifty_move_draw(self) -> bool {
        self.value() >= 100
    }

    pub const fn increment(&mut self) {
        self.0 += 1;
    }

    pub const fn clear(mut self) {
        self.0 = 0;
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

/// The fullmove count in a chess game.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FullmoveNumber(pub u16);

impl FullmoveNumber {
    pub const fn new(number: u16) -> Self {
        Self(number)
    }

    pub const fn increment(&mut self) {
        self.0 += 1;
    }

    pub const fn decrement(&mut self) {
        self.0 -= 1;
    }

    pub const fn clear(mut self) {
        self.0 = 0;
    }

    pub const fn value(self) -> u16 {
        self.0
    }
}

/// A complete chess position.
///
/// # Default
///
/// [`Position::default`] returns the starting position.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::position::Position;
/// # use gambit_models::piece::colour::Colour;
/// assert_eq!(Position::default().side_to_move, Colour::White);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    /// The arrangement of pieces on the board.
    pub board: Mailbox,
    /// Which colour moves next.
    pub side_to_move: Colour,
    /// Castling rights still available to each side.
    pub castling_rights: CastlingRights,
    /// The target square of an en passant capture, if any.
    pub en_passant: Option<Square>,
    /// Plies since the last capture or pawn push.
    pub halfmove_clock: HalfmoveClock,
    /// Fullmove count (starts at 1, increments after Black's move).
    pub fullmove_number: FullmoveNumber,
}

impl Position {
    /// The standard chess starting position.
    ///
    /// ```rust
    /// # use gambit_models::position::Position;
    /// # use gambit_models::piece::colour::Colour;
    /// assert_eq!(Position::STARTING_POSITION.side_to_move, Colour::White);
    /// ```
    pub const STARTING_POSITION: Position = Position {
        board: Mailbox::STARTING_POSITION,
        side_to_move: Colour::White,
        castling_rights: CastlingRights::ALL,
        en_passant: None,
        halfmove_clock: HalfmoveClock::ZERO,
        fullmove_number: FullmoveNumber(1),
    };
}

impl Default for Position {
    fn default() -> Self {
        Position::STARTING_POSITION
    }
}
