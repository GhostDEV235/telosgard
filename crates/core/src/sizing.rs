//! Position-size calculator (PRD FR-4).
//! `lots = floor_0.01( balance × risk% / (stop_distance × contract_size) )`

use crate::money::{DollarMove, Lots, Percent, Usd};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SizingInput {
    pub balance: Usd,
    pub risk: Percent,
    pub stop: DollarMove,
    pub contract_oz: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sizing {
    /// A tradeable size. `risk` is the actual money lost if the stop is hit, `risk_pct` its share of the balance.
    Ok { lots: Lots, risk: Usd, risk_pct: Percent },
    /// Even the minimum 0.01 lot would risk more than the plan allows.
    TooSmall { min_lot_risk: Usd, min_lot_risk_pct: Percent },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizingError {
    BalanceNotPositive,
    RiskOutOfRange,
    StopNotPositive,
    ContractNotPositive,
}

pub fn size(i: SizingInput) -> Result<Sizing, SizingError> {
    if i.balance.0 <= 0 {
        return Err(SizingError::BalanceNotPositive);
    }
    if i.risk.0 == 0 || i.risk.0 > 10_000 {
        return Err(SizingError::RiskOutOfRange);
    }
    if i.stop.0 <= 0 {
        return Err(SizingError::StopNotPositive);
    }
    if i.contract_oz == 0 {
        return Err(SizingError::ContractNotPositive);
    }
    let budget = i.risk.apply(i.balance);
    let loss_per_lot = i.stop.value(Lots(100), i.contract_oz);
    // Integer division rounds down: never risk more than the budget.
    let lots = Lots(budget.0 * 100 / loss_per_lot.0);
    if lots.0 == 0 {
        let min_lot_risk = i.stop.value(Lots(1), i.contract_oz);
        return Ok(Sizing::TooSmall { min_lot_risk, min_lot_risk_pct: Percent::of(min_lot_risk, i.balance) });
    }
    let risk = i.stop.value(lots, i.contract_oz);
    Ok(Sizing::Ok { lots, risk, risk_pct: Percent::of(risk, i.balance) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money::DEFAULT_CONTRACT_OZ;

    fn input(balance: &str, risk: &str, stop: &str) -> SizingInput {
        SizingInput {
            balance: Usd::parse(balance).unwrap(),
            risk: Percent::parse(risk).unwrap(),
            stop: DollarMove::parse(stop).unwrap(),
            contract_oz: DEFAULT_CONTRACT_OZ,
        }
    }

    #[test]
    fn ten_k_one_percent_five_dollar_stop_is_point_two_lots() {
        assert_eq!(
            size(input("10000", "1", "5")),
            Ok(Sizing::Ok { lots: Lots(20), risk: Usd(10_000), risk_pct: Percent(100) })
        );
    }

    #[test]
    fn rounds_down_never_up() {
        // $10,000 × 1% = $100; $7 stop → $700 per lot → 0.1428… → 0.14 lots, risking $98.
        assert_eq!(
            size(input("10000", "1", "7")),
            Ok(Sizing::Ok { lots: Lots(14), risk: Usd(9_800), risk_pct: Percent(98) })
        );
    }

    #[test]
    fn small_account_wide_stop_is_too_small_to_trade_safely() {
        // $500 × 1% = $5, but 0.01 lot with an $8 stop already loses $8 (1.6%).
        assert_eq!(
            size(input("500", "1", "8")),
            Ok(Sizing::TooSmall { min_lot_risk: Usd(800), min_lot_risk_pct: Percent(160) })
        );
    }

    #[test]
    fn contract_size_is_a_parameter() {
        // A 10 oz "mini" contract: $5 stop × 10 oz = $50 per lot → $100 / $50 = 2.00 lots.
        let i = SizingInput { contract_oz: 10, ..input("10000", "1", "5") };
        assert_eq!(size(i), Ok(Sizing::Ok { lots: Lots(200), risk: Usd(10_000), risk_pct: Percent(100) }));
    }

    #[test]
    fn rejects_nonsense_inputs() {
        assert_eq!(size(input("0", "1", "5")), Err(SizingError::BalanceNotPositive));
        assert_eq!(size(input("1000", "0", "5")), Err(SizingError::RiskOutOfRange));
        assert_eq!(size(input("1000", "101", "5")), Err(SizingError::RiskOutOfRange));
        assert_eq!(size(input("1000", "1", "0")), Err(SizingError::StopNotPositive));
        assert_eq!(size(input("1000", "1", "-2")), Err(SizingError::StopNotPositive));
        let i = SizingInput { contract_oz: 0, ..input("1000", "1", "5") };
        assert_eq!(size(i), Err(SizingError::ContractNotPositive));
    }
}
