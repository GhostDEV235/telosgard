//! Curriculum (D2): every written lesson's status and results, and a panel per lesson at
//! `#lesson-<NN>` with its questions, anonymous Explain-Back answers, notes, and hiding.
//! Lesson text itself lives in the repo (ADR 0006): this page shows and steers, it never edits.

use crate::api::{self, Me};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde_json::json;
use telosgard_ui::controls::{PILL, PILL_GHOST};

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub lesson: String,
    pub hidden: bool,
    pub started: u32,
    pub passed: u32,
    pub waiting: u32,
    pub answered: u32,
    pub wrong: u32,
    pub guessed: u32,
    pub got_it: u32,
    pub almost: u32,
    pub not_yet: u32,
}

impl Stats {
    /// Share of quiz answers that were right, when there are enough to mean something.
    pub fn right_rate(&self) -> Option<f64> {
        (self.answered >= 5).then(|| 1.0 - self.wrong as f64 / self.answered as f64)
    }
}

pub fn doc_id(n: u8) -> String {
    format!("lesson-{n:02}")
}

pub fn pct(v: f64) -> String {
    format!("{:.0}%", v * 100.0)
}

/// Reads the selected lesson from `#lesson-<NN>`.
pub fn selected_from_hash() -> Option<u8> {
    window().location().hash().ok()?.strip_prefix("#lesson-")?.parse().ok()
}

pub async fn load_stats() -> Result<Vec<Stats>, String> {
    let r = api::get("/api/admin/curriculum").await;
    match r.parse::<Vec<Stats>>() {
        Some(v) if r.ok() => Ok(v),
        _ => Err(r.message()),
    }
}

#[component]
pub fn Curriculum(me: Me, selected: RwSignal<Option<u8>>) -> impl IntoView {
    let stats = RwSignal::new(None::<Vec<Stats>>);
    let err = RwSignal::new(None::<String>);
    let reload = move || {
        spawn_local(async move {
            match load_stats().await {
                Ok(v) => stats.set(Some(v)),
                Err(e) => err.set(Some(e)),
            }
        });
    };
    reload();
    let lessons = telosgard_content::lessons_in(false);
    let mut stages: Vec<u8> = lessons.iter().map(|l| l.stage).collect();
    stages.dedup();
    let me = StoredValue::new(me);
    view! {
        <header class="pt-4 pb-6">
            <p class="text-eyebrow text-eyebrow-ink">"Course"</p>
            <h1 class="text-large">"Curriculum"</h1>
            <p class="mt-2 max-w-[62ch] text-callout text-text-muted">"How each written lesson is going, across every account. Open one to see its questions, the Explain-Back answers (without names), notes, and to hide it for rework. Lesson text is changed in the repo, never here."</p>
        </header>
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}
        <div class="grid gap-5">
            {stages.into_iter().map(|stage| {
                let in_stage: Vec<_> = lessons.iter().filter(|l| l.stage == stage).map(|l| (l.id, l.title)).collect();
                view! {
                    <section class="overflow-hidden rounded-card bg-surface shadow-card" aria-label=format!("Stage {stage}")>
                        <h2 class="border-b border-line/40 px-5 py-3 text-headline">{format!("Stage {stage}")}</h2>
                        <div class="hidden grid-cols-[minmax(0,2.4fr)_repeat(4,minmax(0,1fr))] gap-3 px-5 pt-2 text-footnote font-semibold text-text-muted md:grid" aria-hidden="true">
                            <span>"Lesson"</span><span class="text-right">"Passed"</span><span class="text-right">"Quiz right"</span><span class="text-right">"Explain-Back"</span><span class="text-right">"Waiting"</span>
                        </div>
                        <ul class="divide-y divide-line/40">
                            {in_stage.into_iter().map(|(id, title)| {
                                let row = move || stats.get().and_then(|v| v.into_iter().find(|s| s.lesson == doc_id(id))).unwrap_or_default();
                                let english_only = telosgard_content::lesson_in(true, id).is_none();
                                view! {
                                    <li>
                                        <a href=format!("#lesson-{id:02}") class="press grid gap-1 px-5 py-3 hover:bg-raised/60 md:grid-cols-[minmax(0,2.4fr)_repeat(4,minmax(0,1fr))] md:items-center md:gap-3">
                                            <span class="min-w-0">
                                                <span class="tabular mr-2 text-text-muted">{id}</span>
                                                <span class="text-body font-semibold">{title}</span>
                                                <span class="ml-2 inline-flex flex-wrap gap-1.5 align-middle">
                                                    {move || row().hidden.then(|| view! { <span class="rounded-full bg-warn-bg px-2 py-0.5 text-footnote font-semibold text-warn-ink">"Hidden"</span> })}
                                                    {english_only.then(|| view! { <span class="rounded-full bg-raised px-2 py-0.5 text-footnote font-semibold">"English only"</span> })}
                                                </span>
                                            </span>
                                            <span class="tabular text-footnote text-text-muted md:text-right md:text-callout md:text-text">{move || { let s = row(); format!("{} of {} passed", s.passed, s.started) }}</span>
                                            <span class="tabular text-footnote text-text-muted md:text-right md:text-callout md:text-text">{move || row().right_rate().map(|r| format!("{} right", pct(r))).unwrap_or_else(|| "– quiz".into())}</span>
                                            <span class="tabular text-footnote text-text-muted md:text-right md:text-callout md:text-text">{move || { let s = row(); format!("{} · {} · {}", s.got_it, s.almost, s.not_yet) }}</span>
                                            <span class="tabular text-footnote text-text-muted md:text-right md:text-callout md:text-text">{move || format!("{} waiting", row().waiting)}</span>
                                        </a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    </section>
                }
            }).collect_view()}
        </div>
        <p class="mt-3 text-footnote text-text-muted">"Explain-Back shows Got it · Almost · Not yet. Waiting: accounts that passed the lesson before and not this one; a hide pauses them."</p>
        {move || selected.get().map(|id| view! { <LessonPanel id=id me=me.get_value() on_change=Callback::new(move |_| reload()) /> })}
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct QStat {
    q: String,
    answered: u32,
    wrong: u32,
    guessed: u32,
    accounts: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Answer {
    grade: String,
    answer: String,
    at: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Note {
    author: Option<String>,
    body: String,
    at: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Detail {
    hidden: bool,
    hidden_at: Option<f64>,
    reason: Option<String>,
    waiting: u32,
    questions: Vec<QStat>,
    answers: Vec<Answer>,
    notes: Vec<Note>,
}

fn close() {
    let _ = window().location().set_hash("curriculum");
}

fn grade_label(g: &str) -> (&'static str, &'static str) {
    match g {
        "got_it" => ("Got it", "bg-ok-bg text-ok-ink"),
        "almost" => ("Almost", "bg-warn-bg text-warn-ink"),
        "not_yet" => ("Not yet", "bg-bad-bg text-bad-ink"),
        _ => ("–", "bg-raised text-text"),
    }
}

#[component]
fn LessonPanel(id: u8, me: Me, on_change: Callback<()>) -> impl IntoView {
    let detail = RwSignal::new(None::<Result<Detail, String>>);
    let note = RwSignal::new(None::<(bool, String)>);
    let load = move || {
        spawn_local(async move {
            let r = api::get(&format!("/api/admin/curriculum/{}", doc_id(id))).await;
            detail.set(Some(match r.parse::<Detail>() {
                Some(d) if r.ok() => Ok(d),
                _ => Err(r.message()),
            }));
        });
    };
    load();
    let refresh = move || {
        load();
        on_change.run(());
    };
    let lesson = telosgard_content::lesson(id);
    let title = lesson.map(|l| l.title).unwrap_or("Not written yet");
    let me = StoredValue::new(me);
    view! {
        <div class="anim-fade fixed inset-0 z-40 bg-scrim" on:click=move |_| close() aria-hidden="true"></div>
        <div
            id="lesson-panel"
            role="dialog"
            aria-modal="true"
            aria-label=format!("Lesson {id}")
            class="anim-sheet fixed inset-y-0 right-0 z-50 w-full max-w-[640px] overflow-y-auto bg-bg px-[max(16px,env(safe-area-inset-right,0px))] pt-[calc(16px+env(safe-area-inset-top,0px))] pb-[calc(24px+env(safe-area-inset-bottom,0px))] shadow-sheet md:px-6"
        >
            <div class="flex items-start justify-between gap-3">
                <div class="min-w-0">
                    <p class="text-eyebrow text-eyebrow-ink">{format!("Lesson {id}")}</p>
                    <h2 class="font-display text-title1">{title}</h2>
                </div>
                <button type="button" class="press grid size-11 shrink-0 place-items-center rounded-full bg-surface shadow-card" aria-label="Close" on:click=move |_| close()>
                    <svg viewBox="0 0 24 24" class="size-5" aria-hidden="true"><path d="M6 6l12 12M18 6L6 18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
                </button>
            </div>
            <p class="mt-2"><a class="text-callout underline underline-offset-4" href=format!("/#lesson-{id}") target="_blank" rel="noopener">"Preview it in the course ↗"</a></p>
            {move || note.get().map(|(ok, t)| view! {
                <p role=if ok { "status" } else { "alert" } class=format!("mt-3 rounded-btn px-3 py-2 text-callout {}", if ok { "bg-ok-bg text-ok-ink" } else { "bg-bad-bg text-bad-ink" })>{t}</p>
            })}
            {move || match detail.get() {
                None => view! { <p class="mt-4 text-callout text-text-muted" role="status">"Loading…"</p> }.into_any(),
                Some(Err(e)) => view! { <p role="alert" class="mt-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> }.into_any(),
                Some(Ok(d)) => view! { <Body id=id d=d me=me.get_value() note=note refresh=Callback::new(move |_| refresh()) /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn Body(id: u8, d: Detail, me: Me, note: RwSignal<Option<(bool, String)>>, refresh: Callback<()>) -> impl IntoView {
    let lesson = telosgard_content::lesson(id);
    let text_of = move |q: &str| -> String {
        lesson
            .and_then(|l| l.quiz.iter().chain(l.recall.iter()).find(|x| x.id == q))
            .map(|x| x.q.to_string())
            .unwrap_or_else(|| "(question no longer in the lesson)".into())
    };
    let hidden = d.hidden;
    let waiting = d.waiting;
    let reason = RwSignal::new(String::new());
    let confirm = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let toggle = move |hide: bool| {
        busy.set(true);
        spawn_local(async move {
            let path = format!("/api/admin/curriculum/{}/{}", doc_id(id), if hide { "hide" } else { "unhide" });
            let r = api::post(&path, json!({ "reason": reason.get_untracked() })).await;
            busy.set(false);
            confirm.set(false);
            if r.ok() {
                note.set(Some((true, if hide { format!("Hidden. {} paused at it; Undo is in the audit log.", plural(r.body.get("paused").and_then(|v| v.as_u64()).unwrap_or(0) as u32, "account")) } else { "Back for everyone.".into() })));
                refresh.run(());
            } else {
                note.set(Some((false, r.message())));
            }
        });
    };
    let draft = RwSignal::new(String::new());
    let add_note = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let text = draft.get_untracked();
        if text.trim().is_empty() {
            return;
        }
        spawn_local(async move {
            let r = api::post(&format!("/api/admin/curriculum/{}/notes", doc_id(id)), json!({ "body": text })).await;
            if r.ok() {
                draft.set(String::new());
                refresh.run(());
            } else {
                note.set(Some((false, r.message())));
            }
        });
    };
    view! {
        <section class="mt-5 rounded-card bg-surface p-4 shadow-card">
            <p class="flex flex-wrap items-center gap-2">
                {if hidden {
                    view! { <span class="rounded-full bg-warn-bg px-2.5 py-0.5 text-callout font-semibold text-warn-ink">"Hidden"</span> }.into_any()
                } else {
                    view! { <span class="rounded-full bg-ok-bg px-2.5 py-0.5 text-callout font-semibold text-ok-ink">"Live"</span> }.into_any()
                }}
                <span class="text-callout text-text-muted">{format!("{} waiting at it right now", plural(waiting, "account"))}</span>
            </p>
            {d.reason.clone().filter(|_| hidden).map(|r| view! { <p class="mt-2 text-callout">{format!("Why: {r}")}</p> })}
            {d.hidden_at.filter(|_| hidden).map(|t| view! { <p class="text-footnote text-text-muted">{format!("Hidden since {}", api::when(t))}</p> })}
            {me.can("lessons.hide").then(|| if hidden {
                view! { <button type="button" class=format!("{PILL_GHOST} mt-3") disabled=move || busy.get() on:click=move |_| toggle(false)>"Unhide: bring it back"</button> }.into_any()
            } else {
                view! {
                    <div class="mt-3">
                        <label class="block text-callout font-semibold" for="hide-reason">"Hide for rework"</label>
                        <p class="text-footnote text-text-muted">"Learners who reach it wait at a “being updated” card; those who passed it keep it. Its questions leave Spaced Review."</p>
                        <input id="hide-reason" type="text" maxlength="300" placeholder="Why (optional), e.g. q3's answer is ambiguous" class="mt-2 block min-h-11 w-full rounded-field border border-line bg-surface px-3 text-body text-text" prop:value=reason on:input=move |ev| reason.set(event_target_value(&ev)) />
                        {move || if confirm.get() {
                            view! {
                                <div class="mt-3 rounded-btn bg-warn-bg p-3 text-warn-ink" role="alert">
                                    <p class="text-callout font-semibold">{format!("This pauses {} on it right now.", plural(waiting, "account"))}</p>
                                    <div class="mt-2 flex flex-wrap gap-2">
                                        <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| toggle(true)>"Hide the lesson"</button>
                                        <button type="button" class=PILL_GHOST on:click=move |_| confirm.set(false)>"Cancel"</button>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <button type="button" class=format!("{PILL_GHOST} mt-3") on:click=move |_| confirm.set(true)>"Hide…"</button> }.into_any()
                        }}
                    </div>
                }.into_any()
            })}
        </section>

        <section class="mt-6">
            <h3 class="text-headline">"Questions"</h3>
            {if d.questions.is_empty() {
                view! { <p class="mt-2 text-callout text-text-muted">"No answers yet."</p> }.into_any()
            } else {
                view! {
                    <ul class="mt-2 divide-y divide-line/40 overflow-hidden rounded-card bg-surface shadow-card">
                        {d.questions.iter().map(|q| {
                            let right = if q.answered == 0 { 0.0 } else { 1.0 - q.wrong as f64 / q.answered as f64 };
                            let tone = if q.answered >= 5 && right < 0.6 { "text-bad-ink" } else { "text-text" };
                            view! {
                                <li class="grid gap-1 px-4 py-3 sm:grid-cols-[minmax(0,1fr)_auto] sm:items-baseline sm:gap-4">
                                    <p class="text-callout"><span class="tabular mr-2 font-semibold text-text-muted">{q.q.clone()}</span>{text_of(&q.q)}</p>
                                    <p class=format!("tabular text-footnote sm:text-right {tone}")>{format!("{} right · {} answers · {} guessed · {}", pct(right), q.answered, q.guessed, plural(q.accounts, "account"))}</p>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                }.into_any()
            }}
        </section>

        {me.can("answers.read").then(|| view! {
            <section class="mt-6">
                <h3 class="text-headline">"Explain-Back answers"</h3>
                <p class="text-footnote text-text-muted">"Newest first, without names."</p>
                {if d.answers.is_empty() {
                    view! { <p class="mt-2 text-callout text-text-muted">"None yet."</p> }.into_any()
                } else {
                    view! {
                        <ul class="mt-2 divide-y divide-line/40 overflow-hidden rounded-card bg-surface shadow-card">
                            {d.answers.iter().map(|a| {
                                let (label, tone) = grade_label(&a.grade);
                                view! {
                                    <li class="px-4 py-3">
                                        <p class="flex flex-wrap items-center justify-between gap-2">
                                            <span class=format!("rounded-full px-2 py-0.5 text-footnote font-semibold {tone}")>{label}</span>
                                            <span class="tabular text-footnote text-text-muted">{api::when(a.at)}</span>
                                        </p>
                                        <p class="mt-1 text-callout">{a.answer.clone()}</p>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }}
            </section>
        })}

        <section class="mt-6">
            <h3 class="text-headline">"Notes"</h3>
            {me.can("lessons.notes").then(|| view! {
                <form class="mt-2 grid gap-2" on:submit=add_note>
                    <label class="sr-only" for="note-body">"New note"</label>
                    <textarea id="note-body" rows="3" maxlength="2000" placeholder="What should change in this lesson?" class="block w-full rounded-field border border-line bg-surface px-3 py-2 text-body text-text" prop:value=draft on:input=move |ev| draft.set(event_target_value(&ev))></textarea>
                    <div><button type="submit" class=PILL_GHOST>"Add note"</button></div>
                </form>
            })}
            {if d.notes.is_empty() {
                view! { <p class="mt-2 text-callout text-text-muted">"No notes yet."</p> }.into_any()
            } else {
                view! {
                    <ul class="mt-3 grid gap-2">
                        {d.notes.iter().map(|n| view! {
                            <li class="rounded-card bg-surface p-3 shadow-card">
                                <p class="text-callout">{n.body.clone()}</p>
                                <p class="mt-1 text-footnote text-text-muted">{format!("{} · {}", n.author.clone().unwrap_or_else(|| "unknown".into()), api::when(n.at))}</p>
                            </li>
                        }).collect_view()}
                    </ul>
                }.into_any()
            }}
        </section>
    }
}

pub fn plural(n: u32, word: &str) -> String {
    format!("{n} {word}{}", if n == 1 { "" } else { "s" })
}
