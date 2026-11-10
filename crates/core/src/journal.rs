//! Journal Entries, deterministic Hard Rule evaluation, and the Adherence Verdict lifecycle (PRD FR-6).

use crate::money::{Lots, Percent, Price, Usd};
use crate::plan::{HardRules, Session};
use crate::{DAY_MS, Millis};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Buy,
    Sell,
}

/// The trade fields of a Journal Entry. Locked once a verdict exists (FR-6.5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeEntry {
    pub at: Millis,
    pub session: Session,
    pub direction: Direction,
    pub entry: Price,
    pub stop: Option<Price>,
    pub take_profit: Option<Price>,
    pub lots: Lots,
    pub balance_before: Usd,
    pub contract_oz: u32,
    pub setup_reason: String,
    pub plan_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardRule {
    StopLoss,
    RiskPerTrade,
    MinRewardRisk,
    Session,
    TradesPerDay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardRuleResult {
    pub rule: HardRule,
    pub pass: bool,
    /// Plain-language reason, e.g. "Risk 1.5% is above your 1% limit".
    pub detail: String,
}

/// Evaluates every Hard Rule, always in the same order. `trades_earlier_today` excludes this entry.
pub fn evaluate_hard_rules(e: &TradeEntry, rules: &HardRules, trades_earlier_today: u32) -> Vec<HardRuleResult> {
    let r = |rule, pass, detail: String| HardRuleResult { rule, pass, detail };
    // Distance from entry to stop/target in the trade's favour (positive = correct side).
    let favour = |p: Price| match e.direction {
        Direction::Buy => p - e.entry,
        Direction::Sell => e.entry - p,
    };
    let risk_dist = e.stop.map(|s| -favour(s)).filter(|d| d.0 > 0);

    let stop = match (e.stop, risk_dist) {
        (None, _) => r(HardRule::StopLoss, false, "No stop-loss was set. Every trade needs one.".into()),
        (Some(s), None) => {
            let side = if e.direction == Direction::Buy { "below" } else { "above" };
            r(HardRule::StopLoss, false, format!("The stop at {s} must be {side} the entry at {}.", e.entry))
        }
        (Some(s), Some(d)) => r(HardRule::StopLoss, true, format!("Stop-loss set at {s}, a {d} move away."))
    };

    let risk = match risk_dist {
        None => r(HardRule::RiskPerTrade, false, "Risk can't be measured without a valid stop-loss.".into()),
        Some(d) => {
            let money = d.value(e.lots, e.contract_oz);
            let pct = Percent::of(money, e.balance_before);
            // Exact comparison: money / balance <= max_risk.
            let ok = money.0 as i128 * 10_000 <= rules.max_risk.0 as i128 * e.balance_before.0 as i128;
            let verdict = if ok { "within" } else { "above" };
            r(HardRule::RiskPerTrade, ok, format!("Risk {money} ({pct}) is {verdict} your {} limit.", rules.max_risk))
        }
    };

    let reward_dist = e.take_profit.map(favour).filter(|d| d.0 > 0);
    let min = rules.min_reward_risk;
    let rr = match (risk_dist, e.take_profit, reward_dist) {
        (None, _, _) => r(HardRule::MinRewardRisk, false, "Reward-to-risk can't be measured without a valid stop-loss.".into()),
        (_, None, _) => r(HardRule::MinRewardRisk, false, "No take-profit was set, so reward-to-risk can't be measured.".into()),
        (_, Some(tp), None) => r(HardRule::MinRewardRisk, false, format!("The take-profit at {tp} is on the losing side of the entry.")),
        (Some(risk), _, Some(reward)) => {
            let ok = reward.0 * 100 >= min.0 as i64 * risk.0;
            let ratio = Percent((reward.0 * 100 / risk.0) as u32);
            let verdict = if ok { "meets" } else { "is below" };
            r(HardRule::MinRewardRisk, ok, format!("Reward {reward} vs risk {risk} is {}R, which {verdict} your {}R minimum.", ratio_str(ratio), ratio_str(min)))
        }
    };

    let in_session = rules.sessions.contains(&e.session);
    let session = r(
        HardRule::Session,
        in_session,
        if in_session { format!("{} is one of your sessions.", session_name(e.session)) } else { format!("{} isn't one of your plan's sessions.", session_name(e.session)) },
    );

    let nth = trades_earlier_today + 1;
    let ok = nth <= rules.max_trades_per_day;
    let per_day = r(HardRule::TradesPerDay, ok, format!("Trade {nth} of the day; your limit is {}.", rules.max_trades_per_day));

    vec![stop, risk, rr, session, per_day]
}

fn ratio_str(p: Percent) -> String {
    p.to_string().trim_end_matches('%').to_string()
}

pub fn session_name(s: Session) -> &'static str {
    match s {
        Session::Asian => "The Asian session",
        Session::London => "The London session",
        Session::NewYork => "The New York session",
    }
}

/// Profit (positive) or loss (negative) when the trade closes at `exit`.
pub fn pnl(e: &TradeEntry, exit: Price) -> Usd {
    let moved = match e.direction {
        Direction::Buy => exit - e.entry,
        Direction::Sell => e.entry - exit,
    };
    moved.value(e.lots, e.contract_oz)
}

/// The Learner's calendar day for a timestamp, given their UTC offset in minutes.
pub fn local_day(at: Millis, utc_offset_min: i32) -> i64 {
    (at + utc_offset_min as i64 * 60_000).div_euclid(DAY_MS)
}

/// The verdict lifecycle. A verdict is final when Judged, or when a Dispute has been re-judged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Adherence {
    #[default]
    Pending,
    Judged { adherent: bool },
    DisputePending { first: bool },
    Disputed { first: bool, second: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisputeError {
    NotJudged,
    AlreadyAdherent,
    HardRuleFailed,
    AlreadyDisputed,
}

/// Claude can never overturn a Hard Rule failure.
pub fn combine(hard: &[HardRuleResult], claude_adherent: bool) -> bool {
    claude_adherent && hard.iter().all(|r| r.pass)
}

impl Adherence {
    pub fn final_adherent(self) -> Option<bool> {
        match self {
            Adherence::Judged { adherent } => Some(adherent),
            Adherence::Disputed { second, .. } => Some(second),
            Adherence::Pending | Adherence::DisputePending { .. } => None,
        }
    }
    pub fn dispute(self, hard: &[HardRuleResult]) -> Result<Adherence, DisputeError> {
        match self {
            Adherence::Pending => Err(DisputeError::NotJudged),
            Adherence::DisputePending { .. } | Adherence::Disputed { .. } => Err(DisputeError::AlreadyDisputed),
            Adherence::Judged { adherent: true } => Err(DisputeError::AlreadyAdherent),
            Adherence::Judged { adherent: false } if hard.iter().any(|r| !r.pass) => Err(DisputeError::HardRuleFailed),
            Adherence::Judged { adherent: false } => Ok(Adherence::DisputePending { first: false }),
        }
    }
    /// Stores Claude's second verdict. Only valid from `DisputePending`; returns `self` unchanged otherwise.
    pub fn resolve_dispute(self, second: bool) -> Adherence {
        match self {
            Adherence::DisputePending { first } => Adherence::Disputed { first, second },
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money::DEFAULT_CONTRACT_OZ;
    use crate::plan::sample_rules;

    fn buy() -> TradeEntry {
        TradeEntry {
            at: 0,
            session: Session::London,
            direction: Direction::Buy,
            entry: Price::parse("2350").unwrap(),
            stop: Price::parse("2345"),        // $5 below
            take_profit: Price::parse("2360"), // $10 above → 2R
            lots: Lots(20),
            balance_before: Usd::parse("10000").unwrap(),
            contract_oz: DEFAULT_CONTRACT_OZ,
            setup_reason: "H4 uptrend, bounce off 2345 support, bullish engulfing".into(),
            plan_version: 1,
        }
    }

    fn result(e: &TradeEntry, rule: HardRule, earlier: u32) -> HardRuleResult {
        let (hard, _) = sample_rules();
        evaluate_hard_rules(e, &hard, earlier).into_iter().find(|r| r.rule == rule).unwrap()
    }

    #[test]
    fn a_clean_trade_passes_every_rule_in_a_fixed_order() {
        let (hard, _) = sample_rules();
        let rs = evaluate_hard_rules(&buy(), &hard, 0);
        let order: Vec<_> = rs.iter().map(|r| r.rule).collect();
        use HardRule::*;
        assert_eq!(order, vec![StopLoss, RiskPerTrade, MinRewardRisk, Session, TradesPerDay]);
        assert!(rs.iter().all(|r| r.pass), "{rs:?}");
    }

    #[test]
    fn missing_or_wrong_side_stop_fails() {
        let no_stop = TradeEntry { stop: None, ..buy() };
        assert!(!result(&no_stop, HardRule::StopLoss, 0).pass);
        assert!(!result(&no_stop, HardRule::RiskPerTrade, 0).pass, "risk can't be measured without a stop");
        let above = TradeEntry { stop: Price::parse("2355"), ..buy() };
        assert!(!result(&above, HardRule::StopLoss, 0).pass);
        let sell = TradeEntry { direction: Direction::Sell, stop: Price::parse("2355"), take_profit: Price::parse("2340"), ..buy() };
        assert!(result(&sell, HardRule::StopLoss, 0).pass);
    }

    #[test]
    fn risk_exactly_at_limit_passes_one_cent_over_fails() {
        // 0.20 lots × $5 = $100 = 1% of $10,000.
        assert!(result(&buy(), HardRule::RiskPerTrade, 0).pass);
        let over = TradeEntry { lots: Lots(21), ..buy() }; // $105
        let r = result(&over, HardRule::RiskPerTrade, 0);
        assert!(!r.pass);
        assert!(r.detail.contains("1.05%") && r.detail.contains("1%"), "{}", r.detail);
    }

    #[test]
    fn reward_to_risk() {
        let thin = TradeEntry { take_profit: Price::parse("2357.49"), ..buy() }; // 1.498R < 1.5R
        assert!(!result(&thin, HardRule::MinRewardRisk, 0).pass);
        let exact = TradeEntry { take_profit: Price::parse("2357.50"), ..buy() }; // 1.5R
        assert!(result(&exact, HardRule::MinRewardRisk, 0).pass);
        let none = TradeEntry { take_profit: None, ..buy() };
        assert!(!result(&none, HardRule::MinRewardRisk, 0).pass);
        let wrong_side = TradeEntry { take_profit: Price::parse("2340"), ..buy() };
        assert!(!result(&wrong_side, HardRule::MinRewardRisk, 0).pass);
    }

    #[test]
    fn session_and_trades_per_day() {
        let asian = TradeEntry { session: Session::Asian, ..buy() };
        assert!(!result(&asian, HardRule::Session, 0).pass);
        assert!(result(&buy(), HardRule::TradesPerDay, 1).pass); // 2nd of max 2
        assert!(!result(&buy(), HardRule::TradesPerDay, 2).pass); // 3rd
    }

    #[test]
    fn pnl_by_direction() {
        let e = buy();
        assert_eq!(pnl(&e, Price::parse("2360").unwrap()), Usd::parse("200").unwrap());
        assert_eq!(pnl(&e, Price::parse("2345").unwrap()), Usd::parse("-100").unwrap());
        let s = TradeEntry { direction: Direction::Sell, ..buy() };
        assert_eq!(pnl(&s, Price::parse("2345").unwrap()), Usd::parse("100").unwrap());
    }

    #[test]
    fn local_day_respects_offset() {
        let late_utc = DAY_MS - 60_000; // 23:59 UTC on day 0
        assert_eq!(local_day(late_utc, 0), 0);
        assert_eq!(local_day(late_utc, 60), 1); // 00:59 next day at UTC+1
        assert_eq!(local_day(0, -300), -1); // 19:00 previous day at UTC−5
    }

    #[test]
    fn claude_cannot_overturn_a_hard_failure() {
        let (hard, _) = sample_rules();
        let ok = evaluate_hard_rules(&buy(), &hard, 0);
        assert!(combine(&ok, true));
        assert!(!combine(&ok, false));
        let bad = evaluate_hard_rules(&TradeEntry { lots: Lots(50), ..buy() }, &hard, 0);
        assert!(!combine(&bad, true));
    }

    #[test]
    fn dispute_lifecycle() {
        let (hard, _) = sample_rules();
        let ok = evaluate_hard_rules(&buy(), &hard, 0);
        assert_eq!(Adherence::Pending.final_adherent(), None);
        assert_eq!(Adherence::Pending.dispute(&ok), Err(DisputeError::NotJudged));
        assert_eq!(Adherence::Judged { adherent: true }.dispute(&ok), Err(DisputeError::AlreadyAdherent));

        let judged = Adherence::Judged { adherent: false };
        assert_eq!(judged.final_adherent(), Some(false));
        let pending = judged.dispute(&ok).unwrap();
        assert_eq!(pending, Adherence::DisputePending { first: false });
        assert_eq!(pending.final_adherent(), None, "a pending dispute is not final");
        let done = pending.resolve_dispute(true);
        assert_eq!(done, Adherence::Disputed { first: false, second: true });
        assert_eq!(done.final_adherent(), Some(true));
        assert_eq!(done.dispute(&ok), Err(DisputeError::AlreadyDisputed), "one dispute per verdict");
        assert_eq!(judged.resolve_dispute(true), judged);
    }

    #[test]
    fn hard_rule_failures_cannot_be_disputed() {
        let (hard, _) = sample_rules();
        let bad = evaluate_hard_rules(&TradeEntry { stop: None, ..buy() }, &hard, 0);
        assert_eq!(Adherence::Judged { adherent: false }.dispute(&bad), Err(DisputeError::HardRuleFailed));
    }
}
