#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum AlgebraicError {
    #[error("invalid square {0}")]
    InvalidSquare(String),
    #[error("invalid piece letter {0}")]
    InvalidPiece(char),
    #[error("invalid promotion target {0}")]
    InvalidPromotion(char),
    #[error("malformed move: {0}")]
    Malformed(String),
    #[error("ambiguous move: {0}")]
    AmbiguousMove(String),
    #[error("{0} is not a legal move in this position")]
    NoSuchMove(String),
}
