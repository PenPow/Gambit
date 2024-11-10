use std::io::{self, BufRead, Lines, StdinLock};

use crate::{board::Board, fen::FENError, VERSION};

pub struct UCI {
	iterator: Lines<StdinLock<'static>>,
	board: Board
}

impl UCI {
	pub fn new(board: Board) -> Self {
		let stdin = io::stdin();
		let iterator = stdin.lock().lines();

		Self {
			iterator,
			board
		}
	}

	pub fn init(mut self) {
		assert_eq!(self.next_line().as_str(), "uci");

		self.id();

		println!("uciok");

		loop {
			let command = self.next_line();

			let mut parts = command.as_str().split_whitespace();
			let command = parts.next().unwrap();
			let arguments = parts.collect();

			match command {
				"ucinewgame" => self.board = Board::from_start_pos(),
				"position" => self.board = self.handle_position_command(&arguments).expect("Expected valid board after position issued"),
				"isready" =>  {
					println!("readyok")
				}
				"debug" => self.board.debug(),
				"exit" => std::process::exit(0),
				"" => continue,
				_ => unimplemented!()
			}
		}
	}

	fn id(&self) {
		println!("id name Gambit {}", VERSION);
		println!("id author Joshua Clements");
	}

	fn handle_position_command(&self, arguments: &Vec<&str>) -> Result<Board, FENError> {
		dbg!(arguments);

		let board = if arguments[0] == "startpos" || arguments[0] == "moves" {
			Board::from_start_pos()
		} else if arguments[0] == "fen" {
			let fen = arguments
								.iter()
								.skip(1)
								.take_while(|&&x| x != "moves")
								.copied()
								.collect::<Vec<&str>>()
								.join(" ");

			Board::from_fen(fen.as_str())?
		} else {
			panic!("Invalid position command recieved");
		};

		let moves = arguments
			.iter()
			.skip_while(|&&x| x != "moves")
			.skip(1)
			.copied()
			.collect::<Vec<&str>>();

		for algebraic_move in moves {
			todo!("Algebraic moves not supported")
		}

		Ok(board)
	}

	fn next_line(&mut self) -> String {
		self.iterator.next().unwrap().unwrap()
	}
}