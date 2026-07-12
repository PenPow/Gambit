use crate::cli::BaseCli;
use crate::engine::git::current_repo_root;
use crate::engine::parse_spec;
use crate::subcommands::compare::{CompareRequest, compare_engines};
use clap::Parser;

mod cli;
pub mod engine;
pub mod fastchess;
pub mod stats;
mod subcommands;

fn main() -> anyhow::Result<()> {
    let args = BaseCli::parse();

    let is_json_out = args.json;

    match args.command {
        cli::Command::Plan { input } => {
            subcommands::plan::run(input);

            Ok(())
        }
        cli::Command::Perft { depth, exclude } => {
            let result = subcommands::perft::run(depth, exclude.as_deref())?;

            if is_json_out {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                subcommands::perft::output_pretty(&result)
            }

            std::process::exit(if result.success { 0 } else { 1 });
        }
        cli::Command::Compare {
            base,
            head,
            games,
            time_control,
            concurrency,
            openings_file,
            sprt_elo0,
            sprt_elo1,
            sprt_alpha,
            sprt_beta,
            sprt_rounds_cap,
            min_meaningful_elo,
            own_binary_name,
        } => {
            let repo_root = current_repo_root()?;

            let request =
                CompareRequest::new(parse_spec(&base), parse_spec(&head), games, repo_root)
                    .with_time_control(time_control)
                    .with_concurrency(concurrency.unwrap_or_else(|| {
                        std::thread::available_parallelism()
                            .map(|n| n.get())
                            .unwrap_or(1)
                    }))
                    .with_openings_file(openings_file)
                    .with_sprt_bounds(sprt_elo0, sprt_elo1)
                    .with_error_rates(sprt_alpha, sprt_beta)
                    .with_min_meaningful_elo(min_meaningful_elo)
                    .with_sprt_rounds_cap(sprt_rounds_cap)
                    .with_own_binary_name(own_binary_name);

            let report = compare_engines(&request)?;

            if is_json_out {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{}", report.analysis);
            }

            Ok(())
        }
    }
}
