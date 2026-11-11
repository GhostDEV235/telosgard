//! Journal tab (FR-6, FR-11) and the Live Readiness dashboard (FR-7), on real data.

use crate::journal_data::{JournalData, local_label, today};
use crate::screens::shell::{LargeTitle, SubBar};
use crate::state::{Progress, Toasts};
use crate::store::TradeRecord;
use crate::ui::chart::Ring;
use crate::ui::clock::SessionClock;
use crate::ui::controls::{NumberField, PILL, PILL_GHOST, PillLink};
use gold_core::journal::{Adherence, Direction};
use gold_core::money::{Percent, Usd};
use gold_core::readiness::{REQUIRED_ADHERENCE, REQUIRED_DAYS, REQUIRED_TRADES};
use leptos::prelude::*;

pub fn verdict_chip(t: &TradeRecord) -> (&'static str, &'static str) {
    if !t.taken() {
        return ("Skipped", "border border-line text-text-muted");
    }
    match t.adherence {
        Adherence::Pending => ("Awaiting verdict", "border border-line text-text-muted"),
        Adherence::DisputePending { .. } => ("Dispute pending", "border border-line text-text-muted"),
        _ => match t.adherence.final_adherent() {
            Some(true) => ("✓ Followed plan", "border border-up text-up"),
            _ => ("✗ Broke plan", "border border-down text-down"),
        },
    }
}

pub fn pnl_text(p: Usd) -> (String, &'static str) {
    if p.0 >= 0 { (format!("▲ +{p}"), "text-up") } else { (format!("▼ {p}"), "text-down") }
}

#[component]
fn ReadinessCard() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    move || {
        if journal.current_plan().is_none() {
            return view! {
                <a href="#plan" class="press block rounded-card bg-surface p-5 shadow-card hover:bg-raised">
                    <p class="text-eyebrow uppercase text-text-muted">"Live Readiness"</p>
                    <p class="mt-1 text-headline">"Write your Trading Plan first ›"</p>
                    <p class="mt-1 text-callout text-text-muted">"Every Demo Trade is judged against it."</p>
                </a>
            }
            .into_any();
        }
        let (counted, pct) = journal.readiness_progress().map_or((0, 0), |(_, p)| (p.counted, p.counted * 100 / REQUIRED_TRADES));
        view! {
            <a href="#readiness" class="press block rounded-card bg-surface p-5 shadow-card hover:bg-raised">
                <div class="flex items-center justify-between gap-3">
                    <div>
                        <p class="text-eyebrow uppercase text-text-muted">"Live Readiness"</p>
                        <p class="tabular mt-1 text-headline">{format!("{counted} of {REQUIRED_TRADES} trades counted")}</p>
                    </div>
                    <span class="text-title2 text-text-muted" aria-hidden="true">"›"</span>
                </div>
                <div class="mt-4 h-1.5 overflow-hidden rounded-full bg-bg">
                    <div class="h-full rounded-full bg-text-muted" style=format!("width:{}%", pct.min(100))></div>
                </div>
            </a>
        }
        .into_any()
    }
}

#[component]
pub fn Journal() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let day = today();
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title="Journal" />
            <div class="mt-6 grid items-start gap-5 md:grid-cols-2">
                <div class="grid gap-5">
                    <ReadinessCard />
                    <section class="rounded-card bg-surface p-5 shadow-card">
                        <p class="text-eyebrow uppercase text-text-muted">"Today"</p>
                        {move || {
                            if journal.checklist(day).is_some() {
                                view! { <p class="mt-1 text-headline text-up">"✓ Pre-Session Checklist saved"</p> }.into_any()
                            } else {
                                view! { <p class="mt-1 text-headline">"Pre-Session Checklist not done yet"</p> }.into_any()
                            }
                        }}
                        <div class="mt-4 flex flex-wrap gap-3">
                            <PillLink href="#checklist">{move || if journal.checklist(day).is_some() { "Open checklist" } else { "Start checklist" }}</PillLink>
                            <a href="#review" class=PILL_GHOST>"Weekly review"</a>
                        </div>
                    </section>
                </div>
                <SessionClock />
            </div>

            <div class="mt-8 flex flex-wrap items-center justify-between gap-3">
                <h2 class="text-title2">"Entries"</h2>
                <div class="flex flex-wrap gap-3">
                    <a href="#trade-skip" class=PILL_GHOST>"Log a skipped setup"</a>
                    <PillLink href="#trade-new">"Log a trade"</PillLink>
                </div>
            </div>
            {move || {
                let trades = journal.trades.get();
                if trades.is_empty() {
                    return view! {
                        <section class="mt-4 rounded-card bg-surface p-8 text-center shadow-card">
                            <p class="text-title2">"No entries yet"</p>
                            <p class="mx-auto mt-2 max-w-md text-callout text-text-muted">
                                "You'll journal your first Demo Trade in Lesson 40. Skipped setups are worth logging too: they teach discipline."
                            </p>
                        </section>
                    }
                        .into_any();
                }
                view! {
                    <ul class="mt-4 grid gap-2">
                        {trades
                            .into_iter()
                            .map(|(id, t)| {
                                let (chip, chip_class) = verdict_chip(&t);
                                let dir = match t.entry.direction { Direction::Buy => ("▲ Buy", "text-up"), Direction::Sell => ("▼ Sell", "text-down") };
                                let pnl = t.result.as_ref().map(|r| pnl_text(r.pnl));
                                let r_txt = t.r().map(|r| r.to_string());
                                view! {
                                    <li>
                                        <a href=format!("#{id}") class="press flex flex-wrap items-center gap-x-4 gap-y-1 rounded-btn bg-surface px-4 py-3 shadow-card hover:bg-raised">
                                            <span class="tabular w-36 text-callout text-text-muted">{local_label(t.entry.at)}</span>
                                            <span class=format!("w-16 text-callout font-semibold {}", dir.1)>{dir.0}</span>
                                            <span class="tabular text-callout">{t.entry.entry.to_string()}</span>
                                            {pnl.map(|(s, c)| view! { <span class=format!("tabular text-callout font-semibold {c}")>{s}</span> })}
                                            {r_txt.map(|r| view! { <span class="tabular text-callout text-text-muted">{r}</span> })}
                                            <span class=format!("ml-auto rounded-full px-2.5 py-1 text-footnote font-semibold {chip_class}")>{chip}</span>
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                }
                    .into_any()
            }}
        </main>
    }
}

#[component]
pub fn Readiness() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let progress = expect_context::<Progress>();
    let toasts = expect_context::<Toasts>();
    let fresh = RwSignal::new(String::new());

    // Reaching readiness is permanent: write it once.
    Effect::new(move |_| {
        if journal.readiness_progress().is_some_and(|(_, p)| p.reached) {
            journal.mark_ready(progress);
        }
    });

    let conditions = [
        format!("{REQUIRED_TRADES} journaled Demo Trades with a final verdict"),
        format!("At least {REQUIRED_DAYS} days (8 weeks) from your first counted trade to your latest"),
        format!("At least {REQUIRED_ADHERENCE} of them followed your plan"),
        "No breach of your plan's max total loss. A breach ends the run; the next starts from zero with a fresh demo balance.".to_string(),
    ];

    let new_run = move |_| match Usd::parse(&fresh.get_untracked()).filter(|u| u.0 > 0) {
        Some(b) => {
            journal.end_active_run("breach");
            let r = journal.start_run(b);
            toasts.show(format!("Readiness Run {} started", r.run.number));
            fresh.set(String::new());
        }
        None => toasts.show("Enter your fresh demo balance first"),
    };

    let body = move || {
        if journal.current_plan().is_none() {
            return view! {
                <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
                    <p class="text-headline">"No Trading Plan yet"</p>
                    <p class="mt-1 text-callout text-text-muted">"Readiness is measured against your plan's rules, so it starts once you've written one."</p>
                    <div class="mt-4"><PillLink href="#plan">"Write your plan"</PillLink></div>
                </section>
            }
            .into_any();
        }
        let Some((run, p)) = journal.readiness_progress() else {
            return view! {
                <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
                    <p class="text-headline">"Readiness Run 1 starts with your first journaled trade"</p>
                    <p class="mt-1 text-callout text-text-muted">"Its starting balance is the demo balance you enter on that trade."</p>
                </section>
            }
            .into_any();
        };
        let pct_adh = p.adherence.0 as f64 / 10_000.0;
        let drawdown_pct = if p.max_loss.0 > 0 { (p.drawdown.0 * 100 / p.max_loss.0).clamp(0, 100) } else { 0 };
        view! {
            {p.reached.then(|| view! {
                <div class="anim-fade mt-6 flex items-center gap-3 rounded-card bg-gold-fill p-5 text-on-gold" role="status">
                    <span class="text-title1" aria-hidden="true">"🏅"</span>
                    <div>
                        <p class="text-headline">"Live Readiness reached"</p>
                        <p class="text-callout">"The Live Track and the Advanced Track are now open, for good."</p>
                    </div>
                </div>
            })}
            {p.breached.then(|| view! {
                <section class="mt-6 rounded-card border border-down bg-surface p-5 shadow-card" role="alert">
                    <p class="text-headline text-down">{format!("▼ Run {} breached your max total loss", run.run.number)}</p>
                    <p class="mt-1 text-callout">"This run is over. Reset your demo account to a fresh balance, then start the next run. Your lessons stay unlocked."</p>
                    <div class="mt-4 flex flex-wrap items-end gap-3">
                        <div class="w-56"><NumberField label="Fresh demo balance" prefix="$" value=fresh /></div>
                        <button type="button" class=PILL on:click=new_run>{format!("Start Run {}", run.run.number + 1)}</button>
                    </div>
                </section>
            })}
            <p class="mt-6 text-eyebrow uppercase text-text-muted">{format!("Readiness Run {} · started with {}", run.run.number, run.run.starting_balance)}</p>
            <section class="mt-2 grid grid-cols-3 gap-2 rounded-card bg-surface p-5 shadow-card">
                <Ring value=p.counted as f64 / REQUIRED_TRADES as f64 caption=format!("{}/{REQUIRED_TRADES}", p.counted) label="Trades" />
                <Ring value=pct_adh / (REQUIRED_ADHERENCE.0 as f64 / 10_000.0) caption=if p.counted == 0 { "—".to_string() } else { p.adherence.to_string() } label="Followed plan" />
                <Ring value=p.days as f64 / REQUIRED_DAYS as f64 caption=format!("{}/{REQUIRED_DAYS}", p.days) label="Days" />
            </section>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow uppercase text-text-muted">"Max total loss"</p>
                <p class="tabular mt-1 text-headline">{format!("{} drawdown of {} allowed", p.drawdown, p.max_loss)}</p>
                <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-bg">
                    <div class="h-full rounded-full bg-down" style=format!("width:{drawdown_pct}%")></div>
                </div>
                <p class="mt-2 text-footnote text-text-muted">{format!("{} of the run's starting balance. Measured from closed trades.", journal.current_plan().map(|p| p.hard.max_total_loss).unwrap_or(Percent(0)))}</p>
            </section>
        }
        .into_any()
    };

    view! {
        <SubBar back="#journal" back_label="Journal" title="Live Readiness" />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <h1 class="text-title1">"Live Readiness"</h1>
            <p class="mt-2 text-callout text-text-muted">
                "Discipline, not profit, unlocks the Live and Advanced Tracks. Losing trades that follow your plan still count."
            </p>
            {body}
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
