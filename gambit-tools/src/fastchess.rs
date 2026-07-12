use crate::engine::EngineHandle;
use anyhow::Context;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SprtParams {
    pub elo0: f64,
    pub elo1: f64,
    pub alpha: f64,
    pub beta: f64,
}

#[derive(Debug, Clone)]
pub enum MatchMode {
    Sprt(SprtParams),
    FixedGames(u32),
}

pub struct FastchessConfig {
    pub time_control: String,
    pub concurrency: usize,
    pub openings_file: Option<PathBuf>,
    pub mode: MatchMode,
    pub sprt_rounds_cap: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct SprtOutcome {
    pub llr: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub elo0: f64,
    pub elo1: f64,
}

#[derive(Debug, Clone)]
pub struct FastchessSummary {
    pub total_games: u32,
    pub head_wins: u32,
    pub head_losses: u32,
    pub draws: u32,
    pub pentanomial: [u32; 5],
    pub sprt: Option<SprtOutcome>,
}

pub fn run_match(
    base: &EngineHandle,
    head: &EngineHandle,
    cfg: &FastchessConfig,
    pgn_out: &Path,
) -> anyhow::Result<FastchessSummary> {
    ensure_tool_available("fastchess")?;

    let mut cmd = Command::new("fastchess");

    cmd.arg("-engine")
        .arg(format!("cmd={}", head.executable.display()))
        .arg(format!("name={}", head.identity));
    cmd.arg("-engine")
        .arg(format!("cmd={}", base.executable.display()))
        .arg(format!("name={}", base.identity));

    cmd.arg("-each").arg(format!("tc={}", cfg.time_control));
    cmd.arg("-games").arg("2");
    cmd.arg("-concurrency").arg(cfg.concurrency.to_string());
    cmd.arg("-report").arg("penta=true");
    cmd.arg("-recover");
    cmd.arg("-pgnout")
        .arg(format!("file={}", pgn_out.display()));

    if let Some(book) = &cfg.openings_file {
        cmd.arg("-openings")
            .arg(format!("file={}", book.display()))
            .arg("format=pgn")
            .arg("order=random");
    }

    match &cfg.mode {
        MatchMode::Sprt(sprt) => {
            cmd.arg("-rounds").arg(cfg.sprt_rounds_cap.to_string());
            cmd.arg("-sprt")
                .arg(format!("elo0={}", sprt.elo0))
                .arg(format!("elo1={}", sprt.elo1))
                .arg(format!("alpha={}", sprt.alpha))
                .arg(format!("beta={}", sprt.beta))
                .arg("model=normalized");
        }
        MatchMode::FixedGames(games) => {
            let rounds = games.div_ceil(2).max(1);
            cmd.arg("-rounds").arg(rounds.to_string());
        }
    }

    let output = cmd.output().context("running fastchess")?;
    if !output.status.success() {
        anyhow::bail!(
            "fastchess exited with {}:\nstdout: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    parse_summary(&String::from_utf8_lossy(&output.stdout))
}

fn ensure_tool_available(tool: &str) -> anyhow::Result<()> {
    Command::new(tool)
        .arg("--version")
        .output()
        .with_context(|| format!("`{tool}` was not found on PATH"))?;
    Ok(())
}

fn parse_summary(stdout: &str) -> anyhow::Result<FastchessSummary> {
    let games_re =
        Regex::new(r"Games:\s*(\d+),\s*Wins:\s*(\d+),\s*Losses:\s*(\d+),\s*Draws:\s*(\d+)")?;

    let penta_re = Regex::new(
        r"Ptnml\(0-2\):\s*\[\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\]",
    )?;

    let llr_re = Regex::new(
        r"LLR:\s*(-?[\d.]+)\s*\([^)]*\)\s*\(\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)\s*\)\s*\[\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)\s*\]",
    )?;

    let games_caps = games_re.captures_iter(stdout).last().context(format!(
        "could not find a `Games | ...` line in fastchess output\nstdout: {}",
        stdout
    ))?;

    let total_games: u32 = games_caps[1].parse()?;
    let head_wins: u32 = games_caps[2].parse()?;
    let head_losses: u32 = games_caps[3].parse()?;
    let draws: u32 = games_caps[4].parse()?;

    let penta_caps = penta_re
        .captures_iter(stdout)
        .last()
        .context(format!("could not find a `Penta | [...]` line in fastchess output (did you pass -report penta=true?)\nstdout: {}", stdout))?;

    let mut pentanomial = [0u32; 5];
    for (i, slot) in pentanomial.iter_mut().enumerate() {
        *slot = penta_caps[i + 1].parse()?;
    }

    let sprt = llr_re
        .captures_iter(stdout)
        .last()
        .map(|caps| -> anyhow::Result<SprtOutcome> {
            Ok(SprtOutcome {
                llr: caps[1].parse()?,
                lower_bound: caps[2].parse()?,
                upper_bound: caps[3].parse()?,
                elo0: caps[4].parse()?,
                elo1: caps[5].parse()?,
            })
        })
        .transpose()?;

    Ok(FastchessSummary {
        total_games,
        head_wins,
        head_losses,
        draws,
        pentanomial,
        sprt,
    })
}
