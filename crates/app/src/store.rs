//! Typed repositories over the Learner's private folder `data/users/<id>/` (ARCHITECTURE §5).
//! Every document carries a `kind`; ids follow the data model table.

use crate::platform::{self, PlatformError};
use crate::state::{Progress, Toasts, now};
use crate::theme::ThemeChoice;
use telosgard_core::Millis;
use telosgard_core::adherence::SoftResult;
use telosgard_core::checklist::PreSessionChecklist;
use telosgard_core::course::LessonId;
use telosgard_core::gate::{Grade, LessonProgress};
use telosgard_core::journal::{Adherence, Emotion, HardRuleResult, RMultiple, TradeEntry, r_multiple};
use telosgard_core::money::{Price, Usd};
use telosgard_core::plan::TradingPlan;
use telosgard_core::readiness::ReadinessRun;
use telosgard_core::retention::{Confidence, QuestionKey, QuestionRecord};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// A stored document: `{ "kind": …, …body }`.
#[derive(Serialize, Deserialize)]
pub struct Stored<T> {
    pub kind: String,
    #[serde(flatten)]
    pub body: T,
}

pub mod kind {
    pub const PROFILE: &str = "profile";
    pub const PROGRESS: &str = "progress";
    pub const ATTEMPT: &str = "attempt";
    pub const PLAN: &str = "plan";
    pub const TRADE: &str = "trade";
    pub const CHECKLIST: &str = "checklist";
    pub const RUN: &str = "run";
    pub const DRILL: &str = "drill";
    pub const READINESS: &str = "readiness";
    pub const REVIEW: &str = "review";
    pub const QREC: &str = "qrec";
}

/// One lesson's Question Records (`qrec-01`): question id → record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QrecDoc {
    pub lesson: u8,
    pub questions: BTreeMap<String, QuestionRecord>,
}

pub fn qrec_doc_id(lesson: u8) -> String {
    format!("qrec-{lesson:02}")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub created_at: Millis,
    pub disclaimer_accepted_at: Option<Millis>,
    /// "system" | "dark" | "light"
    pub theme: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
    pub lesson: String,
    pub answer: String,
    pub grade: Grade,
    pub feedback: String,
    /// Almost: the missing idea. Got it: an optional tip.
    #[serde(default)]
    pub gap: Option<String>,
    pub reteach: Option<String>,
    /// Almost / Not yet: their answer completed, additions in [[brackets]].
    #[serde(default)]
    pub suggestion: Option<String>,
    pub at: Millis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResult {
    pub exit: Price,
    pub pnl: Usd,
    pub balance_after: Usd,
    pub closed_at: Millis,
    /// Emotion Tag chosen when closing (Phase 11; older results have none).
    #[serde(default)]
    pub emotion: Option<Emotion>,
    /// One-line post-trade lesson, separate from the pre-trade reason.
    #[serde(default)]
    pub lesson: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeRecord {
    /// "taken" | "skipped"
    pub status: String,
    pub entry: TradeEntry,
    /// The Learner's local day of the entry (days since 1970-01-01).
    pub checklist_day: i64,
    pub run: u32,
    #[serde(default)]
    pub scenario: Option<String>,
    pub hard_results: Vec<HardRuleResult>,
    pub adherence: Adherence,
    #[serde(default)]
    pub soft_results: Vec<SoftResult>,
    pub verdict_summary: Option<String>,
    #[serde(default)]
    pub first_summary: Option<String>,
    pub dispute_text: Option<String>,
    #[serde(default)]
    pub screenshot: Option<String>,
    pub result: Option<TradeResult>,
}

impl TradeRecord {
    pub fn taken(&self) -> bool {
        self.status == "taken"
    }
    /// Money at risk when placed: 1R for this trade.
    pub fn risk(&self) -> Option<Usd> {
        let e = &self.entry;
        e.stop.map(|s| (e.entry - s).abs().value(e.lots, e.contract_oz)).filter(|u| u.0 > 0)
    }
    /// The closed result in R (P/L ÷ the risk at entry).
    pub fn r(&self) -> Option<RMultiple> {
        r_multiple(self.result.as_ref()?.pnl, self.risk()?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunRecord {
    #[serde(flatten)]
    pub run: ReadinessRun,
    pub ended_at: Option<Millis>,
    pub end_reason: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrillRecord {
    pub answered: u32,
    pub correct: u32,
    pub last_at: Millis,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadinessRecord {
    pub reached_at: Option<Millis>,
}

/// A sortable, unique id suffix: time then randomness (no extra crates on wasm).
pub fn new_id(prefix: &str) -> String {
    let rand = (js_sys::Math::random() * u32::MAX as f64) as u32;
    format!("{prefix}-{:012x}{rand:08x}", now())
}

/// When a document made by `new_id` was created (its first 12 hex digits are the time in ms).
pub fn logged_at(id: &str) -> Option<Millis> {
    let hex = id.rsplit_once('-')?.1.get(..12)?;
    i64::from_str_radix(hex, 16).ok()
}

pub async fn load_doc<T: DeserializeOwned>(id: &str) -> Result<Option<T>, PlatformError> {
    Ok(platform::get::<Stored<T>>(id).await?.map(|s| s.body))
}

pub async fn save_doc<T: Serialize + Clone>(id: &str, kind: &str, body: &T) -> Result<(), PlatformError> {
    platform::set(id, &Stored { kind: kind.to_string(), body: body.clone() }).await
}

pub async fn load_all<T: DeserializeOwned>(kind: &str) -> Result<Vec<(String, T)>, PlatformError> {
    Ok(platform::list::<Stored<T>>(kind).await?.into_iter().map(|(id, s)| (id, s.body)).collect())
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewNote {
    /// Monday of the reviewed week (days since 1970-01-01).
    pub week: i64,
    pub note: String,
}

/// Repositories, kept here so every document shape lives in one file.
#[allow(dead_code)]
pub mod repo {
    use super::*;

    pub async fn plans() -> Result<Vec<TradingPlan>, PlatformError> {
        Ok(load_all::<TradingPlan>(kind::PLAN).await?.into_iter().map(|(_, p)| p).collect())
    }
    pub async fn save_plan(p: &TradingPlan) -> Result<(), PlatformError> {
        save_doc(&format!("plan-v{}", p.version), kind::PLAN, p).await
    }
    pub async fn checklist(day: i64) -> Result<Option<PreSessionChecklist>, PlatformError> {
        load_doc(&format!("checklist-{}", telosgard_core::time::date_string(day))).await
    }
    pub async fn save_checklist(c: &PreSessionChecklist) -> Result<(), PlatformError> {
        save_doc(&c.doc_id(), kind::CHECKLIST, c).await
    }
    pub async fn trades() -> Result<Vec<(String, TradeRecord)>, PlatformError> {
        load_all(kind::TRADE).await
    }
    pub async fn save_trade(id: &str, t: &TradeRecord) -> Result<(), PlatformError> {
        save_doc(id, kind::TRADE, t).await
    }
    pub async fn delete_trade(id: &str) -> Result<(), PlatformError> {
        platform::delete(id).await
    }
    pub async fn delete_run(number: u32) -> Result<(), PlatformError> {
        platform::delete(&format!("run-{number}")).await
    }
    pub async fn runs() -> Result<Vec<RunRecord>, PlatformError> {
        Ok(load_all::<RunRecord>(kind::RUN).await?.into_iter().map(|(_, r)| r).collect())
    }
    pub async fn save_run(r: &RunRecord) -> Result<(), PlatformError> {
        save_doc(&format!("run-{}", r.run.number), kind::RUN, r).await
    }
    pub async fn attempts(lesson: LessonId) -> Result<Vec<Attempt>, PlatformError> {
        let id = lesson.doc_id();
        Ok(load_all::<Attempt>(kind::ATTEMPT).await?.into_iter().map(|(_, a)| a).filter(|a| a.lesson == id).collect())
    }
    pub async fn save_attempt(a: &Attempt) -> Result<(), PlatformError> {
        save_doc(&new_id("attempt"), kind::ATTEMPT, a).await
    }
    pub async fn checklists() -> Result<Vec<PreSessionChecklist>, PlatformError> {
        Ok(load_all::<PreSessionChecklist>(kind::CHECKLIST).await?.into_iter().map(|(_, c)| c).collect())
    }
    pub async fn reviews() -> Result<Vec<ReviewNote>, PlatformError> {
        Ok(load_all::<ReviewNote>(kind::REVIEW).await?.into_iter().map(|(_, r)| r).collect())
    }
    pub async fn save_review(r: &ReviewNote) -> Result<(), PlatformError> {
        save_doc(&format!("review-{}", telosgard_core::time::date_string(r.week)), kind::REVIEW, r).await
    }
    pub async fn readiness() -> Result<ReadinessRecord, PlatformError> {
        Ok(load_doc("readiness").await?.unwrap_or_default())
    }
    pub async fn save_readiness(r: &ReadinessRecord) -> Result<(), PlatformError> {
        save_doc("readiness", kind::READINESS, r).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalReason {
    /// Preview, a view without storage, or permission declined.
    NoStorage,
    /// No signed-in viewer to own a private folder.
    SignedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveState {
    Connecting,
    Saving,
    Saved,
    Local(LocalReason),
    Failed,
}

/// App-wide sync state. Content never waits on it; only saving degrades.
#[derive(Clone, Copy)]
pub struct Store {
    toasts: Toasts,
    pub state: RwSignal<SaveState>,
    pub profile: RwSignal<Option<Profile>>,
    pub drills: RwSignal<HashMap<String, DrillRecord>>,
    /// Question Records by lesson number.
    pub qrecs: RwSignal<HashMap<u8, BTreeMap<String, QuestionRecord>>>,
    /// Lessons whose Question Records changed since the last save.
    qrec_dirty: RwSignal<BTreeSet<u8>>,
    /// The signed-in account: (email, is owner). None when signed out or not yet known.
    pub account: RwSignal<Option<(Option<String>, bool)>>,
}

impl Store {
    pub fn new(toasts: Toasts) -> Store {
        Store { toasts, state: RwSignal::new(SaveState::Connecting), profile: RwSignal::new(None), drills: RwSignal::new(Default::default()), qrecs: RwSignal::new(Default::default()), qrec_dirty: RwSignal::new(Default::default()), account: RwSignal::new(None) }
    }

    fn enabled(&self) -> bool {
        !matches!(self.state.get_untracked(), SaveState::Local(_) | SaveState::Connecting)
    }

    fn report(&self, e: PlatformError) {
        self.state.set(match e {
            PlatformError::NotGranted => SaveState::Local(LocalReason::NoStorage),
            _ => SaveState::Failed,
        });
        self.toasts.show(e.describe());
    }

    /// Connects, then loads profile, progress and drills. Called once, after first render.
    pub fn start(self, progress: Progress, theme: RwSignal<ThemeChoice>, disclaimer: crate::state::DisclaimerOpen) {
        spawn_local(async move {
            let caps = match platform::init().await {
                Ok(c) => c,
                Err(_) => {
                    self.state.set(SaveState::Local(LocalReason::NoStorage));
                    return;
                }
            };
            if caps.signed_in {
                self.account.set(Some((caps.email.clone(), caps.owner)));
                progress.full_access.set(caps.full_access);
            }
            if !caps.db {
                self.state.set(SaveState::Local(if caps.user && !caps.signed_in { LocalReason::SignedOut } else { LocalReason::NoStorage }));
                return;
            }
            let loaded = async {
                let profile = load_doc::<Profile>("profile").await?;
                let lessons = load_all::<LessonProgress>(kind::PROGRESS).await?;
                let drills = load_all::<DrillRecord>(kind::DRILL).await?;
                let qrecs = load_all::<QrecDoc>(kind::QREC).await?;
                Ok::<_, PlatformError>((profile, lessons, drills, qrecs))
            }
            .await;
            match loaded {
                Ok((profile, lessons, drills, qrecs)) => {
                    // Merge stored progress into anything done while loading; remember what needs saving.
                    let mut stored_map = std::collections::HashMap::new();
                    for (doc_id, stored) in lessons {
                        if let Some(id) = LessonId::from_doc_id(&doc_id) {
                            progress.update(id, |p| *p = p.merge(&stored));
                            stored_map.insert(id, stored);
                        }
                    }
                    let dirty: Vec<LessonId> =
                        progress.entries_untracked().into_iter().filter(|(id, p)| stored_map.get(id) != Some(p)).map(|(id, _)| id).collect();
                    self.drills.update(|m| {
                        for (doc_id, d) in drills {
                            let e = m.entry(doc_id.trim_start_matches("drill-").to_string()).or_default();
                            e.answered += d.answered;
                            e.correct += d.correct;
                            e.last_at = e.last_at.max(d.last_at);
                        }
                    });
                    // Answers given while connecting were never saved: stored first, then those on top.
                    self.qrecs.update(|m| {
                        for (_, doc) in qrecs {
                            let mem = m.remove(&doc.lesson).unwrap_or_default();
                            let mut merged = doc.questions;
                            for (q, r) in mem {
                                merged.entry(q).or_default().absorb(&r);
                            }
                            m.insert(doc.lesson, merged);
                        }
                    });
                    let fresh = profile.is_none();
                    let mut profile = profile.unwrap_or(Profile { created_at: now(), disclaimer_accepted_at: None, theme: "system".into() });
                    match profile.theme.as_str() {
                        "dark" => theme.set(ThemeChoice::Dark),
                        "light" => theme.set(ThemeChoice::Light),
                        _ => {}
                    }
                    let accepted_here = !disclaimer.0.get_untracked();
                    if profile.disclaimer_accepted_at.is_some() {
                        disclaimer.0.set(false);
                    } else if accepted_here {
                        profile.disclaimer_accepted_at = Some(now());
                    }
                    self.profile.set(Some(profile.clone()));
                    self.state.set(SaveState::Saved);
                    if fresh || accepted_here {
                        if let Err(e) = save_doc("profile", kind::PROFILE, &profile).await {
                            self.report(e);
                        }
                    }
                    for id in dirty {
                        self.save_lesson(progress, id);
                    }
                    self.flush_qrecs();
                }
                Err(e) => self.report(e),
            }
        });
    }

    /// Saves one lesson, merged with whatever is stored, so a save that raced the load never loses progress.
    pub fn save_lesson(self, progress: Progress, id: LessonId) {
        if !self.enabled() {
            return;
        }
        self.state.set(SaveState::Saving);
        spawn_local(async move {
            let doc = id.doc_id();
            let result = async {
                let stored = load_doc::<LessonProgress>(&doc).await?.unwrap_or_default();
                let merged = progress.get_untracked(id).merge(&stored);
                save_doc(&doc, kind::PROGRESS, &merged).await?;
                Ok::<_, PlatformError>(merged)
            }
            .await;
            match result {
                Ok(merged) => {
                    progress.update(id, |p| *p = merged);
                    self.state.set(SaveState::Saved);
                }
                Err(e) => self.report(e),
            }
        });
    }

    pub fn update_profile(self, f: impl FnOnce(&mut Profile) + 'static) {
        if !self.enabled() {
            return;
        }
        let Some(mut p) = self.profile.get_untracked() else { return };
        let before = p.clone();
        f(&mut p);
        if p == before {
            return;
        }
        self.profile.set(Some(p.clone()));
        self.state.set(SaveState::Saving);
        spawn_local(async move {
            match save_doc("profile", kind::PROFILE, &p).await {
                Ok(()) => self.state.set(SaveState::Saved),
                Err(e) => self.report(e),
            }
        });
    }

    pub fn save_attempt(self, a: Attempt) {
        if !self.enabled() {
            return;
        }
        spawn_local(async move {
            if let Err(e) = repo::save_attempt(&a).await {
                self.report(e);
            }
        });
    }

    /// Runs one save in the background when saving is on; reports failures in the chip and a toast.
    pub fn persist(self, fut: impl std::future::Future<Output = Result<(), PlatformError>> + 'static) {
        if !self.enabled() {
            return;
        }
        spawn_local(async move {
            if let Err(e) = fut.await {
                self.report(e);
            }
        });
    }

    pub fn record_drill(self, drill: &'static str, right: bool) {
        self.drills.update(|m| {
            let d = m.entry(drill.to_string()).or_default();
            d.answered += 1;
            d.correct += right as u32;
            d.last_at = now();
        });
        if !self.enabled() {
            return;
        }
        let rec = self.drills.with_untracked(|m| m.get(drill).cloned().unwrap_or_default());
        spawn_local(async move {
            if let Err(e) = save_doc(&format!("drill-{drill}"), kind::DRILL, &rec).await {
                self.report(e);
            }
        });
    }

    /// Records one answer in memory; `flush_qrecs` saves it (after each quiz and review).
    pub fn record_answer(self, key: &QuestionKey, correct: bool, confidence: Confidence) {
        self.qrecs.update(|m| m.entry(key.lesson).or_default().entry(key.id.clone()).or_default().record(correct, confidence, now()));
        self.qrec_dirty.update(|d| {
            d.insert(key.lesson);
        });
    }

    /// Saves every lesson whose Question Records changed. Kept dirty until storage connects.
    pub fn flush_qrecs(self) {
        if !self.enabled() {
            return;
        }
        let lessons = std::mem::take(&mut *self.qrec_dirty.write());
        for lesson in lessons {
            let doc = QrecDoc { lesson, questions: self.qrecs.with_untracked(|m| m.get(&lesson).cloned().unwrap_or_default()) };
            spawn_local(async move {
                if let Err(e) = save_doc(&qrec_doc_id(lesson), kind::QREC, &doc).await {
                    self.qrec_dirty.update(|d| {
                        d.insert(lesson);
                    });
                    self.report(e);
                }
            });
        }
    }
}
