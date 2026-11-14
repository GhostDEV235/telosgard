//! App-wide state. Phase 3 keeps progress in memory; Phase 4 swaps in the Artifact database.

use telosgard_core::course::LessonId;
use telosgard_core::gate::LessonProgress;
use leptos::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub fn now() -> i64 {
    js_sys::Date::now() as i64
}

#[derive(Clone, Copy)]
pub struct Progress {
    lessons: RwSignal<HashMap<LessonId, LessonProgress>>,
    pub readiness: RwSignal<bool>,
    /// A reviewer account: every lesson is open. Progress is still recorded as usual.
    pub full_access: RwSignal<bool>,
    /// Hidden Lessons (CONTEXT.md): anyone who reaches one waits at a "being updated" card.
    pub hidden: RwSignal<std::collections::HashSet<LessonId>>,
    /// Staff preview Hidden Lessons instead of waiting.
    pub staff: RwSignal<bool>,
}

impl Progress {
    pub fn new() -> Progress {
        Progress { lessons: RwSignal::new(HashMap::new()), readiness: RwSignal::new(false), full_access: RwSignal::new(false), hidden: RwSignal::new(Default::default()), staff: RwSignal::new(false) }
    }
    pub fn get(&self, id: LessonId) -> LessonProgress {
        self.lessons.with(|m| m.get(&id).cloned().unwrap_or_default())
    }
    pub fn get_untracked(&self, id: LessonId) -> LessonProgress {
        self.lessons.with_untracked(|m| m.get(&id).cloned().unwrap_or_default())
    }
    pub fn entries_untracked(&self) -> Vec<(LessonId, LessonProgress)> {
        self.lessons.with_untracked(|m| m.iter().map(|(k, v)| (*k, v.clone())).collect())
    }
    pub fn is_hidden(&self, id: LessonId) -> bool {
        self.hidden.with(|h| h.contains(&id))
    }
    pub fn passed(&self, id: LessonId) -> bool {
        self.lessons.with(|m| m.get(&id).is_some_and(|p| p.is_passed()))
    }
    pub fn unlocked(&self, id: LessonId) -> bool {
        if self.full_access.get() {
            return true;
        }
        let ready = self.readiness.get();
        self.lessons.with(|m| id.is_unlocked(|x| m.get(&x).is_some_and(|p| p.is_passed()), ready))
    }
    pub fn update(&self, id: LessonId, f: impl FnOnce(&mut LessonProgress)) {
        self.lessons.update(|m| f(m.entry(id).or_default()));
    }
    /// The first Core lesson that is unlocked but not passed.
    pub fn up_next(&self) -> Option<LessonId> {
        (1..=telosgard_core::course::LAST_CORE).map(LessonId::Numbered).find(|id| self.unlocked(*id) && !self.passed(*id))
    }
}

#[derive(Clone, Copy)]
pub struct Toasts(pub RwSignal<Option<(i64, String)>>);

impl Toasts {
    pub fn new() -> Toasts {
        Toasts(RwSignal::new(None))
    }
    pub fn show(&self, msg: impl Into<String>) {
        let id = now();
        let sig = self.0;
        sig.set(Some((id, msg.into())));
        set_timeout(
            move || {
                if sig.with_untracked(|t| t.as_ref().is_some_and(|(i, _)| *i == id)) {
                    sig.set(None);
                }
            },
            Duration::from_millis(2800),
        );
    }
}

/// What the server says about the whole site (D3): Maintenance Mode and the current Disclaimer.
#[derive(Clone, Copy)]
pub struct SiteNotices {
    pub maintenance: RwSignal<Option<crate::platform::MaintenanceNotice>>,
    pub disclaimer: RwSignal<Option<crate::platform::ServerDisclaimer>>,
}

impl SiteNotices {
    pub fn new() -> Self {
        SiteNotices { maintenance: RwSignal::new(None), disclaimer: RwSignal::new(None) }
    }
    /// The Disclaimer version on screen: the server's, or 1 (the built-in text) before it answers.
    pub fn disclaimer_version(&self) -> u32 {
        self.disclaimer.with_untracked(|d| d.as_ref().map_or(1, |d| d.version))
    }
}

/// Whether the first-launch disclaimer sheet is showing.
#[derive(Clone, Copy)]
pub struct DisclaimerOpen(pub RwSignal<bool>);
