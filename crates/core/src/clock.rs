//! Session clock (PRD FR-12.2): when the next London–New York overlap and the next rollover start.

use crate::Millis;
use crate::plan::Session;
use crate::time::{ny, sessions_at};

const MIN_MS: Millis = 60_000;
/// Look at most this far ahead (covers a full weekend).
const HORIZON_MIN: i64 = 8 * 24 * 60;

/// Start of the next London–New York overlap strictly after `utc`, to the minute.
pub fn next_overlap_start(utc: Millis) -> Option<Millis> {
    let start = utc - utc.rem_euclid(MIN_MS) + MIN_MS;
    let mut prev = in_overlap(start - MIN_MS);
    (0..HORIZON_MIN).map(|i| start + i * MIN_MS).find(|&t| {
        let now = in_overlap(t);
        let edge = now && !prev;
        prev = now;
        edge
    })
}

/// Start of the next daily rollover (17:00 New York, Monday–Thursday) strictly after `utc`.
pub fn next_rollover(utc: Millis) -> Option<Millis> {
    let start = utc - utc.rem_euclid(MIN_MS) + MIN_MS;
    (0..HORIZON_MIN).map(|i| start + i * MIN_MS).find(|&t| {
        let l = ny(t);
        (1..=4).contains(&l.weekday) && l.minute == 17 * 60
    })
}

/// True while both London and New York are open.
pub fn in_overlap(utc: Millis) -> bool {
    let s = sessions_at(utc);
    s.contains(&Session::London) && s.contains(&Session::NewYork)
}

/// The Monday (as days since 1970-01-01) of the week containing `day`.
pub fn week_start(day: i64) -> i64 {
    // weekday(): 0 = Sunday … 6 = Saturday; Monday-based offset.
    day - ((crate::time::weekday(day) + 6) % 7) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{days_from_civil, utc};

    #[test]
    fn next_overlap_same_day_and_after_the_weekend() {
        // Tuesday 13 January 2026, winter: overlap is 13:00–17:00 UTC.
        assert_eq!(next_overlap_start(utc(2026, 1, 13, 9, 0)), Some(utc(2026, 1, 13, 13, 0)));
        assert_eq!(next_overlap_start(utc(2026, 1, 13, 14, 0)), Some(utc(2026, 1, 14, 13, 0)), "already inside: the next one");
        assert_eq!(next_overlap_start(utc(2026, 1, 16, 18, 0)), Some(utc(2026, 1, 19, 13, 0)), "Friday evening → Monday");
        // March gap: New York on summer time, London not → overlap starts 12:00 UTC.
        assert_eq!(next_overlap_start(utc(2026, 3, 10, 6, 0)), Some(utc(2026, 3, 10, 12, 0)));
        assert!(in_overlap(utc(2026, 1, 13, 14, 0)) && !in_overlap(utc(2026, 1, 13, 18, 0)));
    }

    #[test]
    fn next_rollover_skips_friday_and_weekend() {
        assert_eq!(next_rollover(utc(2026, 1, 13, 9, 0)), Some(utc(2026, 1, 13, 22, 0)));
        assert_eq!(next_rollover(utc(2026, 7, 14, 9, 0)), Some(utc(2026, 7, 14, 21, 0)), "summer: 17:00 EDT = 21:00 UTC");
        assert_eq!(next_rollover(utc(2026, 1, 16, 10, 0)), Some(utc(2026, 1, 19, 22, 0)), "Friday's 17:00 is the weekly close, not a rollover");
    }

    #[test]
    fn weeks_start_on_monday() {
        let tue = days_from_civil(2026, 11, 10);
        assert_eq!(week_start(tue), days_from_civil(2026, 11, 9));
        assert_eq!(week_start(days_from_civil(2026, 11, 9)), days_from_civil(2026, 11, 9));
        assert_eq!(week_start(days_from_civil(2026, 11, 15)), days_from_civil(2026, 11, 9), "Sunday belongs to the week before");
    }
}
