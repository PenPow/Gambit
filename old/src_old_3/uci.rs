use std::thread;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use crate::{board::location::Squares, comm::{CommToEngineMessage, EngineToCommMessage}, VERSION};

pub struct UCI {
	engine_reciever: Receiver<EngineToCommMessage>,
	engine_sender: Sender<CommToEngineMessage>,

	exit: bool,

	stdin_reciever: Receiver<String>
}

impl UCI {
	pub fn new(engine_sender: Sender<CommToEngineMessage>, engine_reciever: Receiver<EngineToCommMessage>) -> Self {
		Self {
			engine_sender,
			engine_reciever,

			exit: false,

			stdin_reciever: UCI::spawn_stdin_thread(),
		}
	}

	pub fn main_loop(&mut self) {
		while !self.exit {
			self.try_recv_stdin();
			self.try_recv_engine_message();
		}
	}

	fn spawn_stdin_thread() -> Receiver<String> {
		let (sender, reciever) = crossbeam_channel::unbounded::<String>();

		thread::spawn(move || {
			loop {
				let next_line = std::io::stdin().lines().next().unwrap().unwrap();

				sender.send(next_line).unwrap();
			}
		});

		reciever
	}

	fn try_recv_engine_message(&self) {
		let message = self.engine_reciever.try_recv();

		if let Ok(msg) = message {
			UCI::handle_incoming_engine_message(msg);
		} else {
			match unsafe { message.unwrap_err_unchecked() } {
				TryRecvError::Empty => {},
				TryRecvError::Disconnected => panic!("Disconnected from reciever")
			}
		}
	}

	fn try_recv_stdin(&mut self) {
		let message = self.stdin_reciever.try_recv();

		if let Ok(msg) = message {
			self.handle_incoming_stdin(msg);
		} else {
			match unsafe { message.unwrap_err_unchecked() } {
				TryRecvError::Empty => {},
				TryRecvError::Disconnected => panic!("Disconnected from reciever")
			}
		}
	}

	fn handle_incoming_engine_message(message: EngineToCommMessage) {
		match message {
			EngineToCommMessage::ReadyOk => println!("readyok"),
		}
	}

	fn handle_incoming_stdin(&mut self, message: String) {
		match message.as_str() {
			"uci" => {
				println!("id name Gambit {}", VERSION);
				println!("id author Joshua Clements");

				println!("uciok");
			},
			"isready" => {
				self.engine_sender.send(CommToEngineMessage::IsReady);
			},
			"debug" => {
				self.engine_sender.send(CommToEngineMessage::Debug);
			},
			"quit" => {
				self.engine_sender.send(CommToEngineMessage::Quit);
				self.exit = true;
			},
			_ => panic!("Unexpected UCI command")
		}
	}
}