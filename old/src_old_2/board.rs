pub mod bitboard;
pub mod castling;
mod fen;
pub mod location;
pub mod piece;
mod zobrist;

use bitboard::Bitboard;
use castling::{CastlingAvailability, CastlingPermissions};
use fen::{FENError, FENParser};
use location::{Square, Squares, SQUARE_BITBOARDS};
use piece::{Piece, Pieces, Side, Sides};
use zobrist::{get_zobrist_key, ZobristKey, ZOBRIST_CASTLING, ZOBRIST_EN_PASSANT, ZOBRIST_PIECES, ZOBRIST_SIDE};
use crate::{helpers::bits, movegen::{piece_move::Move, MoveGenerator}};

#[derive(Clone, Copy, Debug)]
pub struct State {
	pub side_to_move: Side,

	pub half_move_clock: u8,
	pub full_move_number: u8,

	pub en_passant_square: Option<Square>,

	pub castling_availability: CastlingAvailability,	

	zobrist_key: ZobristKey,
	next_move: Move
}

pub struct Board {
	pub state: State,
	history: Vec<State>,

	pub move_generator: MoveGenerator,

	pub piece_list: [Piece; Squares::COUNT],

	pub piece_bitboards: [[Bitboard; Pieces::COUNT]; Sides::COUNT],
	pub side_bitboards: [Bitboard; Sides::COUNT],
}

impl Board {
	const STARTING_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	pub fn from_start_pos() -> Self {
		unsafe { Board::from_fen(Board::STARTING_POSITION_FEN).unwrap_unchecked() }
	}

	pub fn from_fen(fen: &str) -> Result<Self, FENError>  {
		let mut split_sections: Vec<&str> = fen.split_whitespace().collect();

		if split_sections.len() == 4 {
			split_sections.push("0");
			split_sections.push("1");
		};

		if split_sections.len() < 6 { return Err(FENError::InvalidFormat); }

		let mut board = Self {
			state: State {
				side_to_move: FENParser::parse_side_to_move(split_sections[1])?,
				castling_availability: FENParser::parse_castling(split_sections[2])?,
				en_passant_square: FENParser::parse_en_passant_square(split_sections[3])?,
				half_move_clock: split_sections[4].parse::<u8>().map_err(|_| FENError::InvalidHalfmoveClock)?,
				full_move_number: split_sections[5].parse::<u8>().map_err(|_| FENError::InvalidFullmoveNumber)?,

				zobrist_key: 0,
				next_move: Move::NULL,
			},

			history: Vec::new(),

			move_generator: MoveGenerator::new(),

			piece_list: [Pieces::NONE; Squares::COUNT],
			piece_bitboards: [[Bitboard::EMPTY; Pieces::COUNT]; Sides::COUNT],
			side_bitboards: [Bitboard::EMPTY; Sides::COUNT],
		};

		FENParser::parse_piece_placement(split_sections[0], &mut board)?;
		board.load_piece_table();

		board.state.zobrist_key = get_zobrist_key(board.piece_bitboards, board.state.side_to_move, board.state.castling_availability, board.state.en_passant_square);

		Ok(board)
	}

	pub fn occupancy(&self) -> Bitboard {
		self.side_bitboards[Sides::WHITE] | self.side_bitboards[Sides::BLACK]
	}

	#[must_use = "The caller must unmake the move when it is illegal"]
	pub fn make_move(&mut self, m: Move) -> bool {
		let mut current_game_state = self.state;
        current_game_state.next_move = m;
        self.history.push(current_game_state);
		
		let us = self.state.side_to_move;
		let opponent = us ^ 1;
		
		let piece = m.piece();
		let from = m.from();
		let to = m.to();
		let capture = m.capture();
		let promotion = m.promotion();
		let castling = m.castling();

		let is_capture = capture != Pieces::NONE;
		let has_castling_permissions = self.state.castling_availability > 0;

		self.state.half_move_clock += 1;

		if let Some(square) = self.state.en_passant_square {
			self.state.zobrist_key ^= ZOBRIST_EN_PASSANT[square];
			self.state.en_passant_square = None;
			self.state.zobrist_key ^= ZOBRIST_EN_PASSANT[ZOBRIST_EN_PASSANT.len()]
		}

		if is_capture {
			self.remove_piece::<true>(opponent, capture, to);
			self.state.half_move_clock = 0;

			if capture == Pieces::ROOK && has_castling_permissions {
				self.state.zobrist_key ^= ZOBRIST_CASTLING[self.state.castling_availability as usize];
				self.state.castling_availability = self.state.castling_availability & CastlingPermissions::PER_SQUARE[to];
				self.state.zobrist_key ^= ZOBRIST_CASTLING[self.state.castling_availability as usize];
			}
		}

		if piece != Pieces::PAWN {
			self.move_piece::<true>(us, piece, from, to);
		} else {
			let double_step = m.double_step();
			let en_passant = m.en_passant();

			let is_promotion = promotion != Pieces::NONE;

			self.remove_piece::<true>(us, piece, from);
			self.put_piece::<true>(us, if is_promotion { promotion } else { piece }, to);

			self.state.half_move_clock = 0;

			if en_passant {
				self.remove_piece::<true>(opponent, Pieces::PAWN, to ^ 8);
			}

			if double_step {
				self.state.zobrist_key ^= ZOBRIST_EN_PASSANT[self.state.en_passant_square.unwrap_or(ZOBRIST_EN_PASSANT.len())];
				self.state.en_passant_square = Some(to ^ 8);
				self.state.zobrist_key ^= ZOBRIST_EN_PASSANT[to ^ 8];
			}
		}

		if (piece == Pieces::KING || piece == Pieces::ROOK) && has_castling_permissions {
			self.state.zobrist_key ^= ZOBRIST_CASTLING[self.state.castling_availability as usize];
			self.state.castling_availability = self.state.castling_availability & CastlingPermissions::PER_SQUARE[from];
			self.state.zobrist_key ^= ZOBRIST_CASTLING[self.state.castling_availability as usize];
		}

		if castling {
			match to {
				Squares::G1 => self.move_piece::<true>(us, Pieces::ROOK, Squares::H1, Squares::F1),
				Squares::C1 => self.move_piece::<true>(us, Pieces::ROOK, Squares::A1, Squares::D1),
				Squares::G8 => self.move_piece::<true>(us, Pieces::ROOK, Squares::H8, Squares::F8),
				Squares::C8 => self.move_piece::<true>(us, Pieces::ROOK, Squares::A8, Squares::D8),
				_ => panic!("Invalid castling square")
			}
		}

		self.state.zobrist_key ^= ZOBRIST_SIDE;
        self.state.side_to_move ^= 1;

		if us == Sides::BLACK {
			self.state.full_move_number += 1;
		}

		let is_legal = !self.move_generator.is_square_attacked(&self, opponent, self.piece_bitboards[us][Pieces::KING].0.trailing_zeros() as Square);
		if !is_legal {
			self.unmake_move();
		}

		is_legal
	}

	pub fn unmake_move(&mut self) {
		self.state = self.history.pop().unwrap();

		let m = self.state.next_move;

		let us = self.state.side_to_move;
		let opponent = us ^ 1;

		let piece = m.piece();
		let from = m.from();
		let to = m.to();
		let capture = m.capture();
		let promotion = m.promotion();
		let castling = m.castling();
		let en_passant = m.en_passant();

		if promotion == Pieces::NONE {
			self.move_piece::<false>(us, piece, to, from);
		} else {
			self.remove_piece::<false>(us, promotion, to);
			self.put_piece::<false>(us, Pieces::PAWN, from);
		}

		if castling {
			match to {
				Squares::G1 => self.move_piece::<false>(us, Pieces::ROOK, Squares::F1, Squares::H1),
				Squares::C1 => self.move_piece::<false>(us, Pieces::ROOK, Squares::D1, Squares::A1),
				Squares::G8 => self.move_piece::<false>(us, Pieces::ROOK, Squares::F8, Squares::H8),
				Squares::C8 => self.move_piece::<false>(us, Pieces::ROOK, Squares::D8, Squares::A8),
				_ => panic!("Invalid castling square")
			}
		}

		if capture != Pieces::NONE {
			self.put_piece::<false>(opponent, capture, to);
		}

		if en_passant {
			self.put_piece::<false>(opponent, Pieces::PAWN, to ^ 8);
		}
	}

	pub fn put_piece<const UPDATE_ZOBRIST: bool>(&mut self, side: Side, piece: Piece, square: Square) {
		self.piece_bitboards[side][piece] |= SQUARE_BITBOARDS[square];
		self.side_bitboards[side] |= SQUARE_BITBOARDS[square];

		self.piece_list[square] = piece;

		if UPDATE_ZOBRIST {
			let offset: usize = if side == Sides::BLACK { 6 } else { 0 };
			self.state.zobrist_key ^= ZOBRIST_PIECES[piece + offset][square];
		}
	}

	pub fn remove_piece<const UPDATE_ZOBRIST: bool>(&mut self, side: Side, piece: Piece, square: Square) {
		self.piece_bitboards[side][piece] ^= SQUARE_BITBOARDS[square];
		self.side_bitboards[side] ^= SQUARE_BITBOARDS[square];

		self.piece_list[square] = Pieces::NONE;

		if UPDATE_ZOBRIST {
			let offset: usize = if side == Sides::BLACK { 6 } else { 0 };
			self.state.zobrist_key ^= ZOBRIST_PIECES[piece + offset][square];
		}
	}

	pub fn move_piece<const UPDATE_ZOBRIST: bool>(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
		self.remove_piece::<{ UPDATE_ZOBRIST }>(side, piece, from);
		self.put_piece::<{ UPDATE_ZOBRIST }>(side, piece, to);
	}

	fn load_piece_table(&mut self) {
		for (piece_type, (white_pieces, black_pieces)) in self.piece_bitboards[Sides::WHITE].iter().zip(self.piece_bitboards[Sides::BLACK].iter()).enumerate() {
			let mut white_pieces = *white_pieces;
			let mut black_pieces = *black_pieces;
			
			while white_pieces > 0 {
				let square = bits::next(&mut white_pieces);

				self.piece_list[square] = piece_type;
			}

			while black_pieces > 0 {
				let square = bits::next(&mut black_pieces);

				self.piece_list[square] = piece_type;
			}
		}
	}

	fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.state.zobrist_key)?;

		writeln!(f, "Black Kings \n{}", self.piece_bitboards[Sides::BLACK][Pieces::KING])?;
		writeln!(f, "Black Queens \n{}", self.piece_bitboards[Sides::BLACK][Pieces::QUEEN])?;
		writeln!(f, "Black Rooks \n{}", self.piece_bitboards[Sides::BLACK][Pieces::ROOK])?;
		writeln!(f, "Black Bishops \n{}", self.piece_bitboards[Sides::BLACK][Pieces::BISHOP])?;
		writeln!(f, "Black Knights \n{}", self.piece_bitboards[Sides::BLACK][Pieces::KNIGHT])?;
		writeln!(f, "Black Pawns \n{}", self.piece_bitboards[Sides::BLACK][Pieces::PAWN])?;

		writeln!(f, "White Kings \n{}", self.piece_bitboards[Sides::WHITE][Pieces::KING])?;
		writeln!(f, "White Queens \n{}", self.piece_bitboards[Sides::WHITE][Pieces::QUEEN])?;
		writeln!(f, "White Rooks \n{}", self.piece_bitboards[Sides::WHITE][Pieces::ROOK])?;
		writeln!(f, "White Bishops \n{}", self.piece_bitboards[Sides::WHITE][Pieces::BISHOP])?;
		writeln!(f, "White Knights \n{}", self.piece_bitboards[Sides::WHITE][Pieces::KNIGHT])?;
		writeln!(f, "White Pawns \n{}", self.piece_bitboards[Sides::WHITE][Pieces::PAWN])?;

		writeln!(f, "Combined \n{}", self.occupancy())?;

		Ok(())
	}
}

impl Default for Board {
	fn default() -> Self {
		Board::from_start_pos()
	}
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}