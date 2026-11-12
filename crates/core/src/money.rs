//! Money units. Everything is integer cents so no rule ever depends on float rounding.
//!
//! - **Dollar Move**: the main unit. A change in gold's price, e.g. $2,350.00 → $2,355.00 is a $5 move.
//! - **Point** = $0.01, **Pip** = $0.10 (this course's convention; other sources differ).
//! - **Lot** = 100 oz by default, so a $1 move is $100 per lot.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Neg, Sub};

pub const CENTS_PER_PIP: i64 = 10;
pub const DEFAULT_CONTRACT_OZ: u32 = 100;

/// A gold price in USD per ounce, held in cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Price(pub i64);

/// A change in price, held in cents. Signed: negative means price fell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct DollarMove(pub i64);

/// An amount of account money, held in cents. Signed: negative is a loss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Usd(pub i64);

/// A position size, held in hundredths of a lot (0.01 lot is the smallest MT5 step).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Lots(pub i64);

/// A percentage, held in basis points (1% = 100). Also used for ratios ×100 (1.5R = 150).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Percent(pub u32);

impl Price {
    pub fn parse(s: &str) -> Option<Price> {
        parse_cents(s).filter(|c| *c > 0).map(Price)
    }
}

impl Sub for Price {
    type Output = DollarMove;
    fn sub(self, rhs: Price) -> DollarMove {
        DollarMove(self.0 - rhs.0)
    }
}

impl Add<DollarMove> for Price {
    type Output = Price;
    fn add(self, rhs: DollarMove) -> Price {
        Price(self.0 + rhs.0)
    }
}

impl DollarMove {
    pub fn from_points(points: i64) -> DollarMove {
        DollarMove(points)
    }
    pub fn from_pips(pips: i64) -> DollarMove {
        DollarMove(pips * CENTS_PER_PIP)
    }
    pub fn points(self) -> i64 {
        self.0
    }
    /// Pips, rounded toward zero (a move of $0.15 is 1 pip and 5 points).
    pub fn pips(self) -> i64 {
        self.0 / CENTS_PER_PIP
    }
    pub fn abs(self) -> DollarMove {
        DollarMove(self.0.abs())
    }
    pub fn parse(s: &str) -> Option<DollarMove> {
        parse_cents(s).map(DollarMove)
    }
    /// Money made or lost when price moves by `self` while holding `lots` of `contract_oz` ounces.
    pub fn value(self, lots: Lots, contract_oz: u32) -> Usd {
        Usd(self.0 * contract_oz as i64 * lots.0 / 100)
    }
}

impl Usd {
    pub fn parse(s: &str) -> Option<Usd> {
        parse_cents(s).map(Usd)
    }
}

impl Add for Usd {
    type Output = Usd;
    fn add(self, rhs: Usd) -> Usd {
        Usd(self.0 + rhs.0)
    }
}

impl Sub for Usd {
    type Output = Usd;
    fn sub(self, rhs: Usd) -> Usd {
        Usd(self.0 - rhs.0)
    }
}

impl Neg for DollarMove {
    type Output = DollarMove;
    fn neg(self) -> DollarMove {
        DollarMove(-self.0)
    }
}

impl Neg for Usd {
    type Output = Usd;
    fn neg(self) -> Usd {
        Usd(-self.0)
    }
}

impl Percent {
    /// Parses "1", "1.5", "0.25" (percent, up to 2 decimals) into basis points.
    pub fn parse(s: &str) -> Option<Percent> {
        parse_cents(s).filter(|c| *c >= 0).map(|c| Percent(c as u32))
    }
    /// `part` as a share of `whole`, rounded down to 0.01%.
    pub fn of(part: Usd, whole: Usd) -> Percent {
        if whole.0 <= 0 {
            return Percent(0);
        }
        Percent((part.0.max(0) * 10_000 / whole.0) as u32)
    }
    /// `pct` of `amount`, rounded down to the cent.
    pub fn apply(self, amount: Usd) -> Usd {
        Usd(amount.0 * self.0 as i64 / 10_000)
    }
}

/// Parses a decimal string with up to two decimals ("2345.6", "$1,000", "-5") into hundredths.
/// French input works too ("4 100,50", "0,20", "80 $"): a comma followed by one or two digits,
/// with no point anywhere, is a decimal comma; any other comma groups thousands.
fn parse_cents(s: &str) -> Option<i64> {
    let s: String = s.trim().replace('−', "-").chars().filter(|c| !matches!(c, '$' | ' ' | '%' | '\u{a0}' | '\u{202f}')).collect();
    let decimal_comma = !s.contains('.') && s.rsplit_once(',').is_some_and(|(_, tail)| (1..=2).contains(&tail.len()));
    let s: String = if decimal_comma { s.replacen(',', ".", 1) } else { s.replace(',', "") };
    let (neg, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.as_str()),
    };
    let (whole, frac) = s.split_once('.').unwrap_or((s, ""));
    if whole.is_empty() && frac.is_empty() || frac.len() > 2 {
        return None;
    }
    if !whole.chars().chain(frac.chars()).all(|c| c.is_ascii_digit()) {
        return None;
    }
    let w: i64 = if whole.is_empty() { 0 } else { whole.parse().ok()? };
    let f: i64 = format!("{frac:0<2}").parse().ok()?;
    let v = w.checked_mul(100)?.checked_add(f)?;
    Some(if neg { -v } else { v })
}

/// "$4,150.00" in English; "4 150,00 $" in French (narrow no-break spaces), docs/FRENCH.md.
fn fmt_cents(f: &mut fmt::Formatter<'_>, cents: i64, prefix: &str) -> fmt::Result {
    let fr = crate::i18n::is_fr();
    let sign = if cents < 0 { "−" } else { "" };
    let abs = cents.unsigned_abs();
    let whole = (abs / 100).to_string();
    let mut grouped = String::new();
    for (i, ch) in whole.chars().enumerate() {
        if i > 0 && (whole.len() - i) % 3 == 0 {
            grouped.push(if fr { '\u{202f}' } else { ',' });
        }
        grouped.push(ch);
    }
    if fr {
        let unit = if prefix.is_empty() { String::new() } else { format!("\u{a0}{prefix}") };
        write!(f, "{sign}{grouped},{:02}{unit}", abs % 100)
    } else {
        write!(f, "{sign}{prefix}{grouped}.{:02}", abs % 100)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_cents(f, self.0, "$")
    }
}
impl fmt::Display for DollarMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_cents(f, self.0, "$")
    }
}
impl fmt::Display for Usd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_cents(f, self.0, "$")
    }
}
impl fmt::Display for Lots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = if crate::i18n::is_fr() { ',' } else { '.' };
        write!(f, "{}{sep}{:02}", self.0 / 100, self.0 % 100)
    }
}
impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (w, r) = (self.0 / 100, self.0 % 100);
        let (sep, pct) = if crate::i18n::is_fr() { (",", "\u{a0}%") } else { (".", "%") };
        if r == 0 { write!(f, "{w}{pct}") } else { write!(f, "{}{pct}", format!("{w}{sep}{r:02}").trim_end_matches('0')) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn french_formatting_and_parsing() {
        assert_eq!(Price::parse("4 100,50"), Price::parse("4100.50"));
        assert_eq!(DollarMove::parse("0,30"), DollarMove::parse("0.30"));
        assert_eq!(Usd::parse("1,000"), Usd::parse("1000"));
        assert_eq!(Usd::parse("80 $"), Usd::parse("80"));
        crate::i18n::set(crate::i18n::Lang::Fr);
        let shown = (Price::parse("4150").unwrap().to_string(), Lots(20).to_string(), Percent::parse("0.5").unwrap().to_string());
        crate::i18n::set(crate::i18n::Lang::En);
        assert_eq!(shown, ("4\u{202f}150,00\u{a0}$".to_string(), "0,20".to_string(), "0,5\u{a0}%".to_string()));
    }

    #[test]
    fn price_difference_is_a_dollar_move() {
        let a = Price::parse("2350.00").unwrap();
        let b = Price::parse("2355").unwrap();
        assert_eq!(b - a, DollarMove(500));
        assert_eq!(a - b, DollarMove(-500));
        assert_eq!((a - b).abs(), DollarMove(500));
    }

    #[test]
    fn points_and_pips_convert_to_dollar_moves() {
        assert_eq!(DollarMove::from_points(1), DollarMove(1)); // $0.01
        assert_eq!(DollarMove::from_pips(1), DollarMove(10)); // $0.10
        assert_eq!(DollarMove::from_pips(50), DollarMove(500)); // "50 pips" = $5
        let five_dollars = DollarMove::parse("5").unwrap();
        assert_eq!(five_dollars.points(), 500);
        assert_eq!(five_dollars.pips(), 50);
        assert_eq!(DollarMove(15).pips(), 1);
    }

    #[test]
    fn one_dollar_move_is_100_usd_per_standard_lot() {
        assert_eq!(DollarMove(100).value(Lots(100), DEFAULT_CONTRACT_OZ), Usd(10_000));
        assert_eq!(DollarMove(500).value(Lots(20), DEFAULT_CONTRACT_OZ), Usd(10_000)); // $5 × 0.20 lots = $100
        assert_eq!(DollarMove(-300).value(Lots(1), DEFAULT_CONTRACT_OZ), Usd(-300)); // −$3 × 0.01 = −$3
    }

    #[test]
    fn parsing_accepts_common_formats_and_rejects_junk() {
        assert_eq!(Usd::parse("$10,000"), Some(Usd(1_000_000)));
        assert_eq!(Usd::parse("12.5"), Some(Usd(1250)));
        assert_eq!(Usd::parse(".5"), Some(Usd(50)));
        assert_eq!(Usd::parse("-3"), Some(Usd(-300)));
        assert_eq!(Usd::parse("1.234"), None);
        assert_eq!(Usd::parse("abc"), None);
        assert_eq!(Usd::parse(""), None);
        assert_eq!(Price::parse("0"), None);
        assert_eq!(Price::parse("-1"), None);
        assert_eq!(Percent::parse("1.5%"), Some(Percent(150)));
    }

    #[test]
    fn percent_of_and_apply() {
        assert_eq!(Percent::of(Usd(10_000), Usd(1_000_000)), Percent(100)); // $100 of $10k = 1%
        assert_eq!(Percent(100).apply(Usd(1_000_000)), Usd(10_000));
        assert_eq!(Percent(100).apply(Usd(50_000)), Usd(500)); // 1% of $500 = $5
        assert_eq!(Percent::of(Usd(5), Usd(0)), Percent(0));
    }

    #[test]
    fn display_formats() {
        assert_eq!(Usd(1_234_567).to_string(), "$12,345.67");
        assert_eq!(Usd(-1250).to_string(), "−$12.50");
        assert_eq!(Price(234_567).to_string(), "$2,345.67");
        assert_eq!(Lots(20).to_string(), "0.20");
        assert_eq!(Lots(150).to_string(), "1.50");
        assert_eq!(Percent(100).to_string(), "1%");
        assert_eq!(Percent(150).to_string(), "1.5%");
        assert_eq!(Percent(25).to_string(), "0.25%");
    }
}
