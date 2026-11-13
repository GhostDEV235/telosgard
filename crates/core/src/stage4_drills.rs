//! Stage 4 (Wave 4): candle reconstruction. Given a candle's open, high, low and close, pick the
//! drawing that matches and say who won it. Also how smaller candles add up to a bigger one (the
//! timeframe lesson's widget). Deterministic from a seed; prices are illustrative, never real quotes.

use crate::drills::{Candle, Rng};
use crate::i18n::t;
use crate::money::Price;

/// Who won a candle: buyers if it closed above its open, sellers if below, nobody if (almost) level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Buyers,
    Sellers,
    Draw,
}

impl Winner {
    pub const ALL: [Winner; 3] = [Winner::Buyers, Winner::Sellers, Winner::Draw];

    pub fn of(c: &Candle) -> Winner {
        // Same $0.10 tolerance as `Candle::direction` (a doji).
        match (c.close - c.open).0 {
            d if d.abs() <= 10 => Winner::Draw,
            d if d > 0 => Winner::Buyers,
            _ => Winner::Sellers,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Winner::Buyers => t("Buyers: it closed above its open", "Les acheteurs : clôture au-dessus de l’ouverture"),
            Winner::Sellers => t("Sellers: it closed below its open", "Les vendeurs : clôture sous l’ouverture"),
            Winner::Draw => t("Neither: it closed where it opened", "Personne : clôture au niveau de l’ouverture"),
        }
    }
}

/// The wick above the body and the wick below it, in cents.
pub fn wicks(c: &Candle) -> (i64, i64) {
    (c.high.0 - c.open.0.max(c.close.0), c.open.0.min(c.close.0) - c.low.0)
}

/// One bigger candle from consecutive smaller ones: the first open, the highest high, the lowest
/// low and the last close. Four 15-minute candles make one 1-hour candle.
pub fn merge(candles: &[Candle]) -> Option<Candle> {
    let first = candles.first()?;
    let last = candles.last()?;
    Some(Candle {
        open: first.open,
        high: Price(candles.iter().map(|c| c.high.0).max()?),
        low: Price(candles.iter().map(|c| c.low.0).min()?),
        close: last.close,
    })
}

/// "Build the candle": four drawings of which exactly one matches the numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandleQ {
    pub candle: Candle,
    /// Four candidates in display order (A–D); `options[answer] == candle`.
    pub options: [Candle; 4],
    pub answer: usize,
    pub winner: Winner,
}

const DOLLAR: i64 = 100;

impl CandleQ {
    pub fn new(seed: u64) -> CandleQ {
        let mut rng = Rng::new(seed);
        let open = Price((4_060 + rng.below(80) as i64) * DOLLAR);
        // Bullish, bearish or a draw (weights 2:2:1); bodies $3–$12; wicks $1–$9 and never equal,
        // so mirroring them always gives a different drawing.
        let kind = rng.below(5);
        let body = (3 + rng.below(10) as i64) * DOLLAR;
        let close = match kind {
            0 | 1 => Price(open.0 + body),
            2 | 3 => Price(open.0 - body),
            _ => open,
        };
        let up = (1 + rng.below(9) as i64) * DOLLAR;
        let mut down = (1 + rng.below(9) as i64) * DOLLAR;
        if down == up {
            down = if up >= 5 * DOLLAR { up - 3 * DOLLAR } else { up + 3 * DOLLAR };
        }
        let top = open.0.max(close.0);
        let bottom = open.0.min(close.0);
        let candle = Candle { open, high: Price(top + up), low: Price(bottom - down), close };

        // Distractors, each one classic misreading:
        // 1. open and close swapped (the colour backwards); for a draw, a clear body instead;
        let flipped = if close == open {
            Candle { close: Price(open.0 + 6 * DOLLAR), high: Price(open.0 + 6 * DOLLAR + up), ..candle }
        } else {
            Candle { open: close, close: open, ..candle }
        };
        // 2. the wicks mirrored (high and low read from the wrong end);
        let mirrored = Candle { high: Price(top + down), low: Price(bottom - up), ..candle };
        // 3. the body stretched to the high and low (wicks mistaken for body).
        let stretched = if close.0 >= open.0 {
            Candle { open: candle.low, close: candle.high, ..candle }
        } else {
            Candle { open: candle.high, close: candle.low, ..candle }
        };
        let mut options = [candle, flipped, mirrored, stretched];
        // Shuffle (Fisher–Yates) so the answer isn't always first.
        for i in (1..4).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            options.swap(i, j);
        }
        let answer = options.iter().position(|c| *c == candle).unwrap_or(0);
        CandleQ { candle, options, answer, winner: Winner::of(&candle) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(d: i64) -> Price {
        Price(d * DOLLAR)
    }

    #[test]
    fn winner_follows_open_and_close_not_the_wicks() {
        let c = |o, h, l, cl| Candle { open: p(o), high: p(h), low: p(l), close: p(cl) };
        assert_eq!(Winner::of(&c(4100, 4115, 4090, 4108)), Winner::Buyers);
        assert_eq!(Winner::of(&c(4100, 4130, 4095, 4097)), Winner::Sellers, "a long upper wick doesn't make it a buyers' candle");
        assert_eq!(Winner::of(&Candle { close: Price(410_005), ..c(4100, 4106, 4094, 4100) }), Winner::Draw);
        assert_eq!(wicks(&c(4100, 4115, 4090, 4108)), (700, 1000));
    }

    #[test]
    fn four_fifteen_minute_candles_make_one_hour() {
        let c = |o, h, l, cl| Candle { open: p(o), high: p(h), low: p(l), close: p(cl) };
        let quarter = [c(4100, 4104, 4097, 4103), c(4103, 4112, 4101, 4110), c(4110, 4111, 4099, 4101), c(4101, 4106, 4100, 4105)];
        assert_eq!(merge(&quarter), Some(c(4100, 4112, 4097, 4105)));
        assert_eq!(merge(&[]), None);
    }

    #[test]
    fn every_question_has_one_true_drawing_and_three_different_wrong_ones() {
        let mut winners = std::collections::HashSet::new();
        let mut slots = std::collections::HashSet::new();
        for seed in 0..400 {
            let q = CandleQ::new(seed);
            let c = q.candle;
            assert!(c.low.0 < c.open.0.min(c.close.0) && c.high.0 > c.open.0.max(c.close.0), "seed {seed}: real wicks");
            assert_eq!(q.options[q.answer], c, "seed {seed}");
            assert_eq!(q.options.iter().filter(|o| **o == c).count(), 1, "seed {seed}: exactly one match");
            for (i, a) in q.options.iter().enumerate() {
                for b in &q.options[i + 1..] {
                    assert_ne!(a, b, "seed {seed}: options differ");
                }
                assert!(a.low.0 <= a.open.0.min(a.close.0) && a.high.0 >= a.open.0.max(a.close.0), "seed {seed}: every drawing is a valid candle");
            }
            assert_eq!(q.winner, Winner::of(&c));
            winners.insert(q.winner as u8);
            slots.insert(q.answer);
        }
        assert_eq!(winners.len(), 3, "buyers, sellers and draws all appear");
        assert_eq!(slots.len(), 4, "the answer lands in every position");
    }
}
