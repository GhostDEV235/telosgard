//! Journal Entries, deterministic Hard Rule evaluation, and the Adherence Verdict lifecycle (PRD FR-6).

use crate::checklist::PreSessionChecklist;
use crate::money::{Lots, Percent, Price, Usd};
use crate::plan::{HardRules, Session};
use crate::time::{NoTrade, no_trade_window, sessions_at};
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
    /// Entry time, UTC. The session is derived from it.
    pub at: Millis,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardRule {
    StopLoss,
    RiskPerTrade,
    MinRewardRisk,
    Session,
    TradesPerDay,
    DailyLossLimit,
    NoTradeWindow,
    ChecklistDone,
}

/// What the rules need to know about the rest of the Learner's day.
#[derive(Debug, Clone, Copy, Default)]
pub struct DayContext<'a> {
    /// Taken trades earlier the same local day (excludes this entry).
    pub trades_earlier_today: u32,
    /// Net P/L of trades closed earlier the same local day.
    pub realized_today: Usd,
    /// The Pre-Session Checklist saved for the entry's local day.
    pub checklist: Option<&'a PreSessionChecklist>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardRuleResult {
    pub rule: HardRule,
    pub pass: bool,
    /// Plain-language reason, e.g. "Risk 1.5% is above your 1% limit".
    pub detail: String,
}

/// Evaluates every Hard Rule, always in the same order.
pub fn evaluate_hard_rules(e: &TradeEntry, rules: &HardRules, day: &DayContext) -> Vec<HardRuleResult> {
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

    let session = session_rule(e, rules);

    let nth = day.trades_earlier_today + 1;
    let ok = nth <= rules.max_trades_per_day;
    let per_day = r(HardRule::TradesPerDay, ok, format!("Trade {nth} of the day; your limit is {}.", rules.max_trades_per_day));

    vec![stop, risk, rr, session, per_day, daily_loss_rule(e, rules, day), no_trade_rule(e, day), checklist_rule(day)]
}

fn session_rule(e: &TradeEntry, rules: &HardRules) -> HardRuleResult {
    let now = sessions_at(e.at);
    let pass = now.iter().any(|s| rules.sessions.contains(s));
    let detail = match now.as_slice() {
        [] => "No session was running when you entered.".to_string(),
        [a, b] => {
            let verdict = if pass { "which your plan allows" } else { "neither of which your plan allows" };
            format!("Entered in the overlap of {} and {}, {verdict}.", session_name(*a).to_lowercase(), session_name(*b).to_lowercase())
        }
        [one, ..] if pass => format!("{} is one of your sessions.", session_name(*one)),
        [one, ..] => format!("{} isn't one of your plan's sessions.", session_name(*one)),
    };
    HardRuleResult { rule: HardRule::Session, pass, detail }
}

fn daily_loss_rule(e: &TradeEntry, rules: &HardRules, day: &DayContext) -> HardRuleResult {
    let one_r = rules.max_risk.apply(e.balance_before);
    let limit = Usd(one_r.0 * rules.daily_loss_limit.0 as i64 / 100);
    let lost = Usd((-day.realized_today.0).max(0));
    let pass = lost < limit;
    let r = ratio_str(rules.daily_loss_limit);
    let detail = if pass {
        format!("Down {lost} today, under your {limit} ({r}R) daily limit.")
    } else {
        format!("Down {lost} today: your {limit} ({r}R) daily limit was already reached.")
    };
    HardRuleResult { rule: HardRule::DailyLossLimit, pass, detail }
}

fn no_trade_rule(e: &TradeEntry, day: &DayContext) -> HardRuleResult {
    let events = day.checklist.map(|c| c.tier_one()).unwrap_or_default();
    let (pass, detail) = match no_trade_window(e.at, &events) {
        None => (true, "Outside every No-Trade Window.".to_string()),
        Some(NoTrade::Rollover) => (false, "Entered during the daily rollover (17:00–18:00 New York), when spreads widen.".to_string()),
        Some(NoTrade::SundayOpen) => (false, "Entered in the first hour after the Sunday open, when prices gap and spreads are wide.".to_string()),
        Some(NoTrade::MarketClosed) => (false, "The market was closed at that time. Check the entry time.".to_string()),
        Some(NoTrade::NearEvent { name }) => (false, format!("Entered within 5 minutes of {name}, a tier-1 release.")),
    };
    HardRuleResult { rule: HardRule::NoTradeWindow, pass, detail }
}

fn checklist_rule(day: &DayContext) -> HardRuleResult {
    let pass = day.checklist.is_some();
    let detail = if pass {
        "Pre-Session Checklist done for this day.".to_string()
    } else {
        "No Pre-Session Checklist was saved for this day.".to_string()
    };
    HardRuleResult { rule: HardRule::ChecklistDone, pass, detail }
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

/// How the Learner felt when the trade closed (Emotion Tag), chosen when recording the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Emotion {
    Calm,
    Fomo,
    Revenge,
    Fear,
    Overconfident,
}

impl Emotion {
    pub const ALL: [Emotion; 5] = [Emotion::Calm, Emotion::Fomo, Emotion::Revenge, Emotion::Fear, Emotion::Overconfident];
    pub fn label(self) -> &'static str {
        match self {
            Emotion::Calm => "Calm",
            Emotion::Fomo => "FOMO",
            Emotion::Revenge => "Revenge",
            Emotion::Fear => "Fear",
            Emotion::Overconfident => "Overconfident",
        }
    }
}

/// A result measured in units of the planned risk (R), in hundredths: −$100 risking $100 = −100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RMultiple(pub i64);

impl std::fmt::Display for RMultiple {
    /// One decimal, rounded half away from zero, with a sign: "+2.0R", "−1.0R", "+0.5R".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tenths = (self.0.abs() + 5) / 10;
        let sign = if tenths == 0 { "" } else if self.0 < 0 { "−" } else { "+" };
        write!(f, "{sign}{}.{}R", tenths / 10, tenths % 10)
    }
}

/// P/L ÷ risk, in hundredths, rounded toward zero. `None` without a positive risk.
pub fn r_multiple(pnl: Usd, risk: Usd) -> Option<RMultiple> {
    (risk.0 > 0).then(|| RMultiple(pnl.0 * 100 / risk.0))
}

/// How long after logging an entry can still be deleted (for test or mistaken entries). After
/// that it is part of the record: deleting real trades would distort Readiness.
pub const DELETE_WINDOW_MS: Millis = DAY_MS;

pub fn can_delete(logged_at: Millis, now: Millis) -> bool {
    now - logged_at < DELETE_WINDOW_MS
}

/// A Readiness Run left with no entries after a delete is removed too, but only the latest run and
/// only while it is still open: an ended run is history.
pub fn remove_emptied_run(is_latest: bool, ended: bool, entries_left: usize) -> bool {
    is_latest && !ended && entries_left == 0
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
    #[test]
    fn entries_can_be_deleted_for_a_day_after_logging() {
        let t = 1_000_000_000;
        assert!(can_delete(t, t));
        assert!(can_delete(t, t + DAY_MS - 1));
        assert!(!can_delete(t, t + DAY_MS));
        assert!(can_delete(t, t - 5_000), "a device clock a little behind still allows it");
    }

    #[test]
    fn only_the_open_latest_run_is_removed_when_emptied() {
        assert!(remove_emptied_run(true, false, 0));
        assert!(!remove_emptied_run(true, false, 1), "other entries remain");
        assert!(!remove_emptied_run(true, true, 0), "an ended run is history");
        assert!(!remove_emptied_run(false, false, 0), "not the latest run");
    }

    #[test]
    fn r_multiple_from_pnl_and_risk() {
        assert_eq!(r_multiple(Usd(-10_000), Usd(10_000)), Some(RMultiple(-100)));
        assert_eq!(r_multiple(Usd(20_000), Usd(10_000)), Some(RMultiple(200)));
        assert_eq!(r_multiple(Usd(5_000), Usd(10_000)), Some(RMultiple(50)));
        assert_eq!(r_multiple(Usd(-3_333), Usd(10_000)), Some(RMultiple(-33)));
        assert_eq!(r_multiple(Usd(5_000), Usd(0)), None);
    }

    #[test]
    fn r_multiple_display() {
        assert_eq!(RMultiple(-100).to_string(), "−1.0R");
        assert_eq!(RMultiple(200).to_string(), "+2.0R");
        assert_eq!(RMultiple(50).to_string(), "+0.5R");
        assert_eq!(RMultiple(125).to_string(), "+1.3R");
        assert_eq!(RMultiple(-4).to_string(), "0.0R");
        assert_eq!(RMultiple(0).to_string(), "0.0R");
    }

    use super::*;
    use crate::checklist::sample;
    use crate::money::DEFAULT_CONTRACT_OZ;
    use crate::plan::sample_rules;
    use crate::time::{days_from_civil, utc};

    fn checklist() -> PreSessionChecklist {
        sample(days_from_civil(2026, 1, 13))
    }

    /// A normal day: checklist done, nothing earlier.
    fn ok_day(c: &PreSessionChecklist) -> DayContext<'_> {
        DayContext { trades_earlier_today: 0, realized_today: Usd(0), checklist: Some(c) }
    }

    fn buy() -> TradeEntry {
        TradeEntry {
            at: utc(2026, 1, 13, 9, 0), // Tuesday, London session
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

    fn result_in(e: &TradeEntry, rule: HardRule, day: &DayContext) -> HardRuleResult {
        let (hard, _) = sample_rules();
        evaluate_hard_rules(e, &hard, day).into_iter().find(|r| r.rule == rule).unwrap()
    }

    fn result(e: &TradeEntry, rule: HardRule, earlier: u32) -> HardRuleResult {
        let c = checklist();
        result_in(e, rule, &DayContext { trades_earlier_today: earlier, ..ok_day(&c) })
    }

    fn eval(e: &TradeEntry) -> Vec<HardRuleResult> {
        let (hard, _) = sample_rules();
        let c = checklist();
        evaluate_hard_rules(e, &hard, &ok_day(&c))
    }

    #[test]
    fn a_clean_trade_passes_every_rule_in_a_fixed_order() {
        let rs = eval(&buy());
        let order: Vec<_> = rs.iter().map(|r| r.rule).collect();
        use HardRule::*;
        assert_eq!(order, vec![StopLoss, RiskPerTrade, MinRewardRisk, Session, TradesPerDay, DailyLossLimit, NoTradeWindow, ChecklistDone]);
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
    fn session_comes_from_the_entry_time() {
        let asian = TradeEntry { at: utc(2026, 1, 13, 3, 0), ..buy() };
        let r = result(&asian, HardRule::Session, 0);
        assert!(!r.pass);
        assert!(r.detail.contains("Asian"), "{}", r.detail);
        let overlap = TradeEntry { at: utc(2026, 1, 13, 14, 0), ..buy() };
        assert!(result(&overlap, HardRule::Session, 0).pass);
        let gap = TradeEntry { at: utc(2026, 1, 13, 7, 30), ..buy() };
        assert!(!result(&gap, HardRule::Session, 0).pass, "no session running");
    }

    #[test]
    fn trades_per_day() {
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
        let ok = eval(&buy());
        assert!(combine(&ok, true));
        assert!(!combine(&ok, false));
        let bad = eval(&TradeEntry { lots: Lots(50), ..buy() });
        assert!(!combine(&bad, true));
    }

    #[test]
    fn dispute_lifecycle() {
        let ok = eval(&buy());
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
        let bad = eval(&TradeEntry { stop: None, ..buy() });
        assert_eq!(Adherence::Judged { adherent: false }.dispute(&bad), Err(DisputeError::HardRuleFailed));
    }

    #[test]
    fn daily_loss_limit_in_r() {
        // 1R = 1% of $10,000 = $100; limit 3R = $300.
        let c = checklist();
        let down = |cents| DayContext { realized_today: Usd(cents), ..ok_day(&c) };
        assert!(result_in(&buy(), HardRule::DailyLossLimit, &down(-29_999)).pass);
        let r = result_in(&buy(), HardRule::DailyLossLimit, &down(-30_000));
        assert!(!r.pass, "reaching the limit stops the day");
        assert!(r.detail.contains("$300.00") && r.detail.contains("3R"), "{}", r.detail);
        assert!(result_in(&buy(), HardRule::DailyLossLimit, &down(50_000)).pass, "a green day never trips it");
    }

    #[test]
    fn no_trade_windows_use_the_clock_and_the_checklist() {
        let rollover = TradeEntry { at: utc(2026, 1, 13, 22, 30), ..buy() };
        let r = result(&rollover, HardRule::NoTradeWindow, 0);
        assert!(!r.pass && r.detail.contains("rollover"), "{}", r.detail);
        let near_cpi = TradeEntry { at: utc(2026, 1, 13, 13, 27), ..buy() };
        let r = result(&near_cpi, HardRule::NoTradeWindow, 0);
        assert!(!r.pass && r.detail.contains("US CPI"), "{}", r.detail);
        let no_list = DayContext { checklist: None, ..DayContext::default() };
        assert!(result_in(&near_cpi, HardRule::NoTradeWindow, &no_list).pass, "events come from the checklist");
        assert!(result(&buy(), HardRule::NoTradeWindow, 0).pass);
    }

    #[test]
    fn trading_without_a_checklist_breaks_the_plan() {
        let none = DayContext::default();
        assert!(!result_in(&buy(), HardRule::ChecklistDone, &none).pass);
        assert!(result(&buy(), HardRule::ChecklistDone, 0).pass);
        let (hard, _) = sample_rules();
        assert!(!combine(&evaluate_hard_rules(&buy(), &hard, &none), true));
    }
}
