use crossbeam_channel::{Receiver, Sender, TryRecvError};

use crate::{board::{location::Squares, piece::Sides, Board}, comm::{CommToEngineMessage, EngineToCommMessage}, movegen::MoveGenerator};

pub struct Engine {
	reciever: Receiver<CommToEngineMessage>,
	sender: Sender<EngineToCommMessage>,
	board: Board,

	quit: bool,
}

impl Engine {
	pub fn new(sender: Sender<EngineToCommMessage>, reciever: Receiver<CommToEngineMessage>) -> Self {
		Self {
			sender,
			reciever,
			// board: Board::from_start_pos(),
			board: Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ").unwrap(),

			quit: false
		}
	}
	
	pub fn main_loop(&mut self) {
		while !self.quit {
			let message = self.reciever.try_recv();

			if let Ok(msg) = message {
				self.handle_message(msg);
			} else {
				match unsafe { message.unwrap_err_unchecked() } {
					TryRecvError::Empty => {},
					TryRecvError::Disconnected => panic!("Disconnected from reciever")
				}
			}
		}
	}

	fn handle_message(&mut self, message: CommToEngineMessage) {
		match message {
			CommToEngineMessage::Quit => {
				// TODO: Handle shutdown gracefully with search thread
				self.quit = true
			},
			CommToEngineMessage::Debug => {
				dbg!(&self.board);
				dbg!(MoveGenerator::new().generate_moves(&self.board).len());
			},
			CommToEngineMessage::IsReady => {
				self.sender.send(EngineToCommMessage::ReadyOk); // Since the engine is made synchronously, once this runs it will be ready
			}
		}
	}
}