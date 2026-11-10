//! Phase 7 exit check, the deterministic part (IMPLEMENTATION-PLAN): plan v1 → trade with no
//! checklist → checklist → adherent trade → 17:30 New York trade → daily loss limit → dispute →
//! readiness and weekly numbers against a hand calculation. Claude's own verdicts are stand-ins here;
//! the live calls are checked by the Learner in the app.

use gold_core::checklist::{Bias, CalendarEvent, PreSessionChecklist, RiskCommitment, Tier, Zone};
use gold_core::daily::{DayTrade, day_totals};
use gold_core::journal::{Adherence, DayContext, DisputeError, Direction, HardRule, HardRuleResult, TradeEntry, combine, evaluate_hard_rules, pnl};
use gold_core::money::{DollarMove, Lots, Percent, Price, Usd};
use gold_core::plan::{HardRules, PlanHistory, Session, SoftRules};
use gold_core::readiness::{ReadinessRun, TradeSummary};
use gold_core::review::{ReviewEntry, weekly_review};
use gold_core::time::days_from_civil;

fn utc(y: i64, m: u32, d: u32, h: i64, min: i64) -> i64 {
    days_from_civil(y, m, d) * gold_core::DAY_MS + h * 3_600_000 + min * 60_000
}

fn p(s: &str) -> Price {
    Price::parse(s).unwrap()
}

fn buy(at: i64) -> TradeEntry {
    TradeEntry {
        at,
        direction: Direction::Buy,
        entry: p("4100"),
        stop: Some(p("4095")),       // $5 → $100 at 0.20 lots = 1% of $10,000
        take_profit: Some(p("4110")), // 2R
        lots: Lots(20),
        balance_before: Usd(1_000_000),
        contract_oz: 100,
        setup_reason: "H4 uptrend, pullback to 4,095 zone, bullish engulfing".into(),
        plan_version: 1,
    }
}

fn checklist(day: i64, cpi_at: Option<i64>) -> PreSessionChecklist {
    PreSessionChecklist {
        day,
        events: cpi_at.map(|at| vec![CalendarEvent { name: "US CPI".into(), tier: Tier::One, at }]).unwrap_or_default(),
        asian_high: Some(p("4112")),
        asian_low: Some(p("4096")),
        mood: "Dollar weaker".into(),
        bias: Bias::Up,
        zones: vec![Zone { low: p("4090"), high: p("4095") }],
        scenarios: vec!["If price pulls back to 4,090–4,095 and an H1 bullish engulfing closes, I buy.".into()],
        risk: RiskCommitment { per_trade: Percent(100), daily_limit: Percent(300), planned_stops: vec![DollarMove(500)] },
    }
}

fn failed(results: &[HardRuleResult]) -> Vec<HardRule> {
    results.iter().filter(|r| !r.pass).map(|r| r.rule).collect()
}

#[test]
fn phase_7_end_to_end() {
    // 1. Plan v1.
    let mut plans = PlanHistory::default();
    let hard = HardRules {
        max_risk: Percent(100),
        max_total_loss: Percent(1_000),
        min_reward_risk: Percent(150),
        sessions: vec![Session::London, Session::NewYork],
        max_trades_per_day: 5,
        daily_loss_limit: Percent(300),
    };
    let plan = plans.save(hard, SoftRules { setup: "Classic Price Action".into(), extra: vec![] }, 0).unwrap().clone();
    assert_eq!(plan.version, 1);

    // 2. Tuesday 13 Jan, 09:00 UTC (London): a trade with no checklist.
    let a = buy(utc(2026, 1, 13, 9, 0));
    let ra = evaluate_hard_rules(&a, &plan.hard, &DayContext { trades_earlier_today: 0, realized_today: Usd(0), checklist: None });
    assert_eq!(failed(&ra), vec![HardRule::ChecklistDone]);
    assert!(!combine(&ra, true), "Claude can't make it adherent");
    let verdict_a = Adherence::Judged { adherent: combine(&ra, true) };
    assert_eq!(verdict_a.dispute(&ra), Err(DisputeError::HardRuleFailed), "and it can't be disputed");

    // 3. Checklist saved for the day (CPI at 13:30 UTC).
    let tue = checklist(days_from_civil(2026, 1, 13), Some(utc(2026, 1, 13, 13, 30)));
    assert_eq!(tue.validate(), Ok(()));

    // 4. 10:00 UTC: an adherent trade matching the Scenario.
    let b = buy(utc(2026, 1, 13, 10, 0));
    let rb = evaluate_hard_rules(&b, &plan.hard, &DayContext { trades_earlier_today: 1, realized_today: Usd(0), checklist: Some(&tue) });
    assert!(failed(&rb).is_empty(), "{rb:?}");
    let verdict_b = Adherence::Judged { adherent: combine(&rb, true) };
    assert_eq!(verdict_b.final_adherent(), Some(true));

    // 5. 22:30 UTC = 17:30 New York: inside the rollover.
    let c = buy(utc(2026, 1, 13, 22, 30));
    let rc = evaluate_hard_rules(&c, &plan.hard, &DayContext { trades_earlier_today: 2, realized_today: Usd(0), checklist: Some(&tue) });
    assert!(failed(&rc).contains(&HardRule::NoTradeWindow));
    // ...and 13:27 UTC is within 5 minutes of CPI.
    let near = buy(utc(2026, 1, 13, 13, 27));
    let rn = evaluate_hard_rules(&near, &plan.hard, &DayContext { trades_earlier_today: 2, realized_today: Usd(0), checklist: Some(&tue) });
    assert!(failed(&rn).contains(&HardRule::NoTradeWindow));

    // 6. Wednesday 14 Jan: three −1R losses, then one more trade.
    let wed = checklist(days_from_civil(2026, 1, 14), None);
    let losers: Vec<DayTrade> = (0..3)
        .map(|i| {
            let at = utc(2026, 1, 14, 9 + i, 0);
            let e = buy(at);
            DayTrade { at, closed_at: Some(at + 30 * 60_000), pnl: Some(pnl(&e, p("4095"))) }
        })
        .collect();
    assert_eq!(losers[0].pnl, Some(Usd(-10_000)), "−$100 = −1R");
    let fourth_at = utc(2026, 1, 14, 13, 0);
    let (count, realized) = day_totals(&losers, fourth_at, 0);
    assert_eq!((count, realized), (3, Usd(-30_000)));
    let rd = evaluate_hard_rules(&buy(fourth_at), &plan.hard, &DayContext { trades_earlier_today: count, realized_today: realized, checklist: Some(&wed) });
    assert_eq!(failed(&rd), vec![HardRule::DailyLossLimit]);
    // The third trade (after two losses, −2R) was still allowed.
    let (c3, r3) = day_totals(&losers[..2], losers[2].at, 0);
    let r3rules = evaluate_hard_rules(&buy(losers[2].at), &plan.hard, &DayContext { trades_earlier_today: c3, realized_today: r3, checklist: Some(&wed) });
    assert!(failed(&r3rules).is_empty());

    // 7. Dispute a Soft Rule verdict: Claude said "not adherent", all Hard Rules passed.
    let first = Adherence::Judged { adherent: combine(&rb, false) };
    let pending = first.dispute(&rb).unwrap();
    assert_eq!(pending.final_adherent(), None, "not final while pending");
    let second = pending.resolve_dispute(combine(&rb, true));
    assert_eq!(second.final_adherent(), Some(true));
    assert!(second.dispute(&rb).is_err(), "one dispute per verdict");

    // 8. Readiness for run 1 (starting $10,000; max loss 10% = $1,000) over the trades above.
    //    Final verdicts: A ✗, B ✓, C ✗, three Wednesday losers ✓ (plan followed), D ✗ (daily limit).
    //    Balances after: A +$200 (TP), B −$100, C not closed, losers −$100 each, D not closed.
    let run = ReadinessRun::first(0, Usd(1_000_000));
    let mut bal = 1_000_000;
    let mut summaries = Vec::new();
    for (at, adherent, change) in [
        (a.at, false, Some(20_000)),
        (b.at, true, Some(-10_000)),
        (c.at, false, None),
        (losers[0].at, true, Some(-10_000)),
        (losers[1].at, true, Some(-10_000)),
        (losers[2].at, true, Some(-10_000)),
        (fourth_at, false, None),
    ] {
        let after = change.map(|d| {
            bal += d;
            Usd(bal)
        });
        summaries.push(TradeSummary { at, final_adherent: Some(adherent), balance_after: after });
    }
    let progress = run.progress(&summaries, Percent(1_000));
    // Hand calculation: 7 counted, 4 adherent → 57.14% → 5714 bp; latest closed balance $9,800 → $200 drawdown.
    assert_eq!(progress.counted, 7);
    assert_eq!(progress.adherent, 4);
    assert_eq!(progress.adherence, Percent(5_714));
    assert_eq!(progress.drawdown, Usd(20_000));
    assert_eq!(progress.max_loss, Usd(100_000));
    assert_eq!(progress.days, 1, "13 Jan 09:00 → 14 Jan 13:00 is one whole day");
    assert!(!progress.breached && !progress.reached);

    // 9. Weekly Review of the same entries (week of Monday 12 Jan) plus one skipped setup.
    let risk = Some(Usd(10_000));
    let entries = vec![
        ReviewEntry { skipped: false, pnl: Some(Usd(20_000)), risk, failed_rules: failed(&ra), adherent: Some(false) },
        ReviewEntry { skipped: false, pnl: Some(Usd(-10_000)), risk, failed_rules: vec![], adherent: Some(true) },
        ReviewEntry { skipped: false, pnl: None, risk, failed_rules: failed(&rc), adherent: Some(false) },
        ReviewEntry { skipped: false, pnl: Some(Usd(-10_000)), risk, failed_rules: vec![], adherent: Some(true) },
        ReviewEntry { skipped: false, pnl: Some(Usd(-10_000)), risk, failed_rules: vec![], adherent: Some(true) },
        ReviewEntry { skipped: false, pnl: Some(Usd(-10_000)), risk, failed_rules: vec![], adherent: Some(true) },
        ReviewEntry { skipped: false, pnl: None, risk, failed_rules: failed(&rd), adherent: Some(false) },
        ReviewEntry { skipped: true, pnl: None, risk: None, failed_rules: vec![], adherent: None },
    ];
    let w = weekly_review(&entries);
    // Hand calculation: 7 taken, 1 skipped, 5 closed, 1 win → 20%; R = (+2 −1 −1 −1 −1)/5 = −0.4R; net −$200.
    assert_eq!((w.taken, w.skipped, w.closed, w.wins), (7, 1, 5, 1));
    assert_eq!(w.win_rate, Percent(2_000));
    assert_eq!(w.avg_r, -40);
    assert_eq!(w.net, Usd(-20_000));
    assert_eq!(w.not_adherent, 3);
    assert!(w.rule_breaks.iter().any(|(r, _)| *r == HardRule::NoTradeWindow));
    assert!(w.rule_breaks.iter().any(|(r, _)| *r == HardRule::DailyLossLimit));
}
