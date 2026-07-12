use crate::engine::{EngineSpec, prepare_engine};
use crate::fastchess::{FastchessConfig, FastchessSummary, MatchMode, SprtParams, run_match};
use crate::stats::{chi_squared_test, elo_estimate, pentanomial_stats};
use anyhow::Context;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CompareRequest {
    pub base: EngineSpec,
    pub head: EngineSpec,
    pub games: Option<u32>,

    pub repo_root: PathBuf,

    pub time_control: String,
    pub concurrency: usize,
    pub openings_file: Option<PathBuf>,

    pub sprt_elo0: f64,
    pub sprt_elo1: f64,
    pub alpha: f64,
    pub beta: f64,
    pub sprt_rounds_cap: u32,

    pub min_meaningful_elo: f64,

    pub own_binary_name: String,
}

impl CompareRequest {
    pub fn new(base: EngineSpec, head: EngineSpec, games: Option<u32>, repo_root: PathBuf) -> Self {
        Self {
            base,
            head,
            games,
            repo_root,
            time_control: "8+0.08".to_string(),
            concurrency: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            openings_file: None,
            sprt_elo0: 0.0,
            sprt_elo1: 5.0,
            alpha: 0.05,
            beta: 0.05,
            sprt_rounds_cap: 5000,
            min_meaningful_elo: 3.0,
            own_binary_name: "gambit".to_string(),
        }
    }

    pub fn with_time_control(mut self, tc: impl Into<String>) -> Self {
        self.time_control = tc.into();
        self
    }

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn with_openings_file(mut self, path: Option<PathBuf>) -> Self {
        self.openings_file = path;
        self
    }

    pub fn with_sprt_bounds(mut self, elo0: f64, elo1: f64) -> Self {
        self.sprt_elo0 = elo0;
        self.sprt_elo1 = elo1;
        self
    }

    pub fn with_sprt_rounds_cap(mut self, cap: u32) -> Self {
        self.sprt_rounds_cap = cap;
        self
    }

    pub fn with_error_rates(mut self, alpha: f64, beta: f64) -> Self {
        self.alpha = alpha;
        self.beta = beta;
        self
    }

    pub fn with_min_meaningful_elo(mut self, elo: f64) -> Self {
        self.min_meaningful_elo = elo;
        self
    }

    pub fn with_own_binary_name(mut self, name: impl Into<String>) -> Self {
        self.own_binary_name = name.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SprtDecision {
    AcceptH1,
    AcceptH0,
    Inconclusive,
}

#[derive(Debug, Clone, Serialize)]
pub enum TestMethod {
    Sprt {
        llr: f64,
        lower_bound: f64,
        upper_bound: f64,
        elo0: f64,
        elo1: f64,
        decision: SprtDecision,
    },
    ChiSquared {
        statistic: f64,
        p_value: f64,
        degrees_of_freedom: u32,
        estimated_draw_rate: f64,
        low_expected_count_warning: bool,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct ComparisonReport {
    pub base_identity: String,
    pub head_identity: String,

    pub games_played: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub win_pct: f64,
    pub draw_pct: f64,
    pub loss_pct: f64,

    pub elo_diff: f64,
    pub elo_confidence_half_width: f64,
    pub elo_confidence_level: f64,

    pub method: TestMethod,

    pub is_meaningful: bool,

    /// Human-readable narrative summarizing the above.
    pub analysis: String,
}

pub fn compare_engines(req: &CompareRequest) -> anyhow::Result<ComparisonReport> {
    let base_handle = prepare_engine(&req.base, &req.repo_root, &req.own_binary_name)
        .context("preparing base engine")?;
    let head_handle = prepare_engine(&req.head, &req.repo_root, &req.own_binary_name)
        .context("preparing head engine")?;

    let match_dir = tempfile::Builder::new()
        .prefix("gambit-compare-match-")
        .tempdir()
        .context("creating scratch dir for match output")?;

    let pgn_out = match_dir.path().join("games.pgn");

    let mode = match req.games {
        Some(n) => MatchMode::FixedGames(n),
        None => MatchMode::Sprt(SprtParams {
            elo0: req.sprt_elo0,
            elo1: req.sprt_elo1,
            alpha: req.alpha,
            beta: req.beta,
        }),
    };

    let cfg = FastchessConfig {
        time_control: req.time_control.clone(),
        concurrency: req.concurrency,
        openings_file: req.openings_file.clone(),
        mode,
        sprt_rounds_cap: req.sprt_rounds_cap,
    };

    let summary =
        run_match(&base_handle, &head_handle, &cfg, &pgn_out).context("running fastchess match")?;

    build_report(&base_handle.identity, &head_handle.identity, &summary, req)
}

fn build_report(
    base_identity: &str,
    head_identity: &str,
    summary: &FastchessSummary,
    req: &CompareRequest,
) -> anyhow::Result<ComparisonReport> {
    let total = summary.total_games as f64;
    let win_pct = 100.0 * summary.head_wins as f64 / total;
    let draw_pct = 100.0 * summary.draws as f64 / total;
    let loss_pct = 100.0 * summary.head_losses as f64 / total;

    let penta_stats = pentanomial_stats(summary.pentanomial);
    let elo = elo_estimate(penta_stats, 0.95);

    let (method, is_meaningful) = match (&summary.sprt, req.games) {
        (Some(sprt), _) => {
            const EPSILON: f64 = 1e-9;

            let decision = if sprt.llr >= sprt.upper_bound - EPSILON {
                SprtDecision::AcceptH1
            } else if sprt.llr <= sprt.lower_bound + EPSILON {
                SprtDecision::AcceptH0
            } else {
                SprtDecision::Inconclusive
            };

            let meaningful = decision == SprtDecision::AcceptH1;

            (
                TestMethod::Sprt {
                    llr: sprt.llr,
                    lower_bound: sprt.lower_bound,
                    upper_bound: sprt.upper_bound,
                    elo0: sprt.elo0,
                    elo1: sprt.elo1,
                    decision,
                },
                meaningful,
            )
        }
        (None, Some(_)) => {
            let chi2 = chi_squared_test(summary.pentanomial, penta_stats);
            let meaningful = chi2.p_value < req.alpha && elo.diff.abs() >= req.min_meaningful_elo;

            (
                TestMethod::ChiSquared {
                    statistic: chi2.statistic,
                    p_value: chi2.p_value,
                    degrees_of_freedom: chi2.degrees_of_freedom,
                    estimated_draw_rate: chi2.estimated_draw_rate,
                    low_expected_count_warning: chi2.low_expected_count_warning,
                },
                meaningful,
            )
        }
        (None, None) => {
            anyhow::bail!(
                "fastchess ran in fixed-game mode but reported no SPRT outcome and no game count was requested — this should not happen"
            )
        }
    };

    let analysis = render_analysis(
        base_identity,
        head_identity,
        summary,
        win_pct,
        draw_pct,
        loss_pct,
        elo.diff,
        elo.confidence_half_width,
        &method,
        is_meaningful,
        req.min_meaningful_elo,
    );

    Ok(ComparisonReport {
        base_identity: base_identity.to_string(),
        head_identity: head_identity.to_string(),
        games_played: summary.total_games,
        wins: summary.head_wins,
        draws: summary.draws,
        losses: summary.head_losses,
        win_pct,
        draw_pct,
        loss_pct,
        elo_diff: elo.diff,
        elo_confidence_half_width: elo.confidence_half_width,
        elo_confidence_level: elo.confidence_level,
        method,
        is_meaningful,
        analysis,
    })
}

#[allow(clippy::too_many_arguments)]
fn render_analysis(
    base_identity: &str,
    head_identity: &str,
    summary: &FastchessSummary,
    win_pct: f64,
    draw_pct: f64,
    loss_pct: f64,
    elo_diff: f64,
    elo_half_width: f64,
    method: &TestMethod,
    is_meaningful: bool,
    min_meaningful_elo: f64,
) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "### Compare Results: `{head_identity}` vs `{base_identity}`\n\n\
         | Metric | Value |\n\
         | :--- | :--- |\n\
         | **Score** | {}W / {}D / {}L ({} total games) |\n\
         | **Pcnt** | {:.1}% Win / {:.1}% Draw / {:.1}% Loss |\n\
         | **Elo Diff** | **{elo_diff:+.1}** ± {:.1} (95% CI) |\n\n",
        summary.head_wins,
        summary.draws,
        summary.head_losses,
        summary.total_games,
        win_pct,
        draw_pct,
        loss_pct,
        elo_half_width
    ));

    out.push_str("#### Verdict\n");

    match method {
        TestMethod::Sprt {
            elo1,
            decision,
            ..
        } => {
            match decision {
                SprtDecision::AcceptH1 => out.push_str(&format!(
                    "> **SPRT: H1 Accepted**\n\
                     > {head_identity} is confirmed to be at least **{elo1:.1} Elo** stronger than {base_identity}.\n\
                     >\n\
                     > This is a meaningful change."
                )),
                SprtDecision::AcceptH0 => out.push_str(&format!(
                    "> **SPRT: H0 Accepted**\n\
                     > {head_identity} is confirmed to not be at least **{elo1:.1} Elo** stronger than {base_identity}.\n\
                     >\n\
                     > This is **not** a meaningful change."
                )),
                SprtDecision::Inconclusive => out.push_str("> **SPRT: Inconclusive**\n\
                     > The SPRT hit the round cap before determining a verdict."),
            }
        }
        TestMethod::ChiSquared {
            p_value,
            ..
        } => {
            if is_meaningful {
                out.push_str(&format!(
                    "> **Chi Squared: Significant and meaningful changes**\n\
                     > $p < 0.05$ and the estimated Elo gap ({elo_diff:+.1}) exceeds the {min_meaningful_elo:.1} Elo threshold \n\
                     >\n\
                     > This is a meaningful change."
                ))
            } else if *p_value < 0.05 {
                out.push_str(&format!(
                    "> **Chi Squared: Significant but small changes**\n\
                     > $p < 0.05$ but the estimated Elo gap ({elo_diff:+.1}) does not exceed the {min_meaningful_elo:.1} Elo threshold \n\
                     >\n\
                     > This is a statistically significant but non-meaningful change."
                ))
            } else {
                out.push_str("> **Chi Squared: Not significant**\n\
                     > These changes are not statistically significant at the 5% level. \n\
                     >\n\
                     > This is **not** a meaningful change.")
            }
        }
    }

    out.push_str("\n<details>\n<summary>View Statistical Details</summary>\n\n");

    match method {
        TestMethod::Sprt {
            llr,
            lower_bound,
            upper_bound,
            ..
        } => {
            out.push_str(&format!(
                "- **Test Type:** Sequential Probability Ratio Test (SPRT)\n\
                 - **LLR:** `{llr:.2}`\n\
                 - **Decision Bounds:** `[{lower_bound:.2}, {upper_bound:.2}]`\n"
            ));
        }
        TestMethod::ChiSquared {
            statistic,
            p_value,
            degrees_of_freedom,
            estimated_draw_rate,
            low_expected_count_warning,
        } => {
            out.push_str(&format!(
                "- **Test Type:** Chi-squared Goodness-of-Fit (Pentanomial)\n\
                 - **Chi-squared ($\\chi^2$):** `{statistic:.2}`\n\
                 - **p-value:** `{p_value:.4}`\n\
                 - **Degrees of Freedom (df):** `{degrees_of_freedom}`\n\
                 - **Est. Draw Rate:** `{:.1}%`\n",
                estimated_draw_rate * 100.0
            ));
            if *low_expected_count_warning {
                out.push_str(
                    "\n⚠️ **Warning:** At least one pentanomial category had an expected count under 5. \
                     The chi-squared approximation may be less reliable here (running more games is recommended).\n"
                );
            }
        }
    }

    out.push_str("\n</details>\n");

    out
}
