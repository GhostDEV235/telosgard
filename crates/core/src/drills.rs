//! Chart Drills (PRD FR-8): deterministic, seeded, illustrative candles — never real market data.

use crate::money::{DollarMove, Price};
use serde::{Deserialize, Serialize};

pub const ILLUSTRATIVE_LABEL: &str = "Illustrative, not real market data.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candle {
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandleDirection {
    Bullish,
    Bearish,
    /// Open and close within $0.10 of each other.
    Doji,
}

/// SplitMix64: small, fast, and identical on native and wasm32.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// Uniform in `0..n` (modulo bias is negligible for chart-sized `n`). `n` must be > 0.
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

/// `n` candles starting at `start`, each moving up to `max_body` and wicking up to `max_wick`.
/// Each candle opens where the previous one closed.
pub fn generate(seed: u64, n: usize, start: Price, max_body: DollarMove, max_wick: DollarMove) -> Vec<Candle> {
    let mut rng = Rng::new(seed);
    let body = max_body.0.max(0) as u64;
    let wick = max_wick.0.max(0) as u64;
    let mut open = start;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let delta = rng.below(2 * body + 1) as i64 - body as i64;
        // Never let an illustrative price go to or below $1.
        let close = Price((open.0 + delta).max(100));
        let high = Price(open.0.max(close.0) + rng.below(wick + 1) as i64);
        let low = Price((open.0.min(close.0) - rng.below(wick + 1) as i64).max(1));
        out.push(Candle { open, high, low, close });
        open = close;
    }
    out
}

impl Candle {
    pub fn direction(&self) -> CandleDirection {
        let body = self.close - self.open;
        match body.0 {
            b if b.abs() <= 10 => CandleDirection::Doji,
            b if b > 0 => CandleDirection::Bullish,
            _ => CandleDirection::Bearish,
        }
    }
    pub fn range(&self) -> DollarMove {
        self.high - self.low
    }
}

/// You buy at the ask and sell at the bid; the spread is the gap you pay to get in.
pub fn spread(bid: Price, ask: Price) -> DollarMove {
    (ask - bid).abs()
}

pub fn check_direction(c: &Candle, answer: CandleDirection) -> bool {
    c.direction() == answer
}

/// The distance between two prices, accepted within `tolerance` either way.
pub fn check_distance(a: Price, b: Price, answer: DollarMove, tolerance: DollarMove) -> bool {
    ((a - b).abs().0 - answer.0.abs()).abs() <= tolerance.0.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> Price {
        Price::parse(s).unwrap()
    }
    fn chart(seed: u64) -> Vec<Candle> {
        generate(seed, 60, p("2350"), DollarMove(600), DollarMove(300))
    }

    #[test]
    fn same_seed_same_candles_different_seed_different_candles() {
        assert_eq!(chart(42), chart(42));
        assert_ne!(chart(42), chart(43));
        assert_eq!(chart(42).len(), 60);
    }

    #[test]
    fn rng_is_stable_across_platforms() {
        // Pinned SplitMix64 output so a dependency or platform change can't silently reshuffle drills.
        let mut r = Rng::new(0);
        assert_eq!(r.next_u64(), 0xE220A8397B1DCDAF);
        assert_eq!(r.next_u64(), 0x6E789E6AA1B965F4);
        let mut r = Rng::new(7);
        assert!((0..1000).all(|_| r.below(6) < 6));
    }

    #[test]
    fn candles_are_well_formed_and_chained() {
        let cs = chart(7);
        assert_eq!(cs[0].open, p("2350"));
        for w in cs.windows(2) {
            assert_eq!(w[1].open, w[0].close);
        }
        for c in &cs {
            assert!(c.high >= c.open.max(c.close) && c.low <= c.open.min(c.close), "{c:?}");
            assert!((c.close - c.open).abs() <= DollarMove(600));
            assert!(c.low.0 > 0);
        }
    }

    #[test]
    fn generated_charts_mix_directions() {
        let cs = chart(1);
        assert!(cs.iter().any(|c| c.direction() == CandleDirection::Bullish));
        assert!(cs.iter().any(|c| c.direction() == CandleDirection::Bearish));
    }

    #[test]
    fn candle_direction() {
        let c = |o, cl| Candle { open: p(o), high: p("2400"), low: p("2300"), close: p(cl) };
        assert_eq!(c("2350", "2355").direction(), CandleDirection::Bullish);
        assert_eq!(c("2355", "2350").direction(), CandleDirection::Bearish);
        assert_eq!(c("2350", "2350.10").direction(), CandleDirection::Doji);
        assert_eq!(c("2350", "2350.11").direction(), CandleDirection::Bullish);
        assert!(check_direction(&c("2350", "2340"), CandleDirection::Bearish));
        assert_eq!(c("2350", "2355").range(), DollarMove(10_000));
    }

    #[test]
    fn spread_and_distance() {
        assert_eq!(spread(p("2350.00"), p("2350.25")), DollarMove(25));
        assert!(check_distance(p("2350"), p("2357.5"), DollarMove(750), DollarMove(0)));
        assert!(check_distance(p("2357.5"), p("2350"), DollarMove(750), DollarMove(0)), "order doesn't matter");
        assert!(check_distance(p("2350"), p("2357.5"), DollarMove(800), DollarMove(50)));
        assert!(!check_distance(p("2350"), p("2357.5"), DollarMove(801), DollarMove(50)));
    }
}
