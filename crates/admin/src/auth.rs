//! Sign-in, the two-factor code step, and two-factor setup (ADR 0007).

use crate::api::{self, Me};
use crate::GateSignal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};
use telosgard_ui::controls::{PILL, PILL_GHOST};

const FIELD: &str = "mt-1.5 block min-h-11 w-full rounded-field border border-line bg-surface px-3 text-body text-text";

#[component]
pub fn Centered(children: Children) -> impl IntoView {
    view! {
        <main class="gutter grid min-h-dvh place-items-center py-10">
            <div class="w-full max-w-[420px]">{children()}</div>
        </main>
    }
}

#[component]
fn Card(title: &'static str, #[prop(optional)] lead: Option<&'static str>, children: Children) -> impl IntoView {
    view! {
        <section class="rounded-card bg-surface p-6 shadow-card">
            <p class="text-eyebrow text-eyebrow-ink">"Telosgard · Admin"</p>
            <h1 class="mt-1 text-title1">{title}</h1>
            {lead.map(|l| view! { <p class="mt-2 text-callout text-text-muted">{l}</p> })}
            <div class="mt-5">{children()}</div>
        </section>
    }
}

fn error_line(err: RwSignal<Option<String>>) -> impl IntoView {
    move || err.get().map(|e| view! { <p role="alert" class="mt-3 rounded-btn bg-bad-bg px-3 py-2 text-callout text-bad-ink">{e}</p> })
}

fn value(ev: &leptos::ev::Event) -> String {
    event_target_value(ev)
}

#[component]
pub fn SignIn(note: Option<String>) -> impl IntoView {
    let gate = expect_context::<GateSignal>();
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let err = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        busy.set(true);
        err.set(None);
        spawn_local(async move {
            let r = api::post("/api/auth/sign-in/email", json!({ "email": email.get_untracked().trim(), "password": password.get_untracked() })).await;
            busy.set(false);
            if !r.ok() {
                err.set(Some(r.message()));
            } else if r.body.get("twoFactorRedirect").and_then(Value::as_bool) == Some(true) {
                gate.0.set(crate::Gate::Verify { to_course: false });
            } else {
                gate.refresh();
            }
        });
    };
    view! {
        <Centered>
            <Card title="Sign in" lead="For the Owner, Instructors and Support. Learners use the course itself.">
                {note.map(|n| view! { <p role="status" class="mb-4 rounded-btn bg-warn-bg px-3 py-2 text-callout text-warn-ink">{n}</p> })}
                <form on:submit=submit class="grid gap-4">
                    <label class="block text-callout font-semibold">
                        "Email"
                        <input type="email" autocomplete="username" required class=FIELD prop:value=email on:input=move |ev| email.set(value(&ev)) />
                    </label>
                    <label class="block text-callout font-semibold">
                        "Password"
                        <input type="password" autocomplete="current-password" required class=FIELD prop:value=password on:input=move |ev| password.set(value(&ev)) />
                    </label>
                    <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>
                        {move || if busy.get() { "Signing in…" } else { "Sign in" }}
                    </button>
                </form>
                {error_line(err)}
                <p class="mt-5 text-center text-footnote"><a class="underline underline-offset-4" href="/">"Go to the course"</a></p>
            </Card>
        </Centered>
    }
}

/// The code from the authenticator app, or one backup code.
#[component]
pub fn Verify(to_course: bool) -> impl IntoView {
    let gate = expect_context::<GateSignal>();
    let code = RwSignal::new(String::new());
    let backup = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let err = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        busy.set(true);
        err.set(None);
        spawn_local(async move {
            let path = if backup.get_untracked() { "/api/auth/two-factor/verify-backup-code" } else { "/api/auth/two-factor/verify-totp" };
            let c: String = code.get_untracked().chars().filter(|c| !c.is_whitespace()).collect();
            let r = api::post(path, json!({ "code": c })).await;
            busy.set(false);
            if !r.ok() {
                err.set(Some(if r.status == 401 { "That code didn't work. Codes change every 30 seconds; try the current one.".into() } else { r.message() }));
            } else if to_course {
                let _ = window().location().assign("/");
            } else {
                let _ = window().location().set_hash("overview");
                gate.refresh();
            }
        });
    };
    view! {
        <Centered>
            <Card title="Enter your code" lead="This account has two-factor sign-in. Open your authenticator app and enter the 6-digit code for Telosgard.">
                <form on:submit=submit class="grid gap-4">
                    <label class="block text-callout font-semibold">
                        {move || if backup.get() { "Backup code" } else { "6-digit code" }}
                        <input
                            type="text"
                            inputmode=move || if backup.get() { "text" } else { "numeric" }
                            autocomplete="one-time-code"
                            required
                            class=format!("{FIELD} tabular tracking-widest")
                            prop:value=code
                            on:input=move |ev| code.set(value(&ev))
                        />
                    </label>
                    <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>
                        {move || if busy.get() { "Checking…" } else { "Continue" }}
                    </button>
                </form>
                {error_line(err)}
                <button type="button" class="press mt-4 min-h-11 text-callout underline underline-offset-4" on:click=move |_| { backup.update(|b| *b = !*b); code.set(String::new()); }>
                    {move || if backup.get() { "Use the authenticator code instead" } else { "Lost your phone? Use a backup code" }}
                </button>
            </Card>
        </Centered>
    }
}

/// A QR code for the authenticator app, drawn as SVG.
fn qr_svg(text: &str) -> Option<impl IntoView> {
    use qrcodegen::{QrCode, QrCodeEcc};
    let qr = QrCode::encode_text(text, QrCodeEcc::Medium).ok()?;
    let n = qr.size();
    let mut d = String::new();
    for y in 0..n {
        for x in 0..n {
            if qr.get_module(x, y) {
                d.push_str(&format!("M{},{}h1v1h-1z", x + 4, y + 4));
            }
        }
    }
    let size = n + 8;
    // Always dark on white, whatever the theme: authenticator cameras expect it.
    Some(view! {
        <svg viewBox=format!("0 0 {size} {size}") class="mx-auto size-48 rounded-btn" role="img" aria-label="QR code for your authenticator app" shape-rendering="crispEdges">
            <rect width=size height=size fill="#ffffff" />
            <path d=d fill="#111111" />
        </svg>
    })
}

#[derive(Clone, PartialEq)]
struct Enrol {
    uri: String,
    secret: String,
    backup: Vec<String>,
}

#[component]
pub fn Setup(me: Me) -> impl IntoView {
    let gate = expect_context::<GateSignal>();
    let password = RwSignal::new(String::new());
    let code = RwSignal::new(String::new());
    let enrol = RwSignal::new(None::<Enrol>);
    let saved = RwSignal::new(false);
    let copied = RwSignal::new(false);
    let busy = RwSignal::new(false);
    let err = RwSignal::new(None::<String>);

    let start = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        busy.set(true);
        err.set(None);
        spawn_local(async move {
            let r = api::post("/api/auth/two-factor/enable", json!({ "password": password.get_untracked() })).await;
            busy.set(false);
            let uri = r.body.get("totpURI").and_then(Value::as_str).unwrap_or_default().to_string();
            if !r.ok() || uri.is_empty() {
                err.set(Some(r.message()));
                return;
            }
            let secret = uri.split("secret=").nth(1).and_then(|s| s.split('&').next()).unwrap_or_default().to_string();
            let backup = r.body.get("backupCodes").and_then(Value::as_array).map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect()).unwrap_or_default();
            password.set(String::new());
            enrol.set(Some(Enrol { uri, secret, backup }));
        });
    };
    let confirm = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        busy.set(true);
        err.set(None);
        spawn_local(async move {
            let c: String = code.get_untracked().chars().filter(|c| c.is_ascii_digit()).collect();
            let r = api::post("/api/auth/two-factor/verify-totp", json!({ "code": c })).await;
            busy.set(false);
            if r.ok() {
                gate.refresh();
            } else {
                err.set(Some("That code didn't match. Check the app shows Telosgard and enter its current code.".into()));
            }
        });
    };
    let sign_out = move |_| {
        spawn_local(async move {
            let _ = api::post("/api/auth/sign-out", json!({})).await;
            gate.0.set(crate::Gate::SignIn(None));
        });
    };
    let email = me.email.clone().unwrap_or_default();
    let role = me.role_label();

    view! {
        <Centered>
            <Card title="Turn on two-factor" lead="Every dashboard account needs it. You'll enter a code from an authenticator app (Apple Passwords, 1Password, Google Authenticator…) each time you sign in.">
                <p class="mb-4 text-footnote text-text-muted">{format!("Signed in as {email} · {role}")}</p>
                {move || match enrol.get() {
                    None => view! {
                        <form on:submit=start class="grid gap-4">
                            <label class="block text-callout font-semibold">
                                "Confirm your password"
                                <input type="password" autocomplete="current-password" required class=FIELD prop:value=password on:input=move |ev| password.set(value(&ev)) />
                            </label>
                            <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>"Set up"</button>
                        </form>
                    }.into_any(),
                    Some(e) => {
                        let codes = e.backup.join("\n");
                        let codes_for_copy = codes.clone();
                        view! {
                            <ol class="grid gap-5">
                                <li>
                                    <p class="text-headline">"1. Add Telosgard to your authenticator"</p>
                                    <div class="mt-3">{qr_svg(&e.uri)}</div>
                                    <p class="mt-3 text-callout text-text-muted">"Scan the code, or on this phone tap "<a class="underline underline-offset-4" href=e.uri.clone()>"open in authenticator"</a>". Or type this key:"</p>
                                    <p class="tabular mt-1 select-all break-all rounded-btn bg-raised px-3 py-2 font-mono text-callout">{e.secret.clone()}</p>
                                </li>
                                <li>
                                    <p class="text-headline">"2. Save your backup codes"</p>
                                    <p class="mt-1 text-callout text-text-muted">"Each works once if you lose your phone. Keep them in your password manager, not on this device alone."</p>
                                    <pre class="tabular mt-2 select-all rounded-btn bg-raised px-3 py-2 font-mono text-callout leading-7">{codes}</pre>
                                    <div class="mt-2 flex flex-wrap items-center gap-3">
                                        <button type="button" class=PILL_GHOST on:click=move |_| {
                                            let text = codes_for_copy.clone();
                                            spawn_local(async move { copied.set(api::copy(&text).await); });
                                        }>"Copy codes"</button>
                                        {move || copied.get().then(|| view! { <span role="status" class="text-footnote text-text-muted">"Copied"</span> })}
                                    </div>
                                    <label class="mt-3 flex min-h-11 items-center gap-3 text-callout">
                                        <input type="checkbox" class="size-5" prop:checked=saved on:change=move |ev| saved.set(event_target_checked(&ev)) />
                                        "I've saved my backup codes"
                                    </label>
                                </li>
                                <li>
                                    <p class="text-headline">"3. Enter the app's code"</p>
                                    <form on:submit=confirm class="mt-2 grid gap-3">
                                        <label class="block text-callout font-semibold">
                                            "6-digit code"
                                            <input type="text" inputmode="numeric" autocomplete="one-time-code" required class=format!("{FIELD} tabular tracking-widest") prop:value=code on:input=move |ev| code.set(value(&ev)) />
                                        </label>
                                        <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get() || !saved.get()>"Turn on two-factor"</button>
                                    </form>
                                </li>
                            </ol>
                        }.into_any()
                    }
                }}
                {error_line(err)}
                <button type="button" class="press mt-5 min-h-11 text-callout underline underline-offset-4" on:click=sign_out>"Sign out"</button>
            </Card>
        </Centered>
    }
}

#[component]
pub fn NotStaff() -> impl IntoView {
    let gate = expect_context::<GateSignal>();
    let sign_out = move |_| {
        spawn_local(async move {
            let _ = api::post("/api/auth/sign-out", json!({})).await;
            gate.0.set(crate::Gate::SignIn(None));
        });
    };
    view! {
        <Centered>
            <Card title="No dashboard access" lead="This account is a Learner account. The dashboard is for the Owner, Instructors and Support.">
                <div class="flex flex-wrap gap-3">
                    <a href="/" class=PILL>"Go to the course"</a>
                    <button type="button" class=PILL_GHOST on:click=sign_out>"Sign in as someone else"</button>
                </div>
            </Card>
        </Centered>
    }
}

#[component]
pub fn Offline(message: String) -> impl IntoView {
    let gate = expect_context::<GateSignal>();
    view! {
        <Centered>
            <Card title="Can't reach Telosgard">
                <p role="alert" class="text-callout">{message}</p>
                <button type="button" class=format!("{PILL} mt-5") on:click=move |_| gate.refresh()>"Try again"</button>
            </Card>
        </Centered>
    }
}
