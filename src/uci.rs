use std::{thread};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use crate::{board::Board, comm::{CommToEngineMessage, EngineToCommMessage}, search::options::{SearchOptions, StopOptions}, VERSION};

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
			let args: Vec<&str> = msg.split_whitespace().collect();
			self.handle_incoming_stdin(args[0], args[1..].to_vec());
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
			EngineToCommMessage::BestMove(_) => todo!(),
		}
	}

	fn handle_incoming_stdin(&mut self, command: &str, args: Vec<&str>) {
		match command {
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
			"position" => {
				self.engine_sender.send(CommToEngineMessage::Stop(StopOptions::TerminateSearch));

				let fen = match args[0] {
					"fen" => {
						args
							.iter()
							.skip(1)
							.take_while(|&&x| x != "moves")
							.copied()
							.collect::<Vec<&str>>()
							.join(" ")
					},
					"startpos" => String::from(Board::STARTING_POSITION_FEN),
					_ => panic!("Invalid position command recieved")
				};

				if args.contains(&"moves") { todo!("Gambit doesn't support moves in position commands") };

				self.engine_sender.send(CommToEngineMessage::Position(fen));
			},
			"go" => {
				if args.contains(&"ponder") { return; } // TODO: Implement pondering support

				let mut options = SearchOptions::default();

				let mut iterator = args.iter();
				while let Some(arg) = iterator.next() {
					match *arg {
						"searchmoves" => todo!("searchmoves is unimplemented"),
						"ponder" => options.ponder = true,
						"wtime" => options.wtime = iterator.next().and_then(|&wtime| wtime.parse().ok()),
						"btime" => options.btime = iterator.next().and_then(|&btime| btime.parse().ok()),
						"winc" => options.winc = iterator.next().and_then(|&winc| winc.parse().ok()),
						"binc" => options.binc = iterator.next().and_then(|&binc| binc.parse().ok()),
						"movestogo" => options.movestogo = iterator.next().and_then(|&movestogo| movestogo.parse().ok()),
						"depth" => options.depth = iterator.next().and_then(|&depth| depth.parse().ok()),
						"nodes" => options.nodes = iterator.next().and_then(|&nodes| nodes.parse().ok()),
						"mate" => options.mate = iterator.next().and_then(|&mate| mate.parse().ok()),
						"movetime" => todo!("movetime is unimplemented"),
						"infinite" => options.infinite = true,
						_ => panic!("Unexpected go argument")
					}
				}

				self.engine_sender.send(CommToEngineMessage::Go(options));
			},
			"stop" => {
				self.engine_sender.send(CommToEngineMessage::Stop(StopOptions::ReturnBestMove));
			}
			"ucinewgame" => {
				self.engine_sender.send(CommToEngineMessage::Stop(StopOptions::TerminateSearch));
				self.engine_sender.send(CommToEngineMessage::UCINewGame);
			}
			"quit" | "exit" => {
				self.engine_sender.send(CommToEngineMessage::Quit);
				self.exit = true;
			},
			_ => panic!("Unexpected UCI command")
		}
	}
}