//! What the daily Hard Rules need from the rest of the Learner's day (DayContext inputs).

use crate::Millis;
use crate::journal::local_day;
use crate::money::Usd;

/// One taken trade, as the daily rules see it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayTrade {
    pub at: Millis,
    pub closed_at: Option<Millis>,
    pub pnl: Option<Usd>,
}

/// (trades taken earlier the same local day, net P/L of trades closed earlier that day) for an entry at `entry_at`.
pub fn day_totals(trades: &[DayTrade], entry_at: Millis, utc_offset_min: i32) -> (u32, Usd) {
    let day = local_day(entry_at, utc_offset_min);
    let earlier = trades.iter().filter(|t| t.at < entry_at && local_day(t.at, utc_offset_min) == day).count() as u32;
    let realized = trades
        .iter()
        .filter_map(|t| match (t.closed_at, t.pnl) {
            (Some(c), Some(p)) if c <= entry_at && local_day(c, utc_offset_min) == day => Some(p.0),
            _ => None,
        })
        .sum();
    (earlier, Usd(realized))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::utc;

    fn t(h: i64, closed: Option<(i64, i64)>) -> DayTrade {
        DayTrade { at: utc(2026, 1, 13, h, 0), closed_at: closed.map(|(ch, _)| utc(2026, 1, 13, ch, 0)), pnl: closed.map(|(_, cents)| Usd(cents)) }
    }

    #[test]
    fn counts_earlier_trades_and_closed_pnl_on_the_same_day() {
        let trades = [t(9, Some((10, -10_000))), t(11, Some((12, -5_000))), t(13, None), t(15, Some((16, 20_000)))];
        // Entry at 14:00: three trades before it, two of them closed before it (−$150).
        assert_eq!(day_totals(&trades, utc(2026, 1, 13, 14, 0), 0), (3, Usd(-15_000)));
    }

    #[test]
    fn other_days_and_later_closes_do_not_count() {
        let yesterday = DayTrade { at: utc(2026, 1, 12, 15, 0), closed_at: Some(utc(2026, 1, 12, 16, 0)), pnl: Some(Usd(-30_000)) };
        let closes_later = t(9, Some((18, -10_000)));
        assert_eq!(day_totals(&[yesterday, closes_later], utc(2026, 1, 13, 14, 0), 0), (1, Usd(0)));
    }

    #[test]
    fn the_local_day_follows_the_learners_timezone() {
        // 23:30 UTC on the 12th is 00:30 on the 13th at UTC+1.
        let late = DayTrade { at: utc(2026, 1, 12, 23, 30), closed_at: Some(utc(2026, 1, 12, 23, 45)), pnl: Some(Usd(-5_000)) };
        assert_eq!(day_totals(&[late], utc(2026, 1, 13, 9, 0), 60), (1, Usd(-5_000)));
        assert_eq!(day_totals(&[late], utc(2026, 1, 13, 9, 0), 0), (0, Usd(0)));
    }
}
