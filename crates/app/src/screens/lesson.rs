//! Lesson reader: text → videos → quiz → Explain-Back → gate (CURRICULUM authoring order).

use crate::catalog::{self, LessonDraft};
use crate::grading;
use crate::rubrics;
use crate::state::{Progress, Toasts, now};
use crate::store::{Attempt, SaveState, Store, repo};
use gold_core::gate::Grade;
use gold_core::grading::GradeResult;
use leptos::task::spawn_local;
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
            <ExplainBack id=id prompt=d.explain_prompt />
        </div>

        <div class="mt-10">
            <GateStatus id=id />
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum Phase {
    Writing,
    Grading,
    Done(GradeResult),
    Failed(String),
}

fn grade_chip(g: Grade) -> impl IntoView {
    let (label, class) = match g {
        Grade::GotIt => ("✓ Got it", "bg-gold-fill text-on-gold"),
        Grade::Almost => ("Almost", "border border-text-muted text-text"),
        Grade::NotYet => ("Not yet", "border border-line text-text-muted"),
    };
    view! { <span class=format!("inline-block rounded-full px-3 py-1 text-footnote font-semibold {class}")>{label}</span> }
}

fn short_date(ms: i64) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64));
    const M: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!("{} {} · {:02}:{:02}", d.get_date(), M[d.get_month() as usize % 12], d.get_hours(), d.get_minutes())
}

#[component]
fn ExplainBack(id: LessonId, prompt: &'static str) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let store = expect_context::<Store>();
    let toasts = expect_context::<Toasts>();
    let rubric = rubrics::rubric(id);
    let open = RwSignal::new(false);
    let text = RwSignal::new(String::new());
    let phase = RwSignal::new(Phase::Writing);
    let attempts = RwSignal::new(Vec::<Attempt>::new());
    let history_open = RwSignal::new(false);

    // Attempt history, once saving is connected.
    Effect::new(move |loaded: Option<bool>| {
        if loaded == Some(true) || store.state.get() != SaveState::Saved {
            return loaded.unwrap_or(false);
        }
        spawn_local(async move {
            if let Ok(mut list) = repo::attempts(id).await {
                list.sort_by_key(|a| std::cmp::Reverse(a.at));
                attempts.set(list);
            }
        });
        true
    });

    let submit = move |_| {
        let Some(rubric) = rubric else {
            phase.set(Phase::Failed("This lesson doesn't have a grading rubric yet.".into()));
            return;
        };
        let answer = text.get_untracked();
        phase.set(Phase::Grading);
        spawn_local(async move {
            match grading::grade(rubric, &answer).await {
                Ok(res) => {
                    let was_passed = progress.passed(id);
                    let at = now();
                    progress.update(id, |p| p.record_grade(res.grade, at));
                    store.save_lesson(progress, id);
                    let attempt = Attempt {
                        lesson: id.doc_id(),
                        answer,
                        grade: res.grade,
                        feedback: res.feedback.clone(),
                        reteach: res.reteach.clone(),
                        at,
                    };
                    attempts.update(|v| v.insert(0, attempt.clone()));
                    store.save_attempt(attempt);
                    if !was_passed && progress.passed(id) {
                        toasts.show("🏅 Lesson passed");
                    }
                    phase.set(Phase::Done(res));
                }
                Err(e) => phase.set(Phase::Failed(e.describe())),
            }
        });
    };

    let result = move || match phase.get() {
        Phase::Writing => ().into_any(),
        Phase::Grading => view! {
            <p class="anim-fade mt-3 rounded-btn bg-bg p-3.5 text-callout" role="status">"Claude is reading your answer…"</p>
        }
        .into_any(),
        Phase::Failed(msg) => view! {
            <div class="anim-fade mt-3 rounded-btn bg-bg p-3.5" role="alert">
                <p class="text-callout">{msg}</p>
                <button type="button" class=format!("{PILL_GHOST} mt-3") on:click=submit>"Try again"</button>
            </div>
        }
        .into_any(),
        Phase::Done(res) => {
            let extra = match res.grade {
                Grade::GotIt => res.gap.clone().map(|tip| {
                    view! { <p class="mt-3 text-callout"><span class="font-semibold">"One extra tip: "</span>{tip}</p> }.into_any()
                }),
                Grade::Almost => res.gap.clone().map(|gap| {
                    view! {
                        <div class="mt-3 rounded-btn border-l-4 border-fill bg-surface p-3.5">
                            <p class="text-eyebrow uppercase text-text-muted">"What's missing"</p>
                            <p class="mt-1 text-callout">{gap}</p>
                        </div>
                    }
                    .into_any()
                }),
                Grade::NotYet => res.reteach.clone().map(|r| {
                    view! {
                        <div class="mt-3 rounded-btn border-l-4 border-fill bg-surface p-3.5">
                            <p class="text-eyebrow uppercase text-text-muted">"💡 Another way to see it"</p>
                            <p class="mt-1 text-callout">{r}</p>
                        </div>
                    }
                    .into_any()
                }),
            };
            let next = if res.grade == Grade::GotIt {
                view! { <button type="button" class=format!("{PILL} mt-4") on:click=move |_| open.set(false)>"Done"</button> }.into_any()
            } else {
                let label = if res.grade == Grade::Almost { "Rewrite my answer" } else { "Try again" };
                view! { <button type="button" class=format!("{PILL} mt-4") on:click=move |_| phase.set(Phase::Writing)>{label}</button> }.into_any()
            };
            view! {
                <div class="anim-fade mt-3 rounded-btn bg-bg p-4" aria-live="polite">
                    {grade_chip(res.grade)}
                    <p class="mt-2 text-callout">{res.feedback.clone()}</p>
                    {extra}
                    {next}
                </div>
            }
            .into_any()
        }
    };

    let editing = move || matches!(phase.get(), Phase::Writing | Phase::Failed(_));
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card">
            <p class="text-headline">{prompt}</p>
            <p class="mt-1 text-callout text-text-muted">
                "Write it like you're explaining to a friend. Claude reads it for meaning, not wording."
            </p>
            <div class="mt-4 flex flex-wrap items-center gap-3">
                <button type="button" class=PILL on:click=move |_| open.set(true)>
                    {move || if progress.get(id).explain_back_met() { "Answer again" } else { "Write your answer" }}
                </button>
                {move || progress.get(id).grade.map(grade_chip)}
            </div>
            {move || {
                let n = attempts.with(Vec::len);
                (n > 0)
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="press mt-3 min-h-11 text-callout text-text-muted"
                                aria-expanded=move || history_open.get().to_string()
                                on:click=move |_| history_open.update(|o| *o = !*o)
                            >
                                {format!("{n} previous attempt{} ", if n == 1 { "" } else { "s" })}
                                {move || if history_open.get() { "▴" } else { "▾" }}
                            </button>
                        }
                    })
            }}
            <Show when=move || history_open.get()>
                <ul class="anim-fade mt-2 grid gap-2">
                    {move || {
                        attempts
                            .get()
                            .into_iter()
                            .map(|a| {
                                view! {
                                    <li class="rounded-btn bg-bg p-3">
                                        <div class="flex items-center justify-between gap-3">
                                            {grade_chip(a.grade)}
                                            <span class="tabular text-footnote text-text-muted">{short_date(a.at)}</span>
                                        </div>
                                        <p class="mt-2 line-clamp-3 text-callout">{a.answer}</p>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
            </Show>
        </section>
        <Sheet open=open title="Explain-Back">
            <p class="text-callout">{prompt}</p>
            <textarea
                rows="7"
                aria-label="Your answer"
                placeholder="In my own words…"
                disabled=move || !editing()
                class="mt-3 w-full resize-y rounded-field bg-bg p-3.5 text-body text-text placeholder:text-text-muted focus:outline-2 focus:outline-text disabled:opacity-60"
                prop:value=move || text.get()
                on:input=move |ev| text.set(event_target_value(&ev))
            ></textarea>
            <p class="tabular mt-1 text-right text-footnote text-text-muted">{move || format!("{} words", text.get().split_whitespace().count())}</p>
            {result}
            <Show when=editing>
                <div class="mt-4 flex flex-wrap justify-end gap-3">
                    <button type="button" class=PILL_GHOST on:click=move |_| open.set(false)>"Close"</button>
                    <button type="button" class=PILL disabled=move || text.get().split_whitespace().count() < 5 on:click=submit>
                        "Submit for grading"
                    </button>
                </div>
            </Show>
        </Sheet>
    }
}
