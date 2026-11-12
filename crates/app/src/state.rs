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
}

impl Progress {
    pub fn new() -> Progress {
        Progress { lessons: RwSignal::new(HashMap::new()), readiness: RwSignal::new(false), full_access: RwSignal::new(false) }
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

/// Whether the first-launch disclaimer sheet is showing.
#[derive(Clone, Copy)]
pub struct DisclaimerOpen(pub RwSignal<bool>);
