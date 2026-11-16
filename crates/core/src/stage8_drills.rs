//! Stage 8 (Wave 8): "Spot the broken rule". A Demo Trade checked against a fixed example
//! Trading Plan: it breaks one Hard Rule, or none. Deterministic from a seed; prices illustrative.

use crate::drills::Rng;
use crate::i18n::t;

/// The example plan the drill checks against.
pub const MAX_RISK_CENTS: i64 = 10_000; // 1% of $10,000
pub const BALANCE_CENTS: i64 = 1_000_000;
pub const MIN_RR_X10: i64 = 20; // 2:1
pub const DAILY_LIMIT_R: i64 = 2;
/// US CPI at 08:30 New York; No-Trade Window ±5 minutes.
pub const NEWS_AT: u32 = 8 * 60 + 30;
/// Sessions traded: London and New York, 03:00–17:00 New York time.
pub const SESSIONS: (u32, u32) = (3 * 60, 17 * 60);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breach {
    None,
    Risk,
    Reward,
    DailyLimit,
    NoTradeWindow,
    Checklist,
    Session,
}

impl Breach {
    pub const ALL: [Breach; 7] = [Breach::None, Breach::Risk, Breach::Reward, Breach::DailyLimit, Breach::NoTradeWindow, Breach::Checklist, Breach::Session];
    pub fn label(self) -> &'static str {
        match self {
            Breach::None => t("None: it follows the plan", "Aucune : il respecte le plan"),
            Breach::Risk => t("Max risk per trade", "Risque maximum par trade"),
            Breach::Reward => t("Minimum reward-to-risk", "Rapport gain/risque minimum"),
            Breach::DailyLimit => t("Daily Loss Limit", "Perte maximale du jour"),
            Breach::NoTradeWindow => t("No-Trade Windows", "Fenêtres sans trade"),
            Breach::Checklist => t("Pre-Session Checklist", "Checklist avant la séance"),
            Breach::Session => t("Sessions you trade", "Séances tradées"),
        }
    }
}

/// Earlier closed trades today, in R.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Earlier {
    Nothing,
    OneWin,
    OneLoss,
    TwoLosses,
}

impl Earlier {
    pub fn total_r(self) -> i64 {
        match self {
            Earlier::Nothing => 0,
            Earlier::OneWin => 2,
            Earlier::OneLoss => -1,
            Earlier::TwoLosses => -2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleQ {
    /// Minutes after midnight, New York time.
    pub at: u32,
    pub checklist: bool,
    pub earlier: Earlier,
    /// Stop distance and target distance, in cents of price.
    pub stop: i64,
    pub target: i64,
    /// Size in hundredths of a lot (= ounces at 100 oz a lot).
    pub lots: i64,
    pub answer: Breach,
}

impl RuleQ {
    pub fn new(seed: u64) -> RuleQ {
        let mut rng = Rng::new(seed);
        let answer = Breach::ALL[rng.below(Breach::ALL.len() as u64) as usize];
        let stop_d = 5 + rng.below(8) as i64; // $5 to $12
        let stop = stop_d * 100;
        let mut lots = MAX_RISK_CENTS / stop; // risk ≤ 1%
        let mut target = stop * (4 + rng.below(3) as i64) / 2; // 2:1, 2.5:1 or 3:1
        let mut at = 9 * 60 + 5 * rng.below(33) as u32; // 09:00–11:40
        let mut checklist = true;
        let mut earlier = [Earlier::Nothing, Earlier::OneWin, Earlier::OneLoss][rng.below(3) as usize];
        match answer {
            Breach::None => {}
            Breach::Risk => lots = (15_000 + stop - 1) / stop, // about 1.5%
            Breach::Reward => target = stop * 3 / 2,
            Breach::DailyLimit => earlier = Earlier::TwoLosses,
            Breach::NoTradeWindow => at = [8 * 60 + 27, 8 * 60 + 28, 8 * 60 + 32, 8 * 60 + 34][rng.below(4) as usize],
            Breach::Checklist => checklist = false,
            Breach::Session => at = 19 * 60 + 30 + 10 * rng.below(13) as u32, // 19:30–21:30, Asian session
        }
        RuleQ { at, checklist, earlier, stop, target, lots, answer }
    }

    /// Money at risk at the stop, in cents: stop distance × ounces.
    pub fn risk(&self) -> i64 {
        self.stop * self.lots
    }

    /// Every Hard Rule this trade breaks, checked by calculation.
    pub fn breaches(&self) -> Vec<Breach> {
        let mut b = Vec::new();
        if self.risk() > MAX_RISK_CENTS {
            b.push(Breach::Risk);
        }
        if self.target * 10 < self.stop * MIN_RR_X10 {
            b.push(Breach::Reward);
        }
        if self.earlier.total_r() <= -DAILY_LIMIT_R {
            b.push(Breach::DailyLimit);
        }
        if self.at.abs_diff(NEWS_AT) <= 5 {
            b.push(Breach::NoTradeWindow);
        }
        if !self.checklist {
            b.push(Breach::Checklist);
        }
        if !(SESSIONS.0..SESSIONS.1).contains(&self.at) {
            b.push(Breach::Session);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_trade_breaks_exactly_the_rule_it_was_built_to_break() {
        let mut seen = Vec::new();
        for seed in 0..700 {
            let q = RuleQ::new(seed);
            let b = q.breaches();
            if q.answer == Breach::None {
                assert!(b.is_empty(), "seed {seed}: {b:?}");
            } else {
                assert_eq!(b, vec![q.answer], "seed {seed}");
            }
            if !seen.contains(&q.answer) {
                seen.push(q.answer);
            }
        }
        assert_eq!(seen.len(), Breach::ALL.len(), "every answer comes up");
    }

    #[test]
    fn the_risk_breach_is_well_over_one_percent_and_the_rest_well_under() {
        for seed in 0..300 {
            let q = RuleQ::new(seed);
            let risk = q.risk();
            if q.answer == Breach::Risk {
                assert!(risk >= 14_000, "seed {seed}: {risk}");
            } else {
                assert!(risk <= MAX_RISK_CENTS && risk >= 8_000, "seed {seed}: {risk}");
            }
        }
    }
}
