use gambit_models::error::{ParseCastlingRightsError, ParseSquareError};

/// Errors that can arise when parsing a FEN string.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum FenError {
    #[error("expected 6 FEN fields, got {0}")]
    WrongFieldCount(usize),
    #[error("invalid piece placement: {0}")]
    InvalidPlacement(String),
    #[error("invalid rank: expected 8 squares per rank, got {0}")]
    InvalidRankLength(u8),
    #[error("{0}")]
    InvalidSquare(#[from] ParseSquareError),
    #[error("{0}")]
    InvalidCastling(#[from] ParseCastlingRightsError),
    #[error("invalid active color: expected 'w' or 'b', got '{0}'")]
    InvalidColour(char),
    #[error("invalid halfmove clock: {0}")]
    InvalidHalfmoveClock(String),
    #[error("invalid fullmove number: {0}")]
    InvalidFullmoveNumber(String),
}
