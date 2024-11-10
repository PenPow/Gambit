pub mod bitboard;
pub mod castling;
pub mod piece;
pub mod square;
pub mod zobrist;

use bitboard::Bitboard;
use castling::{Castling, CastlingPermissions};
use piece::{Piece, PieceType, Side, Sides};
use square::{Files, Ranks, Square, Squares, SQUARE_BITBOARD_LOOKUP};
use zobrist::{ZobristKey, ZobristRandoms};

use crate::{fen::FENError, moves::{castling::CASTLING_PERMISSIONS, movegen::MoveGenerator, piece_move::Move}};

#[derive(Clone, Copy, Debug)]
pub struct State {
	pub side_to_move: Side,
	pub castling_availability: Castling,
	pub en_passant_square: Option<Square>,
	pub half_moves: u8,
	pub full_moves: u8,

	pub zobrist_key: ZobristKey,
	next_move: Move,
}

#[derive(Debug)]
pub struct Board {
	pub state: State,
	history: Vec<State>,

	pub piece_bitboards: [[Bitboard; 6]; 2],
	pub piece_list: [Piece; 64],
	pub side_bitboards: [Bitboard; 2],

	zobrist_randoms: ZobristRandoms,
}

const STARTING_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
	pub fn from_start_pos() -> Self {
		let board = Board::from_fen(&STARTING_POSITION_FEN);

		return unsafe { board.unwrap_unchecked() };
	}

	pub fn from_fen(fen: &str) -> Result<Self, FENError> {
		let mut split_sections: Vec<&str> = fen.split_whitespace().collect();
		
		if split_sections.len() == 4 {
			split_sections.push("0");
			split_sections.push("1")
		}
		
		if split_sections.len() < 6 {
			return Err(FENError::InvalidFormat);
		};

		let side_to_move = Self::parse_side_to_move(split_sections[1])?;
		let castling_availability = Self::parse_castling(split_sections[2])?;
		let en_passant_square = Self::parse_en_passant_square(split_sections[3])?;
		let half_moves = split_sections[4].parse::<u8>().map_err(|_| FENError::InvalidHalfmoveClock)?;
		let full_moves = split_sections[5].parse::<u8>().map_err(|_| FENError::InvalidFullmoveNumber)?;

		let mut board = Self {
			history: vec![],
			zobrist_randoms: ZobristRandoms::new(),

			piece_bitboards: [[Bitboard(0); 6]; 2],
			piece_list: [PieceType::NONE; 64],
			side_bitboards: [Bitboard(0); 2],
			
			state: State {
				side_to_move,
				castling_availability,
				en_passant_square,
				half_moves,
				full_moves,

				zobrist_key: 0,
				next_move: Move::new(0)
			}
		};

		Self::parse_piece_placement(split_sections[0], &mut board)?;

		Self::load_piece_table(&mut board);

		board.state.zobrist_key = board.zobrist_randoms.get_key(board.piece_bitboards, side_to_move, castling_availability, en_passant_square);

		return Ok(board);
	}

	pub fn make_move(&mut self, piece_move: Move, move_generator: &MoveGenerator) -> bool{
		let mut current_state = self.state;
		current_state.next_move = piece_move;

		self.history.push(current_state);

		let us = self.state.side_to_move;
		let opponent = us ^ 1;

		let piece = piece_move.piece();
		let from = piece_move.from();
		let to = piece_move.to();
		let capture = piece_move.capture();
		let promotion = piece_move.promotion();
		let castling = piece_move.castling();
		let double_step = piece_move.double_step();
		let en_passant_square = piece_move.en_passant();

		let is_capture = capture != PieceType::NONE;
		let is_promotion = promotion != PieceType::NONE;

		self.state.half_moves += 1;

		if self.state.en_passant_square.is_some() {
			self.state.zobrist_key ^= self.zobrist_randoms.en_passant(self.state.en_passant_square);
			self.state.en_passant_square = None;
			self.state.zobrist_key ^= self.zobrist_randoms.en_passant(self.state.en_passant_square);
		}

		if is_capture {
			self.remove_piece(us, piece, to);
			self.state.half_moves = 0;

			if capture == PieceType::ROOK && self.state.castling_availability > 0 {
				self.state.zobrist_key ^= self.zobrist_randoms.castling(self.state.castling_availability);
				self.state.castling_availability = self.state.castling_availability & CASTLING_PERMISSIONS[to];
				self.state.zobrist_key ^= self.zobrist_randoms.castling(self.state.castling_availability);
			}
		}

		if piece != PieceType::PAWN {
			self.move_piece(us, piece, from, to);
		} else {
			self.remove_piece(us, piece, from);
			self.put_piece(us, if !is_promotion { piece } else { promotion }, to);

			self.state.half_moves = 0;

			if en_passant_square {
				self.remove_piece(opponent, PieceType::PAWN, to ^ 8)
			}

			if double_step {
				self.state.zobrist_key ^= self.zobrist_randoms.en_passant(self.state.en_passant_square);
				self.state.en_passant_square = Some(to ^ 8);
				self.state.zobrist_key ^= self.zobrist_randoms.en_passant(self.state.en_passant_square);
			}
		}

		if (piece == PieceType::KING || piece == PieceType::ROOK) && self.state.castling_availability > 0 {
			self.state.zobrist_key ^= self.zobrist_randoms.castling(self.state.castling_availability);
			self.state.castling_availability = self.state.castling_availability & CASTLING_PERMISSIONS[from];
			self.state.zobrist_key ^= self.zobrist_randoms.castling(self.state.castling_availability);
		}

		if castling {
			match to {
				Squares::C1 => self.move_piece(us, PieceType::ROOK, Squares::A1, Squares::D1),
				Squares::G1 => self.move_piece(us, PieceType::ROOK, Squares::H1, Squares::F1),
				Squares::C8 => self.move_piece(us, PieceType::ROOK, Squares::A8, Squares::D8),
				Squares::G8 => self.move_piece(us, PieceType::ROOK, Squares::H8, Squares::F8),
				_ => panic!("Unexpected castling move")
			}
		}

		self.state.zobrist_key ^= self.zobrist_randoms.side_to_move;
        self.state.side_to_move = opponent;

		if us == Sides::BLACK {
            self.state.full_moves += 1;
        }

		let is_legal = !move_generator.is_square_attacked(&self, opponent, self.piece_bitboards[us][PieceType::KING].0.trailing_zeros() as Square);

		if !is_legal {
			self.unmake_move();
		}

		is_legal
	}

	pub fn unmake_move(&mut self) {
		let old_state = self.history.pop().unwrap();

		let piece_move = old_state.next_move;

		let us = old_state.side_to_move;
		let opponent = us ^ 1;

		let piece = piece_move.piece();
		let from = piece_move.from();
		let to = piece_move.to();
		let capture = piece_move.capture();
		let promotion = piece_move.promotion();
		let castling = piece_move.castling();
		let en_passant_square = piece_move.en_passant();

		if promotion == PieceType::NONE {
			self.remove_piece(us, piece, from);
			self.put_piece(us, piece, to);
		} else {
			self.remove_piece(us, promotion, to);
			self.put_piece(us, PieceType::PAWN, from);
		}

		if castling {
			match to {
				Squares::C1 => self.move_piece(us, PieceType::ROOK, Squares::D1, Squares::A1),
				Squares::G1 => self.move_piece(us, PieceType::ROOK, Squares::F1, Squares::H1),
				Squares::C8 => self.move_piece(us, PieceType::ROOK, Squares::D8, Squares::A8),
				Squares::G8 => self.move_piece(us, PieceType::ROOK, Squares::F8, Squares::H8),
				_ => panic!("Unexpected castling move")
			}
		}

		if capture != PieceType::NONE {
			self.put_piece(opponent, capture, to);
		}

		if en_passant_square {
			self.put_piece(opponent, PieceType::PAWN, to ^ 8)
		}

		self.state = old_state;
	}

	fn move_piece(&mut self, side: usize, piece: Piece, from: Square, to: Square) {
		self.remove_piece(side, piece, from);
		self.put_piece(side, piece, to);
	}

	fn put_piece(&mut self, side: usize, piece: Piece, square: Square) {
		self.piece_bitboards[side][piece] |= SQUARE_BITBOARD_LOOKUP[square];
		self.side_bitboards[side] |= SQUARE_BITBOARD_LOOKUP[square];

		self.state.zobrist_key ^= self.zobrist_randoms.piece(side, piece, square);
	}

	fn remove_piece(&mut self, side: usize, piece: Piece, square: Square) {
		self.piece_bitboards[side][piece] ^= SQUARE_BITBOARD_LOOKUP[square];
		self.side_bitboards[side] ^= SQUARE_BITBOARD_LOOKUP[square];

		self.state.zobrist_key ^= self.zobrist_randoms.piece(side, piece, square)
	}

	fn load_piece_table(board: &mut Self) {
		for (piece_type, (white_pieces, black_pieces)) in board.piece_bitboards[Sides::WHITE].iter().zip(board.piece_bitboards[Sides::BLACK].iter()).enumerate() {
			let mut white_pieces = *white_pieces;
			let mut black_pieces = *black_pieces;
			
			while white_pieces > 0 {
				let square = white_pieces.0.trailing_zeros() as Square;
				white_pieces ^= 1u64 << square;

				board.piece_list[square] = piece_type;
			}

			while black_pieces > 0 {
				let square = black_pieces.0.trailing_zeros() as Square;
				black_pieces ^= 1u64 << square;

				board.piece_list[square] = piece_type;
			}
		}
	}

	fn parse_piece_placement(piece_placement: &str, board: &mut Self) -> Result<(), FENError> {
		let mut rank = Ranks::R8 as u8;
		let mut file = Files::A as u8;

		for char in piece_placement.chars() {
			if file > 7 && char != '/' {
				return Err(FENError::InvalidPiecePlacement)
			}

			let square = ((rank * 8) + file) as u64;
			match char {
				'k' => board.piece_bitboards[Sides::BLACK][PieceType::KING] |= 1u64 << square,
				'q' => board.piece_bitboards[Sides::BLACK][PieceType::QUEEN] |= 1u64 << square,
				'r' => board.piece_bitboards[Sides::BLACK][PieceType::ROOK] |= 1u64 << square,
				'b' => board.piece_bitboards[Sides::BLACK][PieceType::BISHOP] |= 1u64 << square,
				'n' => board.piece_bitboards[Sides::BLACK][PieceType::KNIGHT] |= 1u64 << square,
				'p' => board.piece_bitboards[Sides::BLACK][PieceType::PAWN] |= 1u64 << square,

				'K' => board.piece_bitboards[Sides::WHITE][PieceType::KING] |= 1u64 << square,
				'Q' => board.piece_bitboards[Sides::WHITE][PieceType::QUEEN] |= 1u64 << square,
				'R' => board.piece_bitboards[Sides::WHITE][PieceType::ROOK] |= 1u64 << square,
				'B' => board.piece_bitboards[Sides::WHITE][PieceType::BISHOP] |= 1u64 << square,
				'N' => board.piece_bitboards[Sides::WHITE][PieceType::KNIGHT] |= 1u64 << square,
				'P' => board.piece_bitboards[Sides::WHITE][PieceType::PAWN] |= 1u64 << square,

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

	fn parse_side_to_move(side_to_move: &str) -> Result<Side, FENError> {
		match side_to_move {
			"b" => Ok(Sides::BLACK),
			"w" => Ok(Sides::WHITE),
			_ => Err(FENError::InvalidActiveColor)
		}
	}

	fn parse_castling(castling_availability: &str) -> Result<Castling, FENError> {
		let mut castling: u8 = 0;
        for char in castling_availability.chars() {
            match char {
                'K' => castling |= CastlingPermissions::WHITE_KING,
                'Q' => castling |= CastlingPermissions::WHITE_QUEEN,
                'k' => castling |= CastlingPermissions::BLACK_KING,
                'q' => castling |= CastlingPermissions::BLACK_QUEEN,
                '-' => (),
                _ => return Err(FENError::InvalidCastlingRights),
            }
        }

        Ok(castling)
	}

	fn parse_en_passant_square(en_passant_square: &str) -> Result<Option<Square>, FENError> {
		if en_passant_square == "-" {
			Ok(None)
		} else {
			assert_eq!(en_passant_square.len(), 2);

			let en_passant_square = (en_passant_square.to_uppercase()).parse::<Square>().map_err(|_| FENError::InvalidEnPassantTarget)?;

			Ok(Some(en_passant_square))
		}
	}


	#[cfg(debug_assertions)]
	pub fn debug(&self) {
		dbg!(self.state.zobrist_key);

		eprintln!("Black Kings \n{}", self.piece_bitboards[Sides::BLACK][PieceType::KING]);
		eprintln!("Black Queens \n{}", self.piece_bitboards[Sides::BLACK][PieceType::QUEEN]);
		eprintln!("Black Rooks \n{}", self.piece_bitboards[Sides::BLACK][PieceType::ROOK]);
		eprintln!("Black Bishops \n{}", self.piece_bitboards[Sides::BLACK][PieceType::BISHOP]);
		eprintln!("Black Knights \n{}", self.piece_bitboards[Sides::BLACK][PieceType::KNIGHT]);
		eprintln!("Black Pawns \n{}", self.piece_bitboards[Sides::BLACK][PieceType::PAWN]);

		eprintln!("White Kings \n{}", self.piece_bitboards[Sides::WHITE][PieceType::KING]);
		eprintln!("White Queens \n{}", self.piece_bitboards[Sides::WHITE][PieceType::QUEEN]);
		eprintln!("White Rooks \n{}", self.piece_bitboards[Sides::WHITE][PieceType::ROOK]);
		eprintln!("White Bishops \n{}", self.piece_bitboards[Sides::WHITE][PieceType::BISHOP]);
		eprintln!("White Knights \n{}", self.piece_bitboards[Sides::WHITE][PieceType::KNIGHT]);
		eprintln!("White Pawns \n{}", self.piece_bitboards[Sides::WHITE][PieceType::PAWN]);

		eprintln!("Combined \n{}", Self::combine_all_bitboards(&self));
	}

	pub fn combine_all_bitboards(&self) -> Bitboard {
		self.side_bitboards[Sides::WHITE] | self.side_bitboards[Sides::BLACK]
	}
}

impl Default for Board {
	fn default() -> Self {
		Board::from_start_pos()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_starter_fen() {
		todo!()
	}

	#[test]
	fn creates_empty_board() {
		todo!()
	}

	#[test]
	fn parses_random_fen() {
		todo!()
	}

	#[test]
	fn parses_piece_placement_correctly() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_piece_placement() {
		todo!()
	}

	#[test]
	fn parses_side_to_move() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_side_to_move() {
		todo!()
	}

	#[test]
	fn parses_castling_availability() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_castling_availability() {
		todo!()
	}

	#[test]
	fn parses_en_passant_square() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_en_passant_square() {
		todo!()
	}
	
	#[test]
	fn parses_half_moves() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_half_moves() {
		todo!()
	}

	#[test]
	fn parses_full_moves() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_full_moves() {
		todo!()
	}

	#[test]
	fn should_error_on_invalid_fen_length() {
		todo!()
	}

	#[test]
	fn should_infer_last_two_sections() {
		todo!()
	}
}
