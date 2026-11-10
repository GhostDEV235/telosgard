//! App frame: routed screen, tab bar, sub-screen bar, toast, appearance and disclaimer sheets.

use crate::router::{DrillKind, Route, Tab};
use crate::screens::{home::Home, journal, lesson::LessonScreen, practice, tools};
use crate::state::DisclaimerOpen;
use crate::theme::ThemeChoice;
use crate::ui::controls::{PILL, Segmented};
use crate::ui::sheet::{Sheet, Toast};
use leptos::prelude::*;

const DISCLAIMER_KEY: &str = "gs-disclaimer";

#[component]
pub fn Shell() -> impl IntoView {
    let route = expect_context::<RwSignal<Route>>();
    let disclaimer = expect_context::<DisclaimerOpen>();
    let appearance = RwSignal::new(false);
    provide_context(AppearanceOpen(appearance));

    view! {
        <div class="min-h-dvh pb-[calc(84px+env(safe-area-inset-bottom,0px))]">
            {move || match route.get() {
                Route::Today => view! { <Home /> }.into_any(),
                Route::Lesson(id) => view! { <LessonScreen id=id /> }.into_any(),
                Route::Practice => view! { <practice::Practice /> }.into_any(),
                Route::Drill(DrillKind::Direction) => view! { <practice::DirectionDrill /> }.into_any(),
                Route::Drill(kind) => view! { <practice::ChoiceDrill kind=kind /> }.into_any(),
                Route::Tools => view! { <tools::Tools /> }.into_any(),
                Route::Plan => view! { <crate::screens::plan::Plan /> }.into_any(),
                Route::Journal => view! { <journal::Journal /> }.into_any(),
                Route::Readiness => view! { <journal::Readiness /> }.into_any(),
                Route::GraderLab => view! { <crate::screens::lab::GraderLab /> }.into_any(),
                Route::Checklist => view! { <crate::screens::checklist::Checklist /> }.into_any(),
                Route::Review => view! { <crate::screens::review::Review /> }.into_any(),
                Route::TradeNew => view! { <crate::screens::trade::TradeNew /> }.into_any(),
                Route::TradeSkip => view! { <crate::screens::trade::TradeSkip /> }.into_any(),
                Route::Trade(id) => view! { <crate::screens::trade::TradeDetail id=id /> }.into_any(),
            }}
        </div>
        <TabBar />
        <Toast />
        <AppearanceSheet open=appearance />
        <Disclaimer open=disclaimer.0 />
    }
}

/// Fast first-paint hint; the profile document is the source of truth once loaded.
pub fn disclaimer_accepted_locally() -> bool {
    window().local_storage().ok().flatten().and_then(|s| s.get_item(DISCLAIMER_KEY).ok().flatten()).is_some()
}

#[derive(Clone, Copy)]
pub struct AppearanceOpen(pub RwSignal<bool>);

/// Large-title header used by the four top-level tabs.
#[component]
pub fn LargeTitle(title: &'static str, #[prop(into, default = String::new())] eyebrow: String) -> impl IntoView {
    let appearance = expect_context::<AppearanceOpen>().0;
    view! {
        <header class="flex items-end justify-between gap-4 pt-4">
            <div class="min-w-0">
                <p class="text-eyebrow uppercase text-text-muted">{eyebrow}</p>
                <h1 class="text-large">{title}</h1>
            </div>
            <button
                type="button"
                aria-label="Appearance and about"
                class="press mb-1 grid size-10 shrink-0 place-items-center rounded-full bg-surface text-title2 shadow-card"
                on:click=move |_| appearance.set(true)
            >
                <svg viewBox="0 0 24 24" class="size-5" aria-hidden="true">
                    <circle cx="12" cy="12" r="8.5" fill="none" stroke="currentColor" stroke-width="1.8" />
                    <path d="M12 3.5a8.5 8.5 0 0 1 0 17z" fill="currentColor" />
                </svg>
            </button>
        </header>
    }
}

/// Sticky glass bar with a back link, for screens below a tab.
#[component]
pub fn SubBar(back: &'static str, back_label: &'static str, #[prop(into)] title: String) -> impl IntoView {
    view! {
        <div class="glass sticky top-[env(safe-area-inset-top,0px)] z-20 border-b border-line/30">
            <div class="gutter mx-auto grid min-h-12 max-w-[1040px] grid-cols-[1fr_auto_1fr] items-center gap-2">
                <a href=back class="press -ml-2 inline-flex min-h-11 items-center gap-1 justify-self-start rounded-btn px-2 text-body">
                    <span aria-hidden="true" class="text-title2 leading-none">"‹"</span>
                    {back_label}
                </a>
                <p class="max-w-[52vw] truncate text-headline">{title}</p>
                <span></span>
            </div>
        </div>
    }
}

fn tab_icon(tab: Tab) -> impl IntoView {
    let path = match tab {
        Tab::Today => "M4 6.5A2.5 2.5 0 0 1 6.5 4h11A2.5 2.5 0 0 1 20 6.5v5a2.5 2.5 0 0 1-2.5 2.5h-11A2.5 2.5 0 0 1 4 11.5zM6 17.5h12M8 20.5h8",
        Tab::Practice => "M7 3v4M7 15v6M17 3v6M17 17v4M5 7h4v8H5zM15 9h4v8h-4z",
        Tab::Tools => "M6 3.5h12A1.5 1.5 0 0 1 19.5 5v14a1.5 1.5 0 0 1-1.5 1.5H6A1.5 1.5 0 0 1 4.5 19V5A1.5 1.5 0 0 1 6 3.5zM7.5 7h9v3h-9zM8 14h.01M12 14h.01M16 14h.01M8 17.5h.01M12 17.5h.01M16 17.5h.01",
        Tab::Journal => "M6 3.5h11.5a1 1 0 0 1 1 1v15a1 1 0 0 1-1 1H6a1.5 1.5 0 0 1-1.5-1.5V5A1.5 1.5 0 0 1 6 3.5zM8.5 3.5v17M11.5 8h4M11.5 11.5h4",
    };
    view! {
        <svg viewBox="0 0 24 24" class="size-6" aria-hidden="true">
            <path d=path fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    }
}

#[component]
fn TabBar() -> impl IntoView {
    let route = expect_context::<RwSignal<Route>>();
    let tabs = [(Tab::Today, "#today", "Today"), (Tab::Practice, "#practice", "Practice"), (Tab::Tools, "#size", "Tools"), (Tab::Journal, "#journal", "Journal")];
    view! {
        <nav aria-label="Main" class="glass gutter fixed inset-x-0 bottom-0 z-30 border-t border-line/30 pb-[env(safe-area-inset-bottom,0px)]">
            <ul class="mx-auto flex max-w-[560px] justify-around">
                {tabs
                    .into_iter()
                    .map(|(tab, href, label)| {
                        let active = move || route.get().tab() == tab;
                        view! {
                            <li>
                                <a
                                    href=href
                                    aria-current=move || active().then_some("page")
                                    class=move || {
                                        format!(
                                            "press flex min-h-14 min-w-16 flex-col items-center justify-center gap-0.5 px-2 text-[11px] font-semibold {}",
                                            if active() { "text-text" } else { "text-text-muted" },
                                        )
                                    }
                                >
                                    {tab_icon(tab)}
                                    {label}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}

#[component]
fn AppearanceSheet(open: RwSignal<bool>) -> impl IntoView {
    let theme = expect_context::<RwSignal<ThemeChoice>>();
    let disclaimer = expect_context::<DisclaimerOpen>().0;
    view! {
        <Sheet open=open title="Appearance">
            <Segmented
                label="Theme"
                value=theme
                options=vec![(ThemeChoice::System, "System"), (ThemeChoice::Dark, "Dark"), (ThemeChoice::Light, "Light")]
            />
            <p class="mt-3 text-footnote text-text-muted">"System follows your device (or your claude.ai setting)."</p>
            <hr class="my-5 border-line/30" />
            <button
                type="button"
                class="press min-h-11 text-body underline underline-offset-4"
                on:click=move |_| {
                    open.set(false);
                    disclaimer.set(true);
                }
            >
                "About Gold School & disclaimer"
            </button>
        </Sheet>
    }
}

#[component]
fn Disclaimer(open: RwSignal<bool>) -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    let accept = move |_| {
        let at = crate::state::now();
        if let Some(s) = window().local_storage().ok().flatten() {
            let _ = s.set_item(DISCLAIMER_KEY, &at.to_string());
        }
        open.set(false);
        store.update_profile(move |p| p.disclaimer_accepted_at = Some(at));
    };
    view! {
        <Sheet open=open title="Before you start" dismissible=false>
            <div class="grid gap-3 text-body">
                <p>"Gold School teaches you how gold trading works, step by step. It is " <strong>"education, not financial advice"</strong> "."</p>
                <p>"It will never tell you to buy or sell, and it never shows live prices or signals. Charts in drills are illustrative, not real market data."</p>
                <p>"Trading can lose money, including more than you expect, quickly. That's why you'll practise on a " <strong>"demo account"</strong> " until your discipline is proven, and only then learn about going live."</p>
            </div>
            <button type="button" class=format!("{PILL} mt-6 w-full") on:click=accept>"I understand"</button>
        </Sheet>
    }
}
