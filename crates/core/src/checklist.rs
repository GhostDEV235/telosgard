//! Pre-Session Checklist (PRD FR-11.1): the dated routine run before trading.

use crate::Millis;
use crate::money::{DollarMove, Percent, Price, Usd};
use crate::sizing::{Sizing, SizingInput, size};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub name: String,
    pub tier: Tier,
    /// Release time, UTC.
    pub at: Millis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Bias {
    Up,
    Down,
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zone {
    pub low: Price,
    pub high: Price,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskCommitment {
    pub per_trade: Percent,
    /// Daily Loss Limit in R, ×100 (3R = 300).
    pub daily_limit: Percent,
    pub planned_stops: Vec<DollarMove>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreSessionChecklist {
    /// The Learner's local calendar day (days since 1970-01-01).
    pub day: i64,
    pub events: Vec<CalendarEvent>,
    pub asian_high: Option<Price>,
    pub asian_low: Option<Price>,
    /// One line each on the dollar, yields and risk mood.
    pub mood: String,
    pub bias: Bias,
    pub zones: Vec<Zone>,
    /// "If price does X at level Y, I do Z."
    pub scenarios: Vec<String>,
    pub risk: RiskCommitment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChecklistError {
    NoScenario,
    NoZone,
    ZoneInverted,
    AsianRangeInverted,
    RiskMissing,
}

impl PreSessionChecklist {
    pub fn doc_id(&self) -> String {
        format!("checklist-{}", crate::time::date_string(self.day))
    }
    pub fn validate(&self) -> Result<(), ChecklistError> {
        if !self.scenarios.iter().any(|s| !s.trim().is_empty()) {
            return Err(ChecklistError::NoScenario);
        }
        if self.zones.is_empty() {
            return Err(ChecklistError::NoZone);
        }
        if self.zones.iter().any(|z| z.low > z.high) {
            return Err(ChecklistError::ZoneInverted);
        }
        if let (Some(h), Some(l)) = (self.asian_high, self.asian_low) {
            if l > h {
                return Err(ChecklistError::AsianRangeInverted);
            }
        }
        if self.risk.per_trade.0 == 0 || self.risk.daily_limit.0 == 0 {
            return Err(ChecklistError::RiskMissing);
        }
        Ok(())
    }
    /// (name, time) of tier-1 events, for the No-Trade Window rule.
    pub fn tier_one(&self) -> Vec<(&str, Millis)> {
        self.events.iter().filter(|e| e.tier == Tier::One).map(|e| (e.name.as_str(), e.at)).collect()
    }
    /// Position size for each planned stop distance, at the committed risk. `None` when inputs are invalid.
    pub fn planned_sizes(&self, balance: Usd, contract_oz: u32) -> Vec<(DollarMove, Option<Sizing>)> {
        self.risk
            .planned_stops
            .iter()
            .map(|&stop| (stop, size(SizingInput { balance, risk: self.risk.per_trade, stop, contract_oz }).ok()))
            .collect()
    }
}

#[cfg(test)]
pub(crate) fn sample(day: i64) -> PreSessionChecklist {
    let p = |s: &str| Price::parse(s).unwrap();
    PreSessionChecklist {
        day,
        events: vec![
            CalendarEvent { name: "US CPI".into(), tier: Tier::One, at: crate::time::utc(2026, 1, 13, 13, 30) },
            CalendarEvent { name: "US jobless claims".into(), tier: Tier::Three, at: crate::time::utc(2026, 1, 13, 13, 30) },
        ],
        asian_high: Some(p("4112")),
        asian_low: Some(p("4096")),
        mood: "Dollar weaker, yields flat, risk on".into(),
        bias: Bias::Up,
        zones: vec![Zone { low: p("4090"), high: p("4095") }, Zone { low: p("4148"), high: p("4152") }],
        scenarios: vec!["If price pulls back to 4090–4095 and prints a bullish engulfing on H1, I buy with the stop below the wick.".into()],
        risk: RiskCommitment { per_trade: Percent(100), daily_limit: Percent(300), planned_stops: vec![DollarMove(500), DollarMove(800)] },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::days_from_civil;

    fn day() -> i64 {
        days_from_civil(2026, 1, 13)
    }

    #[test]
    fn doc_id_uses_the_local_date() {
        assert_eq!(sample(day()).doc_id(), "checklist-2026-01-13");
    }

    #[test]
    fn a_complete_checklist_is_valid() {
        assert_eq!(sample(day()).validate(), Ok(()));
    }

    #[test]
    fn needs_a_scenario_and_a_zone() {
        let mut c = sample(day());
        c.scenarios = vec!["   ".into()];
        assert_eq!(c.validate(), Err(ChecklistError::NoScenario));
        let mut c = sample(day());
        c.zones.clear();
        assert_eq!(c.validate(), Err(ChecklistError::NoZone));
    }

    #[test]
    fn rejects_inverted_ranges_and_missing_risk() {
        let mut c = sample(day());
        c.zones[0] = Zone { low: Price(410_000), high: Price(409_000) };
        assert_eq!(c.validate(), Err(ChecklistError::ZoneInverted));
        let mut c = sample(day());
        c.asian_low = Some(Price(420_000));
        assert_eq!(c.validate(), Err(ChecklistError::AsianRangeInverted));
        let mut c = sample(day());
        c.risk.per_trade = Percent(0);
        assert_eq!(c.validate(), Err(ChecklistError::RiskMissing));
        let mut c = sample(day());
        c.risk.daily_limit = Percent(0);
        assert_eq!(c.validate(), Err(ChecklistError::RiskMissing));
    }

    #[test]
    fn only_tier_one_events_feed_no_trade_windows() {
        let c = sample(day());
        assert_eq!(c.tier_one(), vec![("US CPI", crate::time::utc(2026, 1, 13, 13, 30))]);
    }

    #[test]
    fn sizes_are_precomputed_for_planned_stops() {
        let c = sample(day());
        let sizes = c.planned_sizes(Usd(1_000_000), 100);
        assert_eq!(sizes.len(), 2);
        // $10,000 · 1% · $5 → 0.20 lots; $8 → 0.12 lots
        assert!(matches!(sizes[0].1, Some(Sizing::Ok { lots, .. }) if lots.0 == 20));
        assert!(matches!(sizes[1].1, Some(Sizing::Ok { lots, .. }) if lots.0 == 12));
    }
}
