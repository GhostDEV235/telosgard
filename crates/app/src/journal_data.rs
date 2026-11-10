//! Plans, checklists, trades, readiness runs and review notes: loaded once storage is connected,
//! kept in signals, and saved through `Store::persist` (in memory only when saving is off).

use crate::platform::PlatformError;
use crate::state::{Progress, now};
use crate::store::{ReadinessRecord, ReviewNote, RunRecord, SaveState, Store, TradeRecord, repo};
use gold_core::Millis;
use gold_core::checklist::PreSessionChecklist;
use gold_core::daily::{DayTrade, day_totals};
use gold_core::journal::{DayContext, local_day};
use gold_core::money::Usd;
use gold_core::plan::{HardRules, PlanError, PlanHistory, SoftRules, TradingPlan};
use gold_core::readiness::{ReadinessProgress, ReadinessRun, TradeSummary};
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::BTreeMap;

/// The device's offset from UTC in minutes (UTC+1 → 60).
pub fn utc_offset_min() -> i32 {
    -(js_sys::Date::new_0().get_timezone_offset() as i32)
}

pub fn today() -> i64 {
    local_day(now(), utc_offset_min())
}

/// A local wall-clock time for display, e.g. "Tue 13 Jan, 09:05".
pub fn local_label(ms: Millis) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64));
    const D: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const M: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!("{} {} {}, {:02}:{:02}", D[d.get_day() as usize % 7], d.get_date(), M[d.get_month() as usize % 12], d.get_hours(), d.get_minutes())
}

pub fn local_hm(ms: Millis) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64));
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

/// UTC millis for "HH:MM" on a local day.
pub fn local_time_to_utc(day: i64, hm: &str) -> Option<Millis> {
    let (h, m) = hm.trim().split_once(':')?;
    let (h, m): (i64, i64) = (h.parse().ok()?, m.parse().ok()?);
    if !(0..24).contains(&h) || !(0..60).contains(&m) {
        return None;
    }
    Some(day * gold_core::DAY_MS + (h * 60 + m - utc_offset_min() as i64) * 60_000)
}

#[derive(Clone, Copy)]
pub struct JournalData {
    pub loaded: RwSignal<bool>,
    pub plans: RwSignal<PlanHistory>,
    /// Newest first.
    pub trades: RwSignal<Vec<(String, TradeRecord)>>,
    pub checklists: RwSignal<BTreeMap<i64, PreSessionChecklist>>,
    pub runs: RwSignal<Vec<RunRecord>>,
    pub readiness: RwSignal<ReadinessRecord>,
    pub reviews: RwSignal<BTreeMap<i64, String>>,
    store: Store,
}

impl JournalData {
    pub fn new(store: Store) -> JournalData {
        JournalData {
            loaded: RwSignal::new(false),
            plans: RwSignal::new(PlanHistory::default()),
            trades: RwSignal::new(Vec::new()),
            checklists: RwSignal::new(BTreeMap::new()),
            runs: RwSignal::new(Vec::new()),
            readiness: RwSignal::new(ReadinessRecord::default()),
            reviews: RwSignal::new(BTreeMap::new()),
            store,
        }
    }

    /// Loads everything once storage reports Saved. Without storage the journal still works in memory.
    pub fn watch_and_load(self, progress: Progress) {
        let started = StoredValue::new(false);
        Effect::new(move |_| {
            if self.store.state.get() != SaveState::Saved || started.get_value() {
                return;
            }
            started.set_value(true);
            spawn_local(async move {
                let loaded = async {
                    Ok::<_, PlatformError>((
                        repo::plans().await?,
                        repo::trades().await?,
                        repo::checklists().await?,
                        repo::runs().await?,
                        repo::readiness().await?,
                        repo::reviews().await?,
                    ))
                }
                .await;
                if let Ok((plans, mut trades, checklists, runs, readiness, reviews)) = loaded {
                    trades.sort_by_key(|(_, t)| std::cmp::Reverse(t.entry.at));
                    self.plans.set(PlanHistory::from_versions(plans));
                    self.trades.set(trades);
                    self.checklists.set(checklists.into_iter().map(|c| (c.day, c)).collect());
                    let mut runs = runs;
                    runs.sort_by_key(|r| r.run.number);
                    self.runs.set(runs);
                    if readiness.reached_at.is_some() {
                        progress.readiness.set(true);
                    }
                    self.readiness.set(readiness);
                    self.reviews.set(reviews.into_iter().map(|r| (r.week, r.note)).collect());
                }
                self.loaded.set(true);
            });
        });
        // Without storage there is nothing to load.
        Effect::new(move |_| {
            if matches!(self.store.state.get(), SaveState::Local(_)) {
                self.loaded.set(true);
            }
        });
    }

    pub fn current_plan(&self) -> Option<TradingPlan> {
        self.plans.with(|h| h.current().cloned())
    }

    pub fn save_plan(self, hard: HardRules, soft: SoftRules) -> Result<TradingPlan, PlanError> {
        let mut h = self.plans.get_untracked();
        let plan = h.save(hard, soft, now())?.clone();
        self.plans.set(h);
        let p = plan.clone();
        self.store.persist(async move { repo::save_plan(&p).await });
        Ok(plan)
    }

    pub fn checklist(&self, day: i64) -> Option<PreSessionChecklist> {
        self.checklists.with(|m| m.get(&day).cloned())
    }

    pub fn save_checklist(self, c: PreSessionChecklist) {
        self.checklists.update(|m| {
            m.insert(c.day, c.clone());
        });
        self.store.persist(async move { repo::save_checklist(&c).await });
    }

    pub fn trade(&self, id: &str) -> Option<TradeRecord> {
        self.trades.with(|v| v.iter().find(|(i, _)| i == id).map(|(_, t)| t.clone()))
    }

    pub fn put_trade(self, id: String, rec: TradeRecord) {
        self.trades.update(|v| {
            match v.iter_mut().find(|(i, _)| *i == id) {
                Some(slot) => slot.1 = rec.clone(),
                None => v.push((id.clone(), rec.clone())),
            }
            v.sort_by_key(|(_, t)| std::cmp::Reverse(t.entry.at));
        });
        self.store.persist(async move { repo::save_trade(&id, &rec).await });
    }

    /// The Readiness Run new trades belong to: the latest one that hasn't ended.
    pub fn active_run(&self) -> Option<RunRecord> {
        self.runs.with(|r| r.iter().rev().find(|r| r.ended_at.is_none()).cloned())
    }

    pub fn start_run(self, starting_balance: Usd) -> RunRecord {
        let at = now();
        let next = match self.runs.with_untracked(|r| r.last().map(|r| r.run)) {
            Some(prev) => prev.next(at, starting_balance),
            None => ReadinessRun::first(at, starting_balance),
        };
        let rec = RunRecord { run: next, ended_at: None, end_reason: None };
        self.runs.update(|r| r.push(rec.clone()));
        let r = rec.clone();
        self.store.persist(async move { repo::save_run(&r).await });
        rec
    }

    pub fn end_active_run(self, reason: &str) {
        let Some(mut run) = self.active_run() else { return };
        run.ended_at = Some(now());
        run.end_reason = Some(reason.to_string());
        self.runs.update(|rs| {
            if let Some(slot) = rs.iter_mut().find(|r| r.run.number == run.run.number) {
                *slot = run.clone();
            }
        });
        self.store.persist(async move { repo::save_run(&run).await });
    }

    /// The day's DayContext inputs for an entry at `at` (excluding `exclude`, the entry itself).
    pub fn day_context_for(&self, at: Millis, exclude: Option<&str>) -> (u32, Usd, Option<PreSessionChecklist>) {
        let offset = utc_offset_min();
        let trades: Vec<DayTrade> = self.trades.with_untracked(|v| {
            v.iter()
                .filter(|(i, t)| t.taken() && Some(i.as_str()) != exclude)
                .map(|(_, t)| DayTrade { at: t.entry.at, closed_at: t.result.as_ref().map(|r| r.closed_at), pnl: t.result.as_ref().map(|r| r.pnl) })
                .collect()
        });
        let (count, realized) = day_totals(&trades, at, offset);
        (count, realized, self.checklist(local_day(at, offset)))
    }

    pub fn day_context<'a>(count: u32, realized: Usd, checklist: Option<&'a PreSessionChecklist>) -> DayContext<'a> {
        DayContext { trades_earlier_today: count, realized_today: realized, checklist }
    }

    /// Readiness progress for the active run, against the current plan's max total loss.
    pub fn readiness_progress(&self) -> Option<(RunRecord, ReadinessProgress)> {
        let run = self.active_run()?;
        let plan = self.current_plan()?;
        let summaries: Vec<TradeSummary> = self.trades.with(|v| {
            v.iter()
                .filter(|(_, t)| t.taken() && t.run == run.run.number)
                .map(|(_, t)| TradeSummary { at: t.entry.at, final_adherent: t.adherence.final_adherent(), balance_after: t.result.as_ref().map(|r| r.balance_after) })
                .collect()
        });
        Some((run.clone(), run.run.progress(&summaries, plan.hard.max_total_loss)))
    }

    /// Writes the permanent unlock once.
    pub fn mark_ready(self, progress: Progress) {
        if self.readiness.with_untracked(|r| r.reached_at.is_some()) {
            return;
        }
        let rec = ReadinessRecord { reached_at: Some(now()) };
        self.readiness.set(rec.clone());
        progress.readiness.set(true);
        self.store.persist(async move { repo::save_readiness(&rec).await });
    }

    pub fn save_review(self, week: i64, note: String) {
        self.reviews.update(|m| {
            m.insert(week, note.clone());
        });
        let r = ReviewNote { week, note };
        self.store.persist(async move { repo::save_review(&r).await });
    }

    /// The latest known balance: last closed trade's balance after, else the active run's start.
    pub fn latest_balance(&self) -> Option<Usd> {
        let closed = self.trades.with_untracked(|v| {
            v.iter().filter(|(_, t)| t.taken()).filter_map(|(_, t)| t.result.as_ref().map(|r| (r.closed_at, r.balance_after))).max_by_key(|(c, _)| *c).map(|(_, b)| b)
        });
        closed.or_else(|| self.active_run().map(|r| r.run.starting_balance))
    }
}
