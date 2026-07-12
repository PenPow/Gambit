use colored::Colorize;
use gambit_perft::{KNOWN_POSITIONS, perft, state_from_fen};
use serde::Serialize;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Serialize)]
pub struct PerftResult {
    pub max_depth: u32,
    pub tests: Vec<PerftTestResult>,
    pub total_time_ms: f64,
    pub success: bool,
}

#[derive(Serialize)]
pub struct PerftTestResult {
    pub name: String,
    pub results: Vec<PerftDepthResult>,
}

#[derive(Serialize)]
pub struct PerftDepthResult {
    pub depth: u32,
    pub nodes: u64,
    pub expected: u64,
    pub elapsed_ms: f64,
    pub mnps: f64,
    pub is_success: bool,
}

pub fn run(depth: u32, exclude: Option<&[String]>) -> anyhow::Result<PerftResult> {
    let excluded: HashSet<&str> = exclude
        .map(|entry| entry.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();

    let mut tests = Vec::new();
    let mut total_ms = 0.0;

    let mut any_failing = false;

    for case in KNOWN_POSITIONS {
        if excluded.contains(case.name) {
            continue;
        }

        let max_run_depth = case
            .depths
            .iter()
            .filter_map(|(d, _)| (*d <= depth).then_some(*d))
            .max()
            .unwrap_or(0);

        if max_run_depth == 0 {
            continue;
        }

        let mut results = Vec::new();

        for (run_depth, expected) in case.depths {
            if *run_depth > max_run_depth {
                break;
            }

            let mut state = state_from_fen(case.fen);

            let start = Instant::now();
            let nodes = perft(&mut state, *run_depth);
            let elapsed = start.elapsed();

            let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
            let mnps = if elapsed_ms > 0.0 {
                nodes as f64 / elapsed_ms / 1000.0
            } else {
                f64::INFINITY
            };

            let is_success = if nodes == *expected {
                true
            } else {
                any_failing = true;

                false
            };

            results.push(PerftDepthResult {
                depth: *run_depth,
                nodes,
                expected: *expected,
                elapsed_ms,
                mnps,
                is_success,
            });

            total_ms += elapsed_ms;
        }

        tests.push(PerftTestResult {
            name: case.name.to_owned(),
            results,
        });
    }

    Ok(PerftResult {
        max_depth: depth,
        tests,
        total_time_ms: total_ms,
        success: !any_failing,
    })
}

pub fn output_pretty(result: &PerftResult) {
    println!("{}", "Gambit Perft".cyan().bold());
    println!(
        "Overall result ... {}",
        if result.success {
            "ok".green().bold()
        } else {
            "FAIL".red().bold()
        }
    );
    println!("Max Depth ... {}\n", result.max_depth.to_string().bold());

    for test in &result.tests {
        println!("{}", test.name.magenta().bold());

        for run in &test.results {
            println!(
                "  Depth {} ... {} {}",
                run.depth,
                if run.is_success {
                    "ok".green().bold()
                } else {
                    "FAIL".red().bold()
                },
                format!(
                    "mnps: {:.7} nodes: {}",
                    run.mnps.to_string().italic(),
                    run.nodes.to_string().italic()
                )
                .dimmed()
            )
        }

        println!();
    }
}
