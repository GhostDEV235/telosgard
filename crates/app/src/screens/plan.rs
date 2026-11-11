//! Trading Plan builder (FR-5): Hard Rules + Soft Rules, versioned. Saving always creates a new version.

use crate::journal_data::{JournalData, local_label};
use crate::screens::shell::SubBar;
use crate::state::Toasts;
use crate::ui::controls::{NumberField, PILL, PILL_GHOST};
use gold_core::money::Percent;
use gold_core::plan::{HardRules, PlanError, Session, SoftRules, TradingPlan};
use leptos::prelude::*;

const CLASSIC_SETUP: &str = "Classic Price Action: trade only in the direction of the H4 trend (read from market structure), at a key support or resistance zone marked in my checklist, after a confirmation candle (pin bar or engulfing) closes at that zone.";

fn plan_error(e: PlanError) -> &'static str {
    match e {
        PlanError::MaxRiskOutOfRange => "Max risk per trade must be between 0.01% and 100%.",
        PlanError::MaxTotalLossOutOfRange => "Max total loss must be between 0.01% and 100%.",
        PlanError::MinRewardRiskNotPositive => "Minimum reward-to-risk must be above 0.",
        PlanError::NoSessions => "Choose at least one session.",
        PlanError::NoTradesAllowed => "Allow at least one trade per day.",
        PlanError::DailyLossLimitNotPositive => "The daily loss limit must be above 0R.",
        PlanError::SetupMissing => "Describe your setup.",
    }
}

fn ratio(p: Percent) -> String {
    p.to_string().trim_end_matches('%').to_string()
}

pub fn session_name(s: Session) -> &'static str {
    match s {
        Session::Asian => "Asian",
        Session::London => "London",
        Session::NewYork => "New York",
    }
}

#[component]
fn PlanSummary(plan: TradingPlan) -> impl IntoView {
    let h = &plan.hard;
    let rows = vec![
        ("Max risk per trade", h.max_risk.to_string()),
        ("Max total loss", format!("{} of the run's starting balance", h.max_total_loss)),
        ("Minimum reward-to-risk", format!("{}R", ratio(h.min_reward_risk))),
        ("Sessions", h.sessions.iter().map(|s| session_name(*s)).collect::<Vec<_>>().join(", ")),
        ("Max trades per day", h.max_trades_per_day.to_string()),
        ("Daily loss limit", format!("{}R", ratio(h.daily_loss_limit))),
        ("Stop-loss on every trade", "Always".into()),
        ("No-Trade Windows", "Always: rollover, Sunday open, ±5 min of tier-1 news".into()),
        ("Pre-Session Checklist", "Always: required on any day you trade".into()),
    ];
    let extra = plan.soft.extra.clone();
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card">
            <p class="text-eyebrow text-text-muted">{format!("Version {} · saved {}", plan.version, local_label(plan.created_at))}</p>
            <h2 class="mt-3 text-headline">"Hard Rules · checked by calculation"</h2>
            <dl class="mt-2 divide-y divide-line/30">
                {rows
                    .into_iter()
                    .map(|(k, v)| {
                        view! {
                            <div class="flex flex-wrap justify-between gap-x-4 gap-y-1 py-2.5">
                                <dt class="text-callout text-text-muted">{k}</dt>
                                <dd class="tabular text-callout font-semibold">{v}</dd>
                            </div>
                        }
                    })
                    .collect_view()}
            </dl>
            <h2 class="mt-5 text-headline">"Soft Rules · checked by Claude"</h2>
            <p class="mt-2 text-callout">{plan.soft.setup.clone()}</p>
            <ul class="mt-2 grid gap-1 text-callout">
                {extra.into_iter().map(|r| view! { <li>"• " {r}</li> }).collect_view()}
                <li>"• The trade is one of that day's written Scenarios."</li>
            </ul>
        </section>
    }
}

#[component]
pub fn Plan() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let editing = RwSignal::new(false);

    let max_risk = RwSignal::new("1".to_string());
    let max_loss = RwSignal::new("10".to_string());
    let min_rr = RwSignal::new("1.5".to_string());
    let per_day = RwSignal::new("2".to_string());
    let daily = RwSignal::new("3".to_string());
    let sessions = RwSignal::new(vec![Session::London, Session::NewYork]);
    let setup = RwSignal::new(CLASSIC_SETUP.to_string());
    let extra = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);

    let start_editing = move || {
        if let Some(p) = journal.current_plan() {
            max_risk.set(ratio(p.hard.max_risk));
            max_loss.set(ratio(p.hard.max_total_loss));
            min_rr.set(ratio(p.hard.min_reward_risk));
            per_day.set(p.hard.max_trades_per_day.to_string());
            daily.set(ratio(p.hard.daily_loss_limit));
            sessions.set(p.hard.sessions.clone());
            setup.set(p.soft.setup.clone());
            extra.set(p.soft.extra.join("\n"));
        }
        error.set(None);
        editing.set(true);
    };

    let save = move |_| {
        let pct = |s: RwSignal<String>| Percent::parse(&s.get_untracked());
        let (Some(r), Some(l), Some(rr), Some(d)) = (pct(max_risk), pct(max_loss), pct(min_rr), pct(daily)) else {
            error.set(Some("Check the numbers: use plain values like 1, 10, 1.5 or 3.".into()));
            return;
        };
        let Ok(n) = per_day.get_untracked().trim().parse::<u32>() else {
            error.set(Some("Max trades per day must be a whole number.".into()));
            return;
        };
        let hard = HardRules { max_risk: r, max_total_loss: l, min_reward_risk: rr, sessions: sessions.get_untracked(), max_trades_per_day: n, daily_loss_limit: d };
        let soft = SoftRules {
            setup: setup.get_untracked().trim().to_string(),
            extra: extra.get_untracked().lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect(),
        };
        match journal.save_plan(hard, soft) {
            Ok(p) => {
                toasts.show(format!("Trading Plan v{} saved", p.version));
                editing.set(false);
            }
            Err(e) => error.set(Some(plan_error(e).into())),
        }
    };

    let session_toggle = move |s: Session| {
        view! {
            <button
                type="button"
                role="checkbox"
                aria-checked=move || sessions.with(|v| v.contains(&s)).to_string()
                on:click=move |_| sessions.update(|v| if let Some(i) = v.iter().position(|x| *x == s) { v.remove(i); } else { v.push(s); })
                class=move || {
                    if sessions.with(|v| v.contains(&s)) {
                        "press min-h-11 rounded-full bg-fill px-4 text-callout font-semibold text-on-fill"
                    } else {
                        "press min-h-11 rounded-full border border-line px-4 text-callout text-text-muted"
                    }
                }
            >
                {session_name(s)}
            </button>
        }
    };

    let form = move || {
        view! {
            <section class="grid gap-4 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow text-text-muted">"Hard Rules"</p>
                <div class="grid gap-4 sm:grid-cols-2">
                    <NumberField label="Max risk per trade" suffix="%" value=max_risk />
                    <NumberField label="Max total loss (of the run's start)" suffix="%" value=max_loss />
                    <NumberField label="Minimum reward-to-risk" suffix="R" value=min_rr />
                    <NumberField label="Daily loss limit" suffix="R" value=daily />
                    <NumberField label="Max trades per day" value=per_day />
                </div>
                <div>
                    <p class="text-footnote font-semibold text-text-muted">"Sessions you trade"</p>
                    <div class="mt-2 flex flex-wrap gap-2">
                        {session_toggle(Session::Asian)}
                        {session_toggle(Session::London)}
                        {session_toggle(Session::NewYork)}
                    </div>
                </div>
                <p class="text-footnote text-text-muted">"Always on, not editable: a stop-loss on every trade, the No-Trade Windows, and a Pre-Session Checklist on any day you trade."</p>
                <p class="mt-2 text-eyebrow text-text-muted">"Soft Rules"</p>
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">"Your setup"</span>
                    <textarea rows="4" class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout" prop:value=move || setup.get() on:input=move |ev| setup.set(event_target_value(&ev))></textarea>
                </label>
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">"Any other rules (one per line)"</span>
                    <textarea rows="3" placeholder="e.g. No trading in the 30 minutes before US CPI" class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout placeholder:text-text-muted" prop:value=move || extra.get() on:input=move |ev| extra.set(event_target_value(&ev))></textarea>
                </label>
                {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">"⚠ " {e}</p> })}
                <div class="flex flex-wrap justify-end gap-3">
                    <Show when=move || journal.current_plan().is_some()>
                        <button type="button" class=PILL_GHOST on:click=move |_| editing.set(false)>"Cancel"</button>
                    </Show>
                    <button type="button" class=PILL on:click=save>
                        {move || journal.current_plan().map_or("Save plan v1".to_string(), |p| format!("Save as v{}", p.version + 1))}
                    </button>
                </div>
            </section>
        }
    };

    view! {
        <SubBar back="#size" back_label="Tools" title="Trading Plan" />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">"Trading Plan"</h1>
            <p class="mt-2 text-callout text-text-muted">
                "Your written rules. You'll build it properly at Lesson 39; every Demo Trade is checked against the version in force when you took it."
            </p>
            <div class="mt-6 grid gap-5">
                {move || match (journal.current_plan(), editing.get()) {
                    (Some(p), false) => {
                        let next = p.version + 1;
                        view! {
                            <PlanSummary plan=p />
                            <div>
                                <button type="button" class=PILL_GHOST on:click=move |_| start_editing()>
                                    {format!("Change the plan (creates v{next})")}
                                </button>
                            </div>
                        }
                            .into_any()
                    }
                    _ => form().into_any(),
                }}
                {move || {
                    let older: Vec<TradingPlan> = journal.plans.with(|h| h.versions().iter().rev().skip(1).cloned().collect());
                    (!older.is_empty())
                        .then(|| {
                            view! {
                                <details class="rounded-card bg-surface p-5 shadow-card">
                                    <summary class="press min-h-11 cursor-pointer text-headline">{format!("Earlier versions ({})", older.len())}</summary>
                                    <div class="mt-3 grid gap-4">{older.into_iter().map(|p| view! { <PlanSummary plan=p /> }).collect_view()}</div>
                                </details>
                            }
                        })
                }}
            </div>
        </main>
    }
}
