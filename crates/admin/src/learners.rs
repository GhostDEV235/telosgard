//! Learners (D1): the searchable list and the profile panel with the actions staff may take.
//! The panel opens from the right on wide screens and fills the screen on phones; its address is
//! `#learner-<id>`, so a profile can be bookmarked or opened from ⌘K.

use crate::api::{self, Me};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde_json::{json, Value};
use telosgard_ui::controls::{PILL, PILL_GHOST, Segmented};

const FIELD: &str = "block min-h-11 w-full rounded-field border border-line bg-surface px-3 text-body text-text";

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: f64,
    pub suspended_at: Option<f64>,
    pub full_access: bool,
    pub delete_after: Option<f64>,
    pub joined: String,
    pub last_active: Option<f64>,
    pub lessons_passed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    All,
    Active,
    Suspended,
    Deleting,
}

fn chips(full_access: bool, suspended: bool, deleting: bool, role: String) -> impl IntoView {
    let chip = |text: String, tone: &'static str| view! { <span class=format!("rounded-full px-2 py-0.5 text-footnote font-semibold {tone}")>{text}</span> };
    view! {
        {(role != "learner").then(|| chip(match role.as_str() { "instructor" => "Instructor".into(), "support" => "Support".into(), r => r.to_string() }, "bg-raised text-text"))}
        {full_access.then(|| chip("Full Access".into(), "bg-ok-bg text-ok-ink"))}
        {(suspended && !deleting).then(|| chip("Suspended".into(), "bg-warn-bg text-warn-ink"))}
        {deleting.then(|| chip("Deleting".into(), "bg-bad-bg text-bad-ink"))}
    }
}

/// Reads the selected Learner from `#learner-<id>`.
pub fn selected_from_hash() -> Option<String> {
    window().location().hash().ok()?.strip_prefix("#learner-").map(String::from).filter(|s| !s.is_empty())
}

#[component]
pub fn Learners(me: Me, selected: RwSignal<Option<String>>) -> impl IntoView {
    let rows = RwSignal::new(None::<Vec<Row>>);
    let err = RwSignal::new(None::<String>);
    let reload = move || {
        spawn_local(async move {
            let r = api::get("/api/admin/learners").await;
            match r.parse::<Vec<Row>>() {
                Some(v) if r.ok() => rows.set(Some(v)),
                _ => err.set(Some(r.message())),
            }
        });
    };
    reload();
    let query = RwSignal::new(String::new());
    let filter = RwSignal::new(Filter::All);
    let shown = move || {
        let q = query.get().trim().to_lowercase();
        let f = filter.get();
        rows.get().unwrap_or_default().into_iter().filter(move |r| {
            let text = q.is_empty() || r.name.to_lowercase().contains(&q) || r.email.to_lowercase().contains(&q);
            let state = match f {
                Filter::All => true,
                Filter::Active => r.suspended_at.is_none() && r.delete_after.is_none(),
                Filter::Suspended => r.suspended_at.is_some() && r.delete_after.is_none(),
                Filter::Deleting => r.delete_after.is_some(),
            };
            text && state
        }).collect::<Vec<_>>()
    };
    let me = StoredValue::new(me);
    view! {
        <header class="pt-4 pb-6">
            <p class="text-eyebrow text-eyebrow-ink">"People"</p>
            <h1 class="text-large">"Learners"</h1>
            <p class="mt-2 max-w-[62ch] text-callout text-text-muted">"Everyone with an account except the Owner. Open a Learner to see their progress or to help them."</p>
        </header>
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}
        <div class="mb-4 grid gap-3 md:grid-cols-[minmax(0,1fr)_auto] md:items-center">
            <label class="block">
                <span class="sr-only">"Search Learners"</span>
                <input type="search" placeholder="Search by name or email" class=FIELD prop:value=query on:input=move |ev| query.set(event_target_value(&ev)) />
            </label>
            <div class="md:w-[420px]">
                <Segmented label="Show" value=filter options=vec![(Filter::All, "All"), (Filter::Active, "Active"), (Filter::Suspended, "Suspended"), (Filter::Deleting, "Deleting")] />
            </div>
        </div>
        <section class="overflow-hidden rounded-card bg-surface shadow-card" aria-label="Learners">
            <div class="hidden grid-cols-[minmax(0,2fr)_minmax(0,1.2fr)_minmax(0,1fr)_minmax(0,0.8fr)] gap-4 border-b border-line/40 px-5 py-2 text-footnote font-semibold text-text-muted md:grid" aria-hidden="true">
                <span>"Learner"</span><span>"Joined"</span><span>"Last active"</span><span class="text-right">"Passed"</span>
            </div>
            {move || match rows.get() {
                None => view! { <p class="p-5 text-callout text-text-muted" role="status">"Loading…"</p> }.into_any(),
                Some(_) => {
                    let list = shown();
                    if list.is_empty() {
                        view! { <p class="p-5 text-callout text-text-muted">"No Learners match."</p> }.into_any()
                    } else {
                        view! {
                            <ul class="divide-y divide-line/40">
                                {list.into_iter().map(|r| {
                                    let href = format!("#learner-{}", r.id);
                                    view! {
                                        <li>
                                            <a href=href class="press grid gap-1 px-5 py-3 hover:bg-raised/60 md:grid-cols-[minmax(0,2fr)_minmax(0,1.2fr)_minmax(0,1fr)_minmax(0,0.8fr)] md:items-center md:gap-4">
                                                <span class="min-w-0">
                                                    <span class="block truncate text-body font-semibold">{if r.name.is_empty() { r.email.clone() } else { r.name.clone() }}</span>
                                                    <span class="block truncate text-footnote text-text-muted">{r.email.clone()}</span>
                                                    <span class="mt-1 flex flex-wrap gap-1.5">{chips(r.full_access, r.suspended_at.is_some(), r.delete_after.is_some(), r.role.clone())}</span>
                                                </span>
                                                <span class="text-footnote text-text-muted md:text-callout md:text-text">
                                                    <span class="md:hidden">"Joined "</span>{api::date(r.created_at)}
                                                    <span class="block text-footnote text-text-muted">{r.joined.clone()}</span>
                                                </span>
                                                <span class="text-footnote text-text-muted md:text-callout md:text-text">
                                                    <span class="md:hidden">"Last active "</span>{r.last_active.map(api::ago).unwrap_or_else(|| "Not yet".into())}
                                                </span>
                                                <span class="tabular text-footnote text-text-muted md:text-right md:text-callout md:text-text">
                                                    {format!("{} lesson{}", r.lessons_passed, if r.lessons_passed == 1 { "" } else { "s" })}
                                                </span>
                                            </a>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }
                }
            }}
        </section>
        {move || selected.get().map(|id| view! { <Profile id=id me=me.get_value() on_change=Callback::new(move |_| reload()) /> })}
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    id: String,
    name: String,
    email: String,
    role: String,
    created_at: f64,
    suspended_at: Option<f64>,
    full_access: bool,
    delete_after: Option<f64>,
    joined: String,
    last_active: Option<f64>,
    progress: Vec<Value>,
    attempts: Vec<Value>,
    drills: Vec<Value>,
    journal_entries: u32,
    checklists: u32,
}

fn num(v: &Value, k: &str) -> Option<f64> {
    v.get(k).and_then(Value::as_f64)
}
fn text(v: &Value, k: &str) -> Option<String> {
    v.get(k).and_then(Value::as_str).map(String::from)
}
fn grade(g: Option<String>) -> &'static str {
    match g.as_deref() {
        Some("got_it") => "Got it",
        Some("almost") => "Almost",
        Some("not_yet") => "Not yet",
        _ => "–",
    }
}

fn close() {
    let _ = window().location().set_hash("learners");
}

#[component]
fn Profile(id: String, me: Me, on_change: Callback<()>) -> impl IntoView {
    let profile = RwSignal::new(None::<Result<Profile, String>>);
    let id = StoredValue::new(id);
    let load = move || {
        spawn_local(async move {
            let r = api::get(&format!("/api/admin/learners/{}", id.get_value())).await;
            profile.set(Some(match r.parse::<Profile>() {
                Some(p) if r.ok() => Ok(p),
                _ => Err(r.message()),
            }));
        });
    };
    load();
    let refresh = move || {
        load();
        on_change.run(());
    };
    let me = StoredValue::new(me);
    // Kept here, not in Actions, so the result of an action survives the profile reloading.
    let note = RwSignal::new(None::<(bool, String)>);
    view! {
        <div class="anim-fade fixed inset-0 z-40 bg-scrim" on:click=move |_| close() aria-hidden="true"></div>
        <div
            role="dialog"
            aria-modal="true"
            aria-label="Learner"
            class="anim-sheet fixed inset-y-0 right-0 z-50 w-full max-w-[560px] overflow-y-auto bg-bg px-[max(16px,env(safe-area-inset-right,0px))] pt-[calc(16px+env(safe-area-inset-top,0px))] pb-[calc(24px+env(safe-area-inset-bottom,0px))] shadow-sheet md:px-6"
        >
            <div class="flex justify-end">
                <button type="button" class="press grid size-11 place-items-center rounded-full bg-surface shadow-card" aria-label="Close" on:click=move |_| close()>
                    <svg viewBox="0 0 24 24" class="size-5" aria-hidden="true"><path d="M6 6l12 12M18 6L6 18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
                </button>
            </div>
            // The result of the last action, outside the part that re-renders when the profile reloads.
            {move || note.get().map(|(ok, t)| view! {
                <p role=if ok { "status" } else { "alert" } class=format!("mb-3 mt-2 rounded-btn px-3 py-2 text-callout {}", if ok { "bg-ok-bg text-ok-ink" } else { "bg-bad-bg text-bad-ink" })>{t}</p>
            })}
            {move || match profile.get() {
                None => view! { <p class="p-2 text-callout text-text-muted" role="status">"Loading…"</p> }.into_any(),
                Some(Err(e)) => view! { <p role="alert" class="rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> }.into_any(),
                Some(Ok(p)) => view! { <ProfileBody p=p me=me.get_value() note=note refresh=Callback::new(move |_| refresh()) /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn ProfileBody(p: Profile, me: Me, note: RwSignal<Option<(bool, String)>>, refresh: Callback<()>) -> impl IntoView {
    let deleting = p.delete_after.is_some();
    let lessons: Vec<String> = p.progress.iter().filter_map(|v| text(v, "lesson")).collect();
    let id = p.id.clone();
    view! {
        <header>
            <h2 class="font-display text-title1">{if p.name.is_empty() { p.email.clone() } else { p.name.clone() }}</h2>
            <p class="mt-1 break-all text-callout text-text-muted">{p.email.clone()}</p>
            <p class="mt-2 flex flex-wrap gap-1.5">{chips(p.full_access, p.suspended_at.is_some(), deleting, p.role.clone())}</p>
        </header>
        {p.delete_after.map(|t| view! {
            <p role="status" class="mt-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{format!("Scheduled for deletion. Erased for good on {} unless cancelled.", api::when(t))}</p>
        })}
        <dl class="mt-5 grid grid-cols-2 gap-3">
            <Fact label="Joined" value=api::date(p.created_at) note=p.joined.clone() />
            <Fact label="Last active" value=p.last_active.map(api::ago).unwrap_or_else(|| "Not yet".into()) note=String::new() />
            <Fact label="Lessons passed" value=p.progress.iter().filter(|v| v.get("passedAt").is_some_and(|x| !x.is_null())).count().to_string() note=String::new() />
            <Fact label="Journal" value=format!("{} entries", p.journal_entries) note=format!("{} checklists", p.checklists) />
        </dl>

        <Actions id=id.clone() me=me note=note deleting=deleting suspended=p.suspended_at.is_some() full_access=p.full_access lessons=lessons refresh=refresh />

        <section class="mt-6">
            <h3 class="text-headline">"Progress"</h3>
            {if p.progress.is_empty() {
                view! { <p class="mt-2 text-callout text-text-muted">"No lesson started yet."</p> }.into_any()
            } else {
                view! {
                    <div class="mt-2 overflow-x-auto rounded-card bg-surface shadow-card">
                        <table class="w-full text-left text-callout">
                            <thead class="text-footnote text-text-muted"><tr><th class="px-4 py-2 font-semibold">"Lesson"</th><th class="px-4 py-2 font-semibold">"Quiz best"</th><th class="px-4 py-2 font-semibold">"Explain-Back"</th><th class="px-4 py-2 font-semibold">"Passed"</th></tr></thead>
                            <tbody class="tabular divide-y divide-line/40">
                                {p.progress.iter().map(|v| view! {
                                    <tr>
                                        <td class="px-4 py-2">{text(v, "lesson").map(|l| api::lesson_label(&l)).unwrap_or_default()}</td>
                                        <td class="px-4 py-2">{num(v, "quizBest").map(|n| format!("{n}%")).unwrap_or_else(|| "–".into())}</td>
                                        <td class="px-4 py-2">{grade(text(v, "grade"))}</td>
                                        <td class="px-4 py-2">{num(v, "passedAt").map(api::date).unwrap_or_else(|| "–".into())}</td>
                                    </tr>
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                }.into_any()
            }}
        </section>

        <section class="mt-6">
            <h3 class="text-headline">"Explain-Back history"</h3>
            {if p.attempts.is_empty() {
                view! { <p class="mt-2 text-callout text-text-muted">"No Explain-Backs yet."</p> }.into_any()
            } else {
                view! {
                    <ul class="mt-2 divide-y divide-line/40 overflow-hidden rounded-card bg-surface shadow-card">
                        {p.attempts.iter().map(|v| {
                            let lesson = text(v, "lesson").map(|l| api::lesson_label(&format!("lesson-{:0>2}", l))).unwrap_or_default();
                            view! {
                                <li class="px-4 py-3">
                                    <div class="flex flex-wrap items-baseline justify-between gap-2">
                                        <p class="text-callout font-semibold">{format!("{lesson} · {}", grade(text(v, "grade")))}</p>
                                        <p class="tabular text-footnote text-text-muted">{num(v, "at").map(api::when).unwrap_or_default()}</p>
                                    </div>
                                    {text(v, "answer").map(|a| view! { <p class="mt-1 text-callout text-text-muted">{a}</p> })}
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                }.into_any()
            }}
        </section>

        {(!p.drills.is_empty()).then(|| view! {
            <section class="mt-6">
                <h3 class="text-headline">"Drills"</h3>
                <ul class="mt-2 grid gap-1 text-callout">
                    {p.drills.iter().map(|v| view! {
                        <li class="tabular">{format!("{}: {} of {} right", text(v, "drill").unwrap_or_default(), num(v, "correct").unwrap_or(0.0), num(v, "answered").unwrap_or(0.0))}</li>
                    }).collect_view()}
                </ul>
            </section>
        })}
    }
}

#[component]
fn Fact(label: &'static str, value: String, note: String) -> impl IntoView {
    view! {
        <div class="rounded-card bg-surface p-3 shadow-card">
            <dt class="text-footnote text-text-muted">{label}</dt>
            <dd class="tabular text-body font-semibold">{value}</dd>
            {(!note.is_empty()).then(|| view! { <dd class="text-footnote text-text-muted">{note}</dd> })}
        </div>
    }
}

/// What the signed-in staff member may do to this Learner. Each answer reloads the profile.
#[component]
fn Actions(id: String, me: Me, note: RwSignal<Option<(bool, String)>>, deleting: bool, suspended: bool, full_access: bool, lessons: Vec<String>, refresh: Callback<()>) -> impl IntoView {
    let id = StoredValue::new(id);
    let busy = RwSignal::new(false);
    let post = move |path: &'static str, body: Value, code: Option<String>, ok_text: String| {
        busy.set(true);
        spawn_local(async move {
            let r = api::call_with_code("POST", &format!("/api/admin/learners/{}/{path}", id.get_value()), Some(body), code).await;
            busy.set(false);
            if r.ok() {
                note.set(Some((true, ok_text)));
                refresh.run(());
            } else {
                note.set(Some((false, r.message())));
            }
        });
    };

    // Reset a lesson (two steps).
    let lesson = RwSignal::new(lessons.first().cloned().unwrap_or_default());
    let confirm_reset = RwSignal::new(false);
    // Password-reset link.
    let link = RwSignal::new(None::<String>);
    let copied = RwSignal::new(false);
    let make_link = move |_| {
        busy.set(true);
        spawn_local(async move {
            let r = api::post(&format!("/api/admin/learners/{}/reset-link", id.get_value()), json!({})).await;
            busy.set(false);
            match r.body.get("url").and_then(Value::as_str) {
                Some(u) if r.ok() => {
                    copied.set(false);
                    link.set(Some(u.to_string()));
                }
                _ => note.set(Some((false, r.message()))),
            }
        });
    };
    // Sensitive actions need the current authenticator code.
    let code = RwSignal::new(String::new());
    let confirm_delete = RwSignal::new(false);
    let has_lessons = !lessons.is_empty();
    let lessons = StoredValue::new(lessons);

    view! {
        <section class="mt-6 rounded-card bg-surface p-4 shadow-card" aria-label="Actions">
            <h3 class="text-headline">"Help this Learner"</h3>
            <div class="mt-3 grid gap-5">
                {(me.can("passwords.reset") && !deleting).then(|| view! {
                    <div>
                        <p class="text-callout font-semibold">"Password-reset link"</p>
                        <p class="text-footnote text-text-muted">"Works once, for 24 hours. Send it to them yourself; it signs them out everywhere when used."</p>
                        <button type="button" class=format!("{PILL_GHOST} mt-2") disabled=move || busy.get() on:click=make_link>"Create reset link"</button>
                        {move || link.get().map(|u| {
                            let u2 = u.clone();
                            view! {
                                <div class="mt-2 rounded-btn bg-raised p-3">
                                    <p class="break-all font-mono text-footnote select-all">{u}</p>
                                    <button type="button" class=format!("{PILL} mt-2") on:click=move |_| { let t = u2.clone(); spawn_local(async move { copied.set(api::copy(&t).await); }); }>
                                        {move || if copied.get() { "Copied" } else { "Copy link" }}
                                    </button>
                                </div>
                            }
                        })}
                    </div>
                })}
                {(me.can("progress.reset") && has_lessons && !deleting).then(|| view! {
                    <div>
                        <label class="block text-callout font-semibold" for="reset-lesson">"Reset a lesson"</label>
                        <p class="text-footnote text-text-muted">"Clears its quiz score, Explain-Back grade and question records. Undo from the audit log."</p>
                        <div class="mt-2 flex flex-wrap gap-2">
                            <select id="reset-lesson" class="min-h-11 rounded-field border border-line bg-surface px-3 text-body text-text" on:change=move |ev| { lesson.set(event_target_value(&ev)); confirm_reset.set(false); }>
                                {lessons.get_value().into_iter().map(|l| view! { <option value=l.clone()>{api::lesson_label(&l)}</option> }).collect_view()}
                            </select>
                            {move || if confirm_reset.get() {
                                view! {
                                    <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| {
                                        confirm_reset.set(false);
                                        let l = lesson.get_untracked();
                                        post("reset-lesson", json!({ "lesson": l }), None, format!("{} reset. Undo it from the audit log.", api::lesson_label(&l)));
                                    }>{move || format!("Reset {}", api::lesson_label(&lesson.get()))}</button>
                                    <button type="button" class=PILL_GHOST on:click=move |_| confirm_reset.set(false)>"Cancel"</button>
                                }.into_any()
                            } else {
                                view! { <button type="button" class=PILL_GHOST on:click=move |_| confirm_reset.set(true)>"Reset…"</button> }.into_any()
                            }}
                        </div>
                    </div>
                })}
                {(me.can("learners.suspend") && !deleting).then(|| view! {
                    <div>
                        <p class="text-callout font-semibold">{if suspended { "Suspended" } else { "Suspend" }}</p>
                        <p class="text-footnote text-text-muted">{if suspended { "Signed out and unable to sign in. All their data is kept." } else { "Signs them out at once and stops them signing in. Nothing is deleted." }}</p>
                        <button type="button" class=format!("{PILL_GHOST} mt-2") disabled=move || busy.get() on:click=move |_| {
                            post("suspend", json!({ "on": !suspended }), None, if suspended { "Unsuspended. They can sign in again.".into() } else { "Suspended and signed out.".into() });
                        }>{if suspended { "Unsuspend" } else { "Suspend" }}</button>
                    </div>
                })}
                {(me.can("fullaccess.set") || me.can("learners.delete")).then(|| view! {
                    <div class="rounded-btn border border-line/60 p-3">
                        <label class="block text-callout font-semibold" for="step-code">"Your authenticator code"</label>
                        <p class="text-footnote text-text-muted">"Needed for Full Access and deletion."</p>
                        <input id="step-code" type="text" inputmode="numeric" autocomplete="one-time-code" maxlength="6" class=format!("{FIELD} tabular mt-2 max-w-[10rem] tracking-widest") prop:value=code on:input=move |ev| code.set(event_target_value(&ev)) />
                        {(me.can("fullaccess.set") && !deleting).then(|| view! {
                            <div class="mt-4">
                                <p class="text-callout font-semibold">"Full Access"</p>
                                <p class="text-footnote text-text-muted">{if full_access { "On: every lesson is open to them, whatever they've passed." } else { "Off: lessons open as they pass the ones before." }}</p>
                                <button type="button" class=format!("{PILL_GHOST} mt-2") disabled=move || busy.get() on:click=move |_| {
                                    let c = code.get_untracked();
                                    code.set(String::new());
                                    post("full-access", json!({ "on": !full_access }), Some(c), if full_access { "Full Access turned off.".into() } else { "Full Access turned on.".into() });
                                }>{if full_access { "Turn off Full Access" } else { "Turn on Full Access" }}</button>
                            </div>
                        })}
                        {me.can("learners.delete").then(|| if deleting {
                            view! {
                                <div class="mt-4">
                                    <p class="text-callout font-semibold">"Deletion scheduled"</p>
                                    <button type="button" class=format!("{PILL_GHOST} mt-2") disabled=move || busy.get() on:click=move |_| {
                                        post("cancel-delete", json!({}), None, "Deletion cancelled. The account is active again.".into());
                                    }>"Cancel deletion"</button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="mt-4">
                                    <p class="text-callout font-semibold">"Delete account"</p>
                                    <p class="text-footnote text-text-muted">"Suspends them now. After 7 days their name, email, journal, plans, answers and screenshots are erased for good; anonymous totals stay. You can cancel within the 7 days."</p>
                                    {move || if confirm_delete.get() {
                                        view! {
                                            <div class="mt-2 flex flex-wrap gap-2">
                                                <button type="button" class="press inline-flex min-h-11 items-center rounded-full bg-down px-5 text-callout font-semibold text-surface" disabled=move || busy.get() on:click=move |_| {
                                                    confirm_delete.set(false);
                                                    let c = code.get_untracked();
                                                    code.set(String::new());
                                                    post("delete", json!({}), Some(c), "Deletion scheduled. Erased for good in 7 days unless you cancel.".into());
                                                }>"Delete in 7 days"</button>
                                                <button type="button" class=PILL_GHOST on:click=move |_| confirm_delete.set(false)>"Keep"</button>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <button type="button" class=format!("{PILL_GHOST} mt-2 text-down") on:click=move |_| confirm_delete.set(true)>"Delete…"</button> }.into_any()
                                    }}
                                </div>
                            }.into_any()
                        })}
                    </div>
                })}
            </div>
        </section>
    }
}
