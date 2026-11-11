//! "Today": an App Store–style card stack, with the next lesson featured on top.

use crate::catalog::{self, SECTIONS};
use crate::router::lesson_href;
use crate::screens::shell::LargeTitle;
use crate::state::Progress;
use crate::ui::controls::PillLink;
use crate::ui::course::StageCard;
use leptos::prelude::*;

fn today_eyebrow() -> String {
    const DAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let d = js_sys::Date::new_0();
    format!("{} {} {}", DAYS[d.get_day() as usize % 7], d.get_date(), MONTHS[d.get_month() as usize % 12])
}

/// Decorative candles for the featured card: drawn, not data, so they carry no label.
fn hero_art() -> impl IntoView {
    let bars = [(18, 46, 30, 40, true), (40, 52, 26, 44, false), (62, 40, 20, 34, true), (84, 30, 12, 26, true), (106, 36, 18, 30, false), (128, 22, 6, 18, true), (150, 14, 2, 10, true)];
    view! {
        <svg viewBox="0 0 170 60" class="h-16 w-auto" aria-hidden="true">
            {bars
                .into_iter()
                .map(|(x, low, high, top, up)| {
                    let body_top = top - 6;
                    view! {
                        <line x1=x x2=x y1=high y2=low class="stroke-on-fill/50" stroke-width="2" stroke-linecap="round" />
                        <rect
                            x=x - 6
                            y=body_top
                            width="12"
                            height="12"
                            rx="2"
                            class={ if up { "fill-fill stroke-on-fill" } else { "fill-on-fill stroke-on-fill" } }
                            stroke-width="2"
                        />
                    }
                })
                .collect_view()}
        </svg>
    }
}

fn minutes_line(id: telosgard_core::course::LessonId) -> String {
    match id {
        telosgard_core::course::LessonId::Numbered(n) => match telosgard_content::lesson(n) {
            Some(l) => format!("About {} minutes, step by step, then a short quiz and an Explain-Back.", l.read_minutes),
            None => "Written when you get close to it, using what worked for you so far.".into(),
        },
        _ => "A short lesson, then a quiz and an Explain-Back.".into(),
    }
}

/// Lessons passed across the core course and in the current stage, counting up on arrival.
#[component]
fn ProgressCard() -> impl IntoView {
    use crate::ui::moments::CountUp;
    let progress = expect_context::<Progress>();
    move || {
        let core: Vec<_> = (1..=telosgard_core::course::LAST_CORE).map(telosgard_core::course::LessonId::Numbered).collect();
        let passed = core.iter().filter(|l| progress.passed(**l)).count() as i64;
        let total = core.len() as i64;
        let focus = progress.up_next().unwrap_or(telosgard_core::course::LessonId::Numbered(1));
        let stage = SECTIONS.iter().find(|s| s.lessons.iter().any(|(l, _)| *l == focus)).unwrap_or(&SECTIONS[0]);
        let in_stage = stage.lessons.iter().filter(|(l, _)| progress.passed(*l)).count();
        let pct = if total == 0 { 0 } else { passed * 100 / total };
        view! {
            <section class="rounded-card bg-surface p-5 shadow-card" aria-label="Your progress">
                <p class="text-eyebrow text-eyebrow-ink">"Your progress"</p>
                <p class="tabular mt-1 font-display text-title1"><CountUp value=passed fmt=|v| v.to_string() /><span class="text-title2 text-text-muted">{format!(" / {total}")}</span></p>
                <p class="text-callout text-text-muted">"core lessons passed"</p>
                <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-bg" aria-hidden="true">
                    <div class="h-full rounded-full bg-gold-fill transition-[width] duration-700" style=format!("width:{pct}%")></div>
                </div>
                <p class="mt-3 text-footnote text-text-muted">{format!("{} · {} · {in_stage} of {} passed", stage.eyebrow, stage.title, stage.lessons.len())}</p>
            </section>
        }
    }
}

/// Where the Learner's progress is going. Content never depends on this; only saving does.
#[component]
fn SaveStatus() -> impl IntoView {
    use crate::store::{LocalReason, SaveState, Store};
    let store = expect_context::<Store>();
    move || {
        let (text, tone) = match store.state.get() {
            SaveState::Connecting => ("Connecting…", "text-text-muted"),
            SaveState::Saving => ("Saving…", "text-text-muted"),
            SaveState::Saved => ("✓ Progress saved to your account", "text-text-muted"),
            SaveState::Local(LocalReason::SignedOut) => ("Not saving: sign in to keep your progress", "text-text"),
            SaveState::Local(LocalReason::NoStorage) => ("Not saving in this view · lessons still work", "text-text"),
            SaveState::Failed => ("⚠ Couldn't save just now · will retry on your next change", "text-down"),
        };
        view! {
            <p role="status" class=format!("mt-2 inline-block rounded-full border border-line/50 px-3 py-1 text-footnote {tone}")>
                {text}
            </p>
        }
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let progress = expect_context::<Progress>();
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title="Today," accent="one step at a time." eyebrow=today_eyebrow() />
            <SaveStatus />

            <div class="mt-6 grid items-start gap-5 lg:grid-cols-[minmax(0,1.7fr)_minmax(0,1fr)]">
            {move || {
                progress
                    .up_next()
                    .map(|id| {
                        view! {
                            <article class="overflow-hidden rounded-card bg-fill text-on-fill">
                                <div class="flex flex-wrap items-end justify-between gap-4 p-6 pb-5">
                                    <div class="min-w-0 max-w-md">
                                        <p class="text-eyebrow opacity-80">"Up next · " {catalog::eyebrow(id)}</p>
                                        <h2 class="mt-2 text-title1">{catalog::title(id)}</h2>
                                        <p class="mt-2 text-callout opacity-85">{minutes_line(id)}</p>
                                    </div>
                                    <div class="opacity-90">{hero_art()}</div>
                                </div>
                                <div class="px-6 pb-6">
                                    <a href=lesson_href(id) class="press inline-flex min-h-11 items-center rounded-full bg-accent px-6 text-callout font-semibold text-on-accent">
                                        "Start lesson ›"
                                    </a>
                                </div>
                            </article>
                        }
                    })
            }}

            <div class="grid gap-5 md:grid-cols-2 lg:grid-cols-1">
                <ProgressCard />
                <crate::ui::clock::SessionClock />
            </div>
            </div>

            <h2 class="mt-9 text-title2">"Your course"</h2>
            <div class="mt-3 grid items-start gap-5 md:grid-cols-2">
                {SECTIONS.iter().map(|s| view! { <StageCard section=s /> }).collect_view()}
            </div>

            <section class="mt-9 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow text-eyebrow-ink">"Your setup"</p>
                <h2 class="mt-1 text-headline">"TradingView for charts, MT5 demo for trades"</h2>
                <p class="mt-1 text-callout text-text-muted">
                    "You won't need either until Stage 3. When you do, the lesson walks you through it."
                </p>
                <div class="mt-4 flex flex-wrap gap-3">
                    <PillLink href="#size">"Try the position-size calculator"</PillLink>
                </div>
            </section>
        </main>
    }
}
