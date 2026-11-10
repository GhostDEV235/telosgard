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
            SaveState::Local(LocalReason::SignedOut) => ("Not saving: sign in to claude.ai to keep your progress", "text-text"),
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
            <LargeTitle title="Today" eyebrow=today_eyebrow() />
            <SaveStatus />

            {move || {
                progress
                    .up_next()
                    .map(|id| {
                        view! {
                            <article class="mt-6 overflow-hidden rounded-card bg-fill text-on-fill">
                                <div class="flex flex-wrap items-end justify-between gap-4 p-6 pb-5">
                                    <div class="min-w-0 max-w-md">
                                        <p class="text-eyebrow uppercase opacity-80">"Up next · " {catalog::eyebrow(id)}</p>
                                        <h2 class="mt-2 text-title1">{catalog::title(id)}</h2>
                                        <p class="mt-2 text-callout opacity-85">"About 5–8 minutes of reading, then a short quiz and an Explain-Back."</p>
                                    </div>
                                    <div class="opacity-90">{hero_art()}</div>
                                </div>
                                <div class="px-6 pb-6">
                                    <a href=lesson_href(id) class="press inline-flex min-h-11 items-center rounded-full bg-on-fill px-6 text-headline text-fill">
                                        "Start lesson ›"
                                    </a>
                                </div>
                            </article>
                        }
                    })
            }}

            <h2 class="mt-9 text-title2">"Your course"</h2>
            <div class="mt-3 grid items-start gap-5 md:grid-cols-2">
                {SECTIONS.iter().map(|s| view! { <StageCard section=s /> }).collect_view()}
            </div>

            <section class="mt-9 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow uppercase text-text-muted">"Your setup"</p>
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
