//! Weekly Review (PRD FR-11.4): win rate, Expectancy in R, rule breaks and Emotion Tags against
//! broken plans, computed from the week's entries.

use crate::journal::{Emotion, HardRule, RMultiple, r_multiple};
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
    /// Emotion Tag set when the result was recorded.
    pub emotion: Option<Emotion>,
}

/// One Emotion Tag's week: how many tagged trades, and how many of those broke the plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmotionCount {
    pub emotion: Emotion,
    pub trades: u32,
    pub broke_plan: u32,
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
    /// Expectancy: mean R over closed trades with a known risk, rounded toward zero.
    /// `None` when no closed trade has a known risk.
    pub expectancy: Option<RMultiple>,
    pub net: Usd,
    /// Hard Rules broken this week, most frequent first (ties in rule order).
    pub rule_breaks: Vec<(HardRule, u32)>,
    /// Taken trades whose final verdict was "not adherent".
    pub not_adherent: u32,
    /// Emotion Tags used this week, in tag order. A trade broke the plan when a Hard Rule failed
    /// or its final verdict was "not adherent".
    pub emotions: Vec<EmotionCount>,
}

pub fn weekly_review(entries: &[ReviewEntry]) -> WeeklyReview {
    let taken: Vec<_> = entries.iter().filter(|e| !e.skipped).collect();
    let closed: Vec<_> = taken.iter().filter_map(|e| e.pnl.map(|p| (p, e.risk))).collect();
    let wins = closed.iter().filter(|(p, _)| p.0 > 0).count() as u32;
    let n = closed.len() as u32;
    let rs: Vec<i64> = closed.iter().filter_map(|(p, r)| r.and_then(|r| r_multiple(*p, r)).map(|r| r.0)).collect();
    let expectancy = (!rs.is_empty()).then(|| RMultiple(rs.iter().sum::<i64>() / rs.len() as i64));

    let mut emotions: std::collections::BTreeMap<Emotion, EmotionCount> = Default::default();
    for e in &taken {
        if let Some(em) = e.emotion {
            let c = emotions.entry(em).or_insert(EmotionCount { emotion: em, trades: 0, broke_plan: 0 });
            c.trades += 1;
            c.broke_plan += (!e.failed_rules.is_empty() || e.adherent == Some(false)) as u32;
        }
    }

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
        expectancy,
        net: Usd(closed.iter().map(|(p, _)| p.0).sum()),
        rule_breaks,
        not_adherent: taken.iter().filter(|e| e.adherent == Some(false)).count() as u32,
        emotions: emotions.into_values().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closed(pnl: i64, risk: i64) -> ReviewEntry {
        ReviewEntry { skipped: false, pnl: Some(Usd(pnl)), risk: Some(Usd(risk)), failed_rules: vec![], adherent: Some(true), emotion: None }
    }

    #[test]
    fn empty_week() {
        let r = weekly_review(&[]);
        assert_eq!((r.taken, r.closed, r.win_rate, r.expectancy, r.net), (0, 0, Percent(0), None, Usd(0)));
        assert!(r.emotions.is_empty());
        assert!(r.rule_breaks.is_empty());
    }

    #[test]
    fn win_rate_average_r_and_net() {
        // +2R, −1R, +1.5R, break-even
        let es = [closed(20_000, 10_000), closed(-10_000, 10_000), closed(15_000, 10_000), closed(0, 10_000)];
        let r = weekly_review(&es);
        assert_eq!((r.taken, r.closed, r.wins), (4, 4, 2));
        assert_eq!(r.win_rate, Percent(5_000));
        assert_eq!(r.expectancy, Some(RMultiple(62))); // (200 − 100 + 150 + 0) / 4 = 62.5 → 62
        assert_eq!(r.net, Usd(25_000));
    }

    #[test]
    fn open_and_skipped_entries() {
        let open = ReviewEntry { pnl: None, adherent: None, ..closed(0, 10_000) };
        let skipped = ReviewEntry { skipped: true, pnl: None, risk: None, failed_rules: vec![], adherent: None, emotion: None };
        let r = weekly_review(&[closed(10_000, 10_000), open, skipped]);
        assert_eq!((r.taken, r.skipped, r.closed, r.wins), (2, 1, 1, 1));
        assert_eq!(r.expectancy, Some(RMultiple(100)));
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
        assert_eq!((r.closed, r.expectancy), (2, Some(RMultiple(200))));
    }

    #[test]
    fn expectancy_of_two_wins_and_two_losses() {
        // +2R, −1R, −1R, +2R → (2 − 1 − 1 + 2) / 4 = +0.5R
        let es = [closed(20_000, 10_000), closed(-10_000, 10_000), closed(-10_000, 10_000), closed(20_000, 10_000)];
        let r = weekly_review(&es);
        assert_eq!(r.expectancy, Some(RMultiple(50)));
        assert_eq!(r.expectancy.unwrap().to_string(), "+0.5R");
    }

    #[test]
    fn emotions_against_broken_plans() {
        let tag = |mut e: ReviewEntry, em: Emotion| {
            e.emotion = Some(em);
            e
        };
        let mut revenge = closed(-10_000, 10_000);
        revenge.failed_rules = vec![HardRule::TradesPerDay];
        revenge.adherent = Some(false);
        let mut fomo_soft = closed(-5_000, 10_000);
        fomo_soft.adherent = Some(false);
        let es = [
            tag(closed(20_000, 10_000), Emotion::Calm),
            tag(closed(-10_000, 10_000), Emotion::Calm),
            tag(revenge, Emotion::Revenge),
            tag(fomo_soft, Emotion::Fomo),
            closed(10_000, 10_000), // untagged
        ];
        let r = weekly_review(&es);
        assert_eq!(
            r.emotions,
            vec![
                EmotionCount { emotion: Emotion::Calm, trades: 2, broke_plan: 0 },
                EmotionCount { emotion: Emotion::Fomo, trades: 1, broke_plan: 1 },
                EmotionCount { emotion: Emotion::Revenge, trades: 1, broke_plan: 1 },
            ]
        );
    }
}
