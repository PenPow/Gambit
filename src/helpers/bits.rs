use crate::board::{bitboard::Bitboard, location::{Square, SQUARE_BITBOARDS}};

pub fn next(bitboard: &mut Bitboard) -> Square {
	let square = bitboard.0.trailing_zeros() as Square;
	*bitboard ^= SQUARE_BITBOARDS[square];

	square
}