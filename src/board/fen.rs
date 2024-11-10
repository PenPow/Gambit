use std::fmt;
use std::error::Error;

use super::{castling::{CastlingAvailability, CastlingPermissions}, location::{Files, Ranks, Square, Squares}, piece::{Pieces, Side, Sides}, Board};

pub struct FENParser;
impl FENParser {
	pub fn parse_side_to_move(char: &str) -> Result<Side, FENError> {
		match char {
			"w" => Ok(Sides::WHITE),
			"b" => Ok(Sides::BLACK),
			_ => Err(FENError::InvalidActiveColor)
		}
	}

	pub fn parse_castling(permissions: &str) -> Result<CastlingAvailability, FENError> {
		let castling = match permissions.len() {
			4 => CastlingPermissions::ALL,
			0 => CastlingPermissions::NONE,

			_ => {
				let mut castling: u8 = 0;

				for char in permissions.chars() {
					match char {
						'K' => castling |= CastlingPermissions::WHITE_KING,
						'Q' => castling |= CastlingPermissions::WHITE_QUEEN,
						'k' => castling |= CastlingPermissions::BLACK_KING,
						'q' => castling |= CastlingPermissions::BLACK_QUEEN,
						'-' => (),
						_ => return Err(FENError::InvalidCastlingRights),
					}
				}

				castling
			}
		};
		
		Ok(castling)
	}

	pub fn parse_en_passant_square(square: &str) -> Result<Option<Square>, FENError> {
		if square == "-" {
			Ok(None)
		} else {
			Ok(Some(Squares::from_algebraic_notation(square)))
		}
	}

	pub fn parse_piece_placement(placement: &str, board: &mut Board) -> Result<(), FENError> {
		let mut rank = Ranks::R8 as u8;
		let mut file = Files::A as u8;
		
		for char in placement.chars() {
			let square = ((rank * 8) + file) as u64;

			match char {
				'k' => board.piece_bitboards[Sides::BLACK][Pieces::KING] |= 1u64 << square,
				'q' => board.piece_bitboards[Sides::BLACK][Pieces::QUEEN] |= 1u64 << square,
				'r' => board.piece_bitboards[Sides::BLACK][Pieces::ROOK] |= 1u64 << square,
				'b' => board.piece_bitboards[Sides::BLACK][Pieces::BISHOP] |= 1u64 << square,
				'n' => board.piece_bitboards[Sides::BLACK][Pieces::KNIGHT] |= 1u64 << square,
				'p' => board.piece_bitboards[Sides::BLACK][Pieces::PAWN] |= 1u64 << square,

				'K' => board.piece_bitboards[Sides::WHITE][Pieces::KING] |= 1u64 << square,
				'Q' => board.piece_bitboards[Sides::WHITE][Pieces::QUEEN] |= 1u64 << square,
				'R' => board.piece_bitboards[Sides::WHITE][Pieces::ROOK] |= 1u64 << square,
				'B' => board.piece_bitboards[Sides::WHITE][Pieces::BISHOP] |= 1u64 << square,
				'N' => board.piece_bitboards[Sides::WHITE][Pieces::KNIGHT] |= 1u64 << square,
				'P' => board.piece_bitboards[Sides::WHITE][Pieces::PAWN] |= 1u64 << square,

				'1'..='8' => {
					if let Some(x) = char.to_digit(10) {
						file += x as u8;
					}
				}

				'/' => {
					if file != 8 {
						return Err(FENError::InvalidPiecePlacement);
					}

					rank -= 1;
					file = 0;
				}
				_ => {}
			};

			if "kqrbnpKQRBNP".contains(char) {
				file += 1;

				if char.is_lowercase() {
					board.side_bitboards[Sides::BLACK] |= 1u64 << square
				} else {
					board.side_bitboards[Sides::WHITE] |= 1u64 << square

				}
			}
		}

		Ok(())
	}
}

#[derive(Debug)]
pub enum FENError {
    InvalidFormat,
    InvalidPiecePlacement,
    InvalidActiveColor,
    InvalidCastlingRights,
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
            FENError::InvalidHalfmoveClock => write!(f, "Invalid halfmove clock in FEN"),
            FENError::InvalidFullmoveNumber => write!(f, "Invalid fullmove number in FEN"),
        }
    }
}