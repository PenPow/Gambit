use gambit_models::moves::Move;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Handshake {
    pub name: String,
    pub author: String,
    pub options: Vec<EngineOption>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SearchInfo {
    pub depth: Option<u32>,
    pub seldepth: Option<u32>,
    pub nodes: Option<u64>,
    pub multipv: Option<u32>,
    pub score: Option<i32>,
    pub time: Option<u64>,
    pub pv: Option<Vec<Move>>,
    pub hashfull: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EngineOptionType {
    Check {
        default: bool,
    },
    Spin {
        default: i64,
        min: i64,
        max: i64,
    },
    Combo {
        default: String,
        choices: Vec<String>,
    },
    Button,
    String {
        default: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EngineOption {
    pub name: String,
    pub option_type: EngineOptionType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Handshake(Handshake),
    ReadyOk,
    Info(SearchInfo),
    BestMove(Option<Move>),
    Error(String),
}
