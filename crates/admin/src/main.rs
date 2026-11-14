//! Telosgard's admin dashboard (ADR 0006, 0007). A separate WASM bundle at /admin, so Learners never
//! download it. Staff sign in with two-factor; every screen calls the Worker's /api/admin routes,
//! which enforce the permissions this UI only reflects. English only for now.

mod api;
mod auth;
mod curriculum;
mod enrollment;
mod learners;
mod pages;
mod settings;
mod shell;

use api::Me;
use leptos::prelude::*;
use leptos::task::spawn_local;
pub use telosgard_ui::theme;

/// Where the dashboard stands for this browser.
#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    Loading,
    SignIn(Option<String>),
    /// The code step after a password; `to_course` when the sign-in started in the student app.
    Verify { to_course: bool },
    Setup(Me),
    NotStaff,
    Ready(Me),
    Offline(String),
}

#[derive(Clone, Copy)]
pub struct GateSignal(pub RwSignal<Gate>);

impl GateSignal {
    /// Asks the Worker who this is and what they may do, and moves to the matching screen.
    pub fn refresh(self) {
        spawn_local(async move {
            let r = api::get("/api/admin/me").await;
            let next = match (r.status, r.code()) {
                (200, _) => match r.parse::<Me>() {
                    Some(me) if me.ready => Gate::Ready(me),
                    Some(me) if me.blocked.as_deref() == Some("two_factor_required") => Gate::Setup(me),
                    Some(_) => {
                        // Session older than 12 hours: sign out so the next sign-in starts fresh.
                        let _ = api::post("/api/auth/sign-out", serde_json::json!({})).await;
                        Gate::SignIn(Some("Dashboard sessions last 12 hours. Sign in again.".into()))
                    }
                    None => Gate::Offline("The server sent something unexpected.".into()),
                },
                (401, _) => {
                    if window().location().hash().unwrap_or_default() == "#verify" {
                        Gate::Verify { to_course: true }
                    } else {
                        Gate::SignIn(None)
                    }
                }
                (403, Some("not_staff")) => Gate::NotStaff,
                _ => Gate::Offline(r.message()),
            };
            self.0.set(next);
        });
    }
}

#[component]
fn App() -> impl IntoView {
    let theme = RwSignal::new(theme::load());
    Effect::new(move |_| {
        let t = theme.get();
        theme::apply(t);
        theme::save(t);
    });
    provide_context(theme);

    let gate = GateSignal(RwSignal::new(Gate::Loading));
    provide_context(gate);
    gate.refresh();

    move || match gate.0.get() {
        Gate::Loading => view! { <auth::Centered><p role="status" class="text-callout text-text-muted">"Loading…"</p></auth::Centered> }.into_any(),
        Gate::SignIn(note) => view! { <auth::SignIn note=note /> }.into_any(),
        Gate::Verify { to_course } => view! { <auth::Verify to_course=to_course /> }.into_any(),
        Gate::Setup(me) => view! { <auth::Setup me=me /> }.into_any(),
        Gate::NotStaff => view! { <auth::NotStaff /> }.into_any(),
        Gate::Offline(msg) => view! { <auth::Offline message=msg /> }.into_any(),
        Gate::Ready(me) => view! { <shell::Shell me=me /> }.into_any(),
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
