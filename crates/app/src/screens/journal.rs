//! Journal tab and the Live Readiness dashboard. Phase 7 adds entries; Phase 3 shows the empty states.

use crate::screens::shell::{LargeTitle, SubBar};
use crate::ui::chart::Ring;
use gold_core::readiness::{REQUIRED_ADHERENCE, REQUIRED_DAYS, REQUIRED_TRADES};
use leptos::prelude::*;

#[component]
pub fn Journal() -> impl IntoView {
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title="Journal" />
            <a href="#readiness" class="press mt-6 block rounded-card bg-surface p-5 shadow-card hover:bg-raised">
                <div class="flex items-center justify-between gap-3">
                    <div>
                        <p class="text-eyebrow uppercase text-text-muted">"Live Readiness"</p>
                        <p class="mt-1 text-headline">"0 of " {REQUIRED_TRADES} " trades counted"</p>
                    </div>
                    <span class="text-title2 text-text-muted" aria-hidden="true">"›"</span>
                </div>
                <div class="mt-4 h-1.5 overflow-hidden rounded-full bg-bg"><div class="h-full w-0 rounded-full bg-text-muted"></div></div>
            </a>
            <section class="mt-5 rounded-card bg-surface p-8 text-center shadow-card">
                <p class="text-title2">"No trades yet"</p>
                <p class="mx-auto mt-2 max-w-md text-callout text-text-muted">
                    "You'll journal your first Demo Trade in Lesson 25. Each entry is checked against your Trading Plan."
                </p>
            </section>
        </main>
    }
}

#[component]
pub fn Readiness() -> impl IntoView {
    let conditions = [
        format!("{REQUIRED_TRADES} journaled Demo Trades with a final verdict"),
        format!("At least {REQUIRED_DAYS} days from your first counted trade to your latest"),
        format!("At least {REQUIRED_ADHERENCE} of them followed your plan"),
        "No breach of your plan's max total loss. A breach starts a new run with a fresh demo balance.".to_string(),
    ];
    view! {
        <SubBar back="#journal" back_label="Journal" title="Live Readiness" />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <h1 class="text-title1">"Live Readiness"</h1>
            <p class="mt-2 text-callout text-text-muted">
                "Discipline, not profit, unlocks the Live and Advanced Tracks. Losing trades that follow your plan still count."
            </p>
            <section class="mt-6 grid grid-cols-3 gap-2 rounded-card bg-surface p-5 shadow-card">
                <Ring value=0.0 caption=format!("0/{REQUIRED_TRADES}") label="Trades" />
                <Ring value=0.0 caption="—".to_string() label="Followed plan" />
                <Ring value=0.0 caption=format!("0/{REQUIRED_DAYS}") label="Days" />
            </section>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow uppercase text-text-muted">"Max total loss"</p>
                <p class="tabular mt-1 text-headline">"$0.00 drawdown"</p>
                <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-bg"><div class="h-full w-0 rounded-full bg-down"></div></div>
                <p class="mt-2 text-footnote text-text-muted">"Set when you write your Trading Plan (Lesson 24)."</p>
            </section>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">"All four, at the same time"</h2>
                <ol class="mt-3 grid gap-3">
                    {conditions
                        .into_iter()
                        .enumerate()
                        .map(|(i, c)| {
                            view! {
                                <li class="flex gap-3 text-callout">
                                    <span class="tabular grid size-6 shrink-0 place-items-center rounded-full bg-bg text-footnote font-semibold">{i + 1}</span>
                                    <span>{c}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </section>
        </main>
    }
}
