//! Stage 3 drill (Wave 3): "Hot or soft?" A US release with its forecast and actual number; the
//! Learner judges the surprise and the usual direction for gold. Deterministic from a seed.
//! The numbers are illustrative, never real past releases.

use crate::drills::Rng;
use crate::i18n::{is_fr, t};

/// A release the drill can quote. `higher_is_hot` is false for unemployment: a higher rate than
/// forecast means a weaker economy, so it's soft.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Release {
    CpiMonthly,
    Payrolls,
    Unemployment,
    RetailSales,
}

impl Release {
    pub const ALL: [Release; 4] = [Release::CpiMonthly, Release::Payrolls, Release::Unemployment, Release::RetailSales];

    pub fn name(self) -> &'static str {
        match self {
            Release::CpiMonthly => t("US CPI (inflation, change on the month)", "CPI américain (inflation, variation sur le mois)"),
            Release::Payrolls => t("US jobs report: new jobs (NFP)", "Rapport américain sur l’emploi : créations d’emplois (NFP)"),
            Release::Unemployment => t("US unemployment rate", "Taux de chômage américain"),
            Release::RetailSales => t("US retail sales (change on the month)", "Ventes au détail américaines (variation sur le mois)"),
        }
    }

    pub fn higher_is_hot(self) -> bool {
        !matches!(self, Release::Unemployment)
    }

    /// Shows a value stored in hundredths: percentages as "0.4%", jobs (in thousands) as "180K".
    pub fn show(self, v: i64) -> String {
        let fr = is_fr();
        match self {
            Release::Payrolls => {
                if fr { format!("{} k", v / 100) } else { format!("{}K", v / 100) }
            }
            _ => {
                let sign = if v < 0 { "−" } else { "" };
                let a = v.abs();
                let s = format!("{sign}{}.{}", a / 100, (a % 100) / 10);
                if fr { format!("{}\u{a0}%", s.replace('.', ",")) } else { format!("{s}%") }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surprise {
    Hot,
    Soft,
    InLine,
}

impl Surprise {
    pub const ALL: [Surprise; 3] = [Surprise::Hot, Surprise::Soft, Surprise::InLine];

    /// The choice as shown: the surprise and the usual reaction together.
    pub fn label(self) -> &'static str {
        match self {
            Surprise::Hot => t("Hot: gold usually falls", "Chaude : l’or baisse en général"),
            Surprise::Soft => t("Soft: gold usually rises", "Molle : l’or monte en général"),
            Surprise::InLine => t("In line: no clear push either way", "Conforme : pas de poussée nette"),
        }
    }
}

/// Hot, soft or in line for `actual` against `forecast` (same units).
pub fn judge(release: Release, forecast: i64, actual: i64) -> Surprise {
    use std::cmp::Ordering::*;
    match (actual.cmp(&forecast), release.higher_is_hot()) {
        (Equal, _) => Surprise::InLine,
        (Greater, true) | (Less, false) => Surprise::Hot,
        _ => Surprise::Soft,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurpriseQ {
    pub release: Release,
    /// Last time's value, shown for realism; it never decides the answer.
    pub previous: i64,
    pub forecast: i64,
    pub actual: i64,
    pub answer: Surprise,
}

impl SurpriseQ {
    pub fn new(seed: u64) -> SurpriseQ {
        let mut rng = Rng::new(seed);
        let release = Release::ALL[rng.below(4) as usize];
        let target = Surprise::ALL[rng.below(3) as usize];
        // Base levels and one step of surprise, in hundredths (jobs: hundredths of a thousand).
        let (base, step, spread) = match release {
            Release::CpiMonthly => (20 + rng.below(3) as i64 * 10, 10 + rng.below(2) as i64 * 10, 20),
            Release::Payrolls => (12_000 + rng.below(10) as i64 * 1_000, 5_000 + rng.below(8) as i64 * 1_000, 6_000),
            Release::Unemployment => (390 + rng.below(4) as i64 * 10, 10 + rng.below(3) as i64 * 10, 20),
            Release::RetailSales => (10 + rng.below(5) as i64 * 10, 30 + rng.below(4) as i64 * 10, 40),
        };
        let forecast = base;
        let up = |hot: bool| if hot == release.higher_is_hot() { step } else { -step };
        let actual = match target {
            Surprise::InLine => forecast,
            Surprise::Hot => forecast + up(true),
            Surprise::Soft => forecast + up(false),
        };
        // "Previous" moves independently, sometimes opposite to the surprise, so last month can't be used as a shortcut.
        let previous = forecast + (rng.below(5) as i64 - 2) * spread / 2;
        SurpriseQ { release, previous, forecast, actual, answer: judge(release, forecast, actual) }
    }

    pub fn prompt(&self) -> String {
        if is_fr() {
            format!("{}. Précédent {}, prévision {}, réel {}. Comment est la surprise, et que fait l’or en général ?", self.release.name(), self.release.show(self.previous), self.release.show(self.forecast), self.release.show(self.actual))
        } else {
            format!("{}. Previous {}, forecast {}, actual {}. What's the surprise, and what does gold usually do?", self.release.name(), self.release.show(self.previous), self.release.show(self.forecast), self.release.show(self.actual))
        }
    }

    pub fn why(&self) -> String {
        let (f, a) = (self.release.show(self.forecast), self.release.show(self.actual));
        let fr = is_fr();
        match (self.answer, self.release) {
            (Surprise::InLine, _) => {
                if fr { format!("Le réel ({a}) est égal à la prévision ({f}) : le marché l’attendait déjà. Le chiffre précédent ne compte pas.") }
                else { format!("The actual ({a}) matches the forecast ({f}): the market already expected it. Last time's number doesn't count.") }
            }
            (Surprise::Soft, Release::Unemployment) => {
                if fr { format!("Un chômage plus élevé que prévu ({a} contre {f}) veut dire un marché de l’emploi plus faible : c’est mou, ce qui pousse en général l’or à la hausse.") }
                else { format!("Unemployment higher than forecast ({a} vs {f}) means a weaker jobs market: that's soft, which usually lifts gold.") }
            }
            (Surprise::Hot, Release::Unemployment) => {
                if fr { format!("Un chômage plus bas que prévu ({a} contre {f}) veut dire un marché de l’emploi plus fort : c’est chaud, ce qui pèse en général sur l’or.") }
                else { format!("Unemployment lower than forecast ({a} vs {f}) means a stronger jobs market: that's hot, which usually weighs on gold.") }
            }
            (Surprise::Hot, _) => {
                if fr { format!("Le réel ({a}) dépasse la prévision ({f}) : c’est chaud. Les traders anticipent des taux plus élevés, donc l’or baisse en général. En général, pas toujours.") }
                else { format!("The actual ({a}) beat the forecast ({f}): that's hot. Traders expect higher rates, so gold usually falls. Usually, not always.") }
            }
            (Surprise::Soft, _) => {
                if fr { format!("Le réel ({a}) est sous la prévision ({f}) : c’est mou. Les traders anticipent des taux plus bas, donc l’or monte en général. En général, pas toujours.") }
                else { format!("The actual ({a}) came in below the forecast ({f}): that's soft. Traders expect lower rates, so gold usually rises. Usually, not always.") }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_and_soft_follow_the_forecast_not_last_month() {
        assert_eq!(judge(Release::CpiMonthly, 30, 50), Surprise::Hot);
        assert_eq!(judge(Release::CpiMonthly, 30, 20), Surprise::Soft);
        assert_eq!(judge(Release::CpiMonthly, 40, 40), Surprise::InLine);
        assert_eq!(judge(Release::Payrolls, 18_000, 9_000), Surprise::Soft);
    }

    #[test]
    fn higher_unemployment_is_soft() {
        assert_eq!(judge(Release::Unemployment, 410, 440), Surprise::Soft);
        assert_eq!(judge(Release::Unemployment, 410, 390), Surprise::Hot);
    }

    #[test]
    fn generated_questions_are_consistent_and_cover_every_answer() {
        let mut seen = std::collections::HashSet::new();
        for seed in 0..300 {
            let q = SurpriseQ::new(seed);
            assert_eq!(q.answer, judge(q.release, q.forecast, q.actual), "seed {seed}");
            seen.insert((q.answer as u8, q.release as u8));
        }
        assert_eq!(seen.len(), 12, "every release with every answer: {seen:?}");
    }

    #[test]
    fn values_read_like_a_calendar() {
        assert_eq!(Release::CpiMonthly.show(30), "0.3%");
        assert_eq!(Release::Payrolls.show(18_000), "180K");
        assert_eq!(Release::Unemployment.show(410), "4.1%");
        assert_eq!(Release::RetailSales.show(-20), "−0.2%");
        let q = SurpriseQ { release: Release::CpiMonthly, previous: 20, forecast: 30, actual: 50, answer: Surprise::Hot };
        assert_eq!(q.prompt(), "US CPI (inflation, change on the month). Previous 0.2%, forecast 0.3%, actual 0.5%. What's the surprise, and what does gold usually do?");
    }
}
