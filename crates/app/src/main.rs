//! Telosgard front-end. See docs/ARCHITECTURE.md.

mod catalog;
mod grading;
mod i18n;
mod journal_data;
mod platform;
mod router;
mod screens;
mod state;
mod store;
pub use telosgard_ui::theme;
mod ui;

use leptos::prelude::*;
use router::Route;

fn current_route() -> Route {
    Route::parse(&window().location().hash().unwrap_or_default())
}

#[component]
fn App() -> impl IntoView {
    let route = RwSignal::new(current_route());
    provide_context(route);
    let _ = window_event_listener(leptos::ev::hashchange, move |_| {
        route.set(current_route());
        window().scroll_to_with_x_and_y(0.0, 0.0);
    });

    let progress = state::Progress::new();
    let toasts = state::Toasts::new();
    let store = store::Store::new(toasts);
    let disclaimer = state::DisclaimerOpen(RwSignal::new(screens::shell::disclaimer_accepted_locally().is_none()));
    let notices = state::SiteNotices::new();
    provide_context(notices);
    provide_context(progress);
    provide_context(toasts);
    provide_context(store);
    provide_context(disclaimer);
    let journal = journal_data::JournalData::new(store);
    provide_context(journal);
    journal.watch_and_load(progress);

    let theme = RwSignal::new(theme::load());
    Effect::new(move |_| {
        let t = theme.get();
        theme::apply(t);
        theme::save(t);
        let name = match t {
            theme::ThemeChoice::System => "system",
            theme::ThemeChoice::Dark => "dark",
            theme::ThemeChoice::Light => "light",
        };
        store.update_profile(move |p| p.theme = name.to_string());
    });
    provide_context(theme);

    // Language: applied before the first render, saved per device and to the profile.
    let lang: i18n::LangSignal = RwSignal::new(i18n::load());
    i18n::apply(lang.get_untracked());
    provide_context(lang);
    Effect::new(move |prev: Option<i18n::Lang>| {
        let l = lang.get();
        if prev.is_some_and(|p| p != l) {
            i18n::save(l);
            store.update_profile(move |p| p.lang = Some(l.code().to_string()));
        }
        l
    });
    // Kept outside the shell, so the Appearance sheet stays open across a language switch.
    provide_context(screens::shell::AppearanceOpen(RwSignal::new(false)));

    // Content renders immediately; saved progress arrives when the platform answers.
    store.start(progress, theme, disclaimer, lang, notices);

    // A language change re-renders the whole shell in the new language.
    view! {
        {move || {
            i18n::apply(lang.get());
            view! { <screens::shell::Shell /> }
        }}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
