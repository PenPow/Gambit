use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromCharError(pub char);

impl Display for TryFromCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}' is not a valid character", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromIntError<T: Display>(pub T);

impl<T: Display> Display for TryFromIntError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}' is not a valid integer", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ParseSquareError {
    #[error("invalid square length: expected 2 characters, got {0}")]
    InvalidLength(usize),
    #[error("invalid file character: '{0}'")]
    InvalidFile(char),
    #[error("invalid rank character: '{0}'")]
    InvalidRank(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ParseCastlingRightsError {
    #[error("invalid castling rights character: {0}")]
    InvalidChar(char),
    #[error("duplicate castling rights flag: '{0}'")]
    DuplicateFlag(char),
    #[error("castling rights was empty string")]
    Empty,
}
