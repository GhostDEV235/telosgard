//! The wide-screen workspace (Phase 14, DESIGN-SYSTEM §4.1): a left sidebar with the app's sections
//! and the course outline, a dot-grid canvas holding the screen as a frame, and a right panel where
//! it adds context. Below 1024 px none of this shows; the bottom tab bar and one column remain.

use crate::i18n::{t, ts};
use crate::catalog::{self, SECTIONS};
use crate::journal_data::{JournalData, today};
use crate::router::{Route, Tab, lesson_href};
use crate::state::Progress;
use telosgard_core::course::LessonId;
use leptos::prelude::*;

/// The small Figma-style label above the frame.
pub fn frame_label(route: &Route) -> String {
    match route {
        Route::Today => t("Today", "Aujourd’hui").into(),
        Route::Lesson(id) => format!("{} · {}", catalog::eyebrow(*id), catalog::title(*id)),
        Route::Practice => t("Practice", "Entraînement").into(),
        Route::Drill(_) => t("Practice · Drill", "Entraînement · Exercice").into(),
        Route::Tools => t("Tools · Position size", "Outils · Taille de position").into(),
        Route::Plan => t("Tools · Trading Plan", "Outils · Plan de trading").into(),
        Route::Journal => "Journal".into(),
        Route::Readiness => t("Journal · Live Readiness", "Journal · Prêt pour le réel").into(),
        Route::Checklist => t("Journal · Pre-Session Checklist", "Journal · Checklist avant la séance").into(),
        Route::Review => t("Journal · Weekly Review", "Journal · Bilan de la semaine").into(),
        Route::TradeNew => t("Journal · Log a Demo Trade", "Journal · Noter un Trade démo").into(),
        Route::TradeSkip => t("Journal · Skipped setup", "Journal · Configuration écartée").into(),
        Route::Trade(_) => t("Journal · Entry", "Journal · Entrée").into(),
        Route::GraderLab => "Lab · Grader".into(),
        Route::WidgetLab => "Lab · Widgets".into(),
        Route::QuestionStats => "Lab · Question stats".into(),
        Route::Join(_) => t("Join", "Rejoindre").into(),
    }
}

/// Routes whose right panel shows Live Readiness and today's checklist.
pub fn has_journal_panel(route: &Route) -> bool {
    route.tab() == Tab::Journal
}

fn nav_icon(tab: Tab) -> impl IntoView {
    crate::screens::shell::tab_icon(tab, "size-5")
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let route = expect_context::<RwSignal<Route>>();
    let appearance = expect_context::<crate::screens::shell::AppearanceOpen>().0;
    let items = [(Tab::Today, "#today", t("Today", "Aujourd’hui")), (Tab::Practice, "#practice", t("Practice", "Entraînement")), (Tab::Tools, "#size", t("Tools", "Outils")), (Tab::Journal, "#journal", "Journal")];
    view! {
        <aside
            aria-label=t("Workspace", "Espace de travail")
            class="sticky top-0 hidden h-dvh w-[240px] shrink-0 flex-col xl:w-[272px] border-r border-hairline bg-bg lg:flex"
        >
            <div class="flex items-center gap-2.5 px-5 pt-5 pb-4">
                <span class="grid size-7 place-items-center rounded-[8px] bg-gold-fill pb-px font-display text-headline font-semibold text-on-gold" aria-hidden="true">"T"</span>
                <span class="font-display text-headline font-medium">"Telosgard"</span>
            </div>
            <nav aria-label=t("Main", "Principal") class="px-3">
                <ul class="grid gap-0.5">
                    {items
                        .into_iter()
                        .map(|(tab, href, label)| {
                            let active = move || route.get().tab() == tab;
                            view! {
                                <li>
                                    <a
                                        href=href
                                        aria-current=move || active().then_some("page")
                                        class=move || format!(
                                            "press flex min-h-10 items-center gap-3 rounded-btn px-2.5 text-callout {}",
                                            if active() { "bg-surface font-semibold text-text" } else { "text-text-muted hover:bg-surface/60 hover:text-text" },
                                        )
                                    >
                                        {nav_icon(tab)}
                                        {label}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
            <div class="mx-5 mt-5 mb-2 border-t border-hairline"></div>
            <p id="outline-h" class="px-5 pb-1 text-eyebrow text-eyebrow-ink">{t("Course", "Cours")}</p>
            <nav aria-labelledby="outline-h" class="min-h-0 flex-1 overflow-y-auto px-3 pb-4">
                <CourseOutline />
            </nav>
            <div class="border-t border-hairline px-3 py-3">
                <button
                    type="button"
                    aria-label=t("Appearance and about", "Apparence et à propos")
                    class="press flex min-h-10 w-full items-center gap-3 rounded-btn px-2.5 text-left text-callout text-text-muted hover:bg-surface/60 hover:text-text"
                    on:click=move |_| appearance.set(true)
                >
                    <svg viewBox="0 0 24 24" class="size-5" aria-hidden="true">
                        <circle cx="12" cy="12" r="8.5" fill="none" stroke="currentColor" stroke-width="1.7" />
                        <path d="M12 3.5a8.5 8.5 0 0 1 0 17z" fill="currentColor" />
                    </svg>
                    {t("Appearance", "Apparence")}
                </button>
            </div>
        </aside>
    }
}

/// The course as a layers-style tree: every stage, the current one open; ✓ passed, ● current, 🔒 locked.
#[component]
fn CourseOutline() -> impl IntoView {
    let progress = expect_context::<Progress>();
    let route = expect_context::<RwSignal<Route>>();
    let viewing = move || match route.get() {
        Route::Lesson(id) => Some(id),
        _ => None,
    };
    // The open stage: the one holding the lesson on screen, else the one holding "up next".
    let open_section = Memo::new(move |_| {
        let focus = viewing().or_else(|| progress.up_next()).unwrap_or(LessonId::Numbered(1));
        SECTIONS.iter().position(|s| s.lessons.iter().any(|(l, _)| *l == focus)).unwrap_or(0)
    });
    view! {
        <ul class="grid grid-cols-1 gap-0.5">
            {SECTIONS
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let done = move || s.lessons.iter().filter(|(l, _)| progress.passed(*l)).count();
                    let total = s.lessons.len();
                    view! {
                        <li>
                            <details class="group min-w-0" prop:open=move || open_section.get() == i>
                                <summary class="press flex min-h-9 cursor-pointer list-none items-center gap-2 rounded-btn px-2.5 text-callout text-text hover:bg-surface/60 [&::-webkit-details-marker]:hidden">
                                    <svg viewBox="0 0 12 12" class="size-3 shrink-0 text-text-muted transition-transform group-open:rotate-90" aria-hidden="true">
                                        <path d="M4.5 2.5 8 6l-3.5 3.5" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
                                    </svg>
                                    <span class="min-w-0 flex-1 truncate">{s.title.get()}</span>
                                    <span class="tabular shrink-0 text-footnote text-text-muted">{move || format!("{}/{total}", done())}</span>
                                </summary>
                                <ul class="mt-0.5 mb-1 ml-4 grid grid-cols-1 gap-px border-l border-hairline pl-1.5">
                                    {s.lessons.iter().map(|(id, _)| {
                                        let id = *id;
                                        let title = catalog::title(id);
                                        let state = move || {
                                            if progress.passed(id) { ("✓", "text-gold", t("passed", "réussie")) }
                                            else if progress.unlocked(id) { ("●", "text-text", t("open", "ouverte")) }
                                            else { ("🔒", "text-text-muted", t("locked", "verrouillée")) }
                                        };
                                        let here = move || viewing() == Some(id);
                                        move || {
                                            let (glyph, tone, word) = state();
                                            let row = format!(
                                                "flex min-h-8 items-center gap-2 rounded-[8px] px-2 text-footnote {}",
                                                if here() { "bg-surface font-semibold text-text" } else { "text-text-muted" },
                                            );
                                            let inner = view! {
                                                <span class=format!("w-4 shrink-0 text-center {tone}") aria-hidden="true">{glyph}</span>
                                                <span class="tabular w-5 shrink-0">{catalog::label(id)}</span>
                                                <span class="min-w-0 flex-1 truncate">{title}</span>
                                                <span class="sr-only">{format!(", {word}")}</span>
                                            };
                                            if glyph == "🔒" {
                                                view! { <li><span class=row>{inner}</span></li> }.into_any()
                                            } else {
                                                view! {
                                                    <li>
                                                        <a href=lesson_href(id) aria-current=move || here().then_some("page") class=format!("press {row} hover:bg-surface/60 hover:text-text")>{inner}</a>
                                                    </li>
                                                }.into_any()
                                            }
                                        }
                                    }).collect_view()}
                                </ul>
                            </details>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

/// Journal-side panel: Live Readiness at a glance and today's checklist.
#[component]
pub fn JournalPanel() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let route = expect_context::<RwSignal<Route>>();
    view! {
        <aside aria-label=t("Readiness at a glance", "Préparation en un coup d’œil") class="sticky top-0 hidden h-dvh w-[272px] shrink-0 overflow-y-auto border-l border-hairline bg-bg px-4 xl:w-[300px] xl:px-5 py-6 lg:block">
            <p class="text-eyebrow text-eyebrow-ink">{t("Live Readiness", "Prêt pour le réel")}</p>
            {move || match journal.readiness_progress() {
                Some((run, p)) => view! {
                    <p class="mt-1 text-footnote text-text-muted">{ts(format!("Run {} · started with {}", run.run.number, run.run.starting_balance), format!("Série {} · départ avec {}", run.run.number, run.run.starting_balance))}</p>
                    <dl class="mt-4 grid gap-3">
                        <div class="flex items-baseline justify-between gap-3 border-b border-hairline pb-2">
                            <dt class="text-footnote text-text-muted">{t("Trades counted", "Trades comptés")}</dt>
                            <dd class="tabular text-callout font-semibold"><crate::ui::moments::CountUp value=p.counted as i64 fmt=|v| format!("{v} / 50") /></dd>
                        </div>
                        <Stat label=t("Followed the plan", "Plan respecté") value=if p.counted == 0 { "—".to_string() } else { p.adherence.to_string() } />
                        <Stat label=t("Days", "Jours") value=format!("{} / 56", p.days) />
                        <Stat label=t("Drawdown", "Drawdown") value=ts(format!("{} of {}", p.drawdown, p.max_loss), format!("{} sur {}", p.drawdown, p.max_loss)) />
                    </dl>
                }.into_any(),
                None => view! {
                    <p class="mt-2 text-callout text-text-muted">{t("Run 1 starts with your first journaled trade.", "La série 1 commence avec ton premier trade noté au journal.")}</p>
                }.into_any(),
            }}
            <a href="#readiness" class="mt-3 inline-block min-h-11 py-2 text-callout underline underline-offset-4">{t("Open Live Readiness", "Ouvrir « Prêt pour le réel »")}</a>
            <div class="my-5 border-t border-hairline"></div>
            <p class="text-eyebrow text-eyebrow-ink">{t("Today", "Aujourd’hui")}</p>
            {move || {
                let done = journal.checklist(today()).is_some();
                view! {
                    <p class="mt-2 flex items-center gap-2 text-callout">
                        <span class=if done { "text-up" } else { "text-text-muted" } aria-hidden="true">{if done { "✓" } else { "○" }}</span>
                        {if done { t("Pre-Session Checklist done", "Checklist avant la séance faite") } else { t("Pre-Session Checklist not done yet", "Checklist avant la séance pas encore faite") }}
                    </p>
                    {(route.get() != Route::Checklist).then(|| view! {
                        <a href="#checklist" class="mt-1 inline-block min-h-11 py-2 text-callout underline underline-offset-4">
                            {if done { t("Review checklist", "Revoir la checklist") } else { t("Start checklist", "Commencer la checklist") }}
                        </a>
                    })}
                }
            }}
        </aside>
    }
}

#[component]
fn Stat(label: &'static str, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="flex items-baseline justify-between gap-3 border-b border-hairline pb-2">
            <dt class="text-footnote text-text-muted">{label}</dt>
            <dd class="tabular text-callout font-semibold">{value}</dd>
        </div>
    }
}
