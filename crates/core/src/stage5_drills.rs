//! Stage 5 (Wave 5): Price Action drills. "Find the level" (which zone was tested at least twice),
//! "Name the structure" (up, down or range, then BOS or CHoCH) and "Name the candle" (pin bar,
//! engulfing, inside bar or nothing). Deterministic from a seed; prices are illustrative.

use crate::drills::{Candle, Rng};
use crate::i18n::t;
use crate::money::Price;

const DOLLAR: i64 = 100;
/// Two swing points within $1 of each other count as the same price.
const SAME: i64 = DOLLAR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Swing {
    pub price: Price,
    /// A swing high (a peak) or a swing low (a dip).
    pub high: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structure {
    Up,
    Down,
    Range,
}

impl Structure {
    pub const ALL: [Structure; 3] = [Structure::Up, Structure::Down, Structure::Range];
    pub fn label(self) -> &'static str {
        match self {
            Structure::Up => t("Uptrend: higher highs and higher lows", "Tendance haussière : sommets et creux plus hauts"),
            Structure::Down => t("Downtrend: lower highs and lower lows", "Tendance baissière : sommets et creux plus bas"),
            Structure::Range => t("Range: no clear direction", "Range : pas de direction claire"),
        }
    }
}

/// The structure from the last two swing highs and the last two swing lows.
pub fn structure(swings: &[Swing]) -> Structure {
    let last_two = |high: bool| {
        let v: Vec<i64> = swings.iter().filter(|s| s.high == high).map(|s| s.price.0).collect();
        (v.len() >= 2).then(|| (v[v.len() - 2], v[v.len() - 1]))
    };
    match (last_two(true), last_two(false)) {
        (Some((h1, h2)), Some((l1, l2))) if h2 > h1 + SAME && l2 > l1 + SAME => Structure::Up,
        (Some((h1, h2)), Some((l1, l2))) if h2 < h1 - SAME && l2 < l1 - SAME => Structure::Down,
        _ => Structure::Range,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Break {
    /// Break of structure: price breaks the last swing in the trend's own direction.
    Bos,
    /// Change of character: price breaks the last swing against the trend, the first sign it may turn.
    Choch,
}

impl Break {
    pub const ALL: [Break; 2] = [Break::Bos, Break::Choch];
    pub fn label(self) -> &'static str {
        match self {
            Break::Bos => t("BOS: break of structure, the trend continues", "BOS : cassure de structure, la tendance continue"),
            Break::Choch => t("CHoCH: change of character, the trend may be turning", "CHoCH : changement de caractère, la tendance tourne peut-être"),
        }
    }
}

/// In a trend, breaking the last swing the trend's way is a BOS; breaking the other way is a CHoCH.
/// A range has no trend to continue or change.
pub fn break_kind(s: Structure, broke_up: bool) -> Option<Break> {
    match (s, broke_up) {
        (Structure::Up, true) | (Structure::Down, false) => Some(Break::Bos),
        (Structure::Up, false) | (Structure::Down, true) => Some(Break::Choch),
        (Structure::Range, _) => None,
    }
}

/// A zigzag of `n` swings starting with a low, for a target structure.
fn zigzag(rng: &mut Rng, target: Structure, n: usize) -> Vec<Swing> {
    let base = 4_080 + rng.below(40) as i64;
    let (step, gap) = (4 + rng.below(5) as i64, 10 + rng.below(6) as i64);
    (0..n)
        .map(|i| {
            let high = i % 2 == 1;
            let k = (i / 2) as i64;
            let drift = match target {
                Structure::Up => k * step,
                Structure::Down => -k * step,
                // A range wobbles by less than $1 so neither side makes a clear new extreme.
                Structure::Range => 0,
            };
            let wobble = if target == Structure::Range { (rng.below(3) as i64 - 1) * 30 } else { 0 };
            let dollars = base + drift + if high { gap } else { 0 };
            Swing { price: Price(dollars * DOLLAR + wobble), high }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureAsk {
    Name,
    /// The next move breaks the last swing high (`up`) or the last swing low.
    Break { up: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureQ {
    pub swings: Vec<Swing>,
    pub structure: Structure,
    pub ask: StructureAsk,
}

impl StructureQ {
    pub fn new(seed: u64) -> StructureQ {
        let mut rng = Rng::new(seed);
        let ask_break = rng.below(2) == 0;
        let target = if ask_break { [Structure::Up, Structure::Down][rng.below(2) as usize] } else { Structure::ALL[rng.below(3) as usize] };
        let swings = zigzag(&mut rng, target, 6);
        let ask = if ask_break { StructureAsk::Break { up: rng.below(2) == 0 } } else { StructureAsk::Name };
        StructureQ { structure: structure(&swings), swings, ask }
    }
    pub fn answer_break(&self) -> Option<Break> {
        match self.ask {
            StructureAsk::Break { up } => break_kind(self.structure, up),
            StructureAsk::Name => None,
        }
    }
}

/// "Find the level": four zones, exactly one with at least two reactions at it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelQ {
    pub swings: Vec<Swing>,
    /// Zones as (bottom, top), in display order A–D.
    pub zones: [(Price, Price); 4],
    pub answer: usize,
}

impl LevelQ {
    pub fn new(seed: u64) -> LevelQ {
        let mut rng = Rng::new(seed);
        // Support (lows that bounce) or resistance (highs that turn down).
        let support = rng.below(2) == 0;
        let level = 4_080 + rng.below(40) as i64;
        let side = if support { 1 } else { -1 };
        // Swings: the level is touched twice; the other turns are at distinct prices away from it.
        let away = [9i64, 17, 26, 13];
        let mut swings = Vec::new();
        for (i, a) in away.iter().enumerate() {
            let touch = i == 1 || i == 3;
            let p = if touch { level * DOLLAR + (rng.below(3) as i64 - 1) * 30 } else { (level + side * a) * DOLLAR };
            swings.push(Swing { price: Price(p), high: !support });
            // The swings between sit $40+ beyond the level, past every zone, so the line is a
            // clean zigzag and only the real zone is touched twice.
            swings.push(Swing { price: Price((level + side * (40 + 3 * i as i64)) * DOLLAR), high: support });
        }
        // Zones $3 wide: the real one around the level, three decoys away from any touch.
        let real = (Price((level - 1) * DOLLAR), Price((level + 2) * DOLLAR));
        let decoys = [6i64, 21, 30].map(|d| {
            let c = level + side * d;
            (Price((c - 1) * DOLLAR), Price((c + 2) * DOLLAR))
        });
        let mut zones = [real, decoys[0], decoys[1], decoys[2]];
        for i in (1..4).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            zones.swap(i, j);
        }
        let answer = zones.iter().position(|z| *z == real).unwrap_or(0);
        LevelQ { swings, zones, answer }
    }

    /// How many turning points sit inside a zone.
    pub fn touches(&self, z: (Price, Price)) -> usize {
        self.swings.iter().filter(|s| s.price.0 >= z.0.0 && s.price.0 <= z.1.0).count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    BullishPin,
    BearishPin,
    BullishEngulfing,
    BearishEngulfing,
    InsideBar,
    None,
}

impl Pattern {
    pub const ALL: [Pattern; 6] = [Pattern::BullishPin, Pattern::BearishPin, Pattern::BullishEngulfing, Pattern::BearishEngulfing, Pattern::InsideBar, Pattern::None];
    pub fn label(self) -> &'static str {
        match self {
            Pattern::BullishPin => t("Bullish pin bar", "Pin bar haussière"),
            Pattern::BearishPin => t("Bearish pin bar", "Pin bar baissière"),
            Pattern::BullishEngulfing => t("Bullish engulfing", "Avalement haussier"),
            Pattern::BearishEngulfing => t("Bearish engulfing", "Avalement baissier"),
            Pattern::InsideBar => t("Inside bar", "Inside bar"),
            Pattern::None => t("No pattern", "Pas de figure"),
        }
    }
}

/// The confirmation pattern formed by `last` (with `prev` before it), by this course's definitions:
/// a pin bar's long wick is at least two thirds of its range and the other wick at most a fifth;
/// an engulfing candle's body covers the previous body in the other direction; an inside bar sits
/// within the previous candle's high and low.
pub fn pattern(prev: &Candle, last: &Candle) -> Pattern {
    let range = (last.high.0 - last.low.0).max(1);
    let top = last.open.0.max(last.close.0);
    let bottom = last.open.0.min(last.close.0);
    let upper = last.high.0 - top;
    let lower = bottom - last.low.0;
    if 3 * lower >= 2 * range && 5 * upper <= range {
        return Pattern::BullishPin;
    }
    if 3 * upper >= 2 * range && 5 * lower <= range {
        return Pattern::BearishPin;
    }
    let prev_up = prev.close.0 > prev.open.0;
    let last_up = last.close.0 > last.open.0;
    let (pt, pb) = (prev.open.0.max(prev.close.0), prev.open.0.min(prev.close.0));
    if !prev_up && last_up && top >= pt && bottom <= pb && top - bottom > pt - pb {
        return Pattern::BullishEngulfing;
    }
    if prev_up && !last_up && top >= pt && bottom <= pb && top - bottom > pt - pb {
        return Pattern::BearishEngulfing;
    }
    if last.high.0 < prev.high.0 && last.low.0 > prev.low.0 {
        return Pattern::InsideBar;
    }
    Pattern::None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternQ {
    pub prev: Candle,
    pub last: Candle,
    pub answer: Pattern,
}

impl PatternQ {
    pub fn new(seed: u64) -> PatternQ {
        let mut rng = Rng::new(seed);
        let target = Pattern::ALL[rng.below(6) as usize];
        let o = (4_090 + rng.below(30) as i64) * DOLLAR;
        let d = |x: i64| x * DOLLAR;
        let c = |open: i64, high: i64, low: i64, close: i64| Candle { open: Price(open), high: Price(high), low: Price(low), close: Price(close) };
        let s = 1 + rng.below(3) as i64; // size factor
        let (prev, last) = match target {
            // A bearish candle, then a long lower wick with the body near the top.
            Pattern::BullishPin => (c(o + d(6 * s), o + d(7 * s), o - d(s), o), c(o, o + d(2 * s) + d(1), o - d(8 * s), o + d(2 * s))),
            Pattern::BearishPin => (c(o - d(6 * s), o + d(s), o - d(7 * s), o), c(o, o + d(8 * s), o - d(2 * s) - d(1), o - d(2 * s))),
            // A bearish candle, then a bullish body that covers it.
            Pattern::BullishEngulfing => (c(o + d(4 * s), o + d(5 * s), o - d(s), o), c(o - d(1), o + d(6 * s), o - d(2), o + d(5 * s))),
            Pattern::BearishEngulfing => (c(o - d(4 * s), o + d(s), o - d(5 * s), o), c(o + d(1), o + d(2), o - d(6 * s), o - d(5 * s))),
            // A wide candle, then a small one inside its high and low.
            Pattern::InsideBar => (c(o, o + d(10 * s), o - d(3 * s), o + d(8 * s)), c(o + d(6 * s), o + d(7 * s), o + d(2 * s), o + d(3 * s))),
            // Two ordinary candles in the same direction with wicks on both sides.
            Pattern::None => (c(o, o + d(5 * s), o - d(2 * s), o + d(3 * s)), c(o + d(3 * s), o + d(9 * s), o + d(2 * s), o + d(7 * s))),
        };
        PatternQ { prev, last, answer: target }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sw(prices: &[i64]) -> Vec<Swing> {
        prices.iter().enumerate().map(|(i, p)| Swing { price: Price(p * DOLLAR), high: i % 2 == 1 }).collect()
    }

    #[test]
    fn structure_reads_the_last_two_highs_and_lows() {
        assert_eq!(structure(&sw(&[4085, 4100, 4090, 4108, 4096, 4115])), Structure::Up);
        assert_eq!(structure(&sw(&[4110, 4120, 4100, 4112, 4092, 4104])), Structure::Down);
        assert_eq!(structure(&sw(&[4090, 4110, 4090, 4110, 4091, 4109])), Structure::Range);
        // Higher highs but lower lows: no clear trend.
        assert_eq!(structure(&sw(&[4090, 4110, 4085, 4115, 4080, 4120])), Structure::Range);
    }

    #[test]
    fn breaking_with_the_trend_is_a_bos_and_against_it_a_choch() {
        assert_eq!(break_kind(Structure::Up, true), Some(Break::Bos));
        assert_eq!(break_kind(Structure::Up, false), Some(Break::Choch));
        assert_eq!(break_kind(Structure::Down, false), Some(Break::Bos));
        assert_eq!(break_kind(Structure::Down, true), Some(Break::Choch));
        assert_eq!(break_kind(Structure::Range, true), None);
    }

    #[test]
    fn generated_structures_match_their_target_and_breaks_always_have_an_answer() {
        let mut seen = std::collections::HashSet::new();
        for seed in 0..300 {
            let q = StructureQ::new(seed);
            assert_eq!(q.swings.len(), 6);
            match q.ask {
                StructureAsk::Name => {
                    seen.insert(q.structure as u8);
                }
                StructureAsk::Break { .. } => {
                    assert_ne!(q.structure, Structure::Range, "seed {seed}");
                    assert!(q.answer_break().is_some(), "seed {seed}");
                    seen.insert(10 + q.answer_break().unwrap() as u8);
                }
            }
        }
        assert_eq!(seen.len(), 5, "up, down, range, BOS and CHoCH all appear: {seen:?}");
    }

    #[test]
    fn exactly_one_zone_is_tested_twice() {
        let mut slots = std::collections::HashSet::new();
        for seed in 0..300 {
            let q = LevelQ::new(seed);
            let counts: Vec<usize> = q.zones.iter().map(|z| q.touches(*z)).collect();
            assert!(counts[q.answer] >= 2, "seed {seed}: {counts:?}");
            assert_eq!(counts.iter().filter(|c| **c >= 2).count(), 1, "seed {seed}: {counts:?}");
            // A clean zigzag: every high above both neighbouring lows.
            for w in q.swings.windows(2) {
                let (h, l) = if w[0].high { (w[0], w[1]) } else { (w[1], w[0]) };
                assert!(h.high && !l.high && h.price.0 > l.price.0, "seed {seed}: zigzag");
            }
            slots.insert(q.answer);
        }
        assert_eq!(slots.len(), 4);
    }

    #[test]
    fn every_pattern_is_generated_and_recognised() {
        let mut seen = std::collections::HashSet::new();
        for seed in 0..300 {
            let q = PatternQ::new(seed);
            for c in [q.prev, q.last] {
                assert!(c.low.0 <= c.open.0.min(c.close.0) && c.high.0 >= c.open.0.max(c.close.0), "seed {seed}: valid candle");
            }
            assert_eq!(pattern(&q.prev, &q.last), q.answer, "seed {seed}: the drawing is the pattern it claims to be");
            seen.insert(q.answer as u8);
        }
        assert_eq!(seen.len(), 6, "every pattern appears: {seen:?}");
    }

    #[test]
    fn pin_bar_needs_a_long_wick_and_a_small_other_wick() {
        let c = |o: i64, h: i64, l: i64, cl: i64| Candle { open: Price(o * DOLLAR), high: Price(h * DOLLAR), low: Price(l * DOLLAR), close: Price(cl * DOLLAR) };
        let prev = c(4110, 4111, 4099, 4100);
        assert_eq!(pattern(&prev, &c(4100, 4103, 4088, 4102)), Pattern::BullishPin);
        // Long lower wick but an equally long upper wick: indecision, not a pin bar.
        assert_ne!(pattern(&prev, &c(4100, 4110, 4088, 4101)), Pattern::BullishPin);
    }
}
