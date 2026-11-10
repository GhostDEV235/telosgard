//! Trading Plan (PRD FR-5). Versioned: saving creates a new version; old versions are never changed.

use crate::Millis;
use crate::money::Percent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Session {
    Asian,
    London,
    NewYork,
}

/// Numeric rules, checked by calculation. A stop-loss is always required and is not configurable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardRules {
    pub max_risk: Percent,
    /// Max total loss, as a share of the Readiness Run's starting balance.
    pub max_total_loss: Percent,
    /// Minimum reward-to-risk, ×100 (1.5R = 150).
    pub min_reward_risk: Percent,
    pub sessions: Vec<Session>,
    pub max_trades_per_day: u32,
}

/// Judgement rules, checked by Claude.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftRules {
    pub setup: String,
    pub extra: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPlan {
    pub version: u32,
    pub created_at: Millis,
    pub hard: HardRules,
    pub soft: SoftRules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanError {
    MaxRiskOutOfRange,
    MaxTotalLossOutOfRange,
    MinRewardRiskNotPositive,
    NoSessions,
    NoTradesAllowed,
    SetupMissing,
}

impl HardRules {
    pub fn validate(&self) -> Result<(), PlanError> {
        if self.max_risk.0 == 0 || self.max_risk.0 > 10_000 {
            return Err(PlanError::MaxRiskOutOfRange);
        }
        if self.max_total_loss.0 == 0 || self.max_total_loss.0 > 10_000 {
            return Err(PlanError::MaxTotalLossOutOfRange);
        }
        if self.min_reward_risk.0 == 0 {
            return Err(PlanError::MinRewardRiskNotPositive);
        }
        if self.sessions.is_empty() {
            return Err(PlanError::NoSessions);
        }
        if self.max_trades_per_day == 0 {
            return Err(PlanError::NoTradesAllowed);
        }
        Ok(())
    }
}

/// All versions of the Learner's plan, oldest first. Only `save` adds to it; nothing mutates a version.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanHistory {
    versions: Vec<TradingPlan>,
}

impl PlanHistory {
    pub fn from_versions(mut versions: Vec<TradingPlan>) -> PlanHistory {
        versions.sort_by_key(|p| p.version);
        PlanHistory { versions }
    }
    pub fn save(&mut self, hard: HardRules, soft: SoftRules, at: Millis) -> Result<&TradingPlan, PlanError> {
        hard.validate()?;
        if soft.setup.trim().is_empty() {
            return Err(PlanError::SetupMissing);
        }
        let version = self.current().map_or(1, |p| p.version + 1);
        self.versions.push(TradingPlan { version, created_at: at, hard, soft });
        Ok(self.versions.last().expect("just pushed"))
    }
    pub fn current(&self) -> Option<&TradingPlan> {
        self.versions.last()
    }
    pub fn get(&self, version: u32) -> Option<&TradingPlan> {
        self.versions.iter().find(|p| p.version == version)
    }
    pub fn versions(&self) -> &[TradingPlan] {
        &self.versions
    }
}

#[cfg(test)]
pub(crate) fn sample_rules() -> (HardRules, SoftRules) {
    (
        HardRules {
            max_risk: Percent(100),
            max_total_loss: Percent(1000),
            min_reward_risk: Percent(150),
            sessions: vec![Session::London, Session::NewYork],
            max_trades_per_day: 2,
        },
        SoftRules {
            setup: "H4 trend up or down, price at a key level, confirmation candle in the trend's direction".into(),
            extra: vec!["No trading 30 minutes either side of US CPI".into()],
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_save_is_version_one_and_versions_increment() {
        let mut h = PlanHistory::default();
        let (hard, soft) = sample_rules();
        assert_eq!(h.save(hard.clone(), soft.clone(), 10).unwrap().version, 1);
        assert_eq!(h.save(hard, soft, 20).unwrap().version, 2);
        assert_eq!(h.current().unwrap().version, 2);
        assert_eq!(h.versions().len(), 2);
    }

    #[test]
    fn old_versions_are_untouched_by_a_new_save() {
        let mut h = PlanHistory::default();
        let (hard, soft) = sample_rules();
        h.save(hard.clone(), soft.clone(), 10).unwrap();
        let v1 = h.get(1).unwrap().clone();
        h.save(HardRules { max_risk: Percent(50), ..hard }, soft, 20).unwrap();
        assert_eq!(h.get(1), Some(&v1));
        assert_eq!(h.get(2).unwrap().hard.max_risk, Percent(50));
        assert_eq!(h.get(3), None);
    }

    #[test]
    fn invalid_plans_are_rejected_and_not_saved() {
        let mut h = PlanHistory::default();
        let (hard, soft) = sample_rules();
        let cases = [
            (HardRules { max_risk: Percent(0), ..hard.clone() }, PlanError::MaxRiskOutOfRange),
            (HardRules { max_risk: Percent(10_001), ..hard.clone() }, PlanError::MaxRiskOutOfRange),
            (HardRules { max_total_loss: Percent(0), ..hard.clone() }, PlanError::MaxTotalLossOutOfRange),
            (HardRules { min_reward_risk: Percent(0), ..hard.clone() }, PlanError::MinRewardRiskNotPositive),
            (HardRules { sessions: vec![], ..hard.clone() }, PlanError::NoSessions),
            (HardRules { max_trades_per_day: 0, ..hard.clone() }, PlanError::NoTradesAllowed),
        ];
        for (bad, err) in cases {
            assert_eq!(h.save(bad, soft.clone(), 1).err(), Some(err));
        }
        let blank = SoftRules { setup: "  ".into(), extra: vec![] };
        assert_eq!(h.save(hard, blank, 1).err(), Some(PlanError::SetupMissing));
        assert!(h.current().is_none());
    }

    #[test]
    fn history_loads_sorted_from_stored_docs() {
        let (hard, soft) = sample_rules();
        let mk = |v| TradingPlan { version: v, created_at: v as i64, hard: hard.clone(), soft: soft.clone() };
        let h = PlanHistory::from_versions(vec![mk(2), mk(1)]);
        assert_eq!(h.current().unwrap().version, 2);
        assert_eq!(h.versions()[0].version, 1);
    }
}
