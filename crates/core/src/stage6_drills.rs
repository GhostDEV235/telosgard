//! Stage 6 (Wave 6): Fibonacci drills. "Pick the leg" (which two swing points the grid is drawn
//! between, in which order), "Place the level" (a retracement or extension price from a leg) and
//! "Find the confluence" (the zone where a Fibonacci level meets the broken swing). Deterministic
//! from a seed; prices are illustrative.

use crate::drills::Rng;
use crate::i18n::t;
use crate::money::Price;
use crate::stage5_drills::Swing;

const DOLLAR: i64 = 100;

/// A Fibonacci level, in basis points of the leg (6180 = 61.8%).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level(pub i64);

impl Level {
    pub const R382: Level = Level(3820);
    pub const R50: Level = Level(5000);
    pub const R618: Level = Level(6180);
    pub const R786: Level = Level(7860);
    pub const X1272: Level = Level(12720);
    pub const X1618: Level = Level(16180);
    pub fn is_extension(self) -> bool {
        self.0 > 10000
    }
    pub fn label(self) -> String {
        let (whole, frac) = (self.0 / 100, self.0 % 100 / 10);
        if frac == 0 { format!("{whole}%") } else { format!("{whole}.{frac}%") }
    }
}

/// A leg from `start` to `end`: low to high in an uptrend, high to low in a downtrend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Leg {
    pub start: Price,
    pub end: Price,
}

impl Leg {
    pub fn up(&self) -> bool {
        self.end.0 > self.start.0
    }
    pub fn size(&self) -> i64 {
        (self.end.0 - self.start.0).abs()
    }
    /// The price at a level. Retracements measure back from the end towards the start; extensions
    /// are measured from the start, past the end (127.2% sits 27.2% of the leg beyond the end).
    pub fn at(&self, l: Level) -> Price {
        let d = self.end.0 - self.start.0;
        let part = (d * l.0) as f64 / 10000.0;
        if l.is_extension() { Price(self.start.0 + part.round() as i64) } else { Price(self.end.0 - part.round() as i64) }
    }
    /// The same leg drawn backwards: the classic mistake, which flips every retracement.
    pub fn reversed(&self) -> Leg {
        Leg { start: self.end, end: self.start }
    }
}

/// Mirrors an uptrend picture into a downtrend around $4,100, keeping prices near the same level.
fn mirror(p: i64) -> i64 {
    8_200 * DOLLAR - p
}

/// "Pick the leg": an uptrend (or downtrend) of six swings and a pullback in progress. Points C and
/// D are the last impulse; the options are that leg, the same leg backwards, an older leg, and the
/// pullback between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegQ {
    pub swings: Vec<Swing>,
    pub up: bool,
    /// Swing indices labelled A, B, C, D.
    pub labelled: [usize; 4],
    /// Each option as (from, to) letters' indices into `labelled`; `answer` is the right one.
    pub options: [(usize, usize); 4],
    pub answer: usize,
}

impl LegQ {
    pub fn new(seed: u64) -> LegQ {
        let mut rng = Rng::new(seed);
        let up = rng.below(2) == 0;
        let base = 4_050 + rng.below(40) as i64;
        let (step, gap, extra) = (6 + rng.below(6) as i64, 14 + rng.below(8) as i64, 6 + rng.below(6) as i64);
        // L0 H1 L2 H3 L4 H5, then a pullback part-way down from H5.
        let mut d = vec![base, base + gap, base + step, base + step + gap, base + 2 * step, base + 2 * step + gap + extra];
        let pull = d[5] - (d[5] - d[4]) * 2 / 5;
        d.push(pull);
        let swings: Vec<Swing> = d
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let high_up = i % 2 == 1;
                let cents = p * DOLLAR;
                if up { Swing { price: Price(cents), high: high_up } } else { Swing { price: Price(mirror(cents)), high: !high_up } }
            })
            .collect();
        // The pullback point isn't a finished swing; it's drawn as where price is now.
        let mut options = [(2, 3), (3, 2), (0, 1), (1, 2)];
        for i in (1..options.len()).rev() {
            options.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let answer = options.iter().position(|o| *o == (2, 3)).unwrap_or(0);
        LegQ { swings, up, labelled: [2, 3, 4, 5], options, answer }
    }
    /// The leg the right answer draws.
    pub fn leg(&self) -> Leg {
        Leg { start: self.swings[4].price, end: self.swings[5].price }
    }
}

/// "Place the level": a leg and one level; four prices, one right.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelPriceQ {
    pub leg: Leg,
    pub level: Level,
    pub options: Vec<Price>,
    pub answer: usize,
}

impl LevelPriceQ {
    pub const ASKED: [Level; 6] = [Level::R382, Level::R50, Level::R618, Level::R786, Level::X1272, Level::X1618];

    pub fn new(seed: u64) -> LevelPriceQ {
        let mut rng = Rng::new(seed);
        let up = rng.below(2) == 0;
        let size = 20 + 5 * rng.below(9) as i64; // $20 to $60 in $5 steps
        let low = 4_040 + rng.below(80) as i64;
        let (lo, hi) = (Price(low * DOLLAR), Price((low + size) * DOLLAR));
        let leg = if up { Leg { start: lo, end: hi } } else { Leg { start: hi, end: lo } };
        let level = Self::ASKED[rng.below(Self::ASKED.len() as u64) as usize];
        let right = leg.at(level);
        // Wrong answers: the leg drawn backwards (the classic flip), and the neighbouring levels.
        let mut wrong: Vec<Price> = Vec::new();
        if level.is_extension() {
            let d = leg.end.0 - leg.start.0;
            wrong.push(Price(leg.end.0 + (d as f64 * level.0 as f64 / 10000.0).round() as i64)); // measured from the end
            wrong.push(Price(leg.start.0 + (d as f64 * (level.0 - 10000) as f64 / 10000.0).round() as i64)); // inside the leg
            wrong.push(leg.at(if level == Level::X1272 { Level::X1618 } else { Level::X1272 }));
        } else {
            wrong.push(leg.reversed().at(level));
            for l in [Level::R382, Level::R50, Level::R618, Level::R786] {
                if l != level {
                    wrong.push(leg.at(l));
                }
            }
        }
        let mut options = vec![right];
        for w in wrong {
            if options.len() < 4 && !options.contains(&w) {
                options.push(w);
            }
        }
        for i in (1..options.len()).rev() {
            options.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let answer = options.iter().position(|p| *p == right).unwrap_or(0);
        LevelPriceQ { leg, level, options, answer }
    }
}

/// Why a zone is or isn't a confluence zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneKind {
    /// A Fibonacci level (50% or 61.8%) at the old swing the leg broke, maybe on a round number.
    Confluence,
    /// Only a Fibonacci level (38.2%), nothing else there.
    FibOnly,
    /// Past 78.6%, where this course ends the pullback play.
    PastInvalidation,
    /// An old swing, but beyond the leg's start: the leg would be broken.
    BeyondLeg,
}

/// "Find the confluence": a leg that broke an old swing, Fibonacci lines, and four zones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfluenceQ {
    pub swings: Vec<Swing>,
    pub leg: Leg,
    /// The level the old swing sits on (50% or 61.8%).
    pub level: Level,
    pub round: bool,
    /// Zones (bottom, top) in display order A–D, with what each one is.
    pub zones: [((Price, Price), ZoneKind); 4],
    pub answer: usize,
}

impl ConfluenceQ {
    pub fn new(seed: u64) -> ConfluenceQ {
        let mut rng = Rng::new(seed);
        let up = rng.below(2) == 0;
        let level = if rng.below(2) == 0 { Level::R50 } else { Level::R618 };
        let size = 80 + 10 * rng.below(3) as i64; // $80, $90 or $100, so the zones stand clear of each other
        let round = rng.below(2) == 0;
        // Uptrend frame, in dollars: old high B at the level; the leg L4 → H5 broke above it.
        let mut b = 4_060 + rng.below(60) as i64;
        if round {
            b = (b + 25) / 50 * 50;
        } else if b % 50 == 0 {
            b += 7;
        }
        let h5 = b + (size * level.0 + 5000) / 10000;
        let l4 = h5 - size;
        let l2 = l4 - 10 - rng.below(6) as i64;
        let now = h5 - size * 3 / 10;
        // An old low, the old high B, the leg's low, the leg's high beyond B (a BOS), and price now.
        let dollars = [l2, b, l4, h5, now];
        let shape = [false, true, false, true, false];
        let f = |p: i64| if up { p * DOLLAR } else { mirror(p * DOLLAR) };
        let swings: Vec<Swing> = dollars.iter().zip(shape).map(|(&p, h)| Swing { price: Price(f(p)), high: if up { h } else { !h } }).collect();
        let leg = Leg { start: Price(f(l4)), end: Price(f(h5)) };
        // Zones are $4 wide around a whole-dollar centre.
        let zone = |c: Price| {
            let c = (c.0 + DOLLAR / 2).div_euclid(DOLLAR) * DOLLAR;
            (Price(c - 2 * DOLLAR), Price(c + 2 * DOLLAR))
        };
        let mut zones = [
            (zone(Price(f(b))), ZoneKind::Confluence),
            (zone(leg.at(Level::R382)), ZoneKind::FibOnly),
            (zone(leg.at(Level(8800))), ZoneKind::PastInvalidation),
            (zone(Price(f(l2))), ZoneKind::BeyondLeg),
        ];
        for i in (1..zones.len()).rev() {
            zones.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let answer = zones.iter().position(|z| z.1 == ZoneKind::Confluence).unwrap_or(0);
        ConfluenceQ { swings, leg, level, round, zones, answer }
    }
    /// The old swing the leg broke (its price is the confluence).
    pub fn old_swing(&self) -> Price {
        self.swings[1].price
    }
    pub fn kind_label(k: ZoneKind) -> &'static str {
        match k {
            ZoneKind::Confluence => t("a Fibonacci level on the old swing the leg broke", "un niveau de Fibonacci sur l’ancien swing cassé par la jambe"),
            ZoneKind::FibOnly => t("only the 38.2% level, nothing else there", "seulement le niveau 38,2 %, rien d’autre"),
            ZoneKind::PastInvalidation => t("beyond 78.6%, where the pullback play is over", "au-delà de 78,6 %, là où le jeu du repli est terminé"),
            ZoneKind::BeyondLeg => t("an old swing, but beyond the leg's start: the leg would be broken", "un ancien swing, mais au-delà du début de la jambe : elle serait cassée"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_measure_back_from_the_end_and_extensions_from_the_start() {
        let up = Leg { start: Price(406_000), end: Price(410_000) };
        assert_eq!(up.at(Level::R50), Price(408_000));
        assert_eq!(up.at(Level::R618), Price(407_528));
        assert_eq!(up.at(Level::R382), Price(408_472));
        assert_eq!(up.at(Level::R786), Price(406_856));
        assert_eq!(up.at(Level::X1272), Price(411_088));
        assert_eq!(up.at(Level::X1618), Price(412_472));
        // A downtrend leg mirrors it: retracements measure back up from the low.
        let down = up.reversed();
        assert_eq!(down.at(Level::R618), Price(408_472));
        assert_eq!(down.at(Level::X1272), Price(404_912));
        assert_eq!(Level::R618.label(), "61.8%");
        assert_eq!(Level::R50.label(), "50%");
    }

    #[test]
    fn the_right_leg_is_the_last_impulse_and_it_broke_structure() {
        for seed in 0..300 {
            let q = LegQ::new(seed);
            assert_eq!(q.options[q.answer], (2, 3));
            let leg = q.leg();
            assert_eq!(leg.up(), q.up, "seed {seed}: drawn in the trend's direction");
            let b = q.swings[3].price.0;
            // The impulse closed beyond the previous swing (a BOS) and is the biggest leg shown.
            assert!(if q.up { leg.end.0 > b } else { leg.end.0 < b }, "seed {seed}");
            let older = (q.swings[3].price.0 - q.swings[2].price.0).abs();
            assert!(leg.size() > older, "seed {seed}");
            // Price now is part-way back, inside the leg.
            let now = q.swings[6].price.0;
            assert!((now - leg.start.0).signum() == (leg.end.0 - leg.start.0).signum() && (now - leg.end.0).abs() < leg.size(), "seed {seed}");
            let mut opts = q.options.to_vec();
            opts.sort();
            opts.dedup();
            assert_eq!(opts.len(), 4);
        }
    }

    #[test]
    fn level_questions_have_four_distinct_prices_and_the_right_one() {
        for seed in 0..500 {
            let q = LevelPriceQ::new(seed);
            assert_eq!(q.options.len(), 4, "seed {seed}");
            let mut o = q.options.clone();
            o.sort_by_key(|p| p.0);
            o.dedup();
            assert_eq!(o.len(), 4, "seed {seed}");
            assert_eq!(q.options[q.answer], q.leg.at(q.level));
        }
    }

    #[test]
    fn exactly_one_zone_has_confluence_and_the_zones_never_overlap() {
        for seed in 0..500 {
            let q = ConfluenceQ::new(seed);
            let right = q.zones[q.answer];
            assert_eq!(right.1, ZoneKind::Confluence);
            let mid = (right.0 .0 .0 + right.0 .1 .0) / 2;
            // The old swing sits on the level, within $1, and in a round-number question it is one.
            assert!((mid - q.leg.at(q.level).0).abs() <= DOLLAR, "seed {seed}: {mid} vs {:?}", q.leg.at(q.level));
            assert_eq!(mid, q.old_swing().0);
            if q.round {
                assert_eq!(mid % (50 * DOLLAR), 0, "seed {seed}");
            } else {
                assert_ne!(mid % (50 * DOLLAR), 0, "seed {seed}");
            }
            let mut zs: Vec<(i64, i64)> = q.zones.iter().map(|z| (z.0 .0 .0, z.0 .1 .0)).collect();
            zs.sort();
            for w in zs.windows(2) {
                assert!(w[1].0 - w[0].1 >= 3 * DOLLAR, "seed {seed}: zones too close {zs:?}");
            }
            // The leg broke the old swing: its end is beyond it.
            let b = q.old_swing().0;
            assert!(if q.leg.up() { q.leg.end.0 > b && q.leg.start.0 < b } else { q.leg.end.0 < b && q.leg.start.0 > b }, "seed {seed}");
        }
    }
}
