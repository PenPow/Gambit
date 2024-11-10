use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum FENError {
    InvalidFormat,
    InvalidPiecePlacement,
    InvalidActiveColor,
    InvalidCastlingRights,
    InvalidEnPassantTarget,
    InvalidHalfmoveClock,
    InvalidFullmoveNumber,
}

impl Error for FENError {}

impl fmt::Display for FENError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FENError::InvalidFormat => write!(f, "Invalid FEN format"),
            FENError::InvalidPiecePlacement => write!(f, "Invalid piece placement in FEN"),
            FENError::InvalidActiveColor => write!(f, "Invalid active color in FEN"),
            FENError::InvalidCastlingRights => write!(f, "Invalid castling rights in FEN"),
            FENError::InvalidEnPassantTarget => write!(f, "Invalid en passant target in FEN"),
            FENError::InvalidHalfmoveClock => write!(f, "Invalid halfmove clock in FEN"),
            FENError::InvalidFullmoveNumber => write!(f, "Invalid fullmove number in FEN"),
        }
    }
}