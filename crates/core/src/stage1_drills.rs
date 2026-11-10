//! Stage 1 Chart Drill questions (FR-8.4): bid/ask, dollar distance, pip conversion, order type.
//! Deterministic from a seed; illustrative prices only.

use crate::drills::Rng;
use crate::money::{DollarMove, Price};
use serde::{Deserialize, Serialize};

/// A price near 4,100 with cents, for illustrative questions.
fn price(rng: &mut Rng) -> Price {
    Price(400_000 + rng.below(30_000) as i64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BidAskAsk {
    Spread,
    BuyPrice,
    SellPrice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BidAskQ {
    pub bid: Price,
    pub ask: Price,
    pub asked: BidAskAsk,
}

impl BidAskQ {
    pub fn new(seed: u64) -> BidAskQ {
        let mut rng = Rng::new(seed);
        let bid = price(&mut rng);
        let ask = Price(bid.0 + 1 + rng.below(100) as i64);
        let asked = [BidAskAsk::Spread, BidAskAsk::BuyPrice, BidAskAsk::SellPrice][rng.below(3) as usize];
        BidAskQ { bid, ask, asked }
    }
    /// The correct answer as a price (for buy/sell) or a distance in cents (for spread).
    pub fn answer_cents(&self) -> i64 {
        match self.asked {
            BidAskAsk::Spread => (self.ask - self.bid).0,
            BidAskAsk::BuyPrice => self.ask.0,
            BidAskAsk::SellPrice => self.bid.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DistanceQ {
    pub from: Price,
    pub to: Price,
}

impl DistanceQ {
    pub fn new(seed: u64) -> DistanceQ {
        let mut rng = Rng::new(seed);
        let from = Price((price(&mut rng).0 / 25) * 25);
        let quarters = 1 + rng.below(80) as i64; // $0.25 … $20.00
        let up = rng.below(2) == 0;
        let to = Price(if up { from.0 + quarters * 25 } else { from.0 - quarters * 25 });
        DistanceQ { from, to }
    }
    pub fn answer(&self) -> DollarMove {
        (self.to - self.from).abs()
    }
}

/// "Someone says N pips; they mean $X per pip. What Dollar Move is that?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipQ {
    pub pips: i64,
    /// The speaker's pip size, in cents: 1, 10 or 100.
    pub pip_cents: i64,
}

impl PipQ {
    pub fn new(seed: u64) -> PipQ {
        let mut rng = Rng::new(seed);
        let pips = [5, 10, 20, 25, 30, 50, 80, 100][rng.below(8) as usize];
        let pip_cents = [1, 10, 100][rng.below(3) as usize];
        PipQ { pips, pip_cents }
    }
    pub fn answer(&self) -> DollarMove {
        DollarMove(self.pips * self.pip_cents)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderKind {
    Market,
    BuyLimit,
    SellLimit,
    BuyStop,
    SellStop,
}

/// "Gold is at P. You want to buy/sell now / only if it reaches T."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderQ {
    pub current: Price,
    pub buy: bool,
    /// `None` = act now.
    pub target: Option<Price>,
}

impl OrderQ {
    pub fn new(seed: u64) -> OrderQ {
        let mut rng = Rng::new(seed);
        let current = Price((price(&mut rng).0 / 100) * 100);
        let buy = rng.below(2) == 0;
        let target = match rng.below(5) {
            0 => None,
            k => {
                let gap = 500 * (1 + rng.below(6) as i64); // $5 … $30 away
                Some(Price(if k % 2 == 0 { current.0 + gap } else { current.0 - gap }))
            }
        };
        OrderQ { current, buy, target }
    }
    pub fn answer(&self) -> OrderKind {
        match (self.buy, self.target) {
            (_, None) => OrderKind::Market,
            (true, Some(t)) if t < self.current => OrderKind::BuyLimit,
            (true, Some(_)) => OrderKind::BuyStop,
            (false, Some(t)) if t > self.current => OrderKind::SellLimit,
            (false, Some(_)) => OrderKind::SellStop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generators_are_deterministic() {
        for s in 0..50 {
            assert_eq!(BidAskQ::new(s), BidAskQ::new(s));
            assert_eq!(DistanceQ::new(s), DistanceQ::new(s));
            assert_eq!(PipQ::new(s), PipQ::new(s));
            assert_eq!(OrderQ::new(s), OrderQ::new(s));
        }
    }

    #[test]
    fn bid_is_below_ask_and_answers_follow_the_rules() {
        for s in 0..200 {
            let q = BidAskQ::new(s);
            assert!(q.bid < q.ask && (q.ask - q.bid).0 <= 100, "spread between 1¢ and $1");
            let expect = match q.asked {
                BidAskAsk::Spread => (q.ask - q.bid).0,
                BidAskAsk::BuyPrice => q.ask.0,
                BidAskAsk::SellPrice => q.bid.0,
            };
            assert_eq!(q.answer_cents(), expect);
        }
    }

    #[test]
    fn distances_are_whole_quarters_and_nonzero() {
        for s in 0..200 {
            let q = DistanceQ::new(s);
            assert!(q.answer().0 > 0 && q.answer().0 % 25 == 0);
        }
    }

    #[test]
    fn pip_sizes_cover_all_three_conventions() {
        let sizes: std::collections::BTreeSet<i64> = (0..200).map(|s| PipQ::new(s).pip_cents).collect();
        assert_eq!(sizes.into_iter().collect::<Vec<_>>(), vec![1, 10, 100]);
        assert_eq!(PipQ { pips: 50, pip_cents: 10 }.answer(), DollarMove(500));
    }

    #[test]
    fn order_types_by_intent() {
        let p = |s: &str| Price::parse(s).unwrap();
        let q = |buy, target: Option<&str>| OrderQ { current: p("4100"), buy, target: target.map(p) };
        assert_eq!(q(true, None).answer(), OrderKind::Market);
        assert_eq!(q(true, Some("4080")).answer(), OrderKind::BuyLimit);
        assert_eq!(q(true, Some("4120")).answer(), OrderKind::BuyStop);
        assert_eq!(q(false, Some("4120")).answer(), OrderKind::SellLimit);
        assert_eq!(q(false, Some("4080")).answer(), OrderKind::SellStop);
        let kinds: std::collections::BTreeSet<_> = (0..300).map(|s| format!("{:?}", OrderQ::new(s).answer())).collect();
        assert_eq!(kinds.len(), 5, "the generator covers every order kind");
    }
}
