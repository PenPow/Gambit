use crate::{movegen::piece_move::Move, search::options::{SearchOptions, StopOptions}};

#[derive(Debug)]
pub enum EngineToCommMessage {
	ReadyOk,
	BestMove(Move)
}

#[derive(Debug)]
pub enum CommToEngineMessage {
	Quit,
	Debug,
	IsReady,
	Position(String),
	UCINewGame,
	Go(SearchOptions),
	Stop(StopOptions)
}