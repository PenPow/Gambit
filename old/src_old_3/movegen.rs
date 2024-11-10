use magics::Magic;
use move_list::MoveList;
use piece_move::MoveBuilder;
use crate::{board::{bitboard::Bitboard, castling::CastlingPermissions, location::{Ranks, Square, Squares, RANK_BITBOARDS, SQUARE_BITBOARDS}, piece::{Piece, Pieces, Side, Sides}, Board}, dbg_assert_square_in_range, helpers::bits};

pub mod move_list;
pub mod piece_move;
mod init;
mod magics;

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

		// self.generate_moves_for_piece::<{ Pieces::KING }>(board, &mut move_list);
		// self.generate_moves_for_piece::<{ Pieces::KNIGHT }>(board, &mut move_list);
		// self.generate_moves_for_piece::<{ Pieces::ROOK }>(board, &mut move_list);
		// self.generate_moves_for_piece::<{ Pieces::BISHOP }>(board, &mut move_list);
		// self.generate_moves_for_piece::<{ Pieces::QUEEN }>(board, &mut move_list);
		self.generate_moves_for_pawns(board, &mut move_list);

		// self.generate_castling_moves(board, &mut move_list);

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

	fn generate_moves_for_pawns(&self, board: &Board, move_list: &mut MoveList) {
		let us = board.state.side_to_move;
		let mut pawns = board.piece_bitboards[us][Pieces::PAWN];

		let empty_squares = !board.occupancy();
		let direction = Sides::get_pawn_movement_direction(us);

		while pawns > 0 {
			let from_square = bits::next(&mut pawns);
			let to_square = Squares::translate(from_square, direction);

			let one_step_moves = SQUARE_BITBOARDS[to_square] & empty_squares;
			let two_step_moves = one_step_moves.0.rotate_left(((Squares::COUNT as i8) + direction) as u32) & empty_squares & RANK_BITBOARDS[Ranks::get_fourth_rank(us)];

			let targets = MoveGenerator::PAWN_CAPTURES[us][from_square];
			let captures = targets & board.side_bitboards[us ^ 1];

			let en_passant_capture = match board.state.en_passant_square {
				Some(en_passant_square) => targets & SQUARE_BITBOARDS[en_passant_square],
				None => Bitboard::EMPTY
			};

			let moves = one_step_moves | two_step_moves | captures | en_passant_capture;

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

		while to > 0 {
			let to_square = bits::next(&mut to);
			dbg_assert_square_in_range!(to_square);

			let is_pawn = const { PIECE_TYPE == Pieces::PAWN };
			let capture: Piece = board.piece_list[to_square];

			let mut binary_move = MoveBuilder::piece(PIECE_TYPE);

			binary_move
				.from(from)
				.to(to_square)
				.capture(capture);

			let is_en_passant_attack = match board.state.en_passant_square {
				Some(square) => is_pawn && (square == to_square),
				None => false
			};

			if is_en_passant_attack {
				binary_move.en_passant();
			}

			let difference = Squares::distance(from, to_square);

			let is_double_step = is_pawn && (difference == 16);
			if is_double_step {
				binary_move.double_step();
			}

			let is_castling = (PIECE_TYPE == Pieces::KING) && (difference == 2);
			if is_castling {
				binary_move.castling();
			}

			let is_promotion = is_pawn && Ranks::square_is_on_rank(to_square, Ranks::get_promotion_rank(board.state.side_to_move));
			if !is_promotion {
				move_list.push(binary_move.to_move());
			} else {
				Pieces::PROMOTION_TARGETS.iter().for_each(|piece| {
					move_list.push(binary_move.clone().promotion(*piece).to_move());
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::time::Instant;

	use super::*;

	fn perft_internal(board: &mut Board, depth: u8, move_generator: &MoveGenerator) -> u64 {
		if depth == 0 { return 1 }

		let move_list = move_generator.generate_moves(board);

		let mut nodes = 0;

		for m in move_list {
			let is_legal = board.make_move(m);
			if is_legal {
				nodes += perft_internal(board, depth - 1, move_generator);

				board.unmake_move();
			}
		}

		nodes
	}

	fn perft(fen: &str, depth: u8) -> Vec<u64> {
		let move_generator = MoveGenerator::new();
		let mut board = Board::from_fen(fen).unwrap();

		let mut total_time: u128 = 0;
		let mut total_nodes: u64 = 0;

		let mut nodes: Vec<u64> = Vec::with_capacity(depth as usize);

		for depth in 1..=depth {
			let start = Instant::now();
			let mut leaf_nodes = 0;

			let nodes_searched = perft_internal(&mut board, depth, &move_generator);
			nodes.push(nodes_searched);
			leaf_nodes += nodes_searched;

			let elapsed = start.elapsed().as_millis();
			let leaves_per_second = ((leaf_nodes * 1000) as f64 / elapsed as f64).floor();

			total_time += elapsed;
			total_nodes += leaf_nodes;

			println!("Perft({}) = {leaf_nodes} ({elapsed}ms, {leaves_per_second} leaves/sec)", depth)
		}

		let final_lnps = ((total_nodes * 1000) as f64 / total_time as f64).floor();
		println!("Total time spent: {total_time}ms");
		println!("Total leaves searched: {total_nodes}");
		println!("Execution speed: {final_lnps} leaves/sec");

		nodes
	}

	#[test]
	fn test_perft_1() {
		const FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
		const DEPTH: u8 = 5;

		const EXPECTED: [u64; DEPTH as usize] = [20, 400, 8902, 197281, 4865609];

		let perft_result = perft(FEN, DEPTH);

		for (expected, actual) in EXPECTED.iter().zip(perft_result) {
			let expected = *expected;

			assert_eq!(expected, actual);
		}
	}

	#[test]
	fn test_perft_2() {
		const FEN: &'static str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
		const DEPTH: u8 = 2;

		const EXPECTED: [u64; DEPTH as usize] = [48, 2039];

		let perft_result = perft(FEN, DEPTH);

		for (expected, actual) in EXPECTED.iter().zip(perft_result) {
			let expected = *expected;

			assert_eq!(expected, actual);
		}
	}

	#[test]
	fn test_perft_3() {
		const FEN: &'static str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
		const DEPTH: u8 = 3;

		const EXPECTED: [u64; DEPTH as usize] = [14, 191, 2812];

		let perft_result = perft(FEN, DEPTH);

		for (expected, actual) in EXPECTED.iter().zip(perft_result) {
			let expected = *expected;

			assert_eq!(expected, actual);
		}
	}
}