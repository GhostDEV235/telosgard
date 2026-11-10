//! Live Readiness (PRD FR-7): 50 counted Demo Trades, first→latest ≥ 56 days (8 weeks), ≥ 90% adherent,
//! and no breach of the max total loss — all within one Readiness Run.
//! A breach ends the run; the next run starts from 0 with a fresh demo balance.

use crate::money::{Percent, Usd};
use crate::{DAY_MS, Millis};
use serde::{Deserialize, Serialize};

pub const REQUIRED_TRADES: u32 = 50;
pub const REQUIRED_DAYS: i64 = 56;
pub const REQUIRED_ADHERENCE: Percent = Percent(9_000);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadinessRun {
    pub number: u32,
    pub started_at: Millis,
    pub starting_balance: Usd,
}

/// What readiness needs from one Journal Entry in the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradeSummary {
    pub at: Millis,
    /// `None` until the verdict is final.
    pub final_adherent: Option<bool>,
    /// `None` until the trade is closed.
    pub balance_after: Option<Usd>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadinessProgress {
    pub counted: u32,
    pub adherent: u32,
    /// Adherent share of counted trades, rounded down; 0 when nothing is counted.
    pub adherence: Percent,
    /// Whole days between the first and latest counted trade.
    pub days: i64,
    /// Starting balance minus the latest closed balance (0 if in profit).
    pub drawdown: Usd,
    /// The loss that ends the run: starting balance × max total loss.
    pub max_loss: Usd,
    /// Any closed balance at or below starting balance − max loss.
    pub breached: bool,
    pub reached: bool,
}

impl ReadinessRun {
    pub fn first(at: Millis, starting_balance: Usd) -> ReadinessRun {
        ReadinessRun { number: 1, started_at: at, starting_balance }
    }
    pub fn next(&self, at: Millis, starting_balance: Usd) -> ReadinessRun {
        ReadinessRun { number: self.number + 1, started_at: at, starting_balance }
    }
    /// `trades` are this run's entries, in any order.
    pub fn progress(&self, trades: &[TradeSummary], max_total_loss: Percent) -> ReadinessProgress {
        let counted: Vec<_> = trades.iter().filter_map(|t| t.final_adherent.map(|a| (t.at, a))).collect();
        let n = counted.len() as u32;
        let adherent = counted.iter().filter(|(_, a)| *a).count() as u32;
        let adherence = if n == 0 { Percent(0) } else { Percent(adherent * 10_000 / n) };
        let days = match (counted.iter().map(|c| c.0).min(), counted.iter().map(|c| c.0).max()) {
            (Some(first), Some(last)) => (last - first) / DAY_MS,
            _ => 0,
        };

        let mut closed: Vec<_> = trades.iter().filter_map(|t| t.balance_after.map(|b| (t.at, b))).collect();
        closed.sort_by_key(|c| c.0);
        let drawdown = closed.last().map_or(Usd(0), |(_, b)| Usd((self.starting_balance - *b).0.max(0)));
        let max_loss = max_total_loss.apply(self.starting_balance);
        let floor = self.starting_balance - max_loss;
        let breached = closed.iter().any(|(_, b)| *b <= floor);

        let reached = n >= REQUIRED_TRADES
            && days >= REQUIRED_DAYS
            // Exact: adherent / n >= 90%.
            && adherent as u64 * 10_000 >= REQUIRED_ADHERENCE.0 as u64 * n as u64
            && !breached;
        ReadinessProgress { counted: n, adherent, adherence, days, drawdown, max_loss, breached, reached }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: Usd = Usd(1_000_000); // $10,000
    const MAX_LOSS: Percent = Percent(1_000); // 10% → $1,000

    fn run() -> ReadinessRun {
        ReadinessRun::first(0, START)
    }

    /// `n` adherent closed trades, one per `gap_days`, each leaving balance at $10,000.
    fn trades(n: u32, gap_days: i64) -> Vec<TradeSummary> {
        (0..n)
            .map(|i| TradeSummary { at: i as i64 * gap_days * DAY_MS, final_adherent: Some(true), balance_after: Some(START) })
            .collect()
    }

    /// `n` trades spread so the first→last span is exactly `span_days`.
    fn spread(n: u32, span_days: i64) -> Vec<TradeSummary> {
        let mut ts = trades(n, 0);
        if let Some(last) = ts.last_mut() {
            last.at = span_days * DAY_MS;
        }
        ts
    }

    #[test]
    fn thresholds_are_fifty_trades_and_eight_weeks() {
        assert_eq!((REQUIRED_TRADES, REQUIRED_DAYS, REQUIRED_ADHERENCE), (50, 56, Percent(9_000)));
    }

    #[test]
    fn runs_number_from_one() {
        let r = run();
        assert_eq!(r.number, 1);
        let r2 = r.next(5, Usd(500_000));
        assert_eq!((r2.number, r2.started_at, r2.starting_balance), (2, 5, Usd(500_000)));
    }

    #[test]
    fn fifty_adherent_trades_over_eight_weeks_reaches_readiness() {
        let p = run().progress(&spread(50, 56), MAX_LOSS);
        assert_eq!((p.counted, p.days, p.adherence), (50, 56, Percent(10_000)));
        assert!(p.reached);
    }

    #[test]
    fn fifty_five_days_is_not_enough() {
        let p = run().progress(&spread(50, 55), MAX_LOSS);
        assert_eq!(p.days, 55);
        assert!(!p.reached);
    }

    #[test]
    fn forty_nine_trades_is_not_enough() {
        assert!(!run().progress(&spread(49, 90), MAX_LOSS).reached);
    }

    #[test]
    fn ninety_percent_adherence_is_the_line() {
        let mut ts = spread(50, 60);
        for t in ts.iter_mut().take(5) {
            t.final_adherent = Some(false);
        }
        let p = run().progress(&ts, MAX_LOSS);
        assert_eq!((p.adherent, p.adherence), (45, Percent(9_000)));
        assert!(p.reached);
        ts[5].final_adherent = Some(false);
        assert!(!run().progress(&ts, MAX_LOSS).reached);
    }

    #[test]
    fn entries_without_a_final_verdict_do_not_count() {
        let mut ts = trades(51, 2);
        ts[0].final_adherent = None;
        ts[50].final_adherent = None; // also excluded from the day span
        let p = run().progress(&ts, MAX_LOSS);
        assert_eq!(p.counted, 49);
        assert_eq!(p.days, 98 - 2);
        assert!(!p.reached);
    }

    #[test]
    fn drawdown_tracks_latest_closed_balance() {
        let ts = [
            TradeSummary { at: 1, final_adherent: Some(true), balance_after: Some(Usd(950_000)) },
            TradeSummary { at: 3, final_adherent: Some(true), balance_after: None }, // still open
            TradeSummary { at: 2, final_adherent: Some(true), balance_after: Some(Usd(970_000)) },
        ];
        let p = run().progress(&ts, MAX_LOSS);
        assert_eq!(p.drawdown, Usd(30_000)); // $10,000 − $9,700
        assert_eq!(p.max_loss, Usd(100_000));
        assert!(!p.breached);
    }

    #[test]
    fn profit_is_zero_drawdown() {
        let ts = [TradeSummary { at: 1, final_adherent: Some(true), balance_after: Some(Usd(1_100_000)) }];
        assert_eq!(run().progress(&ts, MAX_LOSS).drawdown, Usd(0));
    }

    #[test]
    fn touching_max_loss_breaches_even_after_recovering() {
        let mut ts = spread(50, 60);
        ts[3].balance_after = Some(Usd(900_000)); // exactly −$1,000
        let p = run().progress(&ts, MAX_LOSS);
        assert!(p.breached);
        assert!(!p.reached, "a breach blocks readiness even though every other condition holds");
    }

    #[test]
    fn breach_counts_regardless_of_verdict_state() {
        let ts = [TradeSummary { at: 1, final_adherent: None, balance_after: Some(Usd(850_000)) }];
        assert!(run().progress(&ts, MAX_LOSS).breached);
    }

    #[test]
    fn a_new_run_starts_from_zero() {
        let p = run().next(10, START).progress(&[], MAX_LOSS);
        assert_eq!((p.counted, p.days, p.adherence, p.drawdown, p.breached, p.reached), (0, 0, Percent(0), Usd(0), false, false));
    }
}
