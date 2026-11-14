//! Settings (D3, Owner only): Professor Aurel's limits, Maintenance Mode, the Disclaimer in English
//! and French (versioned), and the status of keys and integrations. Every change is in the audit log.

use crate::api::{self, Me};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde_json::{json, Value};
use telosgard_ui::controls::{PILL, PILL_GHOST};
use telosgard_ui::rich::Rich;

const FIELD: &str = "mt-1.5 block min-h-11 w-full rounded-field border border-line bg-surface px-3 text-body text-text";
const AREA: &str = "mt-1.5 block w-full rounded-field border border-line bg-surface px-3 py-2 text-body text-text";
const CARD: &str = "rounded-card bg-surface p-5 shadow-card";

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Limits {
    owner: u32,
    learner: u32,
    total: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Maintenance {
    on: bool,
    message_en: String,
    message_fr: String,
    since: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Disclaimer {
    version: u32,
    en: String,
    fr: String,
    published_at: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Published {
    version: u32,
    published_at: f64,
    published_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct StatusItem {
    name: String,
    state: String,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct SettingsData {
    limits: Limits,
    maintenance: Maintenance,
    disclaimer: Option<Disclaimer>,
    history: Vec<Published>,
    status: Vec<StatusItem>,
}

/// A message under a card's button: (ok, text).
type Note = RwSignal<Option<(bool, String)>>;

fn note_view(note: Note) -> impl IntoView {
    move || {
        note.get().map(|(ok, t)| {
            view! { <p role=if ok { "status" } else { "alert" } class=format!("mt-3 rounded-btn px-3 py-2 text-callout {}", if ok { "bg-ok-bg text-ok-ink" } else { "bg-bad-bg text-bad-ink" })>{t}</p> }
        })
    }
}

#[component]
pub fn Settings(me: Me) -> impl IntoView {
    let data = RwSignal::new(None::<Result<SettingsData, String>>);
    let load = move || {
        spawn_local(async move {
            let r = api::get("/api/admin/settings").await;
            data.set(Some(if r.ok() { r.parse::<SettingsData>().ok_or_else(|| "The server sent something unexpected.".to_string()) } else { Err(r.message()) }));
        });
    };
    load();
    let reload = Callback::new(move |_| load());
    let writable = me.can("settings.write");
    // Kept here, above the cards, so a message survives the reload that follows a save.
    let notes: [Note; 3] = [RwSignal::new(None), RwSignal::new(None), RwSignal::new(None)];
    view! {
        <header class="pt-4 pb-6">
            <p class="text-eyebrow text-eyebrow-ink">"Owner"</p>
            <h1 class="text-large">"Settings"</h1>
            <p class="mt-2 max-w-[62ch] text-callout text-text-muted">"Changes apply at once and are recorded in the audit log. Keys are shown by status only, never their values."</p>
        </header>
        {move || match data.get() {
            None => view! { <p role="status" class="text-callout text-text-muted">"Loading…"</p> }.into_any(),
            Some(Err(e)) => view! { <p role="alert" class="rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> }.into_any(),
            Some(Ok(d)) => view! {
                <div class="grid gap-5 lg:grid-cols-2 lg:items-start">
                    <div class="grid min-w-0 gap-5">
                        <LimitsCard limits=d.limits writable=writable reload=reload note=notes[0] />
                        <MaintenanceCard m=d.maintenance writable=writable reload=reload note=notes[1] />
                        <StatusCard items=d.status />
                    </div>
                    <div class="min-w-0">
                        <DisclaimerCard current=d.disclaimer history=d.history writable=writable reload=reload note=notes[2] />
                    </div>
                </div>
            }.into_any(),
        }}
    }
}

#[component]
fn LimitsCard(limits: Limits, writable: bool, reload: Callback<()>, note: Note) -> impl IntoView {
    let owner = RwSignal::new(limits.owner.to_string());
    let learner = RwSignal::new(limits.learner.to_string());
    let total = RwSignal::new(limits.total.map(|t| t.to_string()).unwrap_or_default());
    let busy = RwSignal::new(false);
    let save = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let num = |s: String| s.trim().parse::<u32>().ok();
        let (Some(o), Some(l)) = (num(owner.get_untracked()), num(learner.get_untracked())) else {
            note.set(Some((false, "Enter whole numbers for the Owner and Learner limits.".into())));
            return;
        };
        let t = total.get_untracked();
        let t = t.trim();
        let total_v = if t.is_empty() { Value::Null } else if let Ok(n) = t.parse::<u32>() { json!(n) } else {
            note.set(Some((false, "Leave the overall limit empty, or enter a whole number.".into())));
            return;
        };
        busy.set(true);
        spawn_local(async move {
            let r = api::call("PUT", "/api/admin/settings/limits", Some(json!({ "owner": o, "learner": l, "total": total_v }))).await;
            busy.set(false);
            note.set(Some(if r.ok() { (true, "Limits saved. They apply to the next request.".into()) } else { (false, r.message()) }));
            if r.ok() {
                reload.run(());
            }
        });
    };
    let field = move |id: &'static str, label: &'static str, hint: &'static str, sig: RwSignal<String>, placeholder: &'static str| view! {
        <div>
            <label class="block text-callout font-semibold" for=id>{label}</label>
            <p class="text-footnote text-text-muted">{hint}</p>
            <input id=id type="text" inputmode="numeric" placeholder=placeholder class=format!("{FIELD} tabular max-w-[12rem]") prop:value=sig disabled=!writable on:input=move |ev| { sig.set(event_target_value(&ev)); note.set(None); } />
        </div>
    };
    view! {
        <section class=CARD aria-labelledby="limits-h">
            <p class="text-eyebrow text-eyebrow-ink">"Professor Aurel"</p>
            <h2 id="limits-h" class="text-headline">"Daily limits"</h2>
            <p class="mt-1 text-footnote text-text-muted">"Requests per account over the last 24 hours: grading, journal verdicts, practice questions and Ask Claude all count."</p>
            <form class="mt-4 grid gap-4" on:submit=save>
                {field("lim-owner", "You (the Owner)", "1 to 5,000.", owner, "")}
                {field("lim-learner", "Each other account", "0 to 1,000. 0 pauses Professor Aurel for everyone but you.", learner, "")}
                {field("lim-total", "All accounts together", "Optional cap on the whole course. Empty means no cap; your own requests never count against it.", total, "No cap")}
                {writable.then(|| view! { <div><button type="submit" class=PILL disabled=move || busy.get()>"Save limits"</button></div> })}
            </form>
            {note_view(note)}
        </section>
    }
}

#[component]
fn MaintenanceCard(m: Maintenance, writable: bool, reload: Callback<()>, note: Note) -> impl IntoView {
    let en = RwSignal::new(m.message_en.clone());
    let fr = RwSignal::new(m.message_fr.clone());
    let busy = RwSignal::new(false);
    let confirm = RwSignal::new(false);
    let on = m.on;
    let send = move |turn_on: bool, ok: &'static str| {
        busy.set(true);
        confirm.set(false);
        spawn_local(async move {
            let r = api::call("PUT", "/api/admin/settings/maintenance", Some(json!({ "on": turn_on, "messageEn": en.get_untracked(), "messageFr": fr.get_untracked() }))).await;
            busy.set(false);
            note.set(Some(if r.ok() { (true, ok.into()) } else { (false, r.message()) }));
            if r.ok() {
                reload.run(());
            }
        });
    };
    view! {
        <section class=CARD aria-labelledby="maint-h">
            <div class="flex flex-wrap items-baseline justify-between gap-2">
                <div>
                    <p class="text-eyebrow text-eyebrow-ink">"The course"</p>
                    <h2 id="maint-h" class="text-headline">"Maintenance Mode"</h2>
                </div>
                <span class=format!("rounded-full px-2 py-0.5 text-footnote font-semibold {}", if on { "bg-warn-bg text-warn-ink" } else { "bg-raised text-text" })>
                    {if on { m.since.map(|s| format!("On since {}", api::when(s))).unwrap_or_else(|| "On".into()) } else { "Off".into() }}
                </span>
            </div>
            <p class="mt-1 text-footnote text-text-muted">"While it's on, Learners see a “being updated” screen instead of the course, and Professor Aurel answers only staff. Saving still works, so nothing they did is lost. Staff use the course as usual, with a banner."</p>
            <div class="mt-4 grid gap-4">
                <div>
                    <label class="block text-callout font-semibold" for="maint-en">"Message in English (optional)"</label>
                    <textarea id="maint-en" rows="2" maxlength="400" placeholder="Back by 18:00 UTC." class=AREA prop:value=en disabled=!writable on:input=move |ev| en.set(event_target_value(&ev))></textarea>
                </div>
                <div>
                    <label class="block text-callout font-semibold" for="maint-fr" lang="en">"Message in French (optional)"</label>
                    <textarea id="maint-fr" lang="fr" rows="2" maxlength="400" placeholder="De retour vers 18 h UTC." class=AREA prop:value=fr disabled=!writable on:input=move |ev| fr.set(event_target_value(&ev))></textarea>
                </div>
            </div>
            {writable.then(|| view! {
                <div class="mt-4 flex flex-wrap gap-2">
                    {move || if on {
                        view! {
                            <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| send(false, "Maintenance Mode is off. Learners are back in the course.")>"Turn off"</button>
                            <button type="button" class=PILL_GHOST disabled=move || busy.get() on:click=move |_| send(true, "Messages saved.")>"Save messages"</button>
                        }.into_any()
                    } else if confirm.get() {
                        view! {
                            <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| send(true, "Maintenance Mode is on. Learners now see the “being updated” screen when they open or reload the course.")>"Turn on for every Learner"</button>
                            <button type="button" class=PILL_GHOST on:click=move |_| confirm.set(false)>"Cancel"</button>
                        }.into_any()
                    } else {
                        view! { <button type="button" class=PILL_GHOST on:click=move |_| confirm.set(true)>"Turn on…"</button> }.into_any()
                    }}
                </div>
            })}
            {note_view(note)}
        </section>
    }
}

#[component]
fn DisclaimerCard(current: Option<Disclaimer>, history: Vec<Published>, writable: bool, reload: Callback<()>, note: Note) -> impl IntoView {
    let version = current.as_ref().map_or(0, |d| d.version);
    let en = RwSignal::new(current.as_ref().map(|d| d.en.clone()).unwrap_or_default());
    let fr = RwSignal::new(current.as_ref().map(|d| d.fr.clone()).unwrap_or_default());
    let original = StoredValue::new((en.get_untracked(), fr.get_untracked()));
    let changed = move || (en.get(), fr.get()) != original.get_value();
    let preview = RwSignal::new(false);
    let confirm = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let publish = move |_| {
        busy.set(true);
        confirm.set(false);
        spawn_local(async move {
            let r = api::post("/api/admin/settings/disclaimer", json!({ "en": en.get_untracked(), "fr": fr.get_untracked() })).await;
            busy.set(false);
            if r.ok() {
                let v = r.body.get("version").and_then(Value::as_u64).unwrap_or(0);
                note.set(Some((true, format!("Version {v} published. Learners will be asked to accept it the next time they open the course."))));
                reload.run(());
            } else {
                note.set(Some((false, r.message())));
            }
        });
    };
    view! {
        <section class=CARD aria-labelledby="disc-h">
            <p class="text-eyebrow text-eyebrow-ink">"First launch"</p>
            <h2 id="disc-h" class="text-headline">"Disclaimer"</h2>
            <p class="mt-1 text-footnote text-text-muted">
                {current.as_ref().map(|d| format!("Version {}, published {}. ", d.version, api::date(d.published_at))).unwrap_or_default()}
                "Leave a blank line between paragraphs; **two asterisks** make text bold. Publishing a new version asks every Learner to read and accept it again."
            </p>
            <div class="mt-4 grid gap-4">
                <div>
                    <label class="block text-callout font-semibold" for="disc-en">"English"</label>
                    <textarea id="disc-en" rows="9" maxlength="4000" class=AREA prop:value=en disabled=!writable on:input=move |ev| { en.set(event_target_value(&ev)); note.set(None); }></textarea>
                </div>
                <div>
                    <label class="block text-callout font-semibold" for="disc-fr">"French"</label>
                    <textarea id="disc-fr" lang="fr" rows="9" maxlength="4000" class=AREA prop:value=fr disabled=!writable on:input=move |ev| { fr.set(event_target_value(&ev)); note.set(None); }></textarea>
                </div>
            </div>
            <div class="mt-4 flex flex-wrap gap-2">
                <button type="button" class=PILL_GHOST aria-expanded=move || preview.get().to_string() on:click=move |_| preview.update(|p| *p = !*p)>{move || if preview.get() { "Hide preview" } else { "Preview" }}</button>
                {writable.then(|| move || if confirm.get() {
                    view! {
                        <button type="button" class=PILL disabled=move || busy.get() on:click=publish>{format!("Publish version {}", version + 1)}</button>
                        <button type="button" class=PILL_GHOST on:click=move |_| confirm.set(false)>"Cancel"</button>
                    }.into_any()
                } else {
                    view! { <button type="button" class=PILL disabled=move || !changed() on:click=move |_| confirm.set(true)>"Publish…"</button> }.into_any()
                })}
            </div>
            {move || confirm.get().then(|| view! { <p class="mt-2 text-footnote text-text-muted">"Every Learner will see the sheet again and must accept it before continuing."</p> })}
            {note_view(note)}
            {move || preview.get().then(|| view! {
                <div class="mt-4 grid gap-4">
                    <div class="rounded-btn bg-raised p-4">
                        <p class="text-footnote font-semibold text-text-muted">"English preview"</p>
                        <div class="mt-2"><Rich text=en.get() /></div>
                    </div>
                    <div class="rounded-btn bg-raised p-4" lang="fr">
                        <p class="text-footnote font-semibold text-text-muted" lang="en">"French preview"</p>
                        <div class="mt-2"><Rich text=fr.get() /></div>
                    </div>
                </div>
            })}
            {(!history.is_empty()).then(|| view! {
                <h3 class="mt-6 text-callout font-semibold">"Versions"</h3>
                <ul class="mt-2 divide-y divide-line/40 text-callout">
                    {history.into_iter().map(|h| view! {
                        <li class="flex flex-wrap justify-between gap-x-4 py-2">
                            <span class="font-semibold">{format!("Version {}", h.version)}</span>
                            <span class="text-text-muted">{format!("{}{}", api::date(h.published_at), h.published_by.map(|b| format!(" · {b}")).unwrap_or_else(|| " · built in".into()))}</span>
                        </li>
                    }).collect_view()}
                </ul>
            })}
        </section>
    }
}

#[component]
fn StatusCard(items: Vec<StatusItem>) -> impl IntoView {
    view! {
        <section class=CARD aria-labelledby="keys-h">
            <p class="text-eyebrow text-eyebrow-ink">"Server"</p>
            <h2 id="keys-h" class="text-headline">"Keys & integrations"</h2>
            <p class="mt-1 text-footnote text-text-muted">"Secrets are set from the terminal with wrangler; this shows only whether they're there and working."</p>
            <ul class="mt-3 divide-y divide-line/40">
                {items.into_iter().map(|i| {
                    let (label, tone) = match i.state.as_str() {
                        "ok" => ("OK", "bg-ok-bg text-ok-ink"),
                        "warn" => ("Check", "bg-warn-bg text-warn-ink"),
                        _ => ("Missing", "bg-bad-bg text-bad-ink"),
                    };
                    view! {
                        <li class="grid gap-1 py-3">
                            <div class="flex items-center justify-between gap-3">
                                <span class="min-w-0 text-callout font-semibold break-words">{i.name}</span>
                                <span class=format!("shrink-0 rounded-full px-2 py-0.5 text-footnote font-semibold {tone}")>{label}</span>
                            </div>
                            <p class="text-footnote text-text-muted break-words">{i.detail}</p>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </section>
    }
}
