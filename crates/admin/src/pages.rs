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
struct Person {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    last_active: f64,
    #[serde(default)]
    run: u32,
    #[serde(default)]
    entries: u32,
    #[serde(default)]
    judged: u32,
    #[serde(default)]
    adherent: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct Group {
    count: u32,
    list: Vec<Person>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct Activation {
    cohort: u32,
    activated: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct Usage {
    calls: u32,
    errors: u32,
    input: u64,
    output: u64,
    cost: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct Claude {
    today: Usage,
    month: Usage,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Numbers {
    learners: u32,
    new_this_week: u32,
    weekly_active: u32,
    events_today: u32,
    activation: Activation,
    at_risk: Group,
    passed_counts: std::collections::HashMap<String, u32>,
    readiness: Group,
    claude: Claude,
}

#[component]
fn Stat(label: &'static str, #[prop(into)] value: Signal<Option<String>>, #[prop(into)] note: Signal<String>) -> impl IntoView {
    view! {
        <div class="rounded-card bg-surface p-5 shadow-card">
            <p class="text-footnote font-semibold text-text-muted">{label}</p>
            <p class="tabular mt-1 font-display text-title1">{move || value.get().unwrap_or_else(|| "–".into())}</p>
            <p class="mt-1 text-footnote text-text-muted">{move || note.get()}</p>
        </div>
    }
}

fn card_title(eyebrow: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <p class="text-eyebrow text-eyebrow-ink">{eyebrow}</p>
        <h2 class="text-headline">{title}</h2>
    }
}

fn person_name(p: &Person) -> String {
    if p.name.is_empty() { p.email.clone() } else { p.name.clone() }
}

#[component]
pub fn Overview(me: Me) -> impl IntoView {
    let numbers = RwSignal::new(None::<Numbers>);
    let lessons = RwSignal::new(None::<Vec<crate::curriculum::Stats>>);
    let err = RwSignal::new(None::<String>);
    let load = move || {
        spawn_local(async move {
            let r = api::get("/api/admin/overview").await;
            match r.parse::<Numbers>() {
                Some(n) if r.ok() => numbers.set(Some(n)),
                _ => err.set(Some(r.message())),
            }
            if let Ok(v) = crate::curriculum::load_stats().await {
                lessons.set(Some(v));
            }
        });
    };
    load();
    let num = move |f: fn(&Numbers) -> u32| Signal::derive(move || numbers.get().as_ref().map(|n| f(n).to_string()));
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
    let can_people = me.can("learners.read");
    // Stage funnel: Stage 1 starts with every Learner; a later stage is reached by passing the lesson before it.
    let written = telosgard_content::lessons_in(false);
    let mut stage_starts: Vec<(u8, u8)> = Vec::new();
    for l in &written {
        if !stage_starts.iter().any(|(s, _)| *s == l.stage) {
            stage_starts.push((l.stage, l.id));
        }
    }
    view! {
        <Title eyebrow="Today" title="Overview" lead="Who is learning, who needs a nudge, and where the course is hard. Activation and At-Risk follow the definitions in the glossary (48 hours, 7 days)." />
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}
        <section aria-label="Key numbers" class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <Stat label="Learners" value=num(|n| n.learners) note="Accounts, not counting staff".to_string() />
            <Stat label="New this week" value=num(|n| n.new_this_week) note="Joined in the last 7 days".to_string() />
            <Stat label="Weekly active" value=num(|n| n.weekly_active) note="Any lesson, quiz, drill or journal activity in 7 days".to_string() />
            <Stat
                label="Activation"
                value=Signal::derive(move || numbers.get().map(|n| if n.activation.cohort == 0 { "–".to_string() } else { crate::curriculum::pct(n.activation.activated as f64 / n.activation.cohort as f64) }))
                note=Signal::derive(move || numbers.get().map(|n| format!("{} of {} who joined 2–30 days ago passed Lesson 1 within 48 hours", n.activation.activated, n.activation.cohort)).unwrap_or_default())
            />
        </section>

        <div class="mt-5 grid gap-5 lg:grid-cols-2">
            <section class="rounded-card bg-surface p-5 shadow-card">
                {card_title("Reach out", "At-Risk Learners")}
                <p class="mt-1 text-footnote text-text-muted">"Active before, nothing for 7 days or more. A prompt to reach out, not a judgement."</p>
                {move || numbers.get().map(|n| {
                    if n.at_risk.count == 0 {
                        view! { <p class="mt-3 text-callout">"Nobody right now."</p> }.into_any()
                    } else if !can_people {
                        view! { <p class="tabular mt-3 text-callout">{crate::curriculum::plural(n.at_risk.count, "Learner")}</p> }.into_any()
                    } else {
                        view! {
                            <ul class="mt-3 divide-y divide-line/40">
                                {n.at_risk.list.iter().map(|p| view! {
                                    <li><a href=format!("#learner-{}", p.id) class="press flex min-h-11 items-center justify-between gap-3 py-2">
                                        <span class="min-w-0 truncate text-callout font-semibold">{person_name(p)}</span>
                                        <span class="tabular shrink-0 text-footnote text-text-muted">{format!("last active {}", api::ago(p.last_active))}</span>
                                    </a></li>
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }
                })}
            </section>

            <section class="rounded-card bg-surface p-5 shadow-card">
                {card_title("Course", "Stage funnel")}
                <p class="mt-1 text-footnote text-text-muted">"How many Learners reached each written Stage."</p>
                {move || numbers.get().map(|n| {
                    let total = n.learners.max(1) as f64;
                    view! {
                        <ul class="mt-3 grid gap-2">
                            {stage_starts.iter().map(|(stage, first)| {
                                let reached = if *first <= 1 { n.learners } else { n.passed_counts.get(&crate::curriculum::doc_id(first - 1)).copied().unwrap_or(0) };
                                let w = (reached as f64 / total * 100.0).round();
                                view! {
                                    <li>
                                        <p class="tabular flex justify-between text-footnote"><span>{format!("Stage {stage}")}</span><span>{reached}</span></p>
                                        <div class="mt-1 h-2 overflow-hidden rounded-full bg-bg" aria-hidden="true"><div class="h-full rounded-full bg-gold-fill" style=format!("width:{w}%")></div></div>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }
                })}
            </section>

            <section class="rounded-card bg-surface p-5 shadow-card">
                {card_title("Course", "Hardest lessons")}
                <p class="mt-1 text-footnote text-text-muted">"Lowest share of right quiz answers (at least 5 answers), with their Explain-Back “Not yet” count."</p>
                {move || lessons.get().map(|v| {
                    let mut ranked: Vec<_> = v.into_iter().filter_map(|s| s.right_rate().map(|r| (r, s))).collect();
                    ranked.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                    if ranked.is_empty() {
                        view! { <p class="mt-3 text-callout">"Not enough answers yet."</p> }.into_any()
                    } else {
                        view! {
                            <ul class="mt-3 divide-y divide-line/40">
                                {ranked.into_iter().take(5).map(|(r, s)| {
                                    let n: u8 = s.lesson.trim_start_matches("lesson-").parse().unwrap_or(0);
                                    let title = telosgard_content::lesson(n).map(|l| l.title).unwrap_or("");
                                    view! {
                                        <li><a href=format!("#{}", s.lesson) class="press flex min-h-11 items-center justify-between gap-3 py-2">
                                            <span class="min-w-0 truncate text-callout"><span class="tabular mr-2 text-text-muted">{n}</span>{title}</span>
                                            <span class="tabular shrink-0 text-footnote text-text-muted">{format!("{} right · {} not yet", crate::curriculum::pct(r), s.not_yet)}</span>
                                        </a></li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }
                })}
            </section>

            <section class="rounded-card bg-surface p-5 shadow-card">
                {card_title("Practice", "On the road to Live Readiness")}
                <p class="mt-1 text-footnote text-text-muted">"Learners in a Readiness Run: Journal Entries in the run, and how many followed the plan."</p>
                {move || numbers.get().map(|n| {
                    if n.readiness.count == 0 {
                        view! { <p class="mt-3 text-callout">"No Readiness Runs in progress."</p> }.into_any()
                    } else if !can_people {
                        view! { <p class="tabular mt-3 text-callout">{crate::curriculum::plural(n.readiness.count, "Learner")}</p> }.into_any()
                    } else {
                        view! {
                            <ul class="mt-3 divide-y divide-line/40">
                                {n.readiness.list.iter().map(|p| {
                                    let rate = if p.judged == 0 { "–".to_string() } else { crate::curriculum::pct(p.adherent as f64 / p.judged as f64) };
                                    view! {
                                        <li><a href=format!("#learner-{}", p.id) class="press flex min-h-11 items-center justify-between gap-3 py-2">
                                            <span class="min-w-0 truncate text-callout font-semibold">{person_name(p)}</span>
                                            <span class="tabular shrink-0 text-footnote text-text-muted">{format!("Run {} · {} of 50 entries · {rate} followed the plan", p.run, p.entries)}</span>
                                        </a></li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }
                })}
            </section>
        </div>

        <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
            {card_title("Platform", "Professor Aurel (Claude)")}
            <p class="mt-1 text-footnote text-text-muted">"Calls through the Worker and the estimated cost at Claude Haiku 5.5 list prices ($0.10 per million input tokens, $0.50 per million output)."</p>
            {move || numbers.get().map(|n| {
                let row = |label: &'static str, u: &Usage| view! {
                    <div class="rounded-btn bg-bg p-3">
                        <p class="text-footnote font-semibold text-text-muted">{label}</p>
                        <p class="tabular mt-1 font-display text-title2">{if u.cost > 0.0 && u.cost < 1.0 { format!("${:.3}", u.cost) } else { format!("${:.2}", u.cost) }}</p>
                        <p class="tabular text-footnote text-text-muted">{format!("{} calls · {} errors · {}k in / {}k out", u.calls, u.errors, u.input / 1000, u.output / 1000)}</p>
                    </div>
                };
                view! { <div class="mt-3 grid gap-3 sm:grid-cols-2">{row("Last 24 hours", &n.claude.today)}{row("This month", &n.claude.month)}</div> }
            })}
            <p class="tabular mt-3 text-footnote text-text-muted">{move || numbers.get().map(|n| format!("{} events recorded in the last 24 hours.", n.events_today))}</p>
        </section>

        {me.can("events.backfill").then(|| view! {
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
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
    #[serde(default)]
    undoable: bool,
    #[serde(default)]
    undone: bool,
}

#[component]
pub fn Audit(me: Me) -> impl IntoView {
    let _ = me;
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
    // Undo: reverses one change. Full Access needs the current authenticator code.
    let undo_msg = RwSignal::new(None::<(bool, String)>);
    let code = RwSignal::new(String::new());
    let undo = move |id: u64| {
        spawn_local(async move {
            let c = code.get_untracked();
            let r = api::call_with_code("POST", &format!("/api/admin/audit/{id}/undo"), Some(json!({})), (!c.is_empty()).then_some(c)).await;
            if r.ok() {
                code.set(String::new());
                undo_msg.set(Some((true, "Undone.".into())));
                rows.set(Vec::new());
                done.set(false);
                more();
            } else {
                undo_msg.set(Some((false, if r.code() == Some("code_required") { "This change needs your authenticator code: enter it above, then Undo again.".into() } else { r.message() })));
            }
        });
    };
    view! {
        <Title eyebrow="Platform" title="Audit log" lead="Every change made from the dashboard: who, what, to which record, and the values before and after. Newest first." />
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}
        <div class="mb-4 flex flex-wrap items-end gap-3">
            <label class="block text-callout font-semibold">
                "Authenticator code"
                <span class="block text-footnote font-normal text-text-muted">"Only for undoing Full Access changes"</span>
                <input type="text" inputmode="numeric" autocomplete="one-time-code" maxlength="6" class="tabular mt-1.5 block min-h-11 w-40 rounded-field border border-line bg-surface px-3 text-body tracking-widest text-text" prop:value=code on:input=move |ev| code.set(event_target_value(&ev)) />
            </label>
            {move || undo_msg.get().map(|(ok, t)| view! { <p role=if ok { "status" } else { "alert" } class=format!("rounded-btn px-3 py-2 text-callout {}", if ok { "bg-ok-bg text-ok-ink" } else { "bg-bad-bg text-bad-ink" })>{t}</p> })}
        </div>
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
                                            <p class="flex items-center gap-3">
                                                {r.undone.then(|| view! { <span class="rounded-full bg-raised px-2 py-0.5 text-footnote font-semibold">"Undone"</span> })}
                                                {r.undoable.then(|| { let id = r.id; view! { <button type="button" class="press min-h-9 rounded-full border border-line px-3 text-footnote font-semibold" on:click=move |_| undo(id)>"Undo"</button> } })}
                                                <span class="tabular text-footnote text-text-muted">{api::when(r.at)}</span>
                                            </p>
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
