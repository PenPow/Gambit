use std::ops::Sub;

use crate::board::{bitboard::Bitboard, castling::CastlingPermissions, piece::{self, Direction, Piece, PieceType, Side, Sides}, square::{self, File, Files, Rank, Ranks, Square, Squares, FILE_BITBOARD_LOOKUP, RANK_BITBOARD_LOOKUP, SQUARE_BITBOARD_LOOKUP}, Board};

use super::{magic::{self, Magic, BISHOP_MAGIC_NUMBERS, ROOK_MAGIC_NUMBERS}, move_list::MoveList, piece_move::{Move, MoveShifts}};

pub struct MoveGenerator {
	king_attack_map: [Bitboard; 64],
	knight_attack_map: [Bitboard; 64],
	pawn_attack_map: [[Bitboard; 64]; 2],
	rook_attack_map: Vec<Bitboard>,
	bishop_attack_map: Vec<Bitboard>,

	rook_magics: [Magic; 64],
	bishop_magics: [Magic; 64],
}

impl MoveGenerator {
	pub fn new() -> Self {
		let magic = Magic::default();

		let mut move_generator = Self {
			king_attack_map: [Bitboard(0); 64],
			knight_attack_map: [Bitboard(0); 64],
			pawn_attack_map: [[Bitboard(0); 64]; 2],

			rook_attack_map: vec![Bitboard(0); 102_400],
			bishop_attack_map: vec![Bitboard(0); 5248],

			rook_magics: [magic; 64],
			bishop_magics: [magic; 64],
		};

		move_generator.init_king_map();
		move_generator.init_knight_map();
		move_generator.init_pawn_map();
		move_generator.init_magics(PieceType::ROOK);
		move_generator.init_magics(PieceType::BISHOP);

		move_generator
	}

	pub fn generate_moves(&self, board: &Board, move_list: &mut MoveList) {
		// self.generate_moves_for_piece(board, PieceType::KING, move_list);		
		// self.generate_moves_for_piece(board, PieceType::KNIGHT, move_list);
		// self.generate_moves_for_piece(board, PieceType::ROOK, move_list);
		// self.generate_moves_for_piece(board, PieceType::BISHOP, move_list);
		// self.generate_moves_for_piece(board, PieceType::QUEEN, move_list);

		self.generate_moves_for_pawn(board, move_list);

		// self.generate_castling_moves(board, move_list);
	}

	fn generate_moves_for_piece(&self, board: &Board, piece_type: Piece, move_list: &mut MoveList) {
		let us = board.state.side_to_move;
		
		let mut pieces = board.piece_bitboards[us][piece_type];

		while pieces > 0 {
			let from_square = pieces.0.trailing_zeros() as Square;
			pieces ^= 1u64 << from_square;

			let to_bitboard = match piece_type {
				PieceType::KING | PieceType::KNIGHT => {
					let attack_map = if piece_type == PieceType::KING { self.king_attack_map } else { self.knight_attack_map };

					attack_map[from_square]
				},
				PieceType::QUEEN | PieceType::ROOK | PieceType::BISHOP => self.get_slider_attacks(piece_type, from_square, board.combine_all_bitboards()),
				_ => unreachable!()
			};

			// if piece_type == PieceType::ROOK { eprintln!("{}", to_bitboard); dbg!(from_square); };

			self.add_move_to_list(board, piece_type, from_square, to_bitboard & !board.side_bitboards[us], move_list);
		}
	}

	fn generate_moves_for_pawn(&self, board: &Board, move_list: &mut MoveList) {
		let us = board.state.side_to_move;

		let mut pieces = board.piece_bitboards[us][PieceType::PAWN];

		let direction: i8 = if us == Sides::WHITE { 8 } else { -8 };

		let empty_squares = !board.combine_all_bitboards();
		let fourth_rank = RANK_BITBOARD_LOOKUP[Ranks::fourth_rank(us)];

		while pieces > 0 {
			let from_square = pieces.0.trailing_zeros() as Square;
			pieces ^= 1u64 << from_square;
			
			let to = (from_square as i8 + direction) as usize;			
			let mut to_bitboard = Bitboard(0);
			
			let one_step = SQUARE_BITBOARD_LOOKUP[to] & empty_squares;
			let two_steps = Bitboard(one_step.0.rotate_left((64 +  direction) as u32)) & empty_squares & fourth_rank;
			to_bitboard |= one_step | two_steps;

			let targets =  self.pawn_attack_map[us][from_square];
			let captures = targets & board.side_bitboards[us ^ 1];
			let en_passant_capture = match board.state.en_passant_square {
				Some(square) => targets & SQUARE_BITBOARD_LOOKUP[square],
				None => Bitboard(0)
			};

			to_bitboard |= captures | en_passant_capture;

			self.add_move_to_list(board, PieceType::PAWN, from_square, to_bitboard, move_list);
		}
	}

	fn generate_castling_moves(&self, board: &Board, move_list: &mut MoveList) {
		let us = board.state.side_to_move;
		let opponent = us ^ 1;

		let mut pieces = board.piece_bitboards[us][PieceType::KING];

		let from_square = pieces.0.trailing_zeros() as Square;
		pieces ^= 1u64 << from_square;

		let occupied_squares = board.combine_all_bitboards();

		if us == Sides::WHITE && (board.state.castling_availability & (CastlingPermissions::WHITE_KING | CastlingPermissions::WHITE_QUEEN)) > 0 {
			if board.state.castling_availability & CastlingPermissions::WHITE_KING > 0 {
				let is_blocked = (occupied_squares & (SQUARE_BITBOARD_LOOKUP[Squares::F1] | SQUARE_BITBOARD_LOOKUP[Squares::G1])) > 0;

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E1) && !self.is_square_attacked(board, opponent, Squares::F1) {
					let to_bitboard = SQUARE_BITBOARD_LOOKUP[from_square] << 2;
					self.add_move_to_list(board, PieceType::KING, from_square, to_bitboard, move_list);
				}
			}

			if board.state.castling_availability & CastlingPermissions::WHITE_QUEEN > 0 {
				let is_blocked = (occupied_squares & (SQUARE_BITBOARD_LOOKUP[Squares::B1] | SQUARE_BITBOARD_LOOKUP[Squares::C1] | SQUARE_BITBOARD_LOOKUP[Squares::D1])) > 0;

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E1) && !self.is_square_attacked(board, opponent, Squares::D1) {
					let to_bitboard = SQUARE_BITBOARD_LOOKUP[from_square] >> 2;
					self.add_move_to_list(board, PieceType::KING, from_square, to_bitboard, move_list);
				}
			}
		}

		if us == Sides::BLACK && (board.state.castling_availability & (CastlingPermissions::BLACK_KING | CastlingPermissions::BLACK_QUEEN)) > 0 {
			if board.state.castling_availability & CastlingPermissions::BLACK_KING > 0 {
				let is_blocked = (occupied_squares & (SQUARE_BITBOARD_LOOKUP[Squares::F8] | SQUARE_BITBOARD_LOOKUP[Squares::G8])) > 0;

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E8) && !self.is_square_attacked(board, opponent, Squares::F8) {
					let to_bitboard = SQUARE_BITBOARD_LOOKUP[from_square] << 2;
					self.add_move_to_list(board, PieceType::KING, from_square, to_bitboard, move_list);
				}
			}

			if board.state.castling_availability & CastlingPermissions::BLACK_QUEEN > 0 {
				let is_blocked = (occupied_squares & (SQUARE_BITBOARD_LOOKUP[Squares::B8] | SQUARE_BITBOARD_LOOKUP[Squares::C8] | SQUARE_BITBOARD_LOOKUP[Squares::D8])) > 0;

				if !is_blocked && !self.is_square_attacked(board, opponent, Squares::E8) && !self.is_square_attacked(board, opponent, Squares::D8) {
					let to_bitboard = SQUARE_BITBOARD_LOOKUP[from_square] >> 2;
					self.add_move_to_list(board, PieceType::KING, from_square, to_bitboard, move_list);
				}
			}
		}
	}

	pub fn is_square_attacked(&self, board: &Board, attacker: Side, square: Square) -> bool {
		let attackers = board.piece_bitboards[attacker];

		let king_attacking = (self.king_attack_map[square] & attackers[PieceType::KING]) > 0;
		let knight_attacking = (self.knight_attack_map[square] & attackers[PieceType::KNIGHT]) > 0;
		let pawn_attacking = (self.pawn_attack_map[attacker ^ 1][square] & attackers[PieceType::PAWN]) > 0;

		let all_squares = board.combine_all_bitboards();
		
		let rook_attacks = self.get_slider_attacks(PieceType::ROOK, square, all_squares) & attackers[PieceType::ROOK];
		let bishop_attacks = self.get_slider_attacks(PieceType::BISHOP, square, all_squares) & attackers[PieceType::BISHOP];
		let queen_attacks = rook_attacks | bishop_attacks;

		let rook_attacking = rook_attacks > 0;
		let bishop_attacking = bishop_attacks > 0;
		let queen_attacking = queen_attacks > 0;

		return king_attacking
			|| knight_attacking
			|| pawn_attacking
			|| rook_attacking
			|| bishop_attacking
			|| queen_attacking;
	}

	fn add_move_to_list(&self, board: &Board, piece_type: Piece, from: Square, to: Bitboard, move_list: &mut MoveList) {
		let mut to = to;

		let us = board.state.side_to_move;
		let promotion_rank = Ranks::promotion_rank(us);

		while to > 0 {
			let to_square = to.0.trailing_zeros() as Square;
			to ^= 1u64 << to_square;

			let is_pawn = piece_type == PieceType::PAWN;

			let capture = board.piece_list[to_square];
			let en_passant_square = match board.state.en_passant_square {
				Some(square) => is_pawn && (square == to_square),
				None => false
			};

			let promotion = is_pawn && Squares::on_rank(to_square, promotion_rank);
			let double_step = is_pawn && ((to_square as i8) - (from as i8).abs() == 16);
			let castling = (piece_type == PieceType::KING) && (((to_square as i8) - (from as i8)).abs() == 2);

			let mut move_data = piece_type
				| from << MoveShifts::FROM
				| to_square << MoveShifts::TO
				| capture << MoveShifts::CAPTURE
				| (en_passant_square as usize) << MoveShifts::EN_PASSANT
				| (double_step as usize) << MoveShifts::DOUBLE_STEP
				| (castling as usize) << MoveShifts::CASTLING;

			if piece_type == PieceType::QUEEN { panic!() }
			
			if !promotion {
				move_data |= PieceType::NONE << MoveShifts::PROMOTION;
				move_list.push(Move::new(move_data));
			} else {
				[PieceType::QUEEN, PieceType::ROOK, PieceType::BISHOP, PieceType::KNIGHT].iter().for_each(|piece| {
					let promotion_piece = *piece << MoveShifts::PROMOTION;
					move_list.push(Move::new(move_data | promotion_piece));
				})
			}
		}
	}

	fn get_slider_attacks(&self, piece_type: Piece, from: Square, all_squares: Bitboard) -> Bitboard {
		match piece_type {
			PieceType::ROOK => {
				let index = self.rook_magics[from].get_index(all_squares);

                self.rook_attack_map[index]
			},
			PieceType::BISHOP => {
				let index = self.bishop_magics[from].get_index(all_squares);

                self.bishop_attack_map[index]
			},
			PieceType::QUEEN => {
                let rook_index = self.rook_magics[from].get_index(all_squares);
                let bishop_index = self.bishop_magics[from].get_index(all_squares);

                self.rook_attack_map[rook_index] ^ self.bishop_attack_map[bishop_index]
            },
			_ => unreachable!()
		}
	}

	fn init_king_map(&mut self) {
		for i in Squares::ALL_SQUARES_RANGE {
			let square = SQUARE_BITBOARD_LOOKUP[i];

			let moves = 
				(square & !FILE_BITBOARD_LOOKUP[Files::A] & !RANK_BITBOARD_LOOKUP[Ranks::R8]) << 7
				| (square & !RANK_BITBOARD_LOOKUP[Ranks::R8]) << 8
				| (square & !FILE_BITBOARD_LOOKUP[Files::H] & !RANK_BITBOARD_LOOKUP[Ranks::R8]) << 9
				| (square & !RANK_BITBOARD_LOOKUP[Files::H]) << 1
				| (square & !FILE_BITBOARD_LOOKUP[Files::H] & !RANK_BITBOARD_LOOKUP[Ranks::R1]) >> 7
				| (square & !RANK_BITBOARD_LOOKUP[Ranks::R1]) >> 8
				| (square & !FILE_BITBOARD_LOOKUP[Files::A] & !RANK_BITBOARD_LOOKUP[Ranks::R1]) >> 9
				| (square & !FILE_BITBOARD_LOOKUP[Files::A]) >> 1;
			
			self.king_attack_map[i] = moves;
		}
	}

	fn init_knight_map(&mut self) {
		for i in Squares::ALL_SQUARES_RANGE {
			let square = SQUARE_BITBOARD_LOOKUP[i];

			let moves = 
				(square & !RANK_BITBOARD_LOOKUP[Ranks::R8] & !RANK_BITBOARD_LOOKUP[Ranks::R7] & !FILE_BITBOARD_LOOKUP[Files::A]) << 15
				| (square & !RANK_BITBOARD_LOOKUP[Ranks::R8] & !RANK_BITBOARD_LOOKUP[Ranks::R7] & !FILE_BITBOARD_LOOKUP[Files::H]) << 17
				| (square & !FILE_BITBOARD_LOOKUP[Files::A] & !FILE_BITBOARD_LOOKUP[Files::B] & !RANK_BITBOARD_LOOKUP[Ranks::R8]) << 6
				| (square & !FILE_BITBOARD_LOOKUP[Files::G] & !FILE_BITBOARD_LOOKUP[Files::H] & !RANK_BITBOARD_LOOKUP[Ranks::R8]) << 10
				| (square & !RANK_BITBOARD_LOOKUP[Ranks::R1] & !RANK_BITBOARD_LOOKUP[Ranks::R2] & !FILE_BITBOARD_LOOKUP[Files::A]) >> 17
				| (square & !RANK_BITBOARD_LOOKUP[Ranks::R1] & !RANK_BITBOARD_LOOKUP[Ranks::R2] & !FILE_BITBOARD_LOOKUP[Files::H]) >> 15
				| (square & !FILE_BITBOARD_LOOKUP[Files::A] & !FILE_BITBOARD_LOOKUP[Files::B] & !RANK_BITBOARD_LOOKUP[Ranks::R1]) >> 10
				| (square & !FILE_BITBOARD_LOOKUP[Files::G] & !FILE_BITBOARD_LOOKUP[Files::H] & !RANK_BITBOARD_LOOKUP[Ranks::R1]) >> 6;
			
			self.knight_attack_map[i] = moves;
		}
	}

	fn init_pawn_map(&mut self) {
		for i in Squares::ALL_SQUARES_RANGE {
			let square = SQUARE_BITBOARD_LOOKUP[i];

			let white_moves = 
				(square & !FILE_BITBOARD_LOOKUP[Files::A]) << 7
				| (square & !FILE_BITBOARD_LOOKUP[Files::H]) << 9;

			let black_moves = 
				(square & !FILE_BITBOARD_LOOKUP[Files::A]) >> 9
				| (square & !FILE_BITBOARD_LOOKUP[Files::H]) >> 7;
			
			self.pawn_attack_map[Sides::WHITE][i] = white_moves;
			self.pawn_attack_map[Sides::BLACK][i] = black_moves;
		}
	}

	fn create_rook_mask(&self, square: Square) -> Bitboard {
		let (rank, file) = Squares::get_location(square);

		let square_bitboard = SQUARE_BITBOARD_LOOKUP[square];

		let rank_bitboard = RANK_BITBOARD_LOOKUP[rank];
		let file_bitboard = FILE_BITBOARD_LOOKUP[file];

		(rank_bitboard | file_bitboard) & !square_bitboard & !self.get_edge_mask(rank_bitboard, file_bitboard)
	}

	fn create_bishop_mask(&self, square: Square) -> Bitboard {
		let (rank, file) = Squares::get_location(square);

		let square_bitboard = SQUARE_BITBOARD_LOOKUP[square];

		let rank_bitboard = RANK_BITBOARD_LOOKUP[rank];
		let file_bitboard = FILE_BITBOARD_LOOKUP[file];

		let north_west = self.cast_ray(Bitboard(0), square_bitboard, rank, file, Direction::NorthWest);
		let north_east = self.cast_ray(Bitboard(0), square_bitboard, rank, file, Direction::NorthEast);
		let south_west = self.cast_ray(Bitboard(0), square_bitboard, rank, file, Direction::SouthWest);
		let south_east = self.cast_ray(Bitboard(0), square_bitboard, rank, file, Direction::SouthEast);

		!self.get_edge_mask(rank_bitboard, file_bitboard) & (north_west | north_east | south_west | south_east)
	}

	fn cast_ray(&self, bitboard: Bitboard, square_bitboard: Bitboard, rank: Rank, file: File, direction: Direction) -> Bitboard {
		let mut square_bitboard = square_bitboard;
		let mut rank = rank;
		let mut file = file;
		
		let mut ray = Bitboard(0);
		
		let mut done = false;
		while !done {
			done = true;

			match direction {
				Direction::North => {
					if rank != Ranks::R8 {
						square_bitboard <<= 8;

						ray |= square_bitboard;
						rank += 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::NorthEast => {
					if rank != Ranks::R8 && file != Files::H {
						square_bitboard <<= 9;

						ray |= square_bitboard;
						rank += 1;
						file += 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::East => {
					if file != Files::H {
						square_bitboard <<= 1;

						ray |= square_bitboard;
						file += 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::SouthEast => {
					if rank != Ranks::R1 && file != Files::H {
						square_bitboard >>= 7;

						ray |= square_bitboard;
						rank -= 1;
						file += 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::South => {
					if rank != Ranks::R1 {
						square_bitboard >>= 8;

						ray |= square_bitboard;
						rank -= 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::SouthWest => {
					if rank != Ranks::R1 && file != Files::A {
						square_bitboard >>= 9;

						ray |= square_bitboard;
						rank -= 1;
						file -= 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::West => {
					if file != Files::A {
						square_bitboard >>= 1;

						ray |= square_bitboard;
						file -= 1;

						done = (square_bitboard & bitboard) > 0;
					}
				},
				Direction::NorthWest => {
					if rank != Ranks::R8 && file != Files::A {
						square_bitboard <<= 7;

						ray |= square_bitboard;
						rank += 1;
						file -= 1;

						done = (square_bitboard & bitboard) > 0;
					}
				}
			};
		};

		ray
	}

	fn get_edge_mask(&self, rank_bitboard: Bitboard, file_bitboard: Bitboard) -> Bitboard {
		(FILE_BITBOARD_LOOKUP[Files::A] & !file_bitboard)
			| (FILE_BITBOARD_LOOKUP[Files::H] & !file_bitboard)
			| (RANK_BITBOARD_LOOKUP[Ranks::R1] & !rank_bitboard)
			| (RANK_BITBOARD_LOOKUP[Ranks::R8] & !rank_bitboard)
	}

	fn generate_rook_attack_boards(&self, square: Square, blockers: &[Bitboard]) -> Vec<Bitboard> {
		let mut attack_boards: Vec<Bitboard> = vec![];

		let (rank, file) = Squares::get_location(square);
		let square_bitboard = SQUARE_BITBOARD_LOOKUP[square];

		for blocker in blockers.iter() {
			let attacks = self.cast_ray(*blocker, square_bitboard, rank, file, Direction::North)
				| self.cast_ray(*blocker, square_bitboard, rank, file, Direction::East)
				| self.cast_ray(*blocker, square_bitboard, rank, file, Direction::South)
				| self.cast_ray(*blocker, square_bitboard, rank, file, Direction::West);

			attack_boards.push(attacks);
		}

		attack_boards
	}

	fn generate_bishop_attack_boards(&self, square: Square, blockers: &[Bitboard]) -> Vec<Bitboard> {
		let mut attack_boards: Vec<Bitboard> = vec![];

		let (rank, file) = Squares::get_location(square);
		let square_bitboard = SQUARE_BITBOARD_LOOKUP[square];

		for blocker in blockers.iter() {
			let attacks = self.cast_ray(*blocker, square_bitboard, rank, file, Direction::NorthEast)
				| self.cast_ray(*blocker, square_bitboard, rank, file, Direction::SouthEast)
				| self.cast_ray(*blocker, square_bitboard, rank, file, Direction::SouthWest)
				| self.cast_ray(*blocker, square_bitboard, rank, file, Direction::NorthWest);

			attack_boards.push(attacks);
		}

		attack_boards
	}

	fn init_magics(&mut self, piece: Piece) {
		let is_rook = piece == PieceType::ROOK;

		let mut offset = 0;

		for square in Squares::ALL_SQUARES_RANGE {
			let mask = if is_rook { self.create_rook_mask(square) } else { self.create_bishop_mask(square) };

			let bits = mask.0.count_ones();
			let permutations = 2u64.pow(bits);

			let blocker_boards = {
				let mut blocker_boards: Vec<Bitboard> = Vec::new();

				let mut n = Bitboard(0);

				loop {
					blocker_boards.push(n);
					n = Bitboard(n.0.wrapping_sub(mask.0) & mask.0);

					if n == 0 { break; }
				}

				blocker_boards
			};

			let attack_boards = if is_rook { 
				self.generate_rook_attack_boards(square, &blocker_boards)
			} else { 
				self.generate_bishop_attack_boards(square, &blocker_boards) 
			};

			let magic = Magic {
				mask,
				offset,
				shift: (64 - bits) as usize,

				number: if is_rook {
					ROOK_MAGIC_NUMBERS[square]
				} else {
					BISHOP_MAGIC_NUMBERS[square]
				}
			};


			for i in 0..permutations {
				let index = magic.get_index(blocker_boards[i as usize]);

				let table = if is_rook { &mut self.rook_attack_map[..] } else { &mut self.bishop_attack_map[..] };
				
				table[index] = attack_boards[i as usize];
			}

			if is_rook {
				self.rook_magics[square] = magic;
			} else {
				self.bishop_magics[square] = magic;
			}

			offset += permutations;
		}
	}
}