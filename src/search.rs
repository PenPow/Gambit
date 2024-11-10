use std::time::{Duration, Instant};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use options::{SearchOptions, SearchType, StopOptions};
use crate::{board::Board, comm::{CommToEngineMessage, EngineToCommMessage}, movegen::piece_move::Move};

pub mod options;

const MAX_DEPTH: u8 = 3;

pub struct Search {
	reciever: Receiver<CommToEngineMessage>,
	sender: Sender<EngineToCommMessage>,

	board: Board,

	quit: bool,
	return_best_move: bool
}

impl Search {
	pub fn new(sender: Sender<EngineToCommMessage>, reciever: Receiver<CommToEngineMessage>) -> Self {
		Self {
			sender,
			reciever,

			board: Board::from_start_pos(),

			quit: false,
			return_best_move: false,
		}
	}
	
	pub fn main_loop(&mut self) {
		while !self.quit {
			self.try_recv_message();
		}
	}

	fn try_recv_message(&mut self) {
		let message = self.reciever.try_recv();

		if let Ok(msg) = message {
			self.handle_message(msg).unwrap();
		} else {
			match unsafe { message.unwrap_err_unchecked() } {
				TryRecvError::Empty => {},
				TryRecvError::Disconnected => panic!("Disconnected from reciever")
			}
		}
	}

	fn handle_message(&mut self, message: CommToEngineMessage) -> Result<(), Box<dyn std::error::Error>> {
		match message {
			CommToEngineMessage::Stop(option) => {
				self.quit = true;

				if option == StopOptions::ReturnBestMove {
					self.return_best_move = true;
				}
			},
			CommToEngineMessage::Go(options) => {
				// TODO
			},
			CommToEngineMessage::Quit => {
				self.quit = true
			},
			#[cfg(debug_assertions)]
			CommToEngineMessage::Debug => {
				dbg!(&self.board);
			},
			CommToEngineMessage::IsReady => {
				self.sender.send(EngineToCommMessage::ReadyOk)?; // Since the engine is made synchronously, once this runs it will be ready
			},
			CommToEngineMessage::Position(fen) => {
				self.quit = true;
				self.board = Board::from_fen(fen.as_str())?;
			},
			CommToEngineMessage::UCINewGame => {
				self.quit = true;
				self.board = Board::from_start_pos();
			}
		}

		Ok(())
	}

	fn search(&mut self, options: SearchOptions) -> Result<(), Box<dyn std::error::Error + '_>> {
		let mut depth = 1;
		let mut max_depth = MAX_DEPTH;
		let mut best_move = Move::NULL;

		let move_type = options.get_type();
		if move_type == SearchType::Depth {
			max_depth = options.depth.unwrap();
		}

		let time = if options.should_calculate_timeslice() {
			let timeslot = options.calculate_time(&self.board);

			if timeslot == 0 {
				max_depth = 1;
			}

			Some(timeslot)
		} else { 
			None 
		};

		let alpha = f64::INFINITY;
		let beta = f64::NEG_INFINITY;

		let start_time = Instant::now();

		while depth < max_depth {
			self.try_recv_message();
			if self.quit { break; }

			if options.should_calculate_timeslice() {
				if start_time.elapsed() >= Duration::from_millis(time.unwrap()) {
					break;
				}
			}
		}

		if self.return_best_move {
			self.sender.send(EngineToCommMessage::BestMove(best_move));
		}

		Ok(())
	}
}