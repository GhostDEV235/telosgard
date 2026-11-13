//! Dashboard pages. D0 ships the Overview's first numbers and the audit log; the rest say which
//! phase brings them (docs/IMPLEMENTATION-PLAN.md).

use crate::api::{self, Me};
use crate::shell::{Account, Section};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde_json::{json, Value};
use telosgard_ui::controls::PILL_GHOST;

/// The page pattern: a serif title, at most one primary action, then the content.
#[component]
fn Title(eyebrow: &'static str, title: &'static str, #[prop(optional)] lead: Option<&'static str>) -> impl IntoView {
    view! {
        <header class="pt-4 pb-6">
            <p class="text-eyebrow text-eyebrow-ink">{eyebrow}</p>
            <h1 class="text-large">{title}</h1>
            {lead.map(|l| view! { <p class="mt-2 max-w-[62ch] text-callout text-text-muted">{l}</p> })}
        </header>
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Numbers {
    learners: u32,
    new_this_week: u32,
    weekly_active: u32,
    events_today: u32,
}

#[component]
fn Stat(label: &'static str, value: Signal<Option<u32>>, note: &'static str) -> impl IntoView {
    view! {
        <div class="rounded-card bg-surface p-5 shadow-card">
            <p class="text-footnote font-semibold text-text-muted">{label}</p>
            <p class="tabular mt-1 font-display text-title1">{move || value.get().map(|v| v.to_string()).unwrap_or_else(|| "–".into())}</p>
            <p class="mt-1 text-footnote text-text-muted">{note}</p>
        </div>
    }
}

#[component]
pub fn Overview(me: Me) -> impl IntoView {
    let numbers = RwSignal::new(None::<Numbers>);
    let err = RwSignal::new(None::<String>);
    let load = move || {
        spawn_local(async move {
            let r = api::get("/api/admin/overview").await;
            match r.parse::<Numbers>() {
                Some(n) if r.ok() => numbers.set(Some(n)),
                _ => err.set(Some(r.message())),
            }
        });
    };
    load();
    let pick = move |f: fn(&Numbers) -> u32| Signal::derive(move || numbers.get().as_ref().map(f));
    let rebuilding = RwSignal::new(false);
    let rebuilt = RwSignal::new(None::<String>);
    let rebuild = move |_| {
        rebuilding.set(true);
        spawn_local(async move {
            let r = api::post("/api/admin/events/backfill", json!({})).await;
            rebuilding.set(false);
            rebuilt.set(Some(if r.ok() {
                let n = r.body.get("events").and_then(Value::as_u64).unwrap_or(0);
                let u = r.body.get("users").and_then(Value::as_u64).unwrap_or(0);
                format!("Rebuilt {n} past events for {u} accounts.")
            } else {
                r.message()
            }));
            load();
        });
    };
    view! {
        <Title eyebrow="Today" title="Overview" lead="How many people are learning, and how active they are. Activation, the At-Risk list, the Stage funnel and the hardest lessons arrive in phase D2." />
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}
        <section aria-label="Key numbers" class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <Stat label="Learners" value=pick(|n| n.learners) note="Accounts, not counting staff" />
            <Stat label="New this week" value=pick(|n| n.new_this_week) note="Joined in the last 7 days" />
            <Stat label="Weekly active" value=pick(|n| n.weekly_active) note="Any lesson, quiz, drill or journal activity in 7 days" />
            <Stat label="Events today" value=pick(|n| n.events_today) note="Everything recorded in the last 24 hours" />
        </section>
        {me.can("events.backfill").then(|| view! {
            <section class="mt-8 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">"History before the dashboard"</h2>
                <p class="mt-1 max-w-[62ch] text-callout text-text-muted">"Events are recorded from now on. This rebuilds what it can from earlier saves: sign-ups, lessons passed, Explain-Backs and quiz answers, dated approximately. Safe to run again; it replaces its own earlier result."</p>
                <div class="mt-4 flex flex-wrap items-center gap-3">
                    <button type="button" class=PILL_GHOST disabled=move || rebuilding.get() on:click=rebuild>
                        {move || if rebuilding.get() { "Rebuilding…" } else { "Rebuild past history" }}
                    </button>
                    {move || rebuilt.get().map(|m| view! { <p role="status" class="text-callout text-text-muted">{m}</p> })}
                </div>
            </section>
        })}
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuditRow {
    id: u64,
    actor_email: Option<String>,
    actor_role: String,
    action: String,
    entity: String,
    entity_id: Option<String>,
    before: Value,
    after: Value,
    at: f64,
}

#[component]
pub fn Audit() -> impl IntoView {
    let rows = RwSignal::new(Vec::<AuditRow>::new());
    let done = RwSignal::new(false);
    let loading = RwSignal::new(false);
    let err = RwSignal::new(None::<String>);
    let more = move || {
        loading.set(true);
        spawn_local(async move {
            let before = rows.with_untracked(|r| r.last().map(|x| x.id));
            let path = match before {
                Some(id) => format!("/api/admin/audit?limit=50&before={id}"),
                None => "/api/admin/audit?limit=50".into(),
            };
            let r = api::get(&path).await;
            loading.set(false);
            match r.parse::<Vec<AuditRow>>() {
                Some(page) if r.ok() => {
                    done.set(page.len() < 50);
                    rows.update(|v| v.extend(page));
                }
                _ => err.set(Some(r.message())),
            }
        });
    };
    more();
    view! {
        <Title eyebrow="Platform" title="Audit log" lead="Every change made from the dashboard: who, what, to which record, and the values before and after. Newest first." />
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}
        <section class="overflow-hidden rounded-card bg-surface shadow-card">
            {move || {
                let list = rows.get();
                if list.is_empty() && !loading.get() {
                    view! { <p class="p-5 text-callout text-text-muted">"Nothing yet. Changes made from the dashboard will appear here."</p> }.into_any()
                } else {
                    view! {
                        <ul class="divide-y divide-line/40">
                            {list.into_iter().map(|r| {
                                let target = match &r.entity_id { Some(id) => format!("{} · {id}", r.entity), None => r.entity.clone() };
                                let has_values = !r.before.is_null() || !r.after.is_null();
                                view! {
                                    <li class="px-5 py-3">
                                        <div class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
                                            <p class="font-mono text-callout font-semibold">{r.action.clone()}</p>
                                            <p class="tabular text-footnote text-text-muted">{api::when(r.at)}</p>
                                        </div>
                                        <p class="mt-0.5 min-w-0 break-words text-footnote text-text-muted">
                                            {format!("{} ({}) · {target}", r.actor_email.clone().unwrap_or_else(|| "unknown".into()), r.actor_role)}
                                        </p>
                                        {has_values.then(|| view! {
                                            <details class="mt-2">
                                                <summary class="press min-h-8 cursor-pointer text-footnote underline underline-offset-4">"Before and after"</summary>
                                                <div class="mt-2 grid gap-2 md:grid-cols-2">
                                                    <pre class="overflow-x-auto rounded-btn bg-raised p-3 font-mono text-footnote">{format!("Before\n{}", serde_json::to_string_pretty(&r.before).unwrap_or_default())}</pre>
                                                    <pre class="overflow-x-auto rounded-btn bg-raised p-3 font-mono text-footnote">{format!("After\n{}", serde_json::to_string_pretty(&r.after).unwrap_or_default())}</pre>
                                                </div>
                                            </details>
                                        })}
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }
            }}
        </section>
        <div class="mt-4">
            {move || (!done.get() && !rows.get().is_empty()).then(|| view! {
                <button type="button" class=PILL_GHOST disabled=move || loading.get() on:click=move |_| more()>"Load older entries"</button>
            })}
        </div>
    }
}

/// A section whose phase hasn't shipped yet.
#[component]
pub fn Coming(section: Section) -> impl IntoView {
    let (phase, what) = match section {
        Section::Learners => ("D1", "Search every Learner; open a profile with progress, quiz and Explain-Back history; reset a lesson, suspend, turn on Full Access, send a password-reset link, or delete an account."),
        Section::Enrollment => ("D1", "Single-use Invite Links and shareable Invite Codes with a use limit and expiry, and where each Learner came from."),
        Section::Curriculum => ("D2", "Every lesson's status and pass rates per question, Explain-Back answers without names, notes, and hiding a lesson for rework."),
        Section::Settings => ("D3", "Claude's daily limits, maintenance mode, the disclaimer in English and French, and the status of keys and integrations."),
        _ => ("", ""),
    };
    view! {
        <header class="pt-4 pb-6">
            <p class="text-eyebrow text-eyebrow-ink">{format!("Arrives in phase {phase}")}</p>
            <h1 class="text-large">{section.title()}</h1>
        </header>
        <section class="rounded-card border border-dashed border-line bg-surface/60 p-6">
            <p class="max-w-[62ch] text-body">{what}</p>
        </section>
    }
}

/// Phones: the sections that don't fit the tab bar, and the account.
#[component]
pub fn More(sections: Vec<Section>) -> impl IntoView {
    let me = expect_context::<StoredValue<Me>>().get_value();
    let rest: Vec<Section> = sections.into_iter().filter(|s| !matches!(s, Section::Overview | Section::Learners | Section::Curriculum)).collect();
    view! {
        <header class="pt-4 pb-6"><h1 class="text-large">"More"</h1></header>
        <ul class="overflow-hidden rounded-card bg-surface shadow-card divide-y divide-line/40">
            {rest.into_iter().map(|s| view! {
                <li><a href=format!("#{}", s.slug()) class="press flex min-h-12 items-center justify-between px-5 text-body">{s.title()}<span aria-hidden="true" class="text-text-muted">"›"</span></a></li>
            }).collect_view()}
        </ul>
        <section class="mt-6 rounded-card bg-surface p-4 shadow-card"><Account me=me /></section>
    }
}
