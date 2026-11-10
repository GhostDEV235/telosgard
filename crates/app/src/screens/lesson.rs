//! Lesson reader: text → videos → quiz → Explain-Back → gate (CURRICULUM authoring order).

use crate::catalog::{self, LessonDraft};
use crate::state::{Progress, Toasts, now};
use crate::ui::controls::{PILL, PILL_GHOST};
use crate::ui::course::{Callout, CalloutKind, GateStatus, VideoCard};
use crate::ui::quiz::QuizCard;
use crate::ui::sheet::Sheet;
use gold_core::course::LessonId;
use leptos::prelude::*;

#[component]
pub fn LessonScreen(id: LessonId) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let title = catalog::title(id);
    view! {
        <crate::screens::shell::SubBar back="#today" back_label="Today" title=title />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <p class="text-eyebrow uppercase text-text-muted">{catalog::eyebrow(id)}</p>
            <h1 class="mt-1 text-title1">{title}</h1>
            {move || {
                if !progress.unlocked(id) {
                    locked(id).into_any()
                } else if let Some(d) = catalog::draft(id) {
                    reader(id, d).into_any()
                } else {
                    not_written().into_any()
                }
            }}
        </main>
    }
}

fn locked(id: LessonId) -> impl IntoView {
    let why = match id.predecessor() {
        Some(p) if !matches!(id.track(), gold_core::course::Track::Live | gold_core::course::Track::Advanced) => {
            format!("Pass “{}” first. Lessons build on each other, so none can be skipped.", catalog::title(p))
        }
        _ => "This track unlocks with Live Readiness: 50 disciplined Demo Trades over at least 8 weeks.".to_string(),
    };
    view! {
        <section class="mt-6 rounded-card bg-locked p-5">
            <p class="text-headline">"🔒 Locked"</p>
            <p class="mt-1 text-callout">{why}</p>
        </section>
    }
}

fn not_written() -> impl IntoView {
    view! {
        <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
            <p class="text-headline">"This lesson is written when you get close to it"</p>
            <p class="mt-1 text-callout text-text-muted">
                "Each stage is written using what worked for you in the one before: which analogies landed, and where you got “Almost”."
            </p>
        </section>
    }
}

fn reader(id: LessonId, d: &'static LessonDraft) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let toasts = expect_context::<Toasts>();
    let store = expect_context::<crate::store::Store>();
    let on_quiz = move |score: u8| {
        progress.update(id, |p| p.record_quiz(score, now()));
        store.save_lesson(progress, id);
        if progress.passed(id) {
            toasts.show("Lesson passed ✓");
        }
    };
    view! {
        <p class="mt-3 inline-block rounded-full border border-line/50 px-3 py-1 text-footnote text-text-muted">
            "Draft for design review · final text arrives in Phase 6"
        </p>
        <p class="mt-4 text-callout text-text-muted">"By the end, you'll be able to " {d.objective}</p>

        <article class="mt-8 grid gap-5">
            <p class="text-title2">{d.hook}</p>
            {d.body.iter().map(|p| view! { <p class="text-body">{*p}</p> }).collect_view()}
            <Callout kind=CalloutKind::Analogy text=d.analogy />
            <Callout kind=CalloutKind::Example text=d.example />
            <Callout kind=CalloutKind::Mistake text=d.mistake />
            <section class="rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">"Key takeaways"</h2>
                <ul class="mt-3 grid gap-2">
                    {d.takeaways
                        .iter()
                        .map(|t| {
                            view! {
                                <li class="flex gap-3 text-callout">
                                    <span class="text-text-muted" aria-hidden="true">"✓"</span>
                                    <span>{*t}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </section>
        </article>

        <h2 class="mt-10 text-title2">"Watch"</h2>
        <div class="mt-3 grid gap-3">{d.videos.iter().map(|v| view! { <VideoCard video=v /> }).collect_view()}</div>

        <h2 class="mt-10 text-title2">"Check your understanding"</h2>
        <div class="mt-3">
            <QuizCard questions=d.quiz on_done=on_quiz />
        </div>

        <h2 class="mt-10 text-title2">"Explain it back"</h2>
        <div class="mt-3">
            <ExplainBack prompt=d.explain_prompt />
        </div>

        <div class="mt-10">
            <GateStatus id=id />
        </div>
    }
}

#[component]
fn ExplainBack(prompt: &'static str) -> impl IntoView {
    let open = RwSignal::new(false);
    let text = RwSignal::new(String::new());
    let status = RwSignal::new(None::<&'static str>);
    let submit = move |_| {
        // Never auto-passes (FR-3.4). Grading via Claude is wired up in Phase 4.
        status.set(Some(
            "Claude grading isn't connected in this preview yet, so nothing was graded or saved. It arrives in Phase 4.",
        ));
    };
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card">
            <p class="text-headline">{prompt}</p>
            <p class="mt-1 text-callout text-text-muted">
                "Write it like you're explaining to a friend. Claude reads it for meaning, not wording."
            </p>
            <button type="button" class=format!("{PILL} mt-4") on:click=move |_| open.set(true)>
                "Write your answer"
            </button>
        </section>
        <Sheet open=open title="Explain-Back">
            <p class="text-callout">{prompt}</p>
            <textarea
                rows="7"
                aria-label="Your answer"
                placeholder="In my own words…"
                class="mt-3 w-full resize-y rounded-field bg-bg p-3.5 text-body text-text placeholder:text-text-muted focus:outline-2 focus:outline-text"
                prop:value=move || text.get()
                on:input=move |ev| text.set(event_target_value(&ev))
            ></textarea>
            <p class="tabular mt-1 text-right text-footnote text-text-muted">{move || format!("{} words", text.get().split_whitespace().count())}</p>
            {move || status.get().map(|s| view! { <p class="anim-fade mt-3 rounded-btn bg-bg p-3.5 text-callout" role="status">{s}</p> })}
            <div class="mt-4 flex flex-wrap justify-end gap-3">
                <button type="button" class=PILL_GHOST on:click=move |_| open.set(false)>"Close"</button>
                <button type="button" class=PILL disabled=move || text.get().split_whitespace().count() < 5 on:click=submit>
                    "Submit for grading"
                </button>
            </div>
        </Sheet>
    }
}
