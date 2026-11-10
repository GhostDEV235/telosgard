//! Gold School front-end. See docs/ARCHITECTURE.md.

mod catalog;
mod router;
mod screens;
mod state;
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

    provide_context(state::Progress::new());
    provide_context(state::Toasts::new());

    let theme = RwSignal::new(theme::load());
    Effect::new(move |_| {
        let t = theme.get();
        theme::apply(t);
        theme::save(t);
    });
    provide_context(theme);

    view! { <screens::shell::Shell /> }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
