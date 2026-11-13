//! Enrollment (D1): shareable Invite Codes and single-use Invite Links. Telosgard stays invite-only.

use crate::api;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde_json::{json, Value};
use telosgard_ui::controls::{PILL, PILL_GHOST};

const FIELD: &str = "mt-1.5 block min-h-11 w-full rounded-field border border-line bg-surface px-3 text-body text-text";

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Link {
    id: String,
    created_at: f64,
    expires_at: f64,
    used_at: Option<f64>,
    used_by: Option<String>,
    created_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Code {
    id: String,
    code: String,
    created_at: f64,
    expires_at: f64,
    max_uses: u32,
    uses: u32,
    revoked_at: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct Data {
    links: Vec<Link>,
    codes: Vec<Code>,
}

fn origin() -> String {
    window().location().origin().unwrap_or_default()
}

#[component]
pub fn Enrollment() -> impl IntoView {
    let data = RwSignal::new(None::<Data>);
    let err = RwSignal::new(None::<String>);
    let reload = move || {
        spawn_local(async move {
            let r = api::get("/api/admin/enrollment").await;
            match r.parse::<Data>() {
                Some(d) if r.ok() => data.set(Some(d)),
                _ => err.set(Some(r.message())),
            }
        });
    };
    reload();

    // New Invite Code.
    let code = RwSignal::new(String::new());
    let uses = RwSignal::new("10".to_string());
    let days = RwSignal::new("14".to_string());
    let form_err = RwSignal::new(None::<String>);
    let busy = RwSignal::new(false);
    let create_code = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        busy.set(true);
        form_err.set(None);
        spawn_local(async move {
            let body = json!({ "code": code.get_untracked(), "maxUses": uses.get_untracked().trim().parse::<u32>().unwrap_or(0), "days": days.get_untracked().trim().parse::<u32>().unwrap_or(0) });
            let r = api::post("/api/admin/enrollment/codes", body).await;
            busy.set(false);
            if r.ok() {
                code.set(String::new());
                reload();
            } else {
                form_err.set(Some(r.message()));
            }
        });
    };
    let toggle_code = move |id: String, revoke: bool| {
        spawn_local(async move {
            let r = api::post(&format!("/api/admin/enrollment/codes/{id}/{}", if revoke { "revoke" } else { "restore" }), json!({})).await;
            if !r.ok() {
                err.set(Some(r.message()));
            }
            reload();
        });
    };

    // New Invite Link: shown once, since only its fingerprint is stored.
    let fresh = RwSignal::new(None::<String>);
    let copied = RwSignal::new(None::<String>);
    let create_link = move |_| {
        spawn_local(async move {
            let r = api::post("/api/admin/enrollment/links", json!({})).await;
            match r.body.get("url").and_then(Value::as_str) {
                Some(u) if r.ok() => {
                    fresh.set(Some(u.to_string()));
                    reload();
                }
                _ => err.set(Some(r.message())),
            }
        });
    };
    let copy = move |text: String| {
        spawn_local(async move {
            if api::copy(&text).await {
                copied.set(Some(text));
            }
        });
    };
    let delete_link = move |id: String| {
        spawn_local(async move {
            let _ = api::delete(&format!("/api/admin/enrollment/links/{id}")).await;
            reload();
        });
    };

    view! {
        <header class="pt-4 pb-6">
            <p class="text-eyebrow text-eyebrow-ink">"People"</p>
            <h1 class="text-large">"Enrollment"</h1>
            <p class="mt-2 max-w-[62ch] text-callout text-text-muted">"Telosgard is invite-only. A code lets a group join until it runs out or expires; a link lets one person join, once, within 7 days."</p>
        </header>
        {move || err.get().map(|e| view! { <p role="alert" class="mb-4 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })}

        <section class="rounded-card bg-surface p-5 shadow-card" aria-labelledby="codes-h">
            <h2 id="codes-h" class="text-title2">"Invite Codes"</h2>
            <form class="mt-4 grid gap-3 sm:grid-cols-[minmax(0,1.4fr)_minmax(0,0.7fr)_minmax(0,0.7fr)_auto] sm:items-end" on:submit=create_code>
                <label class="block text-callout font-semibold">
                    "Code"
                    <input type="text" autocapitalize="characters" autocomplete="off" placeholder="Leave empty for one like GOLD-7KQ2" class=format!("{FIELD} uppercase placeholder:normal-case") prop:value=code on:input=move |ev| code.set(event_target_value(&ev)) />
                </label>
                <label class="block text-callout font-semibold">
                    "Uses"
                    <input type="number" min="1" max="500" required class=FIELD prop:value=uses on:input=move |ev| uses.set(event_target_value(&ev)) />
                </label>
                <label class="block text-callout font-semibold">
                    "Days"
                    <input type="number" min="1" max="90" required class=FIELD prop:value=days on:input=move |ev| days.set(event_target_value(&ev)) />
                </label>
                <button type="submit" class=PILL disabled=move || busy.get()>"Create code"</button>
            </form>
            {move || form_err.get().map(|e| view! { <p role="alert" class="mt-2 text-callout text-bad-ink">{e}</p> })}
            {move || {
                let codes = data.get().map(|d| d.codes).unwrap_or_default();
                if codes.is_empty() {
                    view! { <p class="mt-4 text-callout text-text-muted">"No codes yet."</p> }.into_any()
                } else {
                    let now = js_sys::Date::now();
                    view! {
                        <ul class="mt-4 divide-y divide-line/40">
                            {codes.into_iter().map(|c| {
                                let (status, tone) = if c.revoked_at.is_some() { ("Revoked", "bg-raised text-text") }
                                    else if c.expires_at <= now { ("Expired", "bg-raised text-text") }
                                    else if c.uses >= c.max_uses { ("Used up", "bg-warn-bg text-warn-ink") }
                                    else { ("Active", "bg-ok-bg text-ok-ink") };
                                let share = format!("{}/#join-{}", origin(), c.code);
                                let share2 = share.clone();
                                let revoked = c.revoked_at.is_some();
                                let id = c.id.clone();
                                view! {
                                    <li class="grid gap-2 py-3 md:grid-cols-[minmax(0,1fr)_auto] md:items-center">
                                        <div class="min-w-0">
                                            <p class="flex flex-wrap items-center gap-2">
                                                <span class="font-mono text-headline">{c.code.clone()}</span>
                                                <span class=format!("rounded-full px-2 py-0.5 text-footnote font-semibold {tone}")>{status}</span>
                                            </p>
                                            <p class="tabular text-footnote text-text-muted">{format!("{} of {} used · expires {}", c.uses, c.max_uses, api::date(c.expires_at))}</p>
                                            <p class="break-all text-footnote text-text-muted">{share}</p>
                                        </div>
                                        <div class="flex flex-wrap gap-2">
                                            <button type="button" class=PILL_GHOST on:click=move |_| copy(share2.clone())>
                                                {let s = format!("{}/#join-{}", origin(), c.code); move || if copied.get().as_deref() == Some(s.as_str()) { "Copied" } else { "Copy link" }}
                                            </button>
                                            <button type="button" class=PILL_GHOST on:click=move |_| toggle_code(id.clone(), !revoked)>{if revoked { "Restore" } else { "Revoke" }}</button>
                                        </div>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }
            }}
        </section>

        <section class="mt-6 rounded-card bg-surface p-5 shadow-card" aria-labelledby="links-h">
            <div class="flex flex-wrap items-center justify-between gap-3">
                <h2 id="links-h" class="text-title2">"Invite Links"</h2>
                <button type="button" class=PILL_GHOST on:click=create_link>"Create invite link"</button>
            </div>
            {move || fresh.get().map(|u| {
                let u2 = u.clone();
                view! {
                    <div class="mt-4 rounded-btn bg-raised p-3" role="status">
                        <p class="text-callout font-semibold">"New link. Copy it now: it can't be shown again."</p>
                        <p class="mt-1 break-all font-mono text-footnote select-all">{u}</p>
                        <button type="button" class=format!("{PILL} mt-2") on:click=move |_| copy(u2.clone())>
                            {move || if copied.get().is_some_and(|c| fresh.get().as_deref() == Some(c.as_str())) { "Copied" } else { "Copy link" }}
                        </button>
                    </div>
                }
            })}
            {move || {
                let links = data.get().map(|d| d.links).unwrap_or_default();
                if links.is_empty() {
                    view! { <p class="mt-4 text-callout text-text-muted">"No links yet."</p> }.into_any()
                } else {
                    let now = js_sys::Date::now();
                    view! {
                        <ul class="mt-4 divide-y divide-line/40">
                            {links.into_iter().map(|l| {
                                let state = match (&l.used_at, &l.used_by) {
                                    (Some(t), Some(who)) => format!("Used by {who} on {}", api::date(*t)),
                                    (Some(t), None) => format!("Used on {}", api::date(*t)),
                                    _ if l.expires_at <= now => "Expired".into(),
                                    _ => format!("Unused · expires {}", api::date(l.expires_at)),
                                };
                                let unused = l.used_at.is_none();
                                let id = l.id.clone();
                                view! {
                                    <li class="flex flex-wrap items-center justify-between gap-2 py-3">
                                        <div class="min-w-0">
                                            <p class="text-callout">{state}</p>
                                            <p class="text-footnote text-text-muted">{format!("Created {}{}", api::date(l.created_at), l.created_by.as_ref().map(|c| format!(" by {c}")).unwrap_or_default())}</p>
                                        </div>
                                        {unused.then(|| view! { <button type="button" class=PILL_GHOST on:click=move |_| delete_link(id.clone())>"Delete"</button> })}
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }
            }}
        </section>
    }
}
