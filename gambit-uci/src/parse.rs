use gambit_models::position::Position;
use gambit_notation::fen::{FenLike, parsers};
use gambit_protocol::{Command, GoParams, SetEngineOption, SetEnginePosition};
use std::time::Duration;

#[derive(Debug)]
pub enum ParseError {
    UnknownCommand(String),
    MalformedArguments(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownCommand(cmd) => write!(f, "unknown command: {cmd}"),
            ParseError::MalformedArguments(msg) => write!(f, "malformed arguments: {msg}"),
        }
    }
}

pub fn parse_line(line: &str) -> Result<Option<Command>, ParseError> {
    let mut tokens = line.split_whitespace();

    let Some(head) = tokens.next() else {
        return Ok(None);
    };

    match head {
        "uci" => Ok(Some(Command::Handshake)),
        "isready" => Ok(Some(Command::IsReady)),
        "ucinewgame" => Ok(Some(Command::NewGame)),
        "stop" => Ok(Some(Command::Stop)),
        "ponderhit" => Ok(Some(Command::PonderHit)),
        "quit" => Ok(Some(Command::Quit)),
        "debug" => Ok(Some(Command::Debug(tokens.next() == Some("on")))),
        "setoption" => parse_setoption(tokens).map(Some),
        "position" => parse_position(tokens).map(Some),
        "go" => parse_go(tokens).map(Some),
        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

fn parse_setoption<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Result<Command, ParseError> {
    if tokens.next() != Some("name") {
        return Err(ParseError::MalformedArguments(
            "setoption: expected 'name'".to_string(),
        ));
    }

    let mut name_parts = Vec::new();
    let mut value = None;

    while let Some(token) = tokens.next() {
        if token == "value" {
            value = Some(tokens.collect::<Vec<_>>().join(" "));
            break;
        }

        name_parts.push(token)
    }

    Ok(Command::SetOption(SetEngineOption {
        name: name_parts.join(" "),
        value,
    }))
}

fn parse_position<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Result<Command, ParseError> {
    let position = match tokens.next() {
        Some("startpos") => Position::STARTING_POSITION,
        Some("fen") => {
            let mut parts = Vec::new();

            for token in tokens.by_ref() {
                if token == "moves" {
                    return Ok(Command::SetPosition(SetEnginePosition {
                        position: parsers::Fen::parse(&parts.join(" "))
                            .expect("position: expected valid fen to be passed")
                            .position(),
                        moves: tokens.map(str::to_owned).collect(),
                    }));
                }

                parts.push(token);
            }

            parsers::Fen::parse(&parts.join(" "))
                .expect("position: expected valid fen to be passed")
                .position()
        }
        _ => {
            return Err(ParseError::MalformedArguments(
                "position: expected 'startpos' or 'fen'".into(),
            ));
        }
    };

    let moves = if tokens.next() == Some("moves") {
        tokens.map(str::to_owned).collect()
    } else {
        Vec::new()
    };

    let params = SetEnginePosition { position, moves };

    Ok(Command::SetPosition(params))
}

fn parse_go<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Result<Command, ParseError> {
    let mut params = GoParams::default();

    while let Some(token) = tokens.next() {
        match token {
            "infinite" => params.infinite = true,
            "ponder" => params.ponder = true,
            "depth" => params.depth = next_num(&mut tokens),
            "nodes" => params.nodes = next_num(&mut tokens),
            "mate" => params.mate = next_num(&mut tokens),
            "movetime" => {
                params.move_time = next_num::<u64>(&mut tokens).map(Duration::from_millis)
            }
            "wtime" => params.wtime = next_num::<u64>(&mut tokens).map(Duration::from_millis),
            "btime" => params.btime = next_num::<u64>(&mut tokens).map(Duration::from_millis),
            "winc" => params.winc = next_num::<u64>(&mut tokens).map(Duration::from_millis),
            "binc" => params.binc = next_num::<u64>(&mut tokens).map(Duration::from_millis),
            "movestogo" => params.moves_to_go = next_num(&mut tokens),
            "searchmoves" => {
                params.search_moves = Some(
                    tokens
                        .next()
                        .map(str::to_owned)
                        .expect("expected search moves"),
                );
            }
            _ => {}
        }
    }

    Ok(Command::Go(params))
}

fn next_num<'a, T: std::str::FromStr>(tokens: &mut impl Iterator<Item = &'a str>) -> Option<T> {
    tokens.next().and_then(|t| t.parse().ok())
}
