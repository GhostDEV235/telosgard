//! App frame: routed screen, tab bar, sub-screen bar, toast, appearance and disclaimer sheets.

use crate::router::{DrillKind, Route, Tab};
use crate::screens::{home::Home, journal, lesson::LessonScreen, practice, tools};
use crate::state::DisclaimerOpen;
use crate::theme::ThemeChoice;
use crate::ui::controls::{PILL, Segmented};
use crate::ui::sheet::{Sheet, Toast};
use crate::i18n::t;
use leptos::prelude::*;

const DISCLAIMER_KEY: &str = "gs-disclaimer";

#[component]
pub fn Shell() -> impl IntoView {
    let route = expect_context::<RwSignal<Route>>();
    let disclaimer = expect_context::<DisclaimerOpen>();
    let appearance = expect_context::<AppearanceOpen>().0;
    provide_context(crate::screens::lesson::TutorLog(RwSignal::new(Vec::new())));
    provide_context(crate::screens::lesson::LessonUi::new());
    provide_context(crate::ui::moments::Moments::new());

    // Wide screens (DESIGN-SYSTEM §4.1): sidebar | dot-grid canvas holding the screen as a frame | panel.
    let panel = move || match route.get() {
        Route::Lesson(id) => Some(view! { <crate::screens::lesson::LessonPanel id=id /> }.into_any()),
        r if crate::screens::workspace::has_journal_panel(&r) => Some(view! { <crate::screens::workspace::JournalPanel /> }.into_any()),
        _ => None,
    };
    // Invite-only (ADR 0004): signed-out visitors see the sign-in screen; an invite link opens Join.
    // A Memo, so routine save-state changes never re-render the app.
    let store = expect_context::<crate::store::Store>();
    #[derive(Clone, PartialEq)]
    enum Gate {
        App,
        SignIn,
        Join(String),
        Reset(String),
    }
    let gate = Memo::new(move |_| match route.get() {
        Route::Join(t) => Gate::Join(t),
        Route::Reset(t) => Gate::Reset(t),
        _ if store.state.get() == crate::store::SaveState::Local(crate::store::LocalReason::SignedOut) => Gate::SignIn,
        _ => Gate::App,
    });
    let app = move || view! {
        <div class="lg:flex">
        <crate::screens::workspace::Sidebar />
        <div class="min-h-dvh min-w-0 flex-1 pb-[calc(84px+env(safe-area-inset-bottom,0px))] lg:canvas-grid lg:px-5 lg:pt-5 lg:pb-10 xl:px-8">
            <p class="hidden pb-2 text-footnote text-text-muted lg:block" aria-hidden="true">{move || crate::screens::workspace::frame_label(&route.get())}</p>
            <div class="lg:frame">
            // Screen transitions: each route arrives with a short fade-up (off under reduced motion).
            {move || view! { <div class="anim-route">{match route.get() {
                Route::Today => view! { <Home /> }.into_any(),
                Route::Lesson(id) => view! { <LessonScreen id=id /> }.into_any(),
                Route::Practice => view! { <practice::Practice /> }.into_any(),
                Route::Drill(DrillKind::Calibration) => view! { <practice::CalibrationDrill /> }.into_any(),
                Route::Drill(DrillKind::Direction) => view! { <practice::DirectionDrill /> }.into_any(),
                Route::Drill(kind) => view! { <practice::ChoiceDrill kind=kind /> }.into_any(),
                Route::Tools => view! { <tools::Tools /> }.into_any(),
                Route::Plan => view! { <crate::screens::plan::Plan /> }.into_any(),
                Route::Journal => view! { <journal::Journal /> }.into_any(),
                Route::Readiness => view! { <journal::Readiness /> }.into_any(),
                Route::GraderLab => view! { <crate::screens::lab::GraderLab /> }.into_any(),
                Route::WidgetLab => view! { <crate::screens::lab::WidgetLab /> }.into_any(),
                Route::QuestionStats => view! { <crate::screens::lab::QuestionStats /> }.into_any(),
                Route::Checklist => view! { <crate::screens::checklist::Checklist /> }.into_any(),
                Route::Review => view! { <crate::screens::review::Review /> }.into_any(),
                Route::TradeNew => view! { <crate::screens::trade::TradeNew /> }.into_any(),
                Route::TradeSkip => view! { <crate::screens::trade::TradeSkip /> }.into_any(),
                Route::Trade(id) => view! { <crate::screens::trade::TradeDetail id=id /> }.into_any(),
                Route::Join(_) | Route::Reset(_) => view! { <Home /> }.into_any(),
            }}</div> }}
            <RiskFooter />
            </div>
        </div>
        {panel}
        </div>
        <TabBar />
    };
    view! {
        {move || match gate.get() {
            Gate::App => app().into_any(),
            Gate::SignIn => view! { <crate::screens::account::SignIn /> }.into_any(),
            Gate::Join(t) => view! { <crate::screens::account::Join token=t /> }.into_any(),
            Gate::Reset(t) => view! { <crate::screens::account::ResetPassword token=t /> }.into_any(),
        }}
        <Toast />
        <crate::ui::moments::BigMomentOverlay />
        <AppearanceSheet open=appearance />
        <Disclaimer open=disclaimer.0 />
    }
}

/// Shown under every screen (Phase 12).
#[component]
fn RiskFooter() -> impl IntoView {
    view! {
        <footer class="gutter mx-auto max-w-[680px] pt-2 pb-6 text-center text-footnote text-text-muted">
            {t("Education, not financial advice. Trading CFDs carries a high risk of losing money.", "De la formation, pas du conseil financier. Trader des CFD comporte un risque élevé de perdre de l’argent.")}
        </footer>
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
pub fn LargeTitle(
    title: &'static str,
    #[prop(into, default = String::new())] eyebrow: String,
    /// The italic accent phrase that ends the headline (DESIGN-SYSTEM v3).
    #[prop(optional)]
    accent: Option<&'static str>,
) -> impl IntoView {
    let appearance = expect_context::<AppearanceOpen>().0;
    view! {
        <header class="flex items-end justify-between gap-4 pt-4">
            <div class="min-w-0">
                <p class="text-eyebrow text-eyebrow-ink">{eyebrow}</p>
                <h1 class="text-large">{title}{accent.map(|a| view! { " " <em>{a}</em> })}</h1>
            </div>
            <button
                type="button"
                aria-label=t("Appearance and about", "Apparence et à propos")
                class="press mb-1 grid size-10 shrink-0 place-items-center rounded-full bg-surface text-title2 shadow-card lg:hidden"
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

pub fn tab_icon(tab: Tab, class: &'static str) -> impl IntoView {
    let path = match tab {
        Tab::Today => "M4 6.5A2.5 2.5 0 0 1 6.5 4h11A2.5 2.5 0 0 1 20 6.5v5a2.5 2.5 0 0 1-2.5 2.5h-11A2.5 2.5 0 0 1 4 11.5zM6 17.5h12M8 20.5h8",
        Tab::Practice => "M7 3v4M7 15v6M17 3v6M17 17v4M5 7h4v8H5zM15 9h4v8h-4z",
        Tab::Tools => "M6 3.5h12A1.5 1.5 0 0 1 19.5 5v14a1.5 1.5 0 0 1-1.5 1.5H6A1.5 1.5 0 0 1 4.5 19V5A1.5 1.5 0 0 1 6 3.5zM7.5 7h9v3h-9zM8 14h.01M12 14h.01M16 14h.01M8 17.5h.01M12 17.5h.01M16 17.5h.01",
        Tab::Journal => "M6 3.5h11.5a1 1 0 0 1 1 1v15a1 1 0 0 1-1 1H6a1.5 1.5 0 0 1-1.5-1.5V5A1.5 1.5 0 0 1 6 3.5zM8.5 3.5v17M11.5 8h4M11.5 11.5h4",
    };
    view! {
        <svg viewBox="0 0 24 24" class=class aria-hidden="true">
            <path d=path fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    }
}

#[component]
fn TabBar() -> impl IntoView {
    let route = expect_context::<RwSignal<Route>>();
    let tabs = [(Tab::Today, "#today", t("Today", "Aujourd’hui")), (Tab::Practice, "#practice", t("Practice", "Entraînement")), (Tab::Tools, "#size", t("Tools", "Outils")), (Tab::Journal, "#journal", "Journal")];
    view! {
        <nav aria-label=t("Main", "Principal") class="glass gutter fixed inset-x-0 bottom-0 z-30 lg:hidden border-t border-line/30 pb-[env(safe-area-inset-bottom,0px)]">
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
                                            "press flex min-h-14 min-w-16 flex-col items-center justify-center gap-0.5 px-2 text-footnote font-medium {}",
                                            if active() { "text-text" } else { "text-text-muted" },
                                        )
                                    }
                                >
                                    {tab_icon(tab, "size-6")}
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
    let lang = expect_context::<crate::i18n::LangSignal>();
    let disclaimer = expect_context::<DisclaimerOpen>().0;
    view! {
        <Sheet open=open title=t("Appearance", "Apparence")>
            <Segmented
                label=t("Theme", "Thème")
                value=theme
                options=vec![(ThemeChoice::System, t("System", "Système")), (ThemeChoice::Dark, t("Dark", "Sombre")), (ThemeChoice::Light, t("Light", "Clair"))]
            />
            <p class="mt-3 text-footnote text-text-muted">{t("System follows your device's setting.", "« Système » suit le réglage de ton appareil.")}</p>
            <div class="mt-5">
                // Each option names itself in its own language, so either reader can find it.
                <Segmented label=t("Language", "Langue") value=lang options=vec![(crate::i18n::Lang::En, "English"), (crate::i18n::Lang::Fr, "Français")] />
            </div>
            <crate::screens::account::AccountSection />
            <hr class="my-5 border-line/30" />
            <button
                type="button"
                class="press min-h-11 text-body underline underline-offset-4"
                on:click=move |_| {
                    open.set(false);
                    disclaimer.set(true);
                }
            >
                {t("About Telosgard & disclaimer", "À propos de Telosgard et avertissement")}
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
        <Sheet open=open title=t("Before you start", "Avant de commencer") dismissible=false>
            {if crate::i18n::is_fr() {
                view! {
                    <div class="grid gap-3 text-body">
                        <p>"Telosgard t’apprend comment fonctionne le trading de l’or, pas à pas. C’est de la " <strong>"formation, pas du conseil financier"</strong> "."</p>
                        <p>"Il ne te dira jamais d’acheter ou de vendre, et n’affiche jamais de prix en direct ni de signaux. Les graphiques des exercices sont des illustrations, pas de vraies données de marché."</p>
                        <p>"Le Professeur Aurel, qui note tes explications, juge ton journal et répond à tes questions, est le tuteur IA de Telosgard."</p>
                        <p>"Le trading peut faire perdre de l’argent, parfois plus que prévu, et vite. C’est pour ça que tu t’entraîneras sur un " <strong>"compte démo"</strong> " jusqu’à ce que ta discipline soit prouvée, et seulement ensuite tu apprendras à passer au réel."</p>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="grid gap-3 text-body">
                        <p>"Telosgard teaches you how gold trading works, step by step. It is " <strong>"education, not financial advice"</strong> "."</p>
                        <p>"It will never tell you to buy or sell, and it never shows live prices or signals. Charts in drills are illustrative, not real market data."</p>
                        <p>"Professor Aurel, who grades your explanations, judges your journal and answers your questions, is Telosgard's AI tutor."</p>
                        <p>"Trading can lose money, including more than you expect, quickly. That's why you'll practise on a " <strong>"demo account"</strong> " until your discipline is proven, and only then learn about going live."</p>
                    </div>
                }.into_any()
            }}
            <button type="button" class=format!("{PILL} mt-6 w-full") on:click=accept>{t("I understand", "J’ai compris")}</button>
        </Sheet>
    }
}
