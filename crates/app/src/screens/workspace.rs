//! The wide-screen workspace (Phase 14, DESIGN-SYSTEM §4.1): a left sidebar with the app's sections
//! and the course outline, a dot-grid canvas holding the screen as a frame, and a right panel where
//! it adds context. Below 1024 px none of this shows; the bottom tab bar and one column remain.

use crate::catalog::{self, SECTIONS};
use crate::journal_data::{JournalData, today};
use crate::router::{Route, Tab, lesson_href};
use crate::state::Progress;
use telosgard_core::course::LessonId;
use leptos::prelude::*;

/// The small Figma-style label above the frame.
pub fn frame_label(route: &Route) -> String {
    match route {
        Route::Today => "Today".into(),
        Route::Lesson(id) => format!("{} · {}", catalog::eyebrow(*id), catalog::title(*id)),
        Route::Practice => "Practice".into(),
        Route::Drill(_) => "Practice · Drill".into(),
        Route::Tools => "Tools · Position size".into(),
        Route::Plan => "Tools · Trading Plan".into(),
        Route::Journal => "Journal".into(),
        Route::Readiness => "Journal · Live Readiness".into(),
        Route::Checklist => "Journal · Pre-Session Checklist".into(),
        Route::Review => "Journal · Weekly Review".into(),
        Route::TradeNew => "Journal · Log a Demo Trade".into(),
        Route::TradeSkip => "Journal · Skipped setup".into(),
        Route::Trade(_) => "Journal · Entry".into(),
        Route::GraderLab => "Lab · Grader".into(),
        Route::WidgetLab => "Lab · Widgets".into(),
        Route::QuestionStats => "Lab · Question stats".into(),
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
    let items = [(Tab::Today, "#today", "Today"), (Tab::Practice, "#practice", "Practice"), (Tab::Tools, "#size", "Tools"), (Tab::Journal, "#journal", "Journal")];
    view! {
        <aside
            aria-label="Workspace"
            class="sticky top-0 hidden h-dvh w-[240px] shrink-0 flex-col xl:w-[272px] border-r border-hairline bg-bg lg:flex"
        >
            <div class="flex items-center gap-2.5 px-5 pt-5 pb-4">
                <span class="grid size-7 place-items-center rounded-[8px] bg-gold-fill pb-px font-display text-headline font-semibold text-on-gold" aria-hidden="true">"T"</span>
                <span class="font-display text-headline font-medium">"Telosgard"</span>
            </div>
            <nav aria-label="Main" class="px-3">
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
            <p id="outline-h" class="px-5 pb-1 text-eyebrow text-eyebrow-ink">"Course"</p>
            <nav aria-labelledby="outline-h" class="min-h-0 flex-1 overflow-y-auto px-3 pb-4">
                <CourseOutline />
            </nav>
            <div class="border-t border-hairline px-3 py-3">
                <button
                    type="button"
                    aria-label="Appearance and about"
                    class="press flex min-h-10 w-full items-center gap-3 rounded-btn px-2.5 text-left text-callout text-text-muted hover:bg-surface/60 hover:text-text"
                    on:click=move |_| appearance.set(true)
                >
                    <svg viewBox="0 0 24 24" class="size-5" aria-hidden="true">
                        <circle cx="12" cy="12" r="8.5" fill="none" stroke="currentColor" stroke-width="1.7" />
                        <path d="M12 3.5a8.5 8.5 0 0 1 0 17z" fill="currentColor" />
                    </svg>
                    "Appearance"
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
                                    <span class="min-w-0 flex-1 truncate">{s.title}</span>
                                    <span class="tabular shrink-0 text-footnote text-text-muted">{move || format!("{}/{total}", done())}</span>
                                </summary>
                                <ul class="mt-0.5 mb-1 ml-4 grid grid-cols-1 gap-px border-l border-hairline pl-1.5">
                                    {s.lessons.iter().map(|(id, title)| {
                                        let id = *id;
                                        let state = move || {
                                            if progress.passed(id) { ("✓", "text-gold", "passed") }
                                            else if progress.unlocked(id) { ("●", "text-text", "open") }
                                            else { ("🔒", "text-text-muted", "locked") }
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
                                                <span class="min-w-0 flex-1 truncate">{*title}</span>
                                                <span class="sr-only">{format!(", {word}")}</span>
                                            };
                                            if word == "locked" {
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
        <aside aria-label="Readiness at a glance" class="sticky top-0 hidden h-dvh w-[272px] shrink-0 overflow-y-auto border-l border-hairline bg-bg px-4 xl:w-[300px] xl:px-5 py-6 lg:block">
            <p class="text-eyebrow text-eyebrow-ink">"Live Readiness"</p>
            {move || match journal.readiness_progress() {
                Some((run, p)) => view! {
                    <p class="mt-1 text-footnote text-text-muted">{format!("Run {} · started with {}", run.run.number, run.run.starting_balance)}</p>
                    <dl class="mt-4 grid gap-3">
                        <div class="flex items-baseline justify-between gap-3 border-b border-hairline pb-2">
                            <dt class="text-footnote text-text-muted">"Trades counted"</dt>
                            <dd class="tabular text-callout font-semibold"><crate::ui::moments::CountUp value=p.counted as i64 fmt=|v| format!("{v} / 50") /></dd>
                        </div>
                        <Stat label="Followed the plan" value=if p.counted == 0 { "—".to_string() } else { p.adherence.to_string() } />
                        <Stat label="Days" value=format!("{} / 56", p.days) />
                        <Stat label="Drawdown" value=format!("{} of {}", p.drawdown, p.max_loss) />
                    </dl>
                }.into_any(),
                None => view! {
                    <p class="mt-2 text-callout text-text-muted">"Run 1 starts with your first journaled trade."</p>
                }.into_any(),
            }}
            <a href="#readiness" class="mt-3 inline-block min-h-11 py-2 text-callout underline underline-offset-4">"Open Live Readiness"</a>
            <div class="my-5 border-t border-hairline"></div>
            <p class="text-eyebrow text-eyebrow-ink">"Today"</p>
            {move || {
                let done = journal.checklist(today()).is_some();
                view! {
                    <p class="mt-2 flex items-center gap-2 text-callout">
                        <span class=if done { "text-up" } else { "text-text-muted" } aria-hidden="true">{if done { "✓" } else { "○" }}</span>
                        {if done { "Pre-Session Checklist done" } else { "Pre-Session Checklist not done yet" }}
                    </p>
                    {(route.get() != Route::Checklist).then(|| view! {
                        <a href="#checklist" class="mt-1 inline-block min-h-11 py-2 text-callout underline underline-offset-4">
                            {if done { "Review checklist" } else { "Start checklist" }}
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
