//! Stage 7 (Wave 7): risk drills. "Place the stop" (beyond structure by about 1–1.5× ATR, and
//! clear of an obvious round number) and "Size it" (lots from balance, risk and stop, rounded
//! down; the screen times each answer against a 30-second target). Deterministic from a seed;
//! prices are illustrative.

use crate::drills::Rng;
use crate::money::{DollarMove, Lots, Percent, Price, Usd, DEFAULT_CONTRACT_OZ};
use crate::sizing::{size, Sizing, SizingInput};

const DOLLAR: i64 = 100;

/// Why a stop option is right or wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopKind {
    /// Beyond the structure by about 1–1.5× ATR, and a few dollars clear of the round number.
    Right,
    /// Just past the wick: well inside one ATR, so ordinary noise takes it out.
    InsideNoise,
    /// On the round number, or a few cents past it, where everyone else's stops sit.
    RoundNumber,
    /// Three ATRs away: safe from noise, but the size shrinks and the target falls under 2:1.
    TooFar,
}

/// "Place the stop": a pullback trade with its structure point (the wick the stop must clear),
/// the ATR, and a round number sitting just beyond the structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StopQ {
    pub long: bool,
    pub entry: Price,
    /// The confirmation candle's low (long) or high (short): the structure.
    pub structure: Price,
    pub atr: DollarMove,
    pub round: Price,
    pub options: Vec<(Price, StopKind)>,
    pub answer: usize,
}

impl StopQ {
    pub fn new(seed: u64) -> StopQ {
        let mut rng = Rng::new(seed);
        let long = rng.below(2) == 0;
        let a = 4 + rng.below(5) as i64; // ATR $4 to $8
        let round = 4_000 + 50 * (1 + rng.below(4) as i64); // 4,050 to 4,200
        // The structure sits ATR − 1 dollars inside the round number, so a stop one ATR beyond the
        // structure would land right on it; the right stop goes a few dollars further.
        let clear = if a <= 5 { 2 } else { 3 };
        let dir = if long { 1 } else { -1 };
        let structure = round + dir * (a - 1);
        let entry = structure + dir * (a + 1 + rng.below(3) as i64);
        let at = |d: i64| Price(d * DOLLAR);
        let right = at(round - dir * clear);
        let noise = Price(structure * DOLLAR - dir * 50);
        let on_round = if rng.below(2) == 0 { at(round) } else { Price(round * DOLLAR - dir * 20) };
        let far = at(structure - dir * 3 * a);
        let mut options = vec![(right, StopKind::Right), (noise, StopKind::InsideNoise), (on_round, StopKind::RoundNumber), (far, StopKind::TooFar)];
        for i in (1..options.len()).rev() {
            options.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let answer = options.iter().position(|o| o.1 == StopKind::Right).unwrap_or(0);
        StopQ { long, entry: at(entry), structure: at(structure), atr: DollarMove(a * DOLLAR), round: at(round), options, answer }
    }
    /// How far a stop sits beyond the structure, in hundredths of an ATR (125 = 1.25× ATR).
    pub fn atrs_beyond(&self, stop: Price) -> i64 {
        (self.structure.0 - stop.0).abs() * 100 / self.atr.0
    }
}

/// "Size it": balance, risk and stop distance in; lots out, rounded down to 0.01.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizeQ {
    pub balance: Usd,
    pub risk: Percent,
    pub stop: DollarMove,
    /// The money the plan allows to lose: balance × risk.
    pub budget: Usd,
    /// What 1.00 lot loses if the stop is hit: stop × 100 oz.
    pub per_lot: Usd,
    pub lots: Lots,
    /// The money actually at risk at the rounded-down size.
    pub at_risk: Usd,
    pub options: Vec<Lots>,
    pub answer: usize,
}

impl SizeQ {
    pub const BALANCES: [i64; 6] = [2_000, 5_000, 8_000, 10_000, 20_000, 25_000];

    pub fn new(seed: u64) -> SizeQ {
        let mut rng = Rng::new(seed);
        loop {
            let balance = Usd(Self::BALANCES[rng.below(Self::BALANCES.len() as u64) as usize] * DOLLAR);
            let risk = if rng.below(4) == 0 { Percent(50) } else { Percent(100) };
            // Stops from $3 to $15, sometimes with 50 cents.
            let stop = DollarMove((3 + rng.below(13) as i64) * DOLLAR + if rng.below(3) == 0 { 50 } else { 0 });
            let Ok(Sizing::Ok { lots, risk: at_risk, .. }) = size(SizingInput { balance, risk, stop, contract_oz: DEFAULT_CONTRACT_OZ }) else { continue };
            let budget = risk.apply(balance);
            let per_lot = stop.value(Lots(100), DEFAULT_CONTRACT_OZ);
            // The classic slips: rounding up, a decimal in the wrong place either way, double risk.
            let exact = budget.0 * 100 % per_lot.0 == 0;
            let mut wrong: Vec<Lots> = Vec::new();
            for w in [(!exact).then_some(lots.0 + 1), Some(lots.0 * 10), (lots.0 >= 10).then_some(lots.0 / 10), Some(lots.0 * 2), Some(lots.0 * 3)].into_iter().flatten() {
                if w > 0 && w != lots.0 && !wrong.iter().any(|l| l.0 == w) {
                    wrong.push(Lots(w));
                }
            }
            wrong.truncate(3);
            let mut options = vec![lots];
            options.extend(wrong);
            for i in (1..options.len()).rev() {
                options.swap(i, rng.below(i as u64 + 1) as usize);
            }
            let answer = options.iter().position(|l| *l == lots).unwrap_or(0);
            return SizeQ { balance, risk, stop, budget, per_lot, lots, at_risk, options, answer };
        }
    }
}

/// The 30-second target for "Size it".
pub const SIZE_TARGET_SECS: u32 = 30;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_right_stop_is_one_to_one_and_a_half_atrs_beyond_structure_and_off_the_round_number() {
        for seed in 0..500 {
            let q = StopQ::new(seed);
            let (stop, kind) = q.options[q.answer];
            assert_eq!(kind, StopKind::Right);
            let k = q.atrs_beyond(stop);
            assert!((100..=150).contains(&k), "seed {seed}: {k}");
            // Beyond the structure, on the losing side of the entry.
            assert!(if q.long { stop < q.structure && q.structure < q.entry } else { stop > q.structure && q.structure > q.entry }, "seed {seed}");
            // At least $2 clear of the round number, and past it.
            assert!((stop.0 - q.round.0).abs() >= 2 * DOLLAR, "seed {seed}");
            assert!(if q.long { stop < q.round } else { stop > q.round }, "seed {seed}");
            // The wrong answers really are wrong.
            for (p, kind) in &q.options {
                match kind {
                    StopKind::InsideNoise => assert!(q.atrs_beyond(*p) < 100),
                    StopKind::RoundNumber => assert!((p.0 - q.round.0).abs() <= 20),
                    StopKind::TooFar => assert!(q.atrs_beyond(*p) >= 300),
                    StopKind::Right => {}
                }
            }
            let mut o: Vec<i64> = q.options.iter().map(|o| o.0 .0).collect();
            o.sort();
            o.dedup();
            assert_eq!(o.len(), 4, "seed {seed}: four distinct prices");
        }
    }

    #[test]
    fn the_size_rounds_down_never_risks_more_than_planned_and_the_options_differ() {
        for seed in 0..500 {
            let q = SizeQ::new(seed);
            assert_eq!(q.options[q.answer], q.lots);
            assert!(q.lots.0 >= 1);
            assert!(q.at_risk <= q.budget, "seed {seed}");
            // One more step up would break the plan.
            assert!(q.per_lot.0 * (q.lots.0 + 1) / 100 > q.budget.0, "seed {seed}");
            assert!(q.options.len() >= 3, "seed {seed}");
            let mut o: Vec<i64> = q.options.iter().map(|l| l.0).collect();
            o.sort();
            o.dedup();
            assert_eq!(o.len(), q.options.len(), "seed {seed}");
        }
    }

    #[test]
    fn a_worked_example() {
        // $10,000 at 1% is $100; a $7 stop loses $700 per lot; 100/700 = 0.142… → 0.14 lots, $98.
        let s = size(SizingInput { balance: Usd(1_000_000), risk: Percent(100), stop: DollarMove(700), contract_oz: 100 });
        assert_eq!(s, Ok(Sizing::Ok { lots: Lots(14), risk: Usd(9_800), risk_pct: Percent(98) }));
    }
}
