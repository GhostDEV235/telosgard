//! Grader lab (`#grader-lab`, not linked from the UI): runs every Stage 1 fixture through the real
//! grader and compares against expectations (IMPLEMENTATION-PLAN Phase 5, rubric tuning).
//! Results are also written to the shared doc `lab/grader-latest` so they can be read back in Claude Code.

use crate::grading;
use crate::platform;
use gold_content::LESSONS;
use crate::screens::shell::SubBar;
use crate::state::{Toasts, now};
use crate::ui::controls::PILL;
use gold_core::gate::Grade;
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
    passed: usize,
    total: usize,
    results: &'a [Outcome],
}

#[component]
pub fn GraderLab() -> impl IntoView {
    let toasts = expect_context::<Toasts>();
    let results = RwSignal::new(Vec::<Outcome>::new());
    let running = RwSignal::new(false);
    let total: usize = LESSONS.iter().map(|l| l.explain.fixtures.len()).sum();

    let run = move |_| {
        if running.get_untracked() {
            return;
        }
        running.set(true);
        results.set(Vec::new());
        spawn_local(async move {
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
                            if stop {
                                break 'outer;
                            }
                            continue;
                        }
                    };
                    results.update(|v| v.push(outcome));
                }
            }
            let list = results.get_untracked();
            let passed = list.iter().filter(|o| o.as_expected).count();
            let doc = LabRun { at: now(), passed, total, results: &list };
            match platform::set_shared("lab/grader-latest", &doc).await {
                Ok(()) => toasts.show("Results saved for Claude Code to read"),
                Err(e) => toasts.show(format!("Results not saved: {}", e.describe())),
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
