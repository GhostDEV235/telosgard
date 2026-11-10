//! Gold School front-end. See docs/ARCHITECTURE.md.

mod catalog;
mod grading;
mod platform;
mod router;
mod rubrics;
mod screens;
mod state;
mod store;
mod theme;
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
    let disclaimer = state::DisclaimerOpen(RwSignal::new(!screens::shell::disclaimer_accepted_locally()));
    provide_context(progress);
    provide_context(toasts);
    provide_context(store);
    provide_context(disclaimer);

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

    // Content renders immediately; saved progress arrives when the platform answers.
    store.start(progress, theme, disclaimer);

    view! { <screens::shell::Shell /> }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
