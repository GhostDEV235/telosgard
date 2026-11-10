//! Weekly Review (PRD FR-11.4): win rate, average R and rule breaks, computed from the week's entries.

use crate::journal::HardRule;
use crate::money::{Percent, Usd};
use serde::{Deserialize, Serialize};

/// What the review needs from one Journal Entry. The caller picks the week.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewEntry {
    pub skipped: bool,
    /// Closed P/L; `None` while open (or for skipped setups).
    pub pnl: Option<Usd>,
    /// Money at risk when the stop was placed (1R for this trade).
    pub risk: Option<Usd>,
    pub failed_rules: Vec<HardRule>,
    /// Final Adherence Verdict, if any.
    pub adherent: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyReview {
    pub taken: u32,
    pub skipped: u32,
    pub closed: u32,
    pub wins: u32,
    /// Wins ÷ closed trades, rounded down. A break-even trade is not a win.
    pub win_rate: Percent,
    /// Mean R over closed trades with a known risk, ×100, rounded toward zero (+1.5R = 150).
    pub avg_r: i64,
    pub net: Usd,
    /// Hard Rules broken this week, most frequent first (ties in rule order).
    pub rule_breaks: Vec<(HardRule, u32)>,
    /// Taken trades whose final verdict was "not adherent".
    pub not_adherent: u32,
}

pub fn weekly_review(entries: &[ReviewEntry]) -> WeeklyReview {
    let taken: Vec<_> = entries.iter().filter(|e| !e.skipped).collect();
    let closed: Vec<_> = taken.iter().filter_map(|e| e.pnl.map(|p| (p, e.risk))).collect();
    let wins = closed.iter().filter(|(p, _)| p.0 > 0).count() as u32;
    let n = closed.len() as u32;
    let rs: Vec<i64> = closed.iter().filter_map(|(p, r)| r.filter(|r| r.0 > 0).map(|r| p.0 * 100 / r.0)).collect();
    let avg_r = if rs.is_empty() { 0 } else { rs.iter().sum::<i64>() / rs.len() as i64 };

    let mut counts: std::collections::BTreeMap<HardRule, u32> = Default::default();
    for rule in taken.iter().flat_map(|e| e.failed_rules.iter()) {
        *counts.entry(*rule).or_default() += 1;
    }
    let mut rule_breaks: Vec<_> = counts.into_iter().collect();
    // BTreeMap gives rule order; a stable sort by count keeps it for ties.
    rule_breaks.sort_by(|a, b| b.1.cmp(&a.1));

    WeeklyReview {
        taken: taken.len() as u32,
        skipped: entries.iter().filter(|e| e.skipped).count() as u32,
        closed: n,
        wins,
        win_rate: if n == 0 { Percent(0) } else { Percent(wins * 10_000 / n) },
        avg_r,
        net: Usd(closed.iter().map(|(p, _)| p.0).sum()),
        rule_breaks,
        not_adherent: taken.iter().filter(|e| e.adherent == Some(false)).count() as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closed(pnl: i64, risk: i64) -> ReviewEntry {
        ReviewEntry { skipped: false, pnl: Some(Usd(pnl)), risk: Some(Usd(risk)), failed_rules: vec![], adherent: Some(true) }
    }

    #[test]
    fn empty_week() {
        let r = weekly_review(&[]);
        assert_eq!((r.taken, r.closed, r.win_rate, r.avg_r, r.net), (0, 0, Percent(0), 0, Usd(0)));
        assert!(r.rule_breaks.is_empty());
    }

    #[test]
    fn win_rate_average_r_and_net() {
        // +2R, −1R, +1.5R, break-even
        let es = [closed(20_000, 10_000), closed(-10_000, 10_000), closed(15_000, 10_000), closed(0, 10_000)];
        let r = weekly_review(&es);
        assert_eq!((r.taken, r.closed, r.wins), (4, 4, 2));
        assert_eq!(r.win_rate, Percent(5_000));
        assert_eq!(r.avg_r, 62); // (200 − 100 + 150 + 0) / 4 = 62.5 → 62
        assert_eq!(r.net, Usd(25_000));
    }

    #[test]
    fn open_and_skipped_entries() {
        let open = ReviewEntry { pnl: None, adherent: None, ..closed(0, 10_000) };
        let skipped = ReviewEntry { skipped: true, pnl: None, risk: None, failed_rules: vec![], adherent: None };
        let r = weekly_review(&[closed(10_000, 10_000), open, skipped]);
        assert_eq!((r.taken, r.skipped, r.closed, r.wins), (2, 1, 1, 1));
        assert_eq!(r.avg_r, 100);
    }

    #[test]
    fn rule_breaks_most_frequent_first() {
        let mut a = closed(-5_000, 10_000);
        a.failed_rules = vec![HardRule::RiskPerTrade, HardRule::ChecklistDone];
        a.adherent = Some(false);
        let mut b = closed(-5_000, 10_000);
        b.failed_rules = vec![HardRule::ChecklistDone];
        b.adherent = Some(false);
        let mut c = closed(-5_000, 10_000);
        c.failed_rules = vec![HardRule::StopLoss];
        c.adherent = Some(false);
        let r = weekly_review(&[a, b, c, closed(1, 10_000)]);
        assert_eq!(r.rule_breaks, vec![(HardRule::ChecklistDone, 2), (HardRule::StopLoss, 1), (HardRule::RiskPerTrade, 1)]);
        assert_eq!(r.not_adherent, 3);
    }

    #[test]
    fn unknown_risk_is_left_out_of_average_r() {
        let mut no_risk = closed(5_000, 0);
        no_risk.risk = None;
        let r = weekly_review(&[closed(20_000, 10_000), no_risk]);
        assert_eq!((r.closed, r.avg_r), (2, 200));
    }
}
