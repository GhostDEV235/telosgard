//! Milestone moments and count-ups (Phase 15, DESIGN-SYSTEM §4.2). A Milestone is a learning
//! achievement (CONTEXT): a lesson passed, a stage completed, Live Readiness reached. Never a profit.
//! Moments fire only from the Learner's own action (a quiz or an Explain-Back that passes the Gate,
//! or the readiness write), never when stored progress loads. All motion stops under reduced motion.

use crate::i18n::{t, ts};
use crate::catalog::SECTIONS;
use crate::router::lesson_href;
use crate::state::Progress;
use crate::ui::controls::{PILL, PILL_GHOST};
use telosgard_core::course::LessonId;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigMoment {
    Stage { eyebrow: &'static str, title: &'static str, lessons: usize, next: Option<(LessonId, &'static str)> },
    Readiness,
}

#[derive(Clone, Copy)]
pub struct Moments {
    /// The lesson whose Gate was just passed in this visit: its Gate card plays the moment.
    pub lesson: RwSignal<Option<LessonId>>,
    pub big: RwSignal<Option<BigMoment>>,
}

impl Moments {
    pub fn new() -> Moments {
        Moments { lesson: RwSignal::new(None), big: RwSignal::new(None) }
    }

    /// Call right after the Learner's own action passed `id`'s Gate.
    pub fn lesson_passed(self, progress: Progress, id: LessonId) {
        self.lesson.set(Some(id));
        let Some(i) = SECTIONS.iter().position(|s| s.lessons.iter().any(|(l, _)| *l == id)) else { return };
        let s = &SECTIONS[i];
        if s.lessons.iter().all(|(l, _)| progress.passed(*l)) {
            let next = SECTIONS.get(i + 1).and_then(|n| n.lessons.first().map(|(l, _)| (*l, n.title.get())));
            self.big.set(Some(BigMoment::Stage { eyebrow: s.eyebrow.get(), title: s.title.get(), lessons: s.lessons.len(), next }));
        }
    }

    pub fn readiness(self) {
        self.big.set(Some(BigMoment::Readiness));
    }
}

pub fn reduced_motion() -> bool {
    window().match_media("(prefers-reduced-motion: reduce)").ok().flatten().is_some_and(|m| m.matches())
}

/// The badge that draws itself, with a short burst in greens and gold. Decorative.
#[component]
pub fn Badge(#[prop(default = 56)] size: u32) -> impl IntoView {
    view! {
        <span class="moment-badge relative inline-grid shrink-0 place-items-center" style=format!("width:{size}px;height:{size}px;--r:{}px", size * 9 / 10) aria-hidden="true">
            <span class="moment-burst">{(0..10).map(|i| view! { <i style=format!("--a:{}deg;--d:{}ms", i * 36, (i % 3) * 40)></i> }).collect_view()}</span>
            <svg viewBox="0 0 56 56" class="size-full">
                <circle cx="28" cy="28" r="25" class="moment-ring" fill="none" stroke="currentColor" stroke-width="3" pathLength="100" />
                <path d="M18 29l7 7 13-15" class="moment-tick" fill="none" stroke="currentColor" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" pathLength="100" />
            </svg>
        </span>
    }
}

/// Stage complete and Live Readiness: one full-canvas card, dismissed by the Learner.
#[component]
pub fn BigMomentOverlay() -> impl IntoView {
    let moments = expect_context::<Moments>();
    let close = move || moments.big.set(None);
    let _ = window_event_listener(leptos::ev::keydown, move |e| {
        if e.key() == "Escape" && moments.big.get_untracked().is_some() {
            moments.big.set(None);
        }
    });
    Effect::new(move |_| {
        if moments.big.get().is_some() {
            set_timeout(
                || {
                    if let Some(el) = document().get_element_by_id("moment-primary").and_then(|e| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(e).ok()) {
                        let _ = el.focus();
                    }
                },
                std::time::Duration::from_millis(60),
            );
        }
    });
    move || {
        moments.big.get().map(|m| {
            let (eyebrow, title, body, action): (String, String, String, Option<(String, String)>) = match m {
                BigMoment::Stage { eyebrow, title, lessons, next } => (
                    ts(format!("{eyebrow} complete"), format!("{eyebrow} terminée")),
                    title.to_string(),
                    ts(
                        format!("All {lessons} lessons understood and explained back. That's a real foundation, built one Gate at a time."),
                        format!("Les {lessons} leçons comprises et expliquées. C’est une vraie base, construite une validation à la fois."),
                    ),
                    next.map(|(id, n)| (lesson_href(id), ts(format!("Start {n} ›"), format!("Commencer « {n} » ›")))),
                ),
                BigMoment::Readiness => (
                    t("Live Readiness", "Prêt pour le réel").into(),
                    t("You're ready for the Live Track", "Le Parcours réel t’attend").into(),
                    t("50 disciplined Demo Trades over at least 8 weeks. Profit was never the test: following your plan was. The Live Track is now open.", "50 Trades démo disciplinés sur au moins 8 semaines. Le gain n’a jamais été le critère : suivre ton plan, si. Le Parcours réel est maintenant ouvert.").into(),
                    Some(("#lesson-41".into(), t("Open the Live Track ›", "Ouvrir le Parcours réel ›").into())),
                ),
            };
            view! {
                <div class="fixed inset-0 z-50 grid place-items-center p-6">
                    <div class="anim-fade absolute inset-0 bg-scrim" aria-hidden="true" on:click=move |_| close()></div>
                    <div role="dialog" aria-modal="true" aria-labelledby="moment-h" class="moment-in relative w-full max-w-md rounded-card bg-surface p-8 text-center shadow-sheet">
                        <div class="text-gold"><Badge size=88 /></div>
                        <p class="mt-5 text-eyebrow text-eyebrow-ink">{eyebrow}</p>
                        <h2 id="moment-h" class="mt-1 font-display text-title1">{title}</h2>
                        <p class="mt-3 text-callout text-text-muted">{body}</p>
                        <div class="mt-6 flex flex-wrap justify-center gap-3">
                            {match action {
                                Some((href, label)) => view! {
                                    <button type="button" class=PILL_GHOST on:click=move |_| close()>{t("Close", "Fermer")}</button>
                                    <a id="moment-primary" href=href class=PILL on:click=move |_| close()>{label}</a>
                                }.into_any(),
                                None => view! { <button id="moment-primary" type="button" class=PILL on:click=move |_| close()>{t("Close", "Fermer")}</button> }.into_any(),
                            }}
                        </div>
                    </div>
                </div>
            }
        })
    }
}

/// A number that counts up to its value once, when first shown (~0.8 s, ease-out). The final text
/// is what screen readers get, and what shows at once under reduced motion.
#[component]
pub fn CountUp(value: i64, fmt: fn(i64) -> String) -> impl IntoView {
    let shown = RwSignal::new(if reduced_motion() || value == 0 { value } else { 0 });
    if shown.get_untracked() != value {
        let start = js_sys::Date::now();
        fn tick(start: f64, value: i64, shown: RwSignal<i64>) {
            let t = ((js_sys::Date::now() - start) / 800.0).min(1.0);
            let eased = 1.0 - (1.0 - t).powi(3);
            let v = if t < 1.0 { (value as f64 * eased).round() as i64 } else { value };
            // Gone (the Learner left the screen): stop quietly.
            if shown.try_set(v).is_some() {
                return;
            }
            if t < 1.0 {
                request_animation_frame(move || tick(start, value, shown));
            }
        }
        request_animation_frame(move || tick(start, value, shown));
    }
    view! {
        <span class="sr-only">{fmt(value)}</span>
        <span aria-hidden="true">{move || fmt(shown.get())}</span>
    }
}
