use crate::subcommands::Args;
use anyhow::Context;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub jobs: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub kind: JobKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobKind {
    // Bench {
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     target: Option<String>,
    // },
    Compare {
        base: String,
        head: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        games: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        time_control: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        concurrency: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_meaningful_elo: Option<f64>,
    },
    // Tournament {
    //     engines: Vec<String>,
    //     #[serde(default = "default_games_per_matchup")]
    //     games_per_matchup: u32,
    // },
    Perft {
        depth: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclude: Option<Vec<String>>,
    },
}

pub fn run(input: PathBuf) {
    let comment = if input == *"-" {
        std::io::read_to_string(std::io::stdin()).unwrap()
    } else {
        std::fs::read_to_string(&input).unwrap()
    };

    let base = std::env::var("GAMBIT_BASE_REF").unwrap_or_else(|_| "main".to_string());
    let head = std::env::var("GAMBIT_HEAD_REF").unwrap_or_else(|_| "HEAD".to_string());

    let plan = parse_comment(&comment, &base, &head).unwrap();
    println!("{}", serde_json::to_string_pretty(&plan).unwrap());
}

fn parse_comment(comment: &str, base: &str, head: &str) -> anyhow::Result<Plan> {
    let mut jobs = Vec::new();

    let gambit_re = Regex::new(r"/gambot\s+(test|bench|compare|tournament|perft)(?:\s+([^\n]+))?")?;

    for line in comment.lines() {
        let line = line.trim();

        if line.is_empty() || !line.starts_with("/gambot") {
            continue;
        }

        if let Some(sections) = gambit_re.captures(line) {
            let cmd = &sections[1];
            let args = Args::parse(sections.get(2).map(|m| m.as_str().trim()).unwrap_or(""));

            let job = match cmd {
                // "bench" => {
                //     let target = args.positional.first().map(|s| s.to_string());
                //
                //     let id = if let Some(t) = &target {
                //         format!("bench-{}", t.replace(['/', '.'], "-"))
                //     } else {
                //         "bench-all".into()
                //     };
                //
                //     Job {
                //         id,
                //         kind: JobKind::Bench { target },
                //     }
                // }
                "compare" => {
                    let games = args
                        .get_flag("--games")
                        .map(|s| s.parse().ok())
                        .unwrap_or(None);
                    let time_control = args.get_flag("--time-control").map(|s| s.to_string());
                    let concurrency = args.get_flag("--concurrency").and_then(|s| s.parse().ok());
                    let min_meaningful_elo = args
                        .get_flag("--min-meaningful-elo")
                        .and_then(|s| s.parse().ok());

                    let (base_ref, head_ref) = match args.positional.len() {
                        0 => (base.to_string(), head.to_string()),
                        1 => {
                            let other = resolve_ref(&args.positional[0], base, head);
                            (other, head.to_string())
                        }
                        2 => {
                            let head_ref = resolve_ref(&args.positional[1], base, head);
                            let base_ref = resolve_ref(&args.positional[0], base, head);
                            (base_ref, head_ref)
                        }
                        _ => anyhow::bail!("compare takes 0, 1 or 2 arguments"),
                    };

                    Job {
                        id: format!("compare-{}-vs-{}", base_ref, head_ref),
                        kind: JobKind::Compare {
                            base: base_ref.to_string(),
                            head: head_ref.to_string(),
                            games,
                            time_control,
                            concurrency,
                            min_meaningful_elo,
                        },
                    }
                }
                // "tournament" => {
                //     let engines = args.positional[0]
                //         .split(',')
                //         .map(|s| resolve_ref(s.trim(), base, head))
                //         .collect::<Vec<_>>();
                //
                //     if engines.is_empty() {
                //         anyhow::bail!("tournament requires at least one engine ref");
                //     }
                //
                //     let games = args
                //         .get_flag("--games")
                //         .map(|s| s.parse().unwrap_or(default_games_per_matchup()))
                //         .unwrap_or(default_games_per_matchup());
                //
                //     let id = format!("tournament-{}", engines.join("-"));
                //
                //     Job {
                //         id,
                //         kind: JobKind::Tournament {
                //             engines,
                //             games_per_matchup: games,
                //         },
                //     }
                // }
                "perft" => {
                    let depth: u32 = args.positional[1]
                        .parse()
                        .with_context(|| "invalid depth")?;

                    let exclude = args
                        .get_flag("--exclude")
                        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect());

                    Job {
                        id: format!("perft-{}", depth),
                        kind: JobKind::Perft { depth, exclude },
                    }
                }
                _ => continue,
            };

            jobs.push(job);
        }
    }

    Ok(Plan { jobs })
}

fn resolve_ref(ref_str: &str, base: &str, head: &str) -> String {
    match ref_str {
        "pr" => head.to_string(),
        "base" => base.to_string(),
        _ => ref_str.to_string(),
    }
}
