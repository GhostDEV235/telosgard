//! Accounts (ADR 0004): the sign-in screen, joining with an invite link, and the account section of
//! the Appearance sheet (sign out; for the owner, invite a tester). Telosgard is invite-only.

use crate::platform::{self, PlatformError};
use crate::ui::controls::{PILL, PILL_GHOST};
use leptos::prelude::*;
use leptos::task::spawn_local;

const FIELD: &str = "mt-1.5 min-h-12 w-full rounded-field bg-surface px-3.5 text-body shadow-card placeholder:text-text-muted";

fn reload_to(hash: &str) {
    let loc = window().location();
    let _ = loc.set_hash(hash);
    let _ = loc.reload();
}

#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <main class="gutter grid min-h-dvh place-items-center py-10">
            <div class="w-full max-w-sm">
                <div class="mb-6 flex items-center gap-2.5">
                    <span class="grid size-8 place-items-center rounded-[9px] bg-gold-fill pb-px font-display text-headline font-semibold text-on-gold" aria-hidden="true">"T"</span>
                    <span class="font-display text-title2">"Telosgard"</span>
                </div>
                {children()}
            </div>
        </main>
    }
}

#[component]
pub fn SignIn() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let (e, p) = (email.get_untracked().trim().to_string(), password.get_untracked());
        if e.is_empty() || p.is_empty() {
            return error.set(Some("Enter your email and password.".into()));
        }
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match platform::auth_sign_in(&e, &p).await {
                Ok(()) => reload_to("today"),
                Err(err) => {
                    error.set(Some(match err {
                        PlatformError::Auth(m) if m.to_lowercase().contains("invalid") => "That email and password don't match.".into(),
                        other => other.describe(),
                    }));
                    busy.set(false);
                }
            }
        });
    };
    view! {
        <Card>
            <h1 class="text-title1">"Sign in to " <em>"learn with purpose."</em></h1>
            <p class="mt-2 text-callout text-text-muted">"Telosgard is invite-only. New here? Open the invite link you were sent."</p>
            <form class="mt-6 grid gap-4" on:submit=submit>
                <label class="block">
                    <span class="text-callout font-semibold">"Email"</span>
                    <input type="email" autocomplete="email" required class=FIELD prop:value=move || email.get() on:input=move |ev| email.set(event_target_value(&ev)) />
                </label>
                <label class="block">
                    <span class="text-callout font-semibold">"Password"</span>
                    <input type="password" autocomplete="current-password" required class=FIELD prop:value=move || password.get() on:input=move |ev| password.set(event_target_value(&ev)) />
                </label>
                {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">{e}</p> })}
                <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>{move || if busy.get() { "Signing in…" } else { "Sign in" }}</button>
            </form>
            <p class="mt-6 text-footnote text-text-muted">"Forgot your password? Ask the person who invited you to reset it."</p>
        </Card>
    }
}

#[component]
pub fn Join(token: String) -> impl IntoView {
    let token = StoredValue::new(token);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let (n, e, p) = (name.get_untracked().trim().to_string(), email.get_untracked().trim().to_string(), password.get_untracked());
        if n.is_empty() || e.is_empty() {
            return error.set(Some("Enter your name and email.".into()));
        }
        if p.chars().count() < 10 {
            return error.set(Some("Choose a password of at least 10 characters.".into()));
        }
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match platform::auth_sign_up(&n, &e, &p, &token.get_value()).await {
                Ok(()) => reload_to("today"),
                Err(err) => {
                    error.set(Some(match err {
                        PlatformError::Auth(m) if m.to_lowercase().contains("invite") => "This invite link has been used or has expired. Ask for a new one.".into(),
                        PlatformError::Auth(m) if m.to_lowercase().contains("exist") => "There's already an account with that email. Sign in instead.".into(),
                        other => other.describe(),
                    }));
                    busy.set(false);
                }
            }
        });
    };
    view! {
        <Card>
            <h1 class="text-title1">"You're " <em>"invited."</em></h1>
            <p class="mt-2 text-callout text-text-muted">"Create your Telosgard account: a patient, risk-first course in trading gold. Education, not financial advice."</p>
            <form class="mt-6 grid gap-4" on:submit=submit>
                <label class="block">
                    <span class="text-callout font-semibold">"Your name"</span>
                    <input type="text" autocomplete="name" required class=FIELD prop:value=move || name.get() on:input=move |ev| name.set(event_target_value(&ev)) />
                </label>
                <label class="block">
                    <span class="text-callout font-semibold">"Email"</span>
                    <input type="email" autocomplete="email" required class=FIELD prop:value=move || email.get() on:input=move |ev| email.set(event_target_value(&ev)) />
                </label>
                <label class="block">
                    <span class="text-callout font-semibold">"Password"</span>
                    <input type="password" autocomplete="new-password" minlength="10" required class=FIELD prop:value=move || password.get() on:input=move |ev| password.set(event_target_value(&ev)) />
                    <span class="mt-1 block text-footnote text-text-muted">"At least 10 characters."</span>
                </label>
                {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">{e}</p> })}
                <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>{move || if busy.get() { "Creating your account…" } else { "Create account" }}</button>
            </form>
            <a href="#today" on:click=move |_| reload_to("today") class="mt-6 inline-block text-callout underline underline-offset-4">"I already have an account"</a>
        </Card>
    }
}

/// Inside the Appearance sheet: who is signed in, sign out, and (owner) invite a tester.
#[component]
pub fn AccountSection() -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    let toasts = expect_context::<crate::state::Toasts>();
    let invite = RwSignal::new(None::<platform::Invite>);
    let busy = RwSignal::new(false);
    let make_invite = move |_| {
        busy.set(true);
        spawn_local(async move {
            match platform::invite_create().await {
                Ok(i) => invite.set(Some(i)),
                Err(e) => toasts.show(e.describe()),
            }
            busy.set(false);
        });
    };
    let copy = move |_| {
        if let Some(i) = invite.get_untracked() {
            spawn_local(async move {
                toasts.show(if platform::copy_text(&i.url).await { "Invite link copied" } else { "Select the link and copy it" });
            });
        }
    };
    let sign_out = move |_| {
        spawn_local(async move {
            let _ = platform::auth_sign_out().await;
            reload_to("today");
        });
    };
    move || {
        store.account.get().map(|(email, owner)| {
            view! {
                <hr class="my-5 border-line/30" />
                <p class="text-eyebrow text-eyebrow-ink">"Account"</p>
                <p class="mt-1 text-callout">{format!("Signed in as {}", email.clone().unwrap_or_default())}</p>
                {owner.then(|| view! {
                    <div class="mt-4">
                        <button type="button" class=PILL_GHOST disabled=move || busy.get() on:click=make_invite>"Invite a tester"</button>
                        <p class="mt-2 text-footnote text-text-muted">"A single-use link, valid for 7 days. Send it however you like; testers can ask Professor Aurel 50 things a day."</p>
                        {move || invite.get().map(|i| view! {
                            <div class="mt-3 rounded-btn bg-surface p-3 shadow-card">
                                <input readonly aria-label="Invite link" class="w-full truncate bg-transparent text-footnote" prop:value=i.url.clone() on:focus=move |ev| { let _ = event_target::<web_sys::HtmlInputElement>(&ev).select(); } />
                                <button type="button" class=format!("{PILL} mt-2") on:click=copy>"Copy link"</button>
                            </div>
                        })}
                    </div>
                })}
                <button type="button" class="press mt-4 min-h-11 text-callout underline underline-offset-4" on:click=sign_out>"Sign out"</button>
            }
        })
    }
}
