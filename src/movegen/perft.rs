use std::time::Instant;
use crate::{board::Board, movegen::MoveGenerator, generate_perft_tests, count};

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

generate_perft_tests! {
	[starter_fen, "Starter FEN", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", [20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956]],
	[kiwipete, "Kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", [48, 2039, 97862, 4085603, 193690690, 8031647685]],
	[cpw_pos_3, "CPW Perft Position 3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", [14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393]],
	[cpw_pos_4, "CPW Perft Position 4", "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", [6, 264, 9467, 422333, 15833292, 706045033]],
	[cpw_pos_4_mirrored, "CPW Perft Position 4 (Mirrored)", "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1", [6, 264, 9467, 422333, 15833292, 706045033]],
	[cpw_pos_5, "CPW Perft Position 5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", [44, 1486, 62379, 2103487, 89941194]],
	[cpw_pos_6, "CPW Perft Position 6", "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", [46, 2079, 89890, 3894594, 164075551, 6923051137, 287188994746, 11923589843526, 490154852788714]]
}