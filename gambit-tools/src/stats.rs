use statrs::distribution::{ChiSquared, ContinuousCDF, Normal};

pub type Pentanomial = [u32; 5];

#[derive(Debug, Clone, Copy)]
pub struct PentaStats {
    pub n_pairs: u32,
    pub mean_pair_score: f64,
    pub variance_pair_score: f64,
}

pub fn pentanomial_stats(counts: Pentanomial) -> PentaStats {
    let n_pairs: u32 = counts.iter().sum();
    let n = n_pairs as f64;
    let mean_pair_score: f64 = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| (i as f64 * 0.5) * c as f64)
        .sum::<f64>()
        / n;

    let variance_pair_score: f64 = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let x = i as f64 * 0.5;
            c as f64 * (x - mean_pair_score).powi(2)
        })
        .sum::<f64>()
        / n;

    PentaStats {
        n_pairs,
        mean_pair_score,
        variance_pair_score,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EloEstimate {
    pub diff: f64,
    pub confidence_half_width: f64,
    pub confidence_level: f64,
}

pub fn elo_estimate(stats: PentaStats, confidence_level: f64) -> EloEstimate {
    let p = (stats.mean_pair_score / 2.0).clamp(1e-6, 1.0 - 1e-6);
    let diff = 400.0 * (p / (1.0 - p)).log10();

    let se_p = (stats.variance_pair_score / stats.n_pairs as f64 / 4.0).sqrt();

    let d_elo_d_p = 400.0 / (std::f64::consts::LN_10 * p * (1.0 - p));
    let se_elo = se_p * d_elo_d_p;

    let z = Normal::new(0.0, 1.0)
        .expect("standard normal is always valid")
        .inverse_cdf(0.5 + confidence_level / 2.0);

    EloEstimate {
        diff,
        confidence_half_width: z * se_elo,
        confidence_level,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChiSquaredResult {
    pub statistic: f64,
    pub p_value: f64,
    pub degrees_of_freedom: u32,
    pub estimated_draw_rate: f64,
    pub low_expected_count_warning: bool,
}

pub fn chi_squared_test(counts: Pentanomial, stats: PentaStats) -> ChiSquaredResult {
    let n = stats.n_pairs as f64;
    let d_hat = (1.0 - 2.0 * stats.variance_pair_score).clamp(0.0, 1.0);
    let p_win = (1.0 - d_hat) / 2.0;
    let p_loss = p_win;

    let expected_probs = [
        p_loss * p_loss,
        2.0 * p_loss * d_hat,
        d_hat * d_hat + 2.0 * p_win * p_loss,
        2.0 * p_win * d_hat,
        p_win * p_win,
    ];

    let mut statistic = 0.0;
    let mut low_expected_count_warning = false;
    for (observed, prob) in counts.iter().zip(expected_probs.iter()) {
        let expected = n * prob;
        if expected < 5.0 {
            low_expected_count_warning = true;
        }
        if expected > 0.0 {
            statistic += (*observed as f64 - expected).powi(2) / expected;
        }
    }

    let degrees_of_freedom = 3;
    let p_value = 1.0
        - ChiSquared::new(degrees_of_freedom as f64)
            .expect("degrees of freedom is a positive constant")
            .cdf(statistic);

    ChiSquaredResult {
        statistic,
        p_value,
        degrees_of_freedom,
        estimated_draw_rate: d_hat,
        low_expected_count_warning,
    }
}
