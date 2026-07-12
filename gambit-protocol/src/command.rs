use gambit_models::position::Position;
use std::time::Duration;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct GoParams {
    pub search_moves: Option<String>,
    pub ponder: bool,
    pub wtime: Option<Duration>,
    pub btime: Option<Duration>,
    pub winc: Option<Duration>,
    pub binc: Option<Duration>,
    pub moves_to_go: Option<u32>,
    pub depth: Option<u32>,
    pub nodes: Option<u64>,
    pub mate: Option<u32>,
    pub move_time: Option<Duration>,
    pub infinite: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SetEngineOption {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SetEnginePosition {
    pub position: Position,
    pub moves: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Handshake,
    IsReady,
    SetOption(SetEngineOption),
    NewGame,
    SetPosition(SetEnginePosition),
    Go(GoParams),
    Stop,
    PonderHit,
    Quit,
    Debug(bool),
}
