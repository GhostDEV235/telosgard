//! Practice tab: Chart Drills. Phase 3 ships the "Bullish or bearish?" drill; the rest come in Phase 7.

use crate::screens::shell::{LargeTitle, SubBar};
use crate::state::now;
use crate::ui::chart::CandleChart;
use crate::ui::controls::PILL;
use gold_core::drills::{Candle, CandleDirection, check_direction, generate};
use gold_core::money::{DollarMove, Price};
use leptos::prelude::*;

#[component]
pub fn Practice() -> impl IntoView {
    let drills = [
        ("#drill-direction", "Bullish or bearish?", "Read a candle's direction from its open and close.", true),
        ("", "Spot the spread", "Find the bid and ask, and the cost of getting in.", false),
        ("", "Measure the move", "Turn two prices into a Dollar Move.", false),
    ];
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title="Practice" />
            <p class="mt-2 max-w-xl text-callout text-text-muted">
                "Quick drills on illustrative charts. They train your eye and never count towards a Gate."
            </p>
            <div class="mt-6 grid gap-4 md:grid-cols-3">
                {drills
                    .into_iter()
                    .map(|(href, title, blurb, ready)| {
                        let inner = view! {
                            <span class="block text-headline">{title}</span>
                            <span class="mt-1 block text-callout text-text-muted">{blurb}</span>
                            <span class="mt-4 block text-footnote font-semibold">{if ready { "Start ›" } else { "Coming soon" }}</span>
                        };
                        if ready {
                            view! { <a href=href class="press block rounded-card bg-surface p-5 shadow-card hover:bg-raised">{inner}</a> }.into_any()
                        } else {
                            view! { <div class="block rounded-card bg-locked p-5" aria-disabled="true">{inner}</div> }.into_any()
                        }
                    })
                    .collect_view()}
            </div>
        </main>
    }
}

/// A chart whose last candle is clearly bullish or bearish (dojis are skipped for this drill).
fn round(seed: u64) -> Vec<Candle> {
    (seed..)
        .map(|s| generate(s, 24, Price(235_000), DollarMove(600), DollarMove(350)))
        .find(|cs| cs.last().is_some_and(|c| c.direction() != CandleDirection::Doji))
        .expect("a non-doji chart exists within a few seeds")
}

#[component]
pub fn DirectionDrill() -> impl IntoView {
    let seed = RwSignal::new(now() as u64);
    let candles = Memo::new(move |_| round(seed.get()));
    let answer = RwSignal::new(None::<CandleDirection>);
    let score = RwSignal::new((0u32, 0u32));
    let store = expect_context::<crate::store::Store>();
    let last = move || candles.with(|cs| *cs.last().expect("non-empty"));

    let pick = move |dir: CandleDirection| {
        if answer.get_untracked().is_some() {
            return;
        }
        answer.set(Some(dir));
        let right = check_direction(&last(), dir);
        store.record_drill("direction", right);
        score.update(|(r, t)| {
            *t += 1;
            if right {
                *r += 1;
            }
        });
    };
    let next = move |_| {
        seed.update(|s| *s = s.wrapping_add(7919));
        answer.set(None);
    };

    let choice = move |dir: CandleDirection, label: &'static str, glyph: &'static str, tone: &'static str| {
        view! {
            <button
                type="button"
                disabled=move || answer.get().is_some()
                on:click=move |_| pick(dir)
                class=move || {
                    let ring = match answer.get() {
                        Some(_) if last().direction() == dir => "ring-2 ring-up",
                        Some(a) if a == dir => "ring-2 ring-down",
                        Some(_) => "opacity-55",
                        None => "",
                    };
                    format!("press flex min-h-14 flex-1 items-center justify-center gap-2 rounded-btn bg-surface shadow-card text-headline {ring}")
                }
            >
                <span class=tone aria-hidden="true">{glyph}</span>
                {label}
            </button>
        }
    };

    view! {
        <SubBar back="#practice" back_label="Practice" title="Bullish or bearish?" />
        <main class="gutter mx-auto max-w-[760px] pt-6 pb-10">
            <div class="flex items-baseline justify-between gap-4">
                <h1 class="text-title1">"Bullish or bearish?"</h1>
                <p class="tabular shrink-0 whitespace-nowrap text-callout text-text-muted">{move || { let (r, t) = score.get(); format!("{r} of {t} right") }}</p>
            </div>
            <p class="mt-2 text-callout text-text-muted">
                "Look at the highlighted candle. Did buyers or sellers win it? Hollow candles closed higher; filled candles closed lower."
            </p>
            {move || {
                store
                    .drills
                    .with(|m| m.get("direction").cloned())
                    .filter(|d| d.answered > 0)
                    .map(|d| view! { <p class="tabular mt-1 text-footnote text-text-muted">{format!("All time: {} of {} right", d.correct, d.answered)}</p> })
            }}
            <section class="mt-5 rounded-card bg-surface p-4 shadow-card md:p-5">
                <CandleChart
                    candles=Signal::derive(move || candles.get())
                    highlight=Signal::derive(move || Some(candles.with(|c| c.len() - 1)))
                    label="Illustrative candlestick chart with the last candle highlighted"
                />
            </section>
            <div class="mt-5 flex gap-3">
                {choice(CandleDirection::Bullish, "Bullish", "▲", "text-up")}
                {choice(CandleDirection::Bearish, "Bearish", "▼", "text-down")}
            </div>
            {move || {
                answer
                    .get()
                    .map(|a| {
                        let c = last();
                        let moved = (c.close - c.open).abs();
                        let truth = c.direction();
                        let (verdict, tone) = if a == truth { ("✓ Right", "text-up") } else { ("✗ Not quite", "text-down") };
                        let explain = match truth {
                            CandleDirection::Bullish => format!("It closed {moved} above where it opened, so buyers won that candle."),
                            _ => format!("It closed {moved} below where it opened, so sellers won that candle."),
                        };
                        view! {
                            <div class="anim-fade mt-5 rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                                <p class=format!("text-headline {tone}")>{verdict}</p>
                                <p class="mt-1 text-callout">{explain}</p>
                                <button type="button" class=format!("{PILL} mt-4") on:click=next>"Next chart ›"</button>
                            </div>
                        }
                    })
            }}
        </main>
    }
}
