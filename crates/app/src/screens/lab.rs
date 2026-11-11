//! Grader lab (`#grader-lab`, not linked from the UI): runs every Stage 1 fixture through the real
//! grader and compares against expectations (IMPLEMENTATION-PLAN Phase 5, rubric tuning).
//! Results are also written to the shared doc `lab/grader-latest` so they can be read back in Claude Code.

use crate::grading;
use crate::platform;
use telosgard_content::LESSONS;
use crate::screens::shell::SubBar;
use crate::state::{Toasts, now};
use crate::ui::controls::PILL;
use telosgard_core::gate::Grade;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Serialize;

const KINDS: [&str; 3] = ["Correct", "Gap", "Misconception"];

#[derive(Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Outcome {
    lesson: u8,
    fixture: &'static str,
    expected: &'static str,
    grade: Option<Grade>,
    as_expected: bool,
    feedback: Option<String>,
    gap: Option<String>,
    reteach: Option<String>,
    suggestion: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LabRun<'a> {
    at: i64,
    started_at: i64,
    /// False while the run is still going (or was stopped part-way).
    complete: bool,
    passed: usize,
    done: usize,
    total: usize,
    results: &'a [Outcome],
}

/// Writes the run so far to `lab/grader-latest`. Called after every answer, so a run that's
/// interrupted (page closed, reload, rate limit) still leaves its results behind.
async fn save(started_at: i64, list: &[Outcome], total: usize, complete: bool) -> Result<(), platform::PlatformError> {
    let passed = list.iter().filter(|o| o.as_expected).count();
    let doc = LabRun { at: now(), started_at, complete, passed, done: list.len(), total, results: list };
    platform::set_shared("lab/grader-latest", &doc).await
}

#[component]
pub fn GraderLab() -> impl IntoView {
    let toasts = expect_context::<Toasts>();
    let results = RwSignal::new(Vec::<Outcome>::new());
    let running = RwSignal::new(false);
    let saved = RwSignal::new(None::<Result<usize, String>>);
    let total: usize = LESSONS.iter().map(|l| l.explain.fixtures.len()).sum();

    let run = move |_| {
        if running.get_untracked() {
            return;
        }
        running.set(true);
        results.set(Vec::new());
        saved.set(None);
        let started_at = now();
        spawn_local(async move {
            let checkpoint = move || async move {
                let list = results.get_untracked();
                let n = list.len();
                saved.set(Some(save(started_at, &list, total, false).await.map(|_| n).map_err(|e| e.describe())));
            };
            'outer: for r in LESSONS.iter() {
                for (k, fx) in r.explain.fixtures.iter().enumerate() {
                    let (answer, expect) = (fx.answer, fx.expect);
                    let outcome = match grading::grade(r, answer).await {
                        Ok(g) => Outcome {
                            lesson: r.id,
                            fixture: KINDS[k],
                            expected: grading::expect_label(expect),
                            grade: Some(g.grade),
                            as_expected: grading::accepts(expect, g.grade),
                            feedback: Some(g.feedback),
                            gap: g.gap,
                            reteach: g.reteach,
                            suggestion: g.suggestion,
                            error: None,
                        },
                        Err(e) => {
                            let stop = matches!(e, grading::GradeError::Platform(platform::PlatformError::NotGranted | platform::PlatformError::RateLimited));
                            results.update(|v| {
                                v.push(Outcome {
                                    lesson: r.id,
                                    fixture: KINDS[k],
                                    expected: grading::expect_label(expect),
                                    grade: None,
                                    as_expected: false,
                                    feedback: None,
                                    gap: None,
                                    reteach: None,
                                    suggestion: None,
                                    error: Some(e.describe()),
                                })
                            });
                            checkpoint().await;
                            if stop {
                                break 'outer;
                            }
                            continue;
                        }
                    };
                    results.update(|v| v.push(outcome));
                    checkpoint().await;
                }
            }
            let list = results.get_untracked();
            match save(started_at, &list, total, list.len() == total).await {
                Ok(()) => {
                    saved.set(Some(Ok(list.len())));
                    toasts.show("Results saved for Claude Code to read");
                }
                Err(e) => {
                    saved.set(Some(Err(e.describe())));
                    toasts.show(format!("Results not saved: {}", e.describe()));
                }
            }
            running.set(false);
        });
    };

    let chip = |g: Option<Grade>| match g {
        Some(Grade::GotIt) => "Got it",
        Some(Grade::Almost) => "Almost",
        Some(Grade::NotYet) => "Not yet",
        None => "Error",
    };

    view! {
        <SubBar back="#today" back_label="Today" title="Grader lab" />
        <main class="gutter mx-auto max-w-[900px] pt-6 pb-10">
            <h1 class="text-title1">"Grader lab"</h1>
            <p class="mt-2 text-callout text-text-muted">
                {format!("Runs all {total} test answers (a correct one, one with a gap, one with a misconception, for each Stage 1 lesson) through Claude, the same way your Explain-Backs are graded. Takes a few minutes.")}
            </p>
            <div class="mt-4 flex flex-wrap items-center gap-4">
                <button type="button" class=PILL disabled=move || running.get() on:click=run>
                    {move || if running.get() { "Running…" } else { "Run all" }}
                </button>
                <p class="tabular text-headline" aria-live="polite">
                    {move || {
                        let r = results.get();
                        if r.is_empty() { String::new() } else { format!("{} of {} as expected · {} of {total} done", r.iter().filter(|o| o.as_expected).count(), r.len(), r.len()) }
                    }}
                </p>
            </div>
            <p class="mt-3 text-callout" role="status">
                {move || match saved.get() {
                    None if running.get() => "Saving after every answer…".to_string(),
                    None => "Results are saved after every answer, so you can leave once it's done.".to_string(),
                    Some(Ok(n)) => format!("✓ Saved {n} of {total} for Claude Code{}", if running.get() { " · keep this page open until it finishes" } else { "" }),
                    Some(Err(e)) => format!("⚠ Not saved: {e}"),
                }}
            </p>
            <ul class="mt-6 grid gap-2">
                {move || {
                    results
                        .get()
                        .into_iter()
                        .map(|o| {
                            let mark = if o.as_expected { ("✓", "text-up") } else { ("✗", "text-down") };
                            view! {
                                <li class="rounded-btn bg-surface p-3 shadow-card">
                                    <details>
                                        <summary class="flex cursor-pointer list-none flex-wrap items-center gap-3">
                                            <span class=format!("w-5 text-headline {}", mark.1)>{mark.0}</span>
                                            <span class="tabular w-16 text-callout">{format!("L{}", o.lesson)}</span>
                                            <span class="w-28 text-callout">{o.fixture}</span>
                                            <span class="text-footnote text-text-muted">"expected " {o.expected}</span>
                                            <span class="ml-auto text-callout font-semibold">{chip(o.grade)}</span>
                                        </summary>
                                        <div class="mt-3 grid gap-2 text-callout">
                                            {o.error.map(|e| view! { <p class="text-down">{e}</p> })}
                                            {o.feedback.map(|f| view! { <p>{f}</p> })}
                                            {o.gap.map(|g| view! { <p><span class="font-semibold">"Gap: "</span>{g}</p> })}
                                            {o.reteach.map(|t| view! { <p><span class="font-semibold">"Re-teach: "</span>{t}</p> })}
                                            {o.suggestion.map(|t| view! { <p><span class="font-semibold">"Suggested answer: "</span>{t}</p> })}
                                        </div>
                                    </details>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </main>
    }
}

/// Widget lab (`#widget-lab`, not linked): every lesson Widget on one page, for checking.
#[component]
pub fn WidgetLab() -> impl IntoView {
    view! {
        <SubBar back="#today" back_label="Today" title="Widget lab" />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <h1 class="text-title1">"Widget lab"</h1>
            <MomentLab />
            <div class="mt-4 grid gap-6">
                {telosgard_content::WIDGETS
                    .iter()
                    .map(|w| view! {
                        <section aria-label=*w>
                            <p class="tabular text-footnote text-text-muted">{*w}</p>
                            <crate::ui::widgets::Widget kind=w />
                        </section>
                    })
                    .collect_view()}
            </div>
        </main>
    }
}

/// Question Records per lesson, for course tuning (FR-13.1). The same data is in `qrec-NN` docs,
/// which Claude Code reads through ArtifactData.
#[component]
pub fn QuestionStats() -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    view! {
        <SubBar back="#today" back_label="Today" title="Question stats" />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <h1 class="text-title1">"Question stats"</h1>
            <p class="mt-1 text-callout text-text-muted">"Per question: times answered, % wrong, % tagged Guessing. Learned = right while Sure twice in a row."</p>
            {move || {
                let recs = store.qrecs.get();
                let mut lessons: Vec<u8> = recs.keys().copied().collect();
                lessons.sort();
                if lessons.is_empty() {
                    return view! { <p class="mt-6 text-callout">"No answers recorded yet."</p> }.into_any();
                }
                lessons
                    .into_iter()
                    .map(|n| {
                        let l = telosgard_content::lesson(n);
                        let m = recs.get(&n).cloned().unwrap_or_default();
                        let rows = m
                            .iter()
                            .map(|(id, r)| {
                                let (a, w, g) = r.stats();
                                let text = l
                                    .and_then(|l| l.quiz.iter().chain(l.recall.iter()).find(|q| q.id == id))
                                    .map(|q| q.q)
                                    .unwrap_or("(no longer in the lesson)");
                                view! {
                                    <tr class="border-t border-line/30 align-top">
                                        <td class="tabular py-2 pr-2">{id.clone()}</td>
                                        <td class="py-2 pr-2">{text}</td>
                                        <td class="tabular py-2 pr-2 text-right">{a}</td>
                                        <td class="tabular py-2 pr-2 text-right">{format!("{w}%")}</td>
                                        <td class="tabular py-2 pr-2 text-right">{format!("{g}%")}</td>
                                        <td class="py-2 text-right">{if r.retired() { "✓" } else { "" }}</td>
                                    </tr>
                                }
                            })
                            .collect_view();
                        view! {
                            <section class="mt-6">
                                <h2 class="text-headline">{format!("Lesson {n}: {}", l.map(|l| l.title).unwrap_or(""))}</h2>
                                <div class="mt-2 overflow-x-auto">
                                    <table class="w-full text-footnote">
                                        <thead class="text-text-muted">
                                            <tr>
                                                <th scope="col" class="py-1 pr-2 text-left">"Id"</th>
                                                <th scope="col" class="py-1 pr-2 text-left">"Question"</th>
                                                <th scope="col" class="py-1 pr-2 text-right">"Answered"</th>
                                                <th scope="col" class="py-1 pr-2 text-right">"Wrong"</th>
                                                <th scope="col" class="py-1 pr-2 text-right">"Guessing"</th>
                                                <th scope="col" class="py-1 text-right">"Learned"</th>
                                            </tr>
                                        </thead>
                                        <tbody>{rows}</tbody>
                                    </table>
                                </div>
                            </section>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
        </main>
    }
}

/// Replays the Milestone moments for review (Phase 15). Nothing here touches progress.
#[component]
fn MomentLab() -> impl IntoView {
    use crate::ui::moments::{Badge, BigMoment, Moments};
    let moments = expect_context::<Moments>();
    let replay = RwSignal::new(0u32);
    view! {
        <section class="mt-4 rounded-card bg-surface p-5 shadow-card" aria-label="Milestone moments">
            <p class="text-eyebrow text-eyebrow-ink">"Milestone moments (preview)"</p>
            <div class="mt-3 flex items-center gap-4 rounded-btn bg-gold-fill p-4 text-on-gold">
                {move || { replay.get(); view! { <Badge size=44 /> } }}
                <p class="text-headline">"Lesson passed"</p>
            </div>
            <div class="mt-4 flex flex-wrap gap-3">
                <button type="button" class=crate::ui::controls::PILL_GHOST on:click=move |_| replay.update(|n| *n += 1)>"Replay lesson passed"</button>
                <button type="button" class=crate::ui::controls::PILL_GHOST on:click=move |_| moments.big.set(Some(BigMoment::Stage {
                    eyebrow: "Stage 1",
                    title: "Foundations",
                    lessons: 12,
                    next: Some((telosgard_core::course::LessonId::Numbered(13), "Sessions & Timing")),
                }))>"Stage complete"</button>
                <button type="button" class=crate::ui::controls::PILL_GHOST on:click=move |_| moments.big.set(Some(BigMoment::Readiness))>"Live Readiness"</button>
            </div>
        </section>
    }
}
