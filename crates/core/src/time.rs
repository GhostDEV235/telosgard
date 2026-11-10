//! Market time without a timezone database (PRD FR-12).
//! Sessions are anchored to their own cities: Asian = Tokyo 09:00–16:00 (00:00–07:00 UTC, no DST),
//! London = 08:00–17:00 London time, New York = 08:00–17:00 New York time. The overlap is London ∩ New York.
//! The market week runs Sunday 18:00 → Friday 17:00 New York, with a daily break 17:00–18:00 New York.

use crate::Millis;
use crate::plan::Session;
use serde::{Deserialize, Serialize};

const MIN_MS: Millis = 60_000;

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's algorithm).
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = m as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// (year, month, day) for days since 1970-01-01.
pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    (y, m, d)
}

/// 0 = Sunday … 6 = Saturday.
pub fn weekday(days: i64) -> u32 {
    (days + 4).rem_euclid(7) as u32
}

/// "YYYY-MM-DD"
pub fn date_string(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Day of month of the `n`th Sunday (1-based) of a month.
fn nth_sunday(y: i64, m: u32, n: u32) -> u32 {
    let first = days_from_civil(y, m, 1);
    let to_sunday = (7 - weekday(first)) % 7;
    1 + to_sunday + 7 * (n - 1)
}

fn last_sunday(y: i64, m: u32) -> u32 {
    let next = if m == 12 { days_from_civil(y + 1, 1, 1) } else { days_from_civil(y, m + 1, 1) };
    let last = next - 1;
    let (_, _, d) = civil_from_days(last - weekday(last) as i64);
    d
}

fn at_utc(y: i64, m: u32, d: u32, minutes: i64) -> Millis {
    days_from_civil(y, m, d) * crate::DAY_MS + minutes * MIN_MS
}

fn year_of(utc: Millis) -> i64 {
    civil_from_days(utc.div_euclid(crate::DAY_MS)).0
}

/// New York's UTC offset in minutes (−300 EST, −240 EDT). DST: 2nd Sunday of March 02:00 → 1st Sunday of November 02:00.
pub fn ny_offset_min(utc: Millis) -> i32 {
    let y = year_of(utc);
    // 02:00 EST = 07:00 UTC; 02:00 EDT = 06:00 UTC.
    let start = at_utc(y, 3, nth_sunday(y, 3, 2), 7 * 60);
    let end = at_utc(y, 11, nth_sunday(y, 11, 1), 6 * 60);
    if (start..end).contains(&utc) { -240 } else { -300 }
}

/// London's UTC offset in minutes (0 GMT, 60 BST). BST: last Sunday of March 01:00 UTC → last Sunday of October 01:00 UTC.
pub fn london_offset_min(utc: Millis) -> i32 {
    let y = year_of(utc);
    let start = at_utc(y, 3, last_sunday(y, 3), 60);
    let end = at_utc(y, 10, last_sunday(y, 10), 60);
    if (start..end).contains(&utc) { 60 } else { 0 }
}

fn local(utc: Millis, offset_min: i32) -> Local {
    let t = utc + offset_min as Millis * MIN_MS;
    let days = t.div_euclid(crate::DAY_MS);
    let minute = (t.rem_euclid(crate::DAY_MS) / MIN_MS) as u32;
    Local { days, minute, weekday: weekday(days) }
}

/// A wall-clock reading in some city.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Local {
    pub days: i64,
    /// Minutes since local midnight.
    pub minute: u32,
    pub weekday: u32,
}

pub fn ny(utc: Millis) -> Local {
    local(utc, ny_offset_min(utc))
}

pub fn london(utc: Millis) -> Local {
    local(utc, london_offset_min(utc))
}

const H17: u32 = 17 * 60;
const H18: u32 = 18 * 60;

/// Gold is quoted (outside the weekend and the daily 17:00–18:00 New York break).
pub fn market_open(utc: Millis) -> bool {
    let t = ny(utc);
    match t.weekday {
        6 => false,             // Saturday
        0 => t.minute >= H18,   // Sunday opens 18:00
        5 => t.minute < H17,    // Friday closes 17:00
        _ => !(H17..H18).contains(&t.minute),
    }
}

/// The sessions running at `utc`, in order Asian, London, New York. Two at once = the overlap. Empty when closed.
pub fn sessions_at(utc: Millis) -> Vec<Session> {
    if !market_open(utc) {
        return Vec::new();
    }
    let in_day = |l: Local, from: u32, to: u32| (1..=5).contains(&l.weekday) && (from..to).contains(&l.minute);
    let utc_min = (utc.rem_euclid(crate::DAY_MS) / MIN_MS) as u32;
    let mut out = Vec::new();
    if (1..=5).contains(&weekday(utc.div_euclid(crate::DAY_MS))) && utc_min < 7 * 60 {
        out.push(Session::Asian);
    }
    if in_day(london(utc), 8 * 60, H17) {
        out.push(Session::London);
    }
    if in_day(ny(utc), 8 * 60, H17) {
        out.push(Session::NewYork);
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NoTrade {
    MarketClosed,
    /// The daily 17:00–18:00 New York rollover.
    Rollover,
    /// The first hour after the Sunday 18:00 New York open.
    SundayOpen,
    /// Within 5 minutes of a tier-1 release.
    NearEvent { name: String },
}

pub const EVENT_BUFFER_MS: Millis = 5 * MIN_MS;

/// Why `utc` falls in a No-Trade Window, if it does. `tier_one` = (name, release time) for the day.
pub fn no_trade_window(utc: Millis, tier_one: &[(&str, Millis)]) -> Option<NoTrade> {
    let t = ny(utc);
    if (1..=4).contains(&t.weekday) && (H17..H18).contains(&t.minute) {
        return Some(NoTrade::Rollover);
    }
    if !market_open(utc) {
        return Some(NoTrade::MarketClosed);
    }
    if t.weekday == 0 && (H18..H18 + 60).contains(&t.minute) {
        return Some(NoTrade::SundayOpen);
    }
    tier_one
        .iter()
        .find(|(_, at)| (utc - at).abs() <= EVENT_BUFFER_MS)
        .map(|(name, _)| NoTrade::NearEvent { name: (*name).to_string() })
}

#[cfg(test)]
pub(crate) fn utc(y: i64, m: u32, d: u32, h: i64, min: i64) -> Millis {
    days_from_civil(y, m, d) * crate::DAY_MS + h * 3_600_000 + min * MIN_MS
}

#[cfg(test)]
mod tests {
    use super::*;
    use Session::*;

    #[test]
    fn civil_dates_are_pinned_to_known_values() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 3, 1), 11_017);
        assert_eq!(days_from_civil(2026, 1, 1), 20_454);
        assert_eq!(days_from_civil(1969, 12, 31), -1);
        for z in -800..80_000 {
            let (y, m, d) = civil_from_days(z);
            assert_eq!(days_from_civil(y, m, d), z);
        }
        assert_eq!(date_string(days_from_civil(2026, 11, 10)), "2026-11-10");
        assert_eq!(weekday(days_from_civil(2026, 11, 10)), 2, "a Tuesday");
        assert_eq!(weekday(0), 4, "1970-01-01 was a Thursday");
    }

    #[test]
    fn new_york_daylight_saving_switches_at_2am_local() {
        assert_eq!(ny_offset_min(utc(2026, 1, 15, 12, 0)), -300);
        assert_eq!(ny_offset_min(utc(2026, 7, 1, 12, 0)), -240);
        // 2026: starts Sunday 8 March, ends Sunday 1 November.
        assert_eq!(ny_offset_min(utc(2026, 3, 8, 6, 59)), -300);
        assert_eq!(ny_offset_min(utc(2026, 3, 8, 7, 0)), -240);
        assert_eq!(ny_offset_min(utc(2026, 11, 1, 5, 59)), -240);
        assert_eq!(ny_offset_min(utc(2026, 11, 1, 6, 0)), -300);
    }

    #[test]
    fn london_summer_time_switches_at_1am_utc() {
        // 2026: starts Sunday 29 March, ends Sunday 25 October.
        assert_eq!(london_offset_min(utc(2026, 3, 29, 0, 59)), 0);
        assert_eq!(london_offset_min(utc(2026, 3, 29, 1, 0)), 60);
        assert_eq!(london_offset_min(utc(2026, 10, 25, 0, 59)), 60);
        assert_eq!(london_offset_min(utc(2026, 10, 25, 1, 0)), 0);
    }

    #[test]
    fn local_wall_clock() {
        let t = ny(utc(2026, 1, 1, 3, 0)); // 22:00 on Wednesday 31 Dec 2025 in New York
        assert_eq!((t.days, t.minute, t.weekday), (days_from_civil(2025, 12, 31), 22 * 60, 3));
        assert_eq!(london(utc(2026, 7, 1, 12, 0)).minute, 13 * 60);
    }

    #[test]
    fn winter_sessions() {
        // Tuesday 13 January 2026 (UK and US both on standard time)
        assert_eq!(sessions_at(utc(2026, 1, 13, 3, 0)), vec![Asian]);
        assert_eq!(sessions_at(utc(2026, 1, 13, 7, 30)), Vec::<Session>::new(), "between Asia and London");
        assert_eq!(sessions_at(utc(2026, 1, 13, 9, 0)), vec![London]);
        assert_eq!(sessions_at(utc(2026, 1, 13, 14, 0)), vec![London, NewYork], "the overlap");
        assert_eq!(sessions_at(utc(2026, 1, 13, 19, 0)), vec![NewYork]);
        assert_eq!(sessions_at(utc(2026, 1, 13, 22, 30)), Vec::<Session>::new(), "rollover break");
    }

    #[test]
    fn summer_sessions_shift_with_daylight_saving() {
        // Tuesday 14 July 2026: BST and EDT
        assert_eq!(sessions_at(utc(2026, 7, 14, 7, 30)), vec![London], "08:30 in London");
        assert_eq!(sessions_at(utc(2026, 7, 14, 12, 30)), vec![London, NewYork], "08:30 in New York");
        assert_eq!(sessions_at(utc(2026, 7, 14, 16, 30)), vec![NewYork], "17:30 in London");
    }

    #[test]
    fn march_gap_when_only_the_us_has_changed_clocks() {
        // Tuesday 10 March 2026: New York on EDT, London still on GMT → overlap starts at 12:00 UTC.
        assert_eq!(sessions_at(utc(2026, 3, 10, 12, 0)), vec![London, NewYork]);
        assert_eq!(sessions_at(utc(2026, 3, 10, 11, 59)), vec![London]);
    }

    #[test]
    fn market_week() {
        assert!(!market_open(utc(2026, 1, 17, 12, 0)), "Saturday");
        assert!(!market_open(utc(2026, 1, 18, 22, 59)), "Sunday 17:59 New York");
        assert!(market_open(utc(2026, 1, 18, 23, 0)), "Sunday 18:00 New York");
        assert!(market_open(utc(2026, 1, 16, 21, 59)), "Friday 16:59 New York");
        assert!(!market_open(utc(2026, 1, 16, 22, 0)), "Friday 17:00 New York");
        assert!(!market_open(utc(2026, 1, 13, 22, 0)), "daily break");
        assert!(market_open(utc(2026, 1, 13, 23, 0)));
    }

    #[test]
    fn no_trade_windows() {
        assert_eq!(no_trade_window(utc(2026, 1, 13, 22, 0), &[]), Some(NoTrade::Rollover));
        assert_eq!(no_trade_window(utc(2026, 1, 13, 22, 59), &[]), Some(NoTrade::Rollover));
        assert_eq!(no_trade_window(utc(2026, 1, 13, 23, 0), &[]), None);
        assert_eq!(no_trade_window(utc(2026, 1, 18, 23, 0), &[]), Some(NoTrade::SundayOpen));
        assert_eq!(no_trade_window(utc(2026, 1, 18, 23, 59), &[]), Some(NoTrade::SundayOpen));
        assert_eq!(no_trade_window(utc(2026, 1, 19, 0, 0), &[]), None);
        assert_eq!(no_trade_window(utc(2026, 1, 17, 12, 0), &[]), Some(NoTrade::MarketClosed));
        assert_eq!(no_trade_window(utc(2026, 1, 16, 22, 30), &[]), Some(NoTrade::MarketClosed), "Friday after the close");
    }

    #[test]
    fn five_minutes_either_side_of_a_tier_one_release() {
        let cpi = [("US CPI", utc(2026, 1, 13, 13, 30))];
        let near = Some(NoTrade::NearEvent { name: "US CPI".into() });
        assert_eq!(no_trade_window(utc(2026, 1, 13, 13, 24), &cpi), None);
        assert_eq!(no_trade_window(utc(2026, 1, 13, 13, 25), &cpi), near);
        assert_eq!(no_trade_window(utc(2026, 1, 13, 13, 35), &cpi), near);
        assert_eq!(no_trade_window(utc(2026, 1, 13, 13, 36), &cpi), None);
    }
}
