use crate::{board::{bitboard::Bitboard, location::{Direction, Directions, Files, KnightJumps, Ranks, Square, Squares, FILE_BITBOARDS, RANK_BITBOARDS, SQUARE_BITBOARDS}, piece::{Piece, Pieces, Side, Sides}}, movegen::NUMBER_OF_ROOK_MOVES};

use super::{magics::{Magic, BISHOP_MAGIC_NUMBERS, ROOK_MAGIC_NUMBERS}, MoveGenerator, MoveLookupTable, NUMBER_OF_BISHOP_MOVES};

impl MoveGenerator {
	pub(in crate::movegen) const fn init_king_moves() -> MoveLookupTable {
		let mut moves: MoveLookupTable = [Bitboard::EMPTY; Squares::COUNT];

		let mut square: Square = 0;
		while square < Squares::COUNT { // i can't wait for the day I can finally use a for loop in a const fn
			let (rank, file) = Squares::get_coordinates(square);

			let mut directions_index = 0;
			while directions_index < Directions::ALL.len() {
				let direction = Directions::ALL[directions_index];
				directions_index += 1;
				
				// TODO: Add comments
				if (rank == Ranks::R1 && (direction == Directions::SOUTH || direction == Directions::SOUTH_EAST || direction == Directions::SOUTH_WEST))
					|| (rank == Ranks::R8 && (direction == Directions::NORTH || direction == Directions::NORTH_EAST || direction == Directions::NORTH_WEST))
					|| (file == Files::A && (direction == Directions::WEST || direction == Directions::NORTH_WEST || direction == Directions::SOUTH_WEST))
					|| (file == Files::H && (direction == Directions::EAST || direction == Directions::NORTH_EAST || direction == Directions::SOUTH_EAST))
				{
					continue;
				}

				let new_square = Squares::translate(square, direction);
				// More of a sanity check than anything else, its ran at compile time so the extra checks don't hurt
				if Squares::distance(square, new_square) == 1 {
					moves[square].0 |= SQUARE_BITBOARDS[new_square].0 // Working with the u64 directly since I can't mark my trait impls as const fns
				}
			}
			
			square += 1;
		}

		moves
	}

	pub(in crate::movegen) const fn init_knight_moves() -> MoveLookupTable {
		let mut moves: MoveLookupTable = [Bitboard::EMPTY; Squares::COUNT];

		let mut square: Square = 0;
		while square < Squares::COUNT { // i can't wait for the day I can finally use a for loop in a const fn
			let (rank, file) = Squares::get_coordinates(square);

			let mut jumps_index = 0;
			while jumps_index < KnightJumps::ALL.len() {
				let direction = KnightJumps::ALL[jumps_index];
				jumps_index += 1;
				
				// TODO: Add comments
				if (rank == Ranks::R1 && (direction == KnightJumps::LONG_SOUTH_EAST || direction == KnightJumps::LONG_SOUTH_WEST || direction == KnightJumps::SHORT_SOUTH_EAST || direction == KnightJumps::SHORT_SOUTH_WEST))
					|| (rank == Ranks::R2 && (direction == KnightJumps::LONG_SOUTH_EAST || direction == KnightJumps::LONG_SOUTH_WEST))
					|| (rank == Ranks::R8 && (direction == KnightJumps::LONG_NORTH_EAST || direction == KnightJumps::LONG_NORTH_WEST || direction == KnightJumps::SHORT_NORTH_EAST || direction == KnightJumps::SHORT_NORTH_WEST))
					|| (rank == Ranks::R7 && (direction == KnightJumps::LONG_NORTH_EAST || direction == KnightJumps::LONG_NORTH_WEST))
					|| (file == Files::A && (direction == KnightJumps::LONG_NORTH_WEST || direction == KnightJumps::LONG_SOUTH_WEST || direction == KnightJumps::SHORT_NORTH_WEST || direction == KnightJumps::SHORT_SOUTH_WEST))
					|| (file == Files::B && (direction == KnightJumps::SHORT_NORTH_WEST || direction == KnightJumps::SHORT_SOUTH_WEST))
					|| (file == Files::H && (direction == KnightJumps::LONG_NORTH_EAST || direction == KnightJumps::LONG_SOUTH_EAST || direction == KnightJumps::SHORT_NORTH_EAST || direction == KnightJumps::SHORT_SOUTH_EAST))
					|| (file == Files::G && (direction == KnightJumps::SHORT_NORTH_EAST || direction == KnightJumps::SHORT_SOUTH_EAST))
				{
					continue;
				}

				let new_square = Squares::translate(square, direction);
				// More of a sanity check than anything else, its ran at compile time so the extra checks don't hurt
				if Squares::distance(square, new_square) == 2 {
					moves[square].0 |= SQUARE_BITBOARDS[new_square].0
				}
			}
			
			square += 1;
		}

		moves
	}

	pub(in crate::movegen) const fn init_pawn_captures() -> [MoveLookupTable; Sides::COUNT] {
		let mut pawns: [[Bitboard; Squares::COUNT]; Sides::COUNT] = [[Bitboard::EMPTY; Squares::COUNT]; Sides::COUNT];

		let mut square = 0usize;
		while square < Squares::COUNT {
            let square_bitboard = SQUARE_BITBOARDS[square].0;

            let white_moves = (square_bitboard & !FILE_BITBOARDS[Files::A].0) << 7 | (square_bitboard & !FILE_BITBOARDS[Files::H].0) << 9;
            let black_moves = (square_bitboard & !FILE_BITBOARDS[Files::A].0) >> 9 | (square_bitboard & !FILE_BITBOARDS[Files::H].0) >> 7;

            pawns[Sides::WHITE][square] = Bitboard(white_moves);
            pawns[Sides::BLACK][square] = Bitboard(black_moves);

			square += 1;
        }

		pawns
	}

	pub(in crate::movegen) const fn init_rook_mask() -> MoveLookupTable {
		let mut masks: MoveLookupTable = [Bitboard::EMPTY; Squares::COUNT];

		let mut square: Square = 0;
		while square < Squares::COUNT { // i can't wait for the day I can finally use a for loop in a const fn
			let (rank, file) = Squares::get_coordinates(square);

			let rank_bitboard = RANK_BITBOARDS[rank];
			let file_bitboard = FILE_BITBOARDS[file];

			let edge_mask = (FILE_BITBOARDS[Files::A].0 & !file_bitboard.0)
				| (FILE_BITBOARDS[Files::H].0 & !file_bitboard.0)
				| (RANK_BITBOARDS[Ranks::R1].0 & !rank_bitboard.0)
				| (RANK_BITBOARDS[Ranks::R8].0 & !rank_bitboard.0);

			let mask = (rank_bitboard.0 | file_bitboard.0) & !edge_mask & !SQUARE_BITBOARDS[square].0;
			masks[square] = Bitboard(mask);

			square += 1;
		}

		masks
	}

	pub(in crate::movegen) const fn init_bishop_mask() -> MoveLookupTable {
		let mut masks: MoveLookupTable = [Bitboard::EMPTY; Squares::COUNT];

		let mut square: Square = 0;
		while square < Squares::COUNT { // i can't wait for the day I can finally use a for loop in a const fn
			let (rank, file) = Squares::get_coordinates(square);

			let rank_bitboard = RANK_BITBOARDS[rank];
			let file_bitboard = FILE_BITBOARDS[file];

			// TODO: Move to separate function?
			let edge_mask = (FILE_BITBOARDS[Files::A].0 & !file_bitboard.0)
				| (FILE_BITBOARDS[Files::H].0 & !file_bitboard.0)
				| (RANK_BITBOARDS[Ranks::R1].0 & !rank_bitboard.0)
				| (RANK_BITBOARDS[Ranks::R8].0 & !rank_bitboard.0);

			let direction_mask = {
				let mut mask: u64 = 0;

				mask |= Self::cast_ray::<{ Directions::NORTH_WEST }>(Bitboard::EMPTY, square).0;
				mask |= Self::cast_ray::<{ Directions::NORTH_EAST }>(Bitboard::EMPTY, square).0;
				mask |= Self::cast_ray::<{ Directions::SOUTH_EAST }>(Bitboard::EMPTY, square).0;
				mask |= Self::cast_ray::<{ Directions::SOUTH_WEST }>(Bitboard::EMPTY, square).0;

				mask
			};

			let mask = direction_mask & !edge_mask;
			masks[square] = Bitboard(mask);

			square += 1;
		}

		masks
	}

	pub(in crate::movegen) fn init_magics<const PIECE_TYPE: Piece>(&mut self) {
		const { 
			let is_valid = PIECE_TYPE == Pieces::ROOK || PIECE_TYPE == Pieces::BISHOP;
			assert!(is_valid, "Only rooks and bishops require magic bitboards") 
		};

		let is_rook = PIECE_TYPE == Pieces::ROOK;

		let mut offset = 0;

		for square in Squares::ALL {
			let mask = if is_rook {
				Self::ROOK_MASK
			} else {
				Self::BISHOP_MASK
			}[square];

			let bits = mask.0.count_ones();
			let permutations = 2u64.pow(bits);

			let end = offset + permutations - 1;

			let blocker_boards = {
				let mut blocker_boards: Vec<Bitboard> = Vec::new();

				let d = mask.0;
				let mut n: u64 = 0;

				loop {
					blocker_boards.push(Bitboard(n));

					n = n.wrapping_sub(d) & d;

					if n == 0 {
						break;
					}
				}

				blocker_boards
			};

			let attack_boards = {
				let mut attack_boards: Vec<Bitboard> = Vec::new();

				for blocker_board in blocker_boards.iter() {
					let blocker_board = *blocker_board;
					let attacks = if is_rook {
						MoveGenerator::cast_ray::<{ Directions::NORTH }>(blocker_board, square)
						| MoveGenerator::cast_ray::<{ Directions::EAST }>(blocker_board, square)
						| MoveGenerator::cast_ray::<{ Directions::SOUTH }>(blocker_board, square)
						| MoveGenerator::cast_ray::<{ Directions::WEST }>(blocker_board, square)
					} else {
						MoveGenerator::cast_ray::<{ Directions::NORTH_EAST }>(blocker_board, square)
						| MoveGenerator::cast_ray::<{ Directions::SOUTH_EAST }>(blocker_board, square)
						| MoveGenerator::cast_ray::<{ Directions::SOUTH_WEST }>(blocker_board, square)
						| MoveGenerator::cast_ray::<{ Directions::NORTH_WEST }>(blocker_board, square)
					};
					
					attack_boards.push(attacks);
				}

				attack_boards
			};

			let magic = Magic {
				mask,
				offset,
				shift: (64 - bits) as u8,
				number: if is_rook {
					ROOK_MAGIC_NUMBERS[square]
				} else {
					BISHOP_MAGIC_NUMBERS[square]
				}
			};

			for i in 0..permutations {
				let index = magic.get_index(blocker_boards[i as usize]);
				let table = if is_rook {
					&mut self.rook_moves[..]
				} else {
					&mut self.bishop_moves[..]
				};

				if table[index] == Bitboard::EMPTY {
					let too_low = index < (offset as usize);
					let too_high = index > (end as usize);

					assert!(!too_low && !too_high, "Invalid magics");
					
					table[index] = attack_boards[i as usize];
				} else {
					panic!("Invalid magics")
				}
			}

			if is_rook {
				self.rook_magics[square] = magic;
			} else {
				self.bishop_magics[square] = magic;
			}

			offset += permutations;
		}

		let table_size = if is_rook {
			NUMBER_OF_ROOK_MOVES
		} else {
			NUMBER_OF_BISHOP_MOVES
		} as u64;

		assert_eq!(offset, table_size, "Invalid magics")
	}

	const fn cast_ray<const DIRECTION: Direction>(bitboard: Bitboard, square: Square) -> Bitboard {
		let (mut rank, mut file) = Squares::get_coordinates(square);

		let mut square = SQUARE_BITBOARDS[square].0;
		let mut ray: u64 = 0;

		let mut done = false;
		while !done {
			done = true;

			match DIRECTION {
				Directions::NORTH => {
					if rank == Ranks::R8 { continue; }

					square <<= 8;
					ray |= square;

					rank += 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::NORTH_EAST => {
					if rank == Ranks::R8 || file == Files::H { continue; }

					square <<= 9;
					ray |= square;

					rank += 1;
					file += 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::EAST => {
					if file == Files::H { continue; }

					square <<= 1;
					ray |= square;

					file += 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::SOUTH_EAST => {
					if rank == Ranks::R1 || file == Files::H { continue; }

					square >>= 7;
					ray |= square;

					rank -= 1;
					file += 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::SOUTH => {
					if rank == Ranks::R1 { continue; }

					square >>= 8;
					ray |= square;

					rank -= 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::SOUTH_WEST => {
					if rank == Ranks::R1 || file == Files::A { continue; }

					square >>= 9;
					ray |= square;

					rank -= 1;
					file -= 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::WEST => {
					if file == Files::A { continue; }

					square >>= 1;
					ray |= square;

					file -= 1;

					done = (square & bitboard.0) > 0;
				},
				Directions::NORTH_WEST => {
					if rank == Ranks::R8 || file == Files::A { continue; }

					square <<= 7;
					ray |= square;

					rank += 1;
					file -= 1;

					done = (square & bitboard.0) > 0;
				},
				_ => unreachable!()
			}
		}

		Bitboard(ray)
	}
}