use magics::Magic;
use move_list::MoveList;
use piece_move::MoveBuilder;
use crate::{board::{bitboard::Bitboard, castling::CastlingPermissions, location::{Directions, Ranks, Square, Squares, RANK_BITBOARDS, SQUARE_BITBOARDS}, piece::{Piece, Pieces, Side, Sides}, Board}, dbg_assert_square_in_range, helpers::bits};

pub mod move_list;
pub mod piece_move;
mod init;
mod magics;

#[cfg(test)]
mod perft;

type MoveLookupTable = [Bitboard; Squares::COUNT]; 

const NUMBER_OF_ROOK_MOVES: usize = 102400;
const NUMBER_OF_BISHOP_MOVES: usize = 5248;

// TODO: Improve how public certain items are
pub struct MoveGenerator {
	rook_moves: Vec<Bitboard>,
	rook_magics: [Magic; Squares::COUNT],

	bishop_moves: Vec<Bitboard>,
	bishop_magics: [Magic; Squares::COUNT],
}

impl MoveGenerator {
	const KING_MOVES: MoveLookupTable = Self::init_king_moves();
	const KNIGHT_MOVES: MoveLookupTable = Self::init_knight_moves();

	const ROOK_MASK: MoveLookupTable = Self::init_rook_mask();
	const BISHOP_MASK: MoveLookupTable = Self::init_bishop_mask();

	const PAWN_CAPTURES: [[Bitboard; Squares::COUNT]; Sides::COUNT] = Self::init_pawn_captures();		

	pub fn new() -> Self {
		let mut generator = Self {
			rook_moves: vec![Bitboard::EMPTY; NUMBER_OF_ROOK_MOVES],
			rook_magics: [Magic::default(); Squares::COUNT],

			bishop_moves: vec![Bitboard::EMPTY; NUMBER_OF_BISHOP_MOVES],
			bishop_magics: [Magic::default(); Squares::COUNT],
		};

		MoveGenerator::init_magics::<{ Pieces::ROOK }>(&mut generator);
		MoveGenerator::init_magics::<{ Pieces::BISHOP }>(&mut generator);

		generator
	}

	pub fn generate_moves(&self, board: &Board) -> MoveList {
		let mut move_list = MoveList::default();

		self.generate_moves_for_piece::<{ Pieces::KING }>(board, &mut move_list);
		self.generate_moves_for_piece::<{ Pieces::KNIGHT }>(board, &mut move_list);
		self.generate_moves_for_piece::<{ Pieces::ROOK }>(board, &mut move_list);
		self.generate_moves_for_piece::<{ Pieces::BISHOP }>(board, &mut move_list);
		self.generate_moves_for_piece::<{ Pieces::QUEEN }>(board, &mut move_list);
		self.generate_moves_for_pawns(board, &mut move_list);

		self.generate_castling_moves(board, &mut move_list);

		move_list
	}

	fn generate_moves_for_piece<const PIECE_TYPE: Piece>(&self, board: &Board, move_list: &mut MoveList) {
		let us = board.state.side_to_move;
		let mut pieces = board.piece_bitboards[us][PIECE_TYPE];
		
		let occupancy = board.occupancy();

		while pieces > 0 {
			let from_square = bits::next(&mut pieces);

			let to_bitboard = match PIECE_TYPE {
				Pieces::KING => MoveGenerator::KING_MOVES[from_square],
				Pieces::KNIGHT => MoveGenerator::KNIGHT_MOVES[from_square],
				Pieces::ROOK => self.get_rook_moves(from_square, occupancy),
				Pieces::BISHOP => self.get_bishop_moves(from_square, occupancy),
				Pieces::QUEEN => self.get_queen_moves(from_square, occupancy),
				_ => unreachable!()
			} & !board.side_bitboards[us];

			self.add_move_to_list::<PIECE_TYPE>(board, from_square, to_bitboard, move_list);
		}
	}

	fn get_rook_moves(&self, square: Square, occupancy: Bitboard) -> Bitboard {
		let index = self.rook_magics[square].get_index(occupancy);

		self.rook_moves[index]
	}

	fn get_bishop_moves(&self, square: Square, occupancy: Bitboard) -> Bitboard {
		let index = self.bishop_magics[square].get_index(occupancy);

		self.bishop_moves[index]
	}

	fn get_queen_moves(&self, square: Square, occupancy: Bitboard) -> Bitboard {
		self.get_rook_moves(square, occupancy) ^ self.get_bishop_moves(square, occupancy)
	}

	// FIXME
	fn generate_moves_for_pawns(&self, board: &Board, move_list: &mut MoveList) {
		let us = board.state.side_to_move;
		let opponent = us ^ 1;

		let opponent_pieces = board.side_bitboards[opponent];
		let empty_squares = !board.occupancy();
		let fourth_rank = RANK_BITBOARDS[Ranks::get_fourth_rank(us)];

		let direction = if us == Sides::WHITE {
			Directions::NORTH
		} else {
			Directions::SOUTH
		};

		let rotation_count = (Squares::COUNT as i8 + direction) as u32;

		let mut pawns = board.piece_bitboards[us][Pieces::PAWN];
		while pawns > 0 {
			let from_square = bits::next(&mut pawns);
			let to = Squares::translate(from_square, direction);

			let mut moves = Bitboard::EMPTY;

			let one_step = SQUARE_BITBOARDS[to] & empty_squares;
			let two_steps = one_step.0.rotate_left(rotation_count) & empty_squares & fourth_rank;

			let targets = MoveGenerator::PAWN_CAPTURES[us][from_square];
			let captures = targets & opponent_pieces;
			let en_passant_capture = match board.state.en_passant_square {
				Some(en_passant_square) => targets & SQUARE_BITBOARDS[en_passant_square],
				None => Bitboard::EMPTY,
			};

			moves |= one_step | two_steps | captures | en_passant_capture;

			self.add_move_to_list::<{ Pieces::PAWN }>(board, from_square, moves, move_list);
		}
	}

	fn generate_castling_moves(&self, board: &Board, list: &mut MoveList) {
		let us = board.state.side_to_move;
		let opponent = us ^ 1;
		let occupancy = board.occupancy();

		let mut king_bitboard = board.piece_bitboards[us][Pieces::KING];
		let from = bits::next(&mut king_bitboard);

		if us == Sides::WHITE {
			if (board.state.castling_availability & CastlingPermissions::WHITE_KING) > 0 {
				let blockers = SQUARE_BITBOARDS[Squares::F1] | SQUARE_BITBOARDS[Squares::G1];
				let is_blocked = (occupancy & blockers) > 0; 

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E1) && !self.is_square_attacked(board, opponent, Squares::F1) {
					let to = SQUARE_BITBOARDS[from] << 2;

					self.add_move_to_list::<{ Pieces::KING }>(board, from, to, list);
				}
			}

			if (board.state.castling_availability & CastlingPermissions::WHITE_QUEEN) > 0 {
				let blockers = SQUARE_BITBOARDS[Squares::B1] | SQUARE_BITBOARDS[Squares::C1] | SQUARE_BITBOARDS[Squares::D1];
				let is_blocked = (occupancy & blockers) > 0; 

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E1) && !self.is_square_attacked(board, opponent, Squares::D1) {
					let to = SQUARE_BITBOARDS[from] >> 2;

					self.add_move_to_list::<{ Pieces::KING }>(board, from, to, list);
				}
			}
		} else {
			if (board.state.castling_availability & CastlingPermissions::BLACK_KING) > 0 {
				let blockers = SQUARE_BITBOARDS[Squares::F8] | SQUARE_BITBOARDS[Squares::G8];
				let is_blocked = (occupancy & blockers) > 0; 

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E8) && !self.is_square_attacked(board, opponent, Squares::F8) {
					let to = SQUARE_BITBOARDS[from] << 2;

					self.add_move_to_list::<{ Pieces::KING }>(board, from, to, list);
				}
			}

			if (board.state.castling_availability & CastlingPermissions::BLACK_QUEEN) > 0 {
				let blockers = SQUARE_BITBOARDS[Squares::B8] | SQUARE_BITBOARDS[Squares::C8] | SQUARE_BITBOARDS[Squares::D8];
				let is_blocked = (occupancy & blockers) > 0; 

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E8) && !self.is_square_attacked(board, opponent, Squares::D8) {
					let to = SQUARE_BITBOARDS[from] >> 2;

					self.add_move_to_list::<{ Pieces::KING }>(board, from, to, list);
				}
			}
		}
	}

	pub fn is_square_attacked(&self, board: &Board, attacker: Side, square: Square) -> bool {
		let occupancy = board.occupancy();
		let attackers = board.piece_bitboards[attacker];

		let king_moves = MoveGenerator::KING_MOVES[square];
		let knight_moves = MoveGenerator::KNIGHT_MOVES[square];
		let pawn_moves = MoveGenerator::PAWN_CAPTURES[attacker ^ 1][square];
		let rook_moves = self.get_rook_moves(square, occupancy);
		let bishop_moves = self.get_bishop_moves(square, occupancy);
		let queen_moves = rook_moves ^ bishop_moves;

		((king_moves & attackers[Pieces::KING]) > 0)
			|| ((knight_moves & attackers[Pieces::KNIGHT]) > 0)
			|| ((pawn_moves & attackers[Pieces::PAWN]) > 0)
			|| ((rook_moves & attackers[Pieces::ROOK]) > 0)
			|| ((bishop_moves & attackers[Pieces::BISHOP]) > 0)
			|| ((queen_moves & attackers[Pieces::QUEEN]) > 0)
	}

	fn add_move_to_list<const PIECE_TYPE: Piece>(&self, board: &Board, from: Square, to: Bitboard, move_list: &mut MoveList) {
		let mut to = to;

        let is_pawn = PIECE_TYPE == Pieces::PAWN;
        let promotion_rank = Ranks::get_promotion_rank(board.state.side_to_move);

		while to > 0 {
			let to_square = bits::next(&mut to);
			dbg_assert_square_in_range!(to_square);

			let capture = board.piece_list[to_square];
			let en_passant = match board.state.en_passant_square {
				Some(square) => is_pawn && (square == to_square),
				None => false
			};

			let promotion = is_pawn && (Squares::get_rank(to_square) == promotion_rank);
			let double_step = is_pawn && ((to_square as i8 - from as i8).abs() == 16);
			let castling = (PIECE_TYPE == Pieces::KING) && ((to_square as i8 - from as i8).abs() == 2);

			let mut m = MoveBuilder::piece(PIECE_TYPE);

			m.from(from).to(to_square).capture(capture);

			if en_passant { m.en_passant(); }
			if double_step { m.double_step(); }
			if castling { m.castling(); }

			if !promotion {
				move_list.push(m.to_move())
			} else {
				Pieces::PROMOTION_TARGETS.iter().for_each(|piece| {
					let piece = *piece;

					move_list.push(m.clone().promotion(piece).to_move())
				})
			}
		}
	}
}