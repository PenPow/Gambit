#[cfg(not(target_pointer_width = "64"))]
compile_error!("Gambit requires a 64 bit compilation target");

mod board;
mod comm;
mod engine;
mod helpers;
mod macros;
mod movegen;
mod uci;

use std::thread;
use engine::Engine;
use movegen::{piece_move::Move, MoveGenerator};
use uci::UCI;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
	// Setup a channel to pass messages between the frontend and the engine
	let (engine_sender, comm_reciever) = crossbeam_channel::unbounded::<comm::EngineToCommMessage>();
	let (comm_sender, engine_reciever) = crossbeam_channel::unbounded::<comm::CommToEngineMessage>();
	
	let mut engine = Engine::new(engine_sender, engine_reciever);
	let mut uci = UCI::new(comm_sender, comm_reciever);
	
	// Spawn the engine in a separate thread to allow pondering, move generation, and search and evaluation while still receiving commands
	// FIXME: How will I handle search? Maybe it runs in another thread, spawned when search begins, with another channel between engine and search? 
	thread::spawn(move || {
		engine.main_loop()
	});

	uci.main_loop();
}