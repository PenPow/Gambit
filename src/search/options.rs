use crate::board::{piece::Sides, Board};

const AVERAGE_GAME_LENGTH: usize = 25;
const LENGTH_ESTIMATE_BUFFER: usize = 5;
const CALCULATION_OVERHEAD: i64 = 50;

#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
	/// Restrict search to these moves only
    pub searchmoves: Vec<String>,
	/// Whether the engine should start searching in pondering mode. Do not exit search.
    pub ponder: bool,
	/// The remaining time for white in ms
    pub wtime: Option<u64>,
	/// The remaining time for black in ms
    pub btime: Option<u64>,
	/// How much time is incremented for white after each move in ms
    pub winc: Option<u64>,
	/// How much time is incremented for black after each move in ms
    pub binc: Option<u64>,
	/// How many moves until the next time control is implemented
    pub movestogo: Option<u64>,
	/// The depth to search at
    pub depth: Option<u8>,
	/// The number of nodes to search
    pub nodes: Option<u64>,
	/// Search for a mate in X moves
    pub mate: Option<u8>,
	/// How long this search should be in ms
    pub movetime: Option<u64>,
	/// Whether the search should last forever, until a stop command is recieved
    pub infinite: bool,
}

impl SearchOptions {
	pub fn calculate_time(&self, board: &Board) -> u64 {
		match self.get_type() {
			SearchType::Infinite => u64::MAX,
			SearchType::MoveTime => self.movetime.unwrap(),
			SearchType::GameTime => {
				let is_white = board.state.side_to_move  == Sides::WHITE;

				let (clock, increment) = if is_white {
					(self.wtime.unwrap(), self.winc.unwrap())
				} else {
					(self.btime.unwrap(), self.binc.unwrap())
				};

				let moves_to_go = if let Some(moves) = self.movestogo {
					moves
				} else {
					let moves_made = board.history.len();
					let moves_by_us = if is_white {
						moves_made / 2
					} else {
						(moves_made - 1) / 2
					};

					(AVERAGE_GAME_LENGTH - (moves_by_us % AVERAGE_GAME_LENGTH) + LENGTH_ESTIMATE_BUFFER) as u64
				};

				let timeslice = ((clock as f64) / (moves_to_go as f64)).round() as i64 + (increment as i64) - CALCULATION_OVERHEAD;

				if timeslice < 0 {
					0
				} else {
					timeslice as u64
				}
			},
			_ => panic!("Unexpected option when calculating timeslice")
		}
	}

	pub fn get_type(&self) -> SearchType {
		if self.infinite { SearchType::Infinite }
		else if self.depth.is_some() { SearchType::Depth }
		else if self.nodes.is_some() { SearchType::Nodes }
		else if self.nodes.is_some() { SearchType::Nodes }
		else if self.movetime.is_some() { SearchType::MoveTime }
		else if self.wtime.is_some() { SearchType::GameTime }
		else { panic!("Invalid search type, no search time specified") }
	}

	pub fn should_calculate_timeslice(&self) -> bool {
		match self.get_type() {
			SearchType::GameTime | SearchType::MoveTime | SearchType::Infinite => true,
			_ => false
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
	Depth,
	// TODO: Add support for searching a number of nodes
	Nodes,
	MoveTime,
	Infinite,
	GameTime
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopOptions {
	ReturnBestMove,
	TerminateSearch
}