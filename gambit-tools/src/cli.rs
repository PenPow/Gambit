use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about, long_about)]
pub struct BaseCli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Plan {
        input: PathBuf,
    },

    Perft {
        depth: u32,
        #[arg(long)]
        exclude: Option<Vec<String>>,
    },

    Compare {
        base: String,
        head: String,

        #[arg(long)]
        games: Option<u32>,

        #[arg(long, default_value = "8+0.08")]
        time_control: String,

        #[arg(long)]
        concurrency: Option<usize>,

        #[arg(long)]
        openings_file: Option<PathBuf>,

        #[arg(long, default_value = "0.0")]
        sprt_elo0: f64,
        #[arg(long, default_value = "5.0")]
        sprt_elo1: f64,
        #[arg(long, default_value = "0.05")]
        sprt_alpha: f64,
        #[arg(long, default_value = "0.05")]
        sprt_beta: f64,
        #[arg(long, default_value = "40000")]
        sprt_rounds_cap: u32,

        #[arg(long, default_value = "3.0")]
        min_meaningful_elo: f64,

        #[arg(long, default_value = "gambit")]
        own_binary_name: String,
    },
}
