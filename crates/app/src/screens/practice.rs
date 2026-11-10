//! Practice tab: Chart Drills. Phase 3 ships the "Bullish or bearish?" drill; the rest come in Phase 7.

use crate::screens::shell::{LargeTitle, SubBar};
use crate::state::now;
use crate::ui::chart::CandleChart;
use crate::ui::controls::PILL;
use crate::router::DrillKind;
use gold_core::drills::{Candle, CandleDirection, Rng, check_direction, generate};
use gold_core::stage1_drills::{BidAskAsk, BidAskQ, DistanceQ, OrderKind, OrderQ, PipQ};
use gold_core::money::{DollarMove, Price};
use leptos::prelude::*;

struct DrillInfo {
    kind: DrillKind,
    title: &'static str,
    blurb: &'static str,
    lesson: &'static str,
}

const DRILLS: [DrillInfo; 5] = [
    DrillInfo { kind: DrillKind::Direction, title: "Bullish or bearish?", blurb: "Read a candle's direction from its open and close.", lesson: "Lesson 5" },
    DrillInfo { kind: DrillKind::BidAsk, title: "Spot the spread", blurb: "Find the bid, the ask, and the cost of getting in.", lesson: "Lesson 6" },
    DrillInfo { kind: DrillKind::Distance, title: "Measure the move", blurb: "Turn two prices into a Dollar Move.", lesson: "Lesson 4" },
    DrillInfo { kind: DrillKind::Pips, title: "Translate the pips", blurb: "Convert someone else's “pips” into dollars.", lesson: "Lesson 7" },
    DrillInfo { kind: DrillKind::Orders, title: "Pick the order", blurb: "Market, limit or stop: match the order to the intent.", lesson: "Lesson 10" },
];

#[component]
pub fn Practice() -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title="Practice" />
            <p class="mt-2 max-w-xl text-callout text-text-muted">
                "Quick drills on illustrative prices and charts. They train your eye and never count towards a Gate."
            </p>
            <div class="mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {DRILLS
                    .iter()
                    .map(|d| {
                        let slug = d.kind.slug();
                        view! {
                            <a href=format!("#drill-{slug}") class="press block rounded-card bg-surface p-5 shadow-card hover:bg-raised">
                                <span class="block text-eyebrow uppercase text-text-muted">{d.lesson}</span>
                                <span class="mt-1 block text-headline">{d.title}</span>
                                <span class="mt-1 block text-callout text-text-muted">{d.blurb}</span>
                                <span class="tabular mt-4 block text-footnote font-semibold">
                                    {move || {
                                        store
                                            .drills
                                            .with(|m| m.get(slug).map(|r| (r.correct, r.answered)))
                                            .map_or("Start ›".to_string(), |(c, a)| format!("{c} of {a} right · Continue ›"))
                                    }}
                                </span>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </main>
    }
}

fn money(p: Price) -> String {
    p.to_string().trim_start_matches('$').to_string()
}

struct Choice {
    prompt: String,
    detail: Option<String>,
    options: Vec<String>,
    answer: usize,
    why: String,
}

fn shuffle_choice(mut options: Vec<(String, bool)>, seed: u64) -> (Vec<String>, usize) {
    let mut rng = Rng::new(seed ^ 0x5eed);
    options.dedup_by(|a, b| a.0 == b.0);
    for i in (1..options.len()).rev() {
        options.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let answer = options.iter().position(|o| o.1).unwrap_or(0);
    (options.into_iter().map(|o| o.0).collect(), answer)
}

fn make_choice(kind: DrillKind, seed: u64) -> Choice {
    match kind {
        DrillKind::BidAsk => {
            let q = BidAskQ::new(seed);
            let spread = q.ask - q.bid;
            let (prompt, opts, why) = match q.asked {
                BidAskAsk::Spread => (
                    "What is the spread?".to_string(),
                    vec![(spread.to_string(), true), (DollarMove(spread.0 * 10).to_string(), false), (DollarMove(spread.0 * 100).to_string(), false), (DollarMove((spread.0 / 10).max(1) + spread.0).to_string(), false)],
                    format!("ask − bid = {} − {} = {spread}.", money(q.ask), money(q.bid)),
                ),
                BidAskAsk::BuyPrice => (
                    "You buy now. Which price do you pay?".to_string(),
                    vec![(money(q.ask), true), (money(q.bid), false), (money(Price((q.ask.0 + q.bid.0) / 2)), false)],
                    "You buy at the ask, the higher price.".to_string(),
                ),
                BidAskAsk::SellPrice => (
                    "You sell (go short) now. Which price do you get?".to_string(),
                    vec![(money(q.bid), true), (money(q.ask), false), (money(Price((q.ask.0 + q.bid.0) / 2)), false)],
                    "You sell at the bid, the lower price.".to_string(),
                ),
            };
            let (options, answer) = shuffle_choice(opts, seed);
            Choice { prompt, detail: Some(format!("Bid (sell) {}  ·  Ask (buy) {}", money(q.bid), money(q.ask))), options, answer, why }
        }
        DrillKind::Distance => {
            let q = DistanceQ::new(seed);
            let a = q.answer();
            let way = if q.to > q.from { "up" } else { "down" };
            let (options, answer) = shuffle_choice(
                vec![(a.to_string(), true), (DollarMove(a.0 * 10).to_string(), false), (DollarMove((a.0 / 10).max(1)).to_string(), false), (DollarMove(a.0 + 100).to_string(), false)],
                seed,
            );
            Choice {
                prompt: format!("Gold moves from {} to {}. How big was the move?", money(q.from), money(q.to)),
                detail: None,
                options,
                answer,
                why: format!("{} − {} = {a}, {way}.", money(q.from.max(q.to)), money(q.from.min(q.to))),
            }
        }
        DrillKind::Pips => {
            let q = PipQ::new(seed);
            let size = DollarMove(q.pip_cents);
            let (options, answer) = shuffle_choice(
                [1, 10, 100].into_iter().map(|c| (DollarMove(q.pips * c).to_string(), c == q.pip_cents)).collect(),
                seed,
            );
            Choice {
                prompt: format!("A trader says “a {}-pip stop on gold” and, when you ask, they mean {size} per pip. What Dollar Move is that?", q.pips),
                detail: None,
                options,
                answer,
                why: format!("{} pips × {size} = {}. Always convert before you trust a pip number.", q.pips, q.answer()),
            }
        }
        _ => {
            let q = OrderQ::new(seed);
            let want = match (q.buy, q.target) {
                (true, None) => "buy right now".to_string(),
                (false, None) => "sell right now".to_string(),
                (b, Some(t)) => format!("{} only if price {} {}", if b { "buy" } else { "sell" }, if t > q.current { "rises to" } else { "falls to" }, money(t)),
            };
            let all = [(OrderKind::Market, "Market order"), (OrderKind::BuyLimit, "Buy limit"), (OrderKind::SellLimit, "Sell limit"), (OrderKind::BuyStop, "Buy stop"), (OrderKind::SellStop, "Sell stop")];
            let correct = q.answer();
            let (options, answer) = shuffle_choice(all.iter().map(|(k, n)| (n.to_string(), *k == correct)).collect(), seed);
            let why = match correct {
                OrderKind::Market => "To act now, use a market order.",
                OrderKind::BuyLimit => "Buying below the current price is a buy limit: it waits for a better price.",
                OrderKind::SellLimit => "Selling above the current price is a sell limit: it waits for a better price.",
                OrderKind::BuyStop => "Buying only once price rises through a level is a buy stop.",
                OrderKind::SellStop => "Selling only once price falls through a level is a sell stop.",
            };
            Choice { prompt: format!("Gold is at {}. You want to {want}. Which order?", money(q.current)), detail: None, options, answer, why: why.into() }
        }
    }
}

#[component]
pub fn ChoiceDrill(kind: DrillKind) -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    let info = DRILLS.iter().find(|d| d.kind == kind).expect("known drill");
    let seed = RwSignal::new(now() as u64);
    let choice = Memo::new(move |_| {
        let c = make_choice(kind, seed.get());
        (c.prompt, c.detail, c.options, c.answer, c.why)
    });
    let picked = RwSignal::new(None::<usize>);
    let score = RwSignal::new((0u32, 0u32));
    let pick = move |i: usize| {
        if picked.get_untracked().is_some() {
            return;
        }
        picked.set(Some(i));
        let right = choice.with_untracked(|c| c.3 == i);
        store.record_drill(kind.slug(), right);
        score.update(|(r, t)| {
            *t += 1;
            *r += right as u32;
        });
    };
    let next = move |_| {
        seed.update(|s| *s = s.wrapping_add(7919));
        picked.set(None);
    };
    view! {
        <SubBar back="#practice" back_label="Practice" title=info.title />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <div class="flex items-baseline justify-between gap-4">
                <h1 class="text-title1">{info.title}</h1>
                <p class="tabular shrink-0 whitespace-nowrap text-callout text-text-muted">{move || { let (r, t) = score.get(); format!("{r} of {t} right") }}</p>
            </div>
            <p class="mt-2 text-callout text-text-muted">{info.blurb} " Prices are illustrative."</p>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                {move || {
                    let (prompt, detail, options, answer, why) = choice.get();
                    view! {
                        {detail.map(|d| view! { <p class="tabular mb-3 rounded-btn bg-bg p-3 text-headline">{d}</p> })}
                        <h2 class="text-headline">{prompt}</h2>
                        <div class="mt-4 grid gap-2">
                            {options
                                .into_iter()
                                .enumerate()
                                .map(|(i, o)| {
                                    let is_answer = i == answer;
                                    view! {
                                        <button
                                            type="button"
                                            disabled=move || picked.get().is_some()
                                            on:click=move |_| pick(i)
                                            class=move || {
                                                let base = "press tabular flex min-h-12 w-full items-center rounded-btn bg-bg px-4 py-3 text-left text-body";
                                                match picked.get() {
                                                    None => format!("{base} hover:bg-raised"),
                                                    Some(_) if is_answer => format!("{base} ring-2 ring-up"),
                                                    Some(p) if p == i => format!("{base} ring-2 ring-down"),
                                                    Some(_) => format!("{base} opacity-55"),
                                                }
                                            }
                                        >
                                            {o}
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                        <Show when=move || picked.get().is_some()>
                            <div class="anim-fade mt-4 rounded-btn bg-bg p-4" aria-live="polite">
                                {move || if picked.get() == Some(answer) {
                                    view! { <p class="text-headline text-up">"✓ Right"</p> }.into_any()
                                } else {
                                    view! { <p class="text-headline text-down">"✗ Not quite"</p> }.into_any()
                                }}
                                <p class="mt-1 text-callout">{why.clone()}</p>
                            </div>
                            <button type="button" class=format!("{PILL} mt-4") on:click=next>"Next question ›"</button>
                        </Show>
                    }
                }}
            </section>
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
