use super::{bitboard::Bitboard, castling::Castling, piece::{Piece, Side, Sides}, square::{Square, Squares}};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

const RNG_SEED: u64 = 2146;

pub type ZobristKey = u64;

#[derive(Debug)]
pub struct ZobristRandoms {
	pieces: [[[u64; 64]; 7]; 2],
	pub side_to_move: u64,
	castling_rights: [u64; 16],
	en_passant: [u64; 64]
}

impl ZobristRandoms {
	pub fn new() -> Self {
		let mut random = ChaChaRng::seed_from_u64(RNG_SEED);

		let mut randoms = Self {
			pieces: [[[0; 64]; 7]; 2],
			side_to_move: 0,
			castling_rights: [0; 16],
			en_passant: [0; 64],
		};

		randoms.pieces.iter_mut().for_each(|side| {
			side.iter_mut().for_each(|piece| {
				piece.iter_mut().for_each(|square| {
					*square = random.gen::<u64>()
				})
			})
		});

		randoms.side_to_move = random.gen::<u64>();

		randoms.castling_rights.iter_mut().for_each(|castling_right| {
			*castling_right = random.gen::<u64>()
		});

		randoms.en_passant.iter_mut().for_each(|square| {
			*square = random.gen::<u64>()
		});

		randoms
	}

	pub fn get_key(&self, bitboards: [[Bitboard; 6]; 2], side_to_move: Side, castling_availability: Castling, en_passant_square: Option<Square>) -> ZobristKey {
		let mut key = 0u64;

		for (side, side_bitboards) in bitboards.iter().enumerate() {
			for (piece, bitboard) in side_bitboards.iter().enumerate() {
				let bitboard = *bitboard;

				for i in Squares::ALL_SQUARES_RANGE {
					if bitboard >> i == 1 {
						key ^= self.piece(side, piece, Square::from(i))
					}
				}
			}
		}

		if side_to_move == Sides::BLACK {
			key ^= self.side_to_move
		}

		key ^= self.castling(castling_availability);
		key ^= self.en_passant(en_passant_square);

		key
	}

	pub fn piece(&self, side: usize, piece: Piece, square: Square) -> u64 {
		self.pieces[side][piece][square]
	}

	pub fn castling(&self, castling_availability: Castling) -> u64 {
		self.castling_rights[castling_availability as usize]
	}

	pub fn en_passant(&self, en_passant_square: Option<Square>) -> u64 {
		match en_passant_square {
			Some(square) => self.en_passant[square],
			None => 0
		}
	}
}