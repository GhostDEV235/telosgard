//! Stage 2 drills (Wave 2): "Which session is it?" and "Is this a No-Trade Window?".
//! Deterministic from a seed. Answers come from `time::sessions_at` and `time::no_trade_window`,
//! the same functions the session clock and the journal's Hard Rules use, so a drill can never
//! disagree with the app.

use crate::Millis;
use crate::drills::Rng;
use crate::plan::Session;
use crate::time::{NoTrade, days_from_civil, london_offset_min, no_trade_window, ny_offset_min, sessions_at};

const MIN_MS: Millis = 60_000;

/// Which city's clock a question is told in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum City {
    NewYork,
    London,
}

impl City {
    pub fn name(self) -> &'static str {
        match self {
            City::NewYork => "New York",
            City::London => "London",
        }
    }
}

/// UTC for a wall-clock time in `city` on a civil date (DST-aware, using the app's own offsets).
pub fn city_to_utc(city: City, y: i64, m: u32, d: u32, minute: u32) -> Millis {
    let naive = days_from_civil(y, m, d) * crate::DAY_MS + minute as Millis * MIN_MS;
    let offset = |utc: Millis| match city {
        City::NewYork => ny_offset_min(utc),
        City::London => london_offset_min(utc),
    } as Millis;
    // Two passes settle the offset away from the change-over hour (drill dates avoid it anyway).
    let guess = naive - offset(naive) * MIN_MS;
    naive - offset(guess) * MIN_MS
}

/// The answer to "which session is it?", as one of five plain choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionAnswer {
    Asian,
    London,
    Overlap,
    NewYork,
    /// Between sessions (late New York evening to Tokyo morning) or during the daily break.
    NoneOpen,
}

impl SessionAnswer {
    pub const ALL: [SessionAnswer; 5] = [SessionAnswer::Asian, SessionAnswer::London, SessionAnswer::Overlap, SessionAnswer::NewYork, SessionAnswer::NoneOpen];

    pub fn from_sessions(s: &[Session]) -> SessionAnswer {
        match (s.contains(&Session::Asian), s.contains(&Session::London), s.contains(&Session::NewYork)) {
            (_, true, true) => SessionAnswer::Overlap,
            (_, true, false) => SessionAnswer::London,
            (_, false, true) => SessionAnswer::NewYork,
            (true, false, false) => SessionAnswer::Asian,
            _ => SessionAnswer::NoneOpen,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SessionAnswer::Asian => "Asian (Tokyo)",
            SessionAnswer::London => "London only",
            SessionAnswer::Overlap => "London and New York: the overlap",
            SessionAnswer::NewYork => "New York only",
            SessionAnswer::NoneOpen => "None of the main sessions",
        }
    }
}

/// "It's 10:30 in London on a Wednesday in July. Which session is open?"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionQ {
    pub city: City,
    /// Weekday name and month, e.g. ("Wednesday", "July").
    pub day: &'static str,
    pub month: &'static str,
    /// Minutes since local midnight in `city`.
    pub minute: u32,
    pub utc: Millis,
    pub answer: SessionAnswer,
}

/// Reference weeks clear of any clock change: mid-July (summer time) and mid-November (winter time).
const DATES: [(i64, u32, u32, &str, &str); 6] = [
    (2026, 7, 14, "Tuesday", "July"),
    (2026, 7, 15, "Wednesday", "July"),
    (2026, 7, 16, "Thursday", "July"),
    (2026, 11, 17, "Tuesday", "November"),
    (2026, 11, 18, "Wednesday", "November"),
    (2026, 11, 19, "Thursday", "November"),
];

impl SessionQ {
    pub fn new(seed: u64) -> SessionQ {
        let mut rng = Rng::new(seed);
        // Aim for each answer about equally often: draw a target, then a time that produces it.
        let target = SessionAnswer::ALL[rng.below(5) as usize];
        loop {
            let (y, m, d, day, month) = DATES[rng.below(DATES.len() as u64) as usize];
            let city = if rng.below(2) == 0 { City::NewYork } else { City::London };
            let minute = (rng.below(48) * 30) as u32; // on the hour or half past
            let utc = city_to_utc(city, y, m, d, minute);
            let answer = SessionAnswer::from_sessions(&sessions_at(utc));
            if answer == target {
                return SessionQ { city, day, month, minute, utc, answer };
            }
        }
    }

    pub fn clock(&self) -> String {
        format!("{:02}:{:02}", self.minute / 60, self.minute % 60)
    }

    pub fn prompt(&self) -> String {
        format!("It's {} on a {} in {}, {} time. Which session is open?", self.clock(), self.day, self.month, self.city.name())
    }
}

/// The answer to "is this a No-Trade Window?".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAnswer {
    Rollover,
    SundayOpen,
    NearEvent,
    Closed,
    Clear,
}

impl WindowAnswer {
    pub const ALL: [WindowAnswer; 5] = [WindowAnswer::Rollover, WindowAnswer::SundayOpen, WindowAnswer::NearEvent, WindowAnswer::Closed, WindowAnswer::Clear];

    pub fn from_rule(r: &Option<NoTrade>) -> WindowAnswer {
        match r {
            Some(NoTrade::Rollover) => WindowAnswer::Rollover,
            Some(NoTrade::SundayOpen) => WindowAnswer::SundayOpen,
            Some(NoTrade::NearEvent { .. }) => WindowAnswer::NearEvent,
            Some(NoTrade::MarketClosed) => WindowAnswer::Closed,
            None => WindowAnswer::Clear,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            WindowAnswer::Rollover => "Yes: the daily rollover",
            WindowAnswer::SundayOpen => "Yes: the first hour after the Sunday open",
            WindowAnswer::NearEvent => "Yes: within 5 minutes of a tier-1 release",
            WindowAnswer::Closed => "Gold isn't trading then",
            WindowAnswer::Clear => "No: you may trade if your plan allows",
        }
    }
}

/// "Thursday, 8:33 am New York time. US CPI comes out at 8:30 today. Is this a No-Trade Window?"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoTradeQ {
    pub day: &'static str,
    /// Minutes since New York midnight.
    pub minute: u32,
    /// A tier-1 release on that day: (name, New York minute).
    pub event: Option<(&'static str, u32)>,
    pub answer: WindowAnswer,
}

/// A winter week (New York on EST) clear of clock changes: Sunday 15 – Saturday 21 November 2026.
const WEEK: [(u32, &str); 6] = [(15, "Sunday"), (16, "Monday"), (17, "Tuesday"), (18, "Wednesday"), (19, "Thursday"), (20, "Friday")];
const EVENTS: [(&str, u32); 3] = [("US CPI", 8 * 60 + 30), ("The US jobs report (NFP)", 8 * 60 + 30), ("The FOMC decision", 14 * 60)];

impl NoTradeQ {
    pub fn new(seed: u64) -> NoTradeQ {
        let mut rng = Rng::new(seed);
        let target = WindowAnswer::ALL[rng.below(5) as usize];
        loop {
            let (d, day) = WEEK[rng.below(WEEK.len() as u64) as usize];
            let event = (rng.below(2) == 0 && day != "Sunday").then(|| EVENTS[rng.below(EVENTS.len() as u64) as usize]);
            // Times cluster where the rules bite: near an event, around 5–6 pm, Sunday evening.
            let minute = match (target, event) {
                (WindowAnswer::NearEvent | WindowAnswer::Clear, Some((_, at))) => (at as i64 + rng.below(25) as i64 - 12).clamp(0, 1439) as u32,
                (WindowAnswer::Rollover | WindowAnswer::Closed | WindowAnswer::SundayOpen, _) => 16 * 60 + 30 + rng.below(28) as u32 * 5,
                _ => 6 * 60 + rng.below(56) as u32 * 15,
            };
            let utc = city_to_utc(City::NewYork, 2026, 11, d, minute);
            let releases: Vec<(&str, Millis)> = event.iter().map(|(n, at)| (*n, city_to_utc(City::NewYork, 2026, 11, d, *at))).collect();
            let answer = WindowAnswer::from_rule(&no_trade_window(utc, &releases));
            if answer == target {
                return NoTradeQ { day, minute, event, answer };
            }
        }
    }

    pub fn clock(&self) -> String {
        let (h, m) = (self.minute / 60, self.minute % 60);
        let (h12, ampm) = match h {
            0 => (12, "am"),
            1..=11 => (h, "am"),
            12 => (12, "pm"),
            _ => (h - 12, "pm"),
        };
        format!("{h12}:{m:02} {ampm}")
    }

    pub fn prompt(&self) -> String {
        let event = match self.event {
            Some((name, at)) => {
                let q = NoTradeQ { minute: at, ..self.clone() };
                format!(" {name} is released at {} today.", q.clock())
            }
            None => " No tier-1 release is scheduled today.".into(),
        };
        format!("{}, {} New York time.{event} Is this a No-Trade Window?", self.day, self.clock())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn city_times_convert_with_daylight_saving() {
        // 10:00 London in July (BST) is 09:00 UTC; in November (GMT) it's 10:00 UTC.
        let h = 3_600_000;
        assert_eq!(city_to_utc(City::London, 2026, 7, 15, 600) % crate::DAY_MS, 9 * h);
        assert_eq!(city_to_utc(City::London, 2026, 11, 18, 600) % crate::DAY_MS, 10 * h);
        // 08:30 New York in November (EST) is 13:30 UTC; in July (EDT) 12:30 UTC.
        assert_eq!(city_to_utc(City::NewYork, 2026, 11, 18, 510) % crate::DAY_MS, 13 * h + 30 * 60_000);
        assert_eq!(city_to_utc(City::NewYork, 2026, 7, 15, 510) % crate::DAY_MS, 12 * h + 30 * 60_000);
    }

    #[test]
    fn session_answers_match_the_session_clock() {
        for seed in 0..400 {
            let q = SessionQ::new(seed);
            assert_eq!(q.answer, SessionAnswer::from_sessions(&sessions_at(q.utc)), "seed {seed}");
        }
    }

    #[test]
    fn session_questions_are_reproducible_and_cover_every_answer() {
        assert_eq!(SessionQ::new(7), SessionQ::new(7));
        let seen: std::collections::HashSet<_> = (0..200).map(|s| format!("{:?}", SessionQ::new(s).answer)).collect();
        assert_eq!(seen.len(), 5, "{seen:?}");
    }

    #[test]
    fn known_session_cases() {
        // 09:00 London on a Wednesday in November: London only (New York is at 04:00).
        let utc = city_to_utc(City::London, 2026, 11, 18, 9 * 60);
        assert_eq!(SessionAnswer::from_sessions(&sessions_at(utc)), SessionAnswer::London);
        // 10:00 New York: the overlap.
        let utc = city_to_utc(City::NewYork, 2026, 11, 18, 10 * 60);
        assert_eq!(SessionAnswer::from_sessions(&sessions_at(utc)), SessionAnswer::Overlap);
        // 20:00 New York in winter is 10:00 in Tokyo: the Asian session.
        let utc = city_to_utc(City::NewYork, 2026, 11, 18, 20 * 60);
        assert_eq!(SessionAnswer::from_sessions(&sessions_at(utc)), SessionAnswer::Asian);
        // 18:30 New York: after the daily break, before Tokyo opens: no main session.
        let utc = city_to_utc(City::NewYork, 2026, 11, 18, 18 * 60 + 30);
        assert_eq!(SessionAnswer::from_sessions(&sessions_at(utc)), SessionAnswer::NoneOpen);
    }

    #[test]
    fn no_trade_answers_match_the_hard_rule() {
        for seed in 0..400 {
            let q = NoTradeQ::new(seed);
            let d = WEEK.iter().find(|(_, n)| *n == q.day).unwrap().0;
            let utc = city_to_utc(City::NewYork, 2026, 11, d, q.minute);
            let releases: Vec<(&str, Millis)> = q.event.iter().map(|(n, at)| (*n, city_to_utc(City::NewYork, 2026, 11, d, *at))).collect();
            assert_eq!(q.answer, WindowAnswer::from_rule(&no_trade_window(utc, &releases)), "seed {seed}: {}", q.prompt());
        }
    }

    #[test]
    fn no_trade_questions_cover_every_answer_and_read_well() {
        let seen: std::collections::HashSet<_> = (0..300).map(|s| format!("{:?}", NoTradeQ::new(s).answer)).collect();
        assert_eq!(seen.len(), 5, "{seen:?}");
        let q = NoTradeQ { day: "Thursday", minute: 8 * 60 + 33, event: Some(("US CPI", 8 * 60 + 30)), answer: WindowAnswer::NearEvent };
        assert_eq!(q.prompt(), "Thursday, 8:33 am New York time. US CPI is released at 8:30 am today. Is this a No-Trade Window?");
        let q = NoTradeQ { day: "Tuesday", minute: 17 * 60 + 15, event: None, answer: WindowAnswer::Rollover };
        assert_eq!(q.clock(), "5:15 pm");
    }
}
