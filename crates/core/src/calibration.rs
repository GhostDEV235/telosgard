//! Calibration drill: predict whether the next candle closes up or down on ten seeded,
//! illustrative charts. The candles are a random walk, so nobody can do better than chance; the
//! point is to see that, and to manage risk over many trades instead of predicting one.

use crate::drills::{Candle, CandleDirection, generate};
use crate::money::{DollarMove, Percent, Price};

pub const ROUND_LEN: usize = 10;
/// Candles shown before the hidden one.
pub const SHOWN: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationChart {
    pub shown: Vec<Candle>,
    /// The candle the Learner predicts; never a doji, so every prediction is right or wrong.
    pub next: Candle,
}

impl CalibrationChart {
    pub fn went_up(&self) -> bool {
        self.next.direction() == CandleDirection::Bullish
    }
}

/// Ten charts, the same for the same seed.
pub fn calibration_round(seed: u64) -> Vec<CalibrationChart> {
    let mut out = Vec::with_capacity(ROUND_LEN);
    let mut s = seed.wrapping_mul(1_000_003);
    while out.len() < ROUND_LEN {
        let mut cs = generate(s, SHOWN + 1, Price(410_000), DollarMove(600), DollarMove(350));
        s = s.wrapping_add(1);
        let next = cs.pop().expect("SHOWN + 1 candles");
        if next.direction() != CandleDirection::Doji {
            out.push(CalibrationChart { shown: cs, next });
        }
    }
    out
}

/// Plain hit rate: right ÷ answered, rounded down. Nothing answered is 0%.
pub fn hit_rate(right: u32, answered: u32) -> Percent {
    if answered == 0 { Percent(0) } else { Percent(right * 10_000 / answered) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_are_reproducible_and_complete() {
        let a = calibration_round(7);
        assert_eq!(a, calibration_round(7));
        assert_ne!(a, calibration_round(8));
        assert_eq!(a.len(), ROUND_LEN);
        for c in &a {
            assert_eq!(c.shown.len(), SHOWN);
            assert_ne!(c.next.direction(), CandleDirection::Doji);
            assert_eq!(c.next.open, c.shown.last().unwrap().close, "the hidden candle continues the chart");
        }
    }

    #[test]
    fn hit_rate_is_plain() {
        assert_eq!(hit_rate(5, 10), Percent(5_000));
        assert_eq!(hit_rate(7, 10), Percent(7_000));
        assert_eq!(hit_rate(1, 3), Percent(3_333));
        assert_eq!(hit_rate(0, 0), Percent(0));
    }

    #[test]
    fn simple_rules_do_no_better_than_chance() {
        // Over many rounds, "always up" and "same as the last candle" both land near 50%.
        let (mut up, mut follow, mut n) = (0u32, 0u32, 0u32);
        for seed in 0..400 {
            for c in calibration_round(seed) {
                n += 1;
                up += c.went_up() as u32;
                let last_up = c.shown.last().unwrap().direction() == CandleDirection::Bullish;
                follow += (last_up == c.went_up()) as u32;
            }
        }
        for hits in [up, follow] {
            let rate = hit_rate(hits, n).0;
            assert!((4_700..=5_300).contains(&rate), "{rate} of {n}");
        }
    }
}
