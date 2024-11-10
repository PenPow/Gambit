#[cfg(not(target_pointer_width = "64"))]
compile_error!("Gambit requires a 64 bit compilation target");

mod board;
mod comm;
mod helpers;
mod macros;
mod movegen;
mod search;
mod uci;

use std::thread;
use search::Search;
use uci::UCI;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
	// Set up a channel to pass messages between the frontend and the engine
	let (engine_sender, comm_reciever) = crossbeam_channel::unbounded::<comm::EngineToCommMessage>();
	let (comm_sender, engine_reciever) = crossbeam_channel::unbounded::<comm::CommToEngineMessage>();
	
	let mut engine = Search::new(engine_sender, engine_reciever);
	let mut uci = UCI::new(comm_sender, comm_reciever);
	
	thread::spawn(move || {
		engine.main_loop()
	});

	uci.main_loop();
}