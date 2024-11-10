mod uci;
mod board;
mod macros;
mod moves;
mod fen;
mod search;

use board::{bitboard::Bitboard, piece::{Piece, PieceType}, square::{Squares, SQUARE_BITBOARD_LOOKUP}, Board};

use fen::FENError;
use moves::{move_list::{self, MoveList}, movegen::MoveGenerator, piece_move::{Move, MoveShifts}};
use search::transposition_table::{self, TTEntry, TranspositionTable};
use uci::UCI;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn perft(board: &mut Board, move_generator: &MoveGenerator, transposition_table: &mut TranspositionTable, depth: u8) -> u64 {
	let mut ml = MoveList::new();

	move_generator.generate_moves(board, &mut ml);
	
	if depth == 1 {
		return ml.length as u64
	}

	// if let Some(leaf_nodes) = transposition_table.get(board.state.zobrist_key) {
	// 	return leaf_nodes.nodes;
	// }
	
	let mut nodes: u64 = 0;

	for i in 0..ml.length {
		let legal = board.make_move(ml.get(i), move_generator);

		if legal { 
			nodes += perft(board, move_generator, transposition_table, depth - 1);

			// dbg!(ml.get(i).debug());

			board.unmake_move() 
		}
	}

	// transposition_table.insert(board.state.zobrist_key, TTEntry { hash: board.state.zobrist_key, nodes });

	nodes
}

fn main() -> Result<(), FENError> {	
	// let mut board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ")?;
	let mut board = Board::from_start_pos();
	let mut tt = TranspositionTable::new();
	let move_generator = MoveGenerator::new();

	// UCI::new(board).init();

	dbg!(perft(&mut board, &move_generator, &mut tt, 3));
	// assert_eq!(perft(&mut board, &move_generator, &mut tt, 2), 400);
	// assert_eq!(perft(&mut board, &move_generator, &mut tt, 3), 8902);
	// assert_eq!(perft(&mut board, &move_generator, &mut tt, 5), 4865609);

	// assert_eq!(perft(&mut board, &mut tt, 5), 4865609);
	// assert_eq!(perft(&mut board, 4), 197281);

	// board.debug();

	// let move_data = PieceType::PAWN
	// 	| (Squares::C2) << MoveShifts::FROM
	// 	| (Squares::C4) << MoveShifts::TO
	// 	| 1 << MoveShifts::DOUBLE_STEP;

	// dbg!(format!("{:b}", move_data));

	// let piece_move = Move::new(move_data);
	// board.make_move(piece_move);

	// board.debug();

	// board.unmake_move();

	// board.debug();

	Ok(())
}