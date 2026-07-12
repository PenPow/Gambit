use gambit_engine::{MATE_THRESHOLD, MATE_VALUE};
use gambit_protocol::{EngineOption, EngineOptionType, Event, SearchInfo};

const UCI_NULL_MOVE: &str = "0000";

pub fn format_event(event: &Event) -> Vec<String> {
    match event {
        Event::Handshake(handshake) => {
            let mut lines = vec![
                format!("id name {}", handshake.name),
                format!("id author {}", handshake.author),
            ];

            lines.extend(handshake.options.iter().map(format_option));
            lines.push("uciok".to_string());

            lines
        }
        Event::ReadyOk => {
            vec!["readyok".to_string()]
        }
        Event::Info(info) => {
            vec![format_info(info)]
        }
        Event::BestMove(mv) => {
            let move_string = mv
                .map(|m| m.to_string())
                .unwrap_or_else(|| UCI_NULL_MOVE.to_string());

            vec![format!("bestmove {move_string}")]
        }
        Event::Error(msg) => {
            vec![format!("info string error raised: {msg}")]
        }
    }
}

fn format_option(option: &EngineOption) -> String {
    match &option.option_type {
        EngineOptionType::Check { default } => {
            format!("option name {} type check default {}", option.name, default)
        }
        EngineOptionType::Spin { default, min, max } => {
            format!(
                "option name {} type spin default {} min {} max {}",
                option.name, default, min, max
            )
        }
        EngineOptionType::Combo { default, choices } => {
            let variables = choices
                .iter()
                .map(|choice| format!("var {choice}"))
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "option name {} type combo default {} {}",
                option.name, default, variables
            )
        }
        EngineOptionType::Button => {
            format!("option name {} type button", option.name)
        }
        EngineOptionType::String { default } => {
            format!(
                "option name {} type string default {}",
                option.name, default
            )
        }
    }
}

fn format_info(info: &SearchInfo) -> String {
    let mut parts = vec!["info".to_string()];

    if let Some(depth) = info.depth {
        parts.push(format!("depth {depth}"));
    }

    if let Some(seldepth) = info.seldepth {
        parts.push(format!("seldepth {seldepth}"));
    }

    if let Some(time) = info.time {
        parts.push(format!("time {time}"));
    }

    if let Some(nodes) = info.nodes {
        parts.push(format!("nodes {nodes}"));
    }

    if let Some(score) = info.score {
        parts.push(format_score(score));
    }

    if let Some(hashfull) = info.hashfull {
        parts.push(format!("hashfull {hashfull}"));
    }

    if let Some(multipv) = info.multipv {
        parts.push(format!("multipv {multipv}"));
    }

    if let Some(pv) = &info.pv {
        if !pv.is_empty() {
            let pv_string = pv
                .iter()
                .map(|mv| mv.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            parts.push(format!("pv {pv_string}"));
        }
    }

    parts.join(" ")
}

fn format_score(score: i32) -> String {
    if score.abs() >= MATE_THRESHOLD {
        let plies_to_mate = MATE_VALUE - score.abs();
        let moves_to_mate = (plies_to_mate + 1) / 2;
        let signed = if score > 0 {
            moves_to_mate
        } else {
            -moves_to_mate
        };
        format!("score mate {signed}")
    } else {
        format!("score cp {score}")
    }
}
