//! Lesson reader: text → videos → quiz → Explain-Back → gate (CURRICULUM authoring order).

use crate::catalog;
use crate::grading;
use gold_content::Lesson;
use crate::state::{Progress, Toasts, now};
use crate::store::{Attempt, SaveState, Store, repo};
use gold_core::gate::Grade;
use gold_core::grading::GradeResult;
use leptos::task::spawn_local;
use crate::ui::controls::{PILL, PILL_GHOST};
use crate::ui::course::{Callout, CalloutKind, GateStatus, VideoCard};
use crate::ui::quiz::{QuizCard, QuizItem};
use crate::ui::sheet::Sheet;
use gold_core::course::LessonId;
use leptos::prelude::*;

#[component]
pub fn LessonScreen(id: LessonId) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let title = catalog::title(id);
    // Only the lock state decides which view shows. Reading progress directly here re-rendered the
    // whole reader on every save, which reset the quiz to question 1 and lost Explain-Back results.
    let unlocked = Memo::new(move |_| progress.unlocked(id));
    view! {
        <crate::screens::shell::SubBar back="#today" back_label="Today" title=title />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <p class="text-eyebrow uppercase text-text-muted">{catalog::eyebrow(id)}</p>
            <h1 class="mt-1 text-title1">{title}</h1>
            {move || {
                if !unlocked.get() {
                    locked(id).into_any()
                } else if let Some(l) = content_for(id) {
                    reader(id, l).into_any()
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

pub fn content_for(id: LessonId) -> Option<&'static Lesson> {
    match id {
        LessonId::Numbered(n) => gold_content::lesson(n),
        _ => None,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Hook,
    Step(usize),
    Analogy,
    Example,
    Mistake,
    Takeaways,
    Videos,
    Recall,
}

fn screens(l: &Lesson) -> Vec<Screen> {
    let mut v = vec![Screen::Hook];
    v.extend((0..l.steps.len()).map(Screen::Step));
    v.extend([Screen::Analogy, Screen::Example, Screen::Mistake, Screen::Takeaways]);
    if !l.videos.is_empty() {
        v.push(Screen::Videos);
    }
    v.push(Screen::Recall);
    v
}

fn screen_view(l: &'static Lesson, s: Screen, ask: RwSignal<Option<usize>>) -> AnyView {
    match s {
        Screen::Hook => view! {
            <p class="text-eyebrow uppercase text-text-muted">"Start with a question"</p>
            <p class="mt-2 text-title2">{l.hook}</p>
            <p class="mt-3 text-callout text-text-muted">"Have a guess in your head, then tap Next."</p>
        }
        .into_any(),
        Screen::Step(i) => {
            let st = &l.steps[i];
            view! {
                <h2 class="text-title2">{st.heading}</h2>
                <div class="lesson-prose mt-3" inner_html=st.text_html></div>
                {st.widget.map(|w| view! { <div class="mt-4"><crate::ui::widgets::Widget kind=w /></div> })}
                {st.why_html.map(|w| view! {
                    <details class="mt-4 rounded-btn bg-bg p-3.5">
                        <summary class="press min-h-11 cursor-pointer py-1 text-callout font-semibold">"Why? Tell me more"</summary>
                        <div class="lesson-prose mt-2 text-callout" inner_html=w></div>
                    </details>
                })}
                <button type="button" class="press mt-4 min-h-11 text-callout underline underline-offset-4" on:click=move |_| ask.set(Some(i))>
                    "Ask Claude about this step"
                </button>
            }
            .into_any()
        }
        Screen::Analogy => view! { <Callout kind=CalloutKind::Analogy text=l.analogy /> }.into_any(),
        Screen::Example => view! { <Callout kind=CalloutKind::Example text=l.example /> }.into_any(),
        Screen::Mistake => view! { <Callout kind=CalloutKind::Mistake text=l.mistake /> }.into_any(),
        Screen::Takeaways => view! {
            <h2 class="text-headline">"Key takeaways"</h2>
            <ul class="mt-3 grid gap-2">
                {l.takeaways.iter().map(|t| view! {
                    <li class="flex gap-3 text-callout"><span class="text-text-muted" aria-hidden="true">"✓"</span><span>{*t}</span></li>
                }).collect_view()}
            </ul>
        }
        .into_any(),
        Screen::Videos => view! {
            <h2 class="text-headline">"Watch (optional)"</h2>
            <div class="mt-3 grid gap-3">{l.videos.iter().map(|v| view! { <VideoCard video=v /> }).collect_view()}</div>
        }
        .into_any(),
        Screen::Recall => view! {
            <p class="text-eyebrow uppercase text-text-muted">"Recall · from memory"</p>
            <p class="mt-1 text-callout text-text-muted">"Three quick questions before the quiz. Practice only."</p>
            <div class="mt-3">
                <QuizCard questions=l.recall.iter().map(|q| QuizItem::course(l.id, q)).collect() on_done=|_| {} practice=true />
            </div>
        }
        .into_any(),
    }
}

fn reader(id: LessonId, l: &'static Lesson) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let toasts = expect_context::<Toasts>();
    let store = expect_context::<crate::store::Store>();
    let all = StoredValue::new(screens(l));
    let n = all.with_value(Vec::len);
    let pos = RwSignal::new(0usize);
    let returning = { let p = progress.get_untracked(id); p.quiz_best.is_some() || p.grade.is_some() };
    let reached_end = RwSignal::new(false);
    let quiz_done = RwSignal::new(false);
    let ask = RwSignal::new(None::<usize>);
    let on_quiz = move |score: u8| {
        quiz_done.set(true);
        let was_passed = progress.passed(id);
        progress.update(id, |p| p.record_quiz(score, now()));
        store.save_lesson(progress, id);
        if !was_passed && progress.passed(id) {
            toasts.show("🏅 Lesson passed");
        }
    };
    let go = move |to: usize| {
        pos.set(to);
        if let Some(el) = document().get_element_by_id("lesson-stepper") {
            el.scroll_into_view();
        }
    };
    let to_quiz = move || {
        reached_end.set(true);
        request_animation_frame(|| {
            if let Some(el) = document().get_element_by_id("lesson-quiz") {
                el.scroll_into_view();
            }
        });
    };

    view! {
        <p class="mt-3 text-footnote text-text-muted">{format!("About {} minutes, step by step", l.read_minutes)}</p>
        <p class="mt-2 text-callout text-text-muted">"By the end, you'll be able to " {l.objective}</p>
        {returning.then(|| view! {
            <button type="button" class="press mt-2 min-h-11 text-callout underline underline-offset-4" on:click=move |_| to_quiz()>"Skip to the quiz ›"</button>
        })}

        <SpacedReview current=l.id />

        <section id="lesson-stepper" class="mt-6 scroll-mt-20 rounded-card bg-surface p-5 shadow-card" aria-label="Lesson steps">
            <div class="flex items-center justify-between gap-3">
                <p class="tabular text-eyebrow uppercase text-text-muted">{move || format!("Step {} of {n}", pos.get() + 1)}</p>
            </div>
            <div class="mt-2 h-1.5 overflow-hidden rounded-full bg-bg" role="progressbar" aria-label="Lesson progress" aria-valuemin="1" aria-valuemax=n aria-valuenow=move || pos.get() + 1>
                <div class="h-full rounded-full bg-text-muted transition-[width]" style=move || format!("width:{}%", (pos.get() + 1) * 100 / n)></div>
            </div>
            <div class="anim-fade mt-5 min-h-48" aria-live="polite">
                {move || all.with_value(|v| screen_view(l, v[pos.get()], ask))}
            </div>
            <div class="mt-6 flex items-center justify-between gap-3">
                <button type="button" class=PILL_GHOST disabled=move || pos.get() == 0 on:click=move |_| go(pos.get_untracked().saturating_sub(1))>"‹ Back"</button>
                {move || if pos.get() + 1 < n {
                    view! { <button type="button" class=PILL on:click=move |_| go(pos.get_untracked() + 1)>"Next ›"</button> }.into_any()
                } else {
                    view! { <button type="button" class=PILL on:click=move |_| to_quiz()>"On to the quiz ›"</button> }.into_any()
                }}
            </div>
        </section>

        <Show when=move || reached_end.get() || returning>
            <div id="lesson-quiz" class="scroll-mt-20">
                <h2 class="mt-10 text-title2">"Check your understanding"</h2>
                <div class="mt-3">
                    <QuizCard questions=l.quiz.iter().map(|q| QuizItem::course(l.id, q)).collect() on_done=on_quiz />
                </div>
            </div>
            <Show when=move || quiz_done.get()>
                <MorePractice lesson=l />
            </Show>

            <h2 class="mt-10 text-title2">"Explain it back"</h2>
            <div class="mt-3">
                <ExplainBack id=id prompt=l.explain.prompt />
            </div>

            <div class="mt-10">
                <GateStatus id=id />
            </div>
        </Show>

        <AskStep lesson=l ask=ask />

        <details class="mt-10 text-footnote text-text-muted">
            <summary class="press min-h-11 cursor-pointer py-2">"Sources for this lesson"</summary>
            <ul class="mt-2 grid gap-2">
                {l.sources
                    .iter()
                    .map(|s| {
                        view! {
                            <li>
                                <a href=s.url target="_blank" rel="noopener noreferrer" class="underline underline-offset-2">{s.name}</a>
                                {format!(" · checked {}", s.checked)}
                                <span class="block">{s.claim}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </details>
    }
}

/// Spaced Review (FR-13.2): up to three questions from earlier passed lessons, picked once storage
/// has answered (so retired questions are known) and stable for the day. Practice only.
#[component]
fn SpacedReview(current: u8) -> impl IntoView {
    use gold_core::retention::{QuestionKey, pick_review};
    let progress = expect_context::<Progress>();
    let store = expect_context::<Store>();
    let ready = Memo::new(move |_| !matches!(store.state.get(), SaveState::Connecting));
    let skipped = RwSignal::new(false);
    let picks = Memo::new(move |_| {
        if !ready.get() {
            return Vec::new();
        }
        // Picked once: answering (or passing this lesson) must not reshuffle a review in progress.
        untrack(|| {
        let passed: Vec<u8> = (1..=gold_core::course::LAST_CORE)
            .filter(|n| progress.passed(LessonId::Numbered(*n)))
            .collect();
        let recs = store.qrecs.get_untracked();
        let questions: Vec<(QuestionKey, &'static gold_content::QuizQ)> = passed
            .iter()
            .filter_map(|n| gold_content::lesson(*n))
            .flat_map(|l| l.quiz.iter().chain(l.recall.iter()).map(move |q| (QuestionKey::new(l.id, q.id), q)))
            .collect();
        let pool: Vec<_> = questions.iter().map(|(k, _)| (k.clone(), recs.get(&k.lesson).and_then(|m| m.get(&k.id)))).collect();
        let seed = (now() / gold_core::DAY_MS) as u64 * 1000 + current as u64;
        pick_review(&pool, &passed, current, 3, seed)
            .into_iter()
            .filter_map(|k| questions.iter().find(|(qk, _)| *qk == k).map(|(_, q)| QuizItem::course(k.lesson, q)))
            .collect::<Vec<_>>()
        })
    });
    move || {
        let items = picks.get();
        (!items.is_empty() && !skipped.get()).then(|| view! {
            <section class="mt-6" aria-label="Spaced Review">
                <div class="flex items-baseline justify-between gap-3">
                    <h2 class="text-headline">"A quick look back"</h2>
                    <button type="button" class="press min-h-11 text-callout underline underline-offset-4" on:click=move |_| skipped.set(true)>"Skip"</button>
                </div>
                <p class="mt-1 text-callout text-text-muted">"Questions from earlier lessons, mostly ones you missed or guessed. Practice only."</p>
                <div class="mt-3">
                    <QuizCard questions=items on_done=|_| {} practice=true label="Spaced Review" />
                </div>
            </section>
        })
    }
}

/// One question and Claude's answer, kept for this page load only.
#[derive(Clone, PartialEq)]
pub struct Asked {
    lesson: u8,
    step: usize,
    question: String,
    answer: String,
    truncated: bool,
}

/// This page load's tutor answers (provided by the shell).
#[derive(Clone, Copy)]
pub struct TutorLog(pub RwSignal<Vec<Asked>>);

/// "Ask Claude about this step" sheet (Phase 12). Plain-text answers that stay on the lesson.
#[component]
fn AskStep(lesson: &'static Lesson, ask: RwSignal<Option<usize>>) -> impl IntoView {
    let log = expect_context::<TutorLog>().0;
    let open = RwSignal::new(false);
    // Keep `open` (the sheet) and `ask` (which step) in step. `set` always notifies, so each side
    // only writes on a real change; otherwise the two effects wake each other forever.
    Effect::new(move |_| {
        let want = ask.get().is_some();
        if open.get_untracked() != want {
            open.set(want);
        }
    });
    Effect::new(move |_| {
        if !open.get() && ask.get_untracked().is_some() {
            ask.set(None);
        }
    });
    let question = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let send = move || {
        let Some(step) = ask.get_untracked() else { return };
        let q = match gold_core::tutor::check_question(&question.get_untracked()) {
            Ok(q) => q,
            Err(gold_core::tutor::QuestionError::Empty) => return error.set(Some("Type a question first.".into())),
            Err(gold_core::tutor::QuestionError::TooLong) => {
                return error.set(Some(format!("Keep it under {} characters.", gold_core::tutor::MAX_QUESTION_CHARS)));
            }
        };
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match grading::ask_step(lesson, step, &q).await {
                Ok((answer, truncated)) => {
                    log.update(|v| v.push(Asked { lesson: lesson.id, step, question: q, answer, truncated }));
                    question.set(String::new());
                }
                Err(msg) => error.set(Some(msg)),
            }
            busy.set(false);
        });
    };
    view! {
        <Sheet open=open title="Ask Claude">
            {move || ask.get().map(|step| {
                let st = &lesson.steps[step];
                let earlier = log.with(|v| v.iter().filter(|a| a.lesson == lesson.id && a.step == step).cloned().collect::<Vec<_>>());
                view! {
                    <p class="text-footnote text-text-muted">{format!("About “{}”. Claude answers about this lesson only, and never tells you what to trade.", st.heading)}</p>
                    <div class="mt-3 grid gap-3" aria-live="polite">
                        {earlier.into_iter().map(|a| view! {
                            <div class="rounded-btn bg-bg p-3.5">
                                <p class="text-callout font-semibold">{a.question}</p>
                                <p class="mt-1.5 whitespace-pre-line text-callout">{a.answer}</p>
                                {a.truncated.then(|| view! { <p class="mt-1 text-footnote text-text-muted">"The answer was cut short. Try a narrower question."</p> })}
                            </div>
                        }).collect_view()}
                    </div>
                }
            })}
            <label class="mt-3 block">
                <span class="text-callout font-semibold">"Your question"</span>
                <textarea rows="3" maxlength="500" class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout placeholder:text-text-muted" placeholder="What part of this step is unclear?" prop:value=move || question.get() on:input=move |ev| question.set(event_target_value(&ev))></textarea>
            </label>
            {move || error.get().map(|e| view! { <p class="mt-2 text-callout text-down" role="alert">{e}</p> })}
            <div class="mt-3 flex items-center justify-between gap-3">
                <p class="text-footnote text-text-muted">"Answers stay until you close the page."</p>
                <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| send()>
                    {move || if busy.get() { "Asking…" } else { "Ask" }}
                </button>
            </div>
        </Sheet>
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

/// The Learner's answer with the missing idea added, additions highlighted (the [[...]] parts).
fn suggestion_view(s: String) -> impl IntoView {
    let parts = gold_core::grading::marked_segments(&s);
    view! {
        <div class="mt-3 rounded-btn border-l-4 border-fill bg-surface p-3.5">
            <p class="text-eyebrow uppercase text-text-muted">"Your answer, with the missing piece added"</p>
            <p class="mt-1.5 text-callout">
                {parts
                    .into_iter()
                    .map(|(t, added)| {
                        if added {
                            view! { <mark class="rounded bg-fill/45 px-0.5 font-semibold text-text underline decoration-2 underline-offset-2">{t}</mark> }.into_any()
                        } else {
                            view! { <span>{t}</span> }.into_any()
                        }
                    })
                    .collect_view()}
            </p>
            <p class="mt-2 text-footnote text-text-muted">"The highlighted part is what was missing. Read it, then explain it again in your own words."</p>
        </div>
    }
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
    let lesson = content_for(id);
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
        let Some(lesson) = lesson else {
            phase.set(Phase::Failed("This lesson doesn't have a grading rubric yet.".into()));
            return;
        };
        let answer = text.get_untracked();
        phase.set(Phase::Grading);
        spawn_local(async move {
            match grading::grade(lesson, &answer).await {
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
                        gap: res.gap.clone(),
                        reteach: res.reteach.clone(),
                        suggestion: res.suggestion.clone(),
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
            let suggested = res.suggestion.clone().map(suggestion_view);
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
                    {suggested}
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
                attempts
                    .with(|a| a.first().cloned())
                    .filter(|_| !open.get())
                    .map(|a| {
                        let reteach = (a.grade == Grade::NotYet).then_some(a.reteach.clone()).flatten();
                        view! {
                            <div class="anim-fade mt-4 rounded-btn bg-bg p-4">
                                <p class="text-eyebrow uppercase text-text-muted">{format!("Your last answer · {}", short_date(a.at))}</p>
                                <p class="mt-1.5 text-callout">{a.feedback.clone()}</p>
                                {(a.grade == Grade::Almost).then_some(a.gap.clone()).flatten().map(|g| view! {
                                    <p class="mt-2 text-callout"><span class="font-semibold">"What's missing: "</span>{g}</p>
                                })}
                                {reteach.map(|r| view! { <p class="mt-2 text-callout"><span class="font-semibold">"💡 Another way to see it: "</span>{r}</p> })}
                                {a.suggestion.clone().map(suggestion_view)}
                            </div>
                        }
                    })
            }}
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum MoreScope {
    ThisLesson,
    SoFar,
}

/// After the quiz: "Want to be quizzed on more concepts?" Claude writes a fresh practice round on
/// this lesson, or mixed with earlier lessons already passed. Practice never affects the Gate.
#[component]
fn MorePractice(lesson: &'static Lesson) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let round = RwSignal::new(None::<(u32, Vec<QuizItem>)>);
    let asking = RwSignal::new(true);
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let seen = StoredValue::new(lesson.quiz.iter().map(|q| q.q.to_string()).collect::<Vec<String>>());
    let rounds = RwSignal::new(0u32);

    let earlier = move || -> Vec<&'static Lesson> {
        gold_content::LESSONS.iter().filter(|l| l.id < lesson.id && progress.passed(LessonId::Numbered(l.id))).collect()
    };
    let start = move |scope: MoreScope| {
        let mut sources: Vec<&'static Lesson> = vec![lesson];
        if scope == MoreScope::SoFar {
            sources.extend(earlier());
        }
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            let avoid = seen.get_value();
            match grading::practice(&sources, &avoid).await {
                Ok(qs) => {
                    seen.update_value(|s| s.extend(qs.iter().map(|q| q.q.clone())));
                    let n = rounds.get_untracked() + 1;
                    rounds.set(n);
                    round.set(Some((n, qs.into_iter().map(QuizItem::from).collect())));
                    asking.set(false);
                }
                Err(msg) => error.set(Some(msg)),
            }
            busy.set(false);
        });
    };
    view! {
        <div class="mt-5 grid gap-5">
            {move || {
                round.get().map(|(n, items)| {
                    view! {
                        <div>
                            <p class="mb-2 text-eyebrow uppercase text-text-muted">{format!("Practice round {n} · written by Claude")}</p>
                            <QuizCard questions=items on_done=move |_| asking.set(true) practice=true />
                        </div>
                    }
                })
            }}
            <Show when=move || asking.get()>
                <section class="anim-fade rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                    <h3 class="text-headline">
                        {move || if rounds.get() == 0 { "Want to be quizzed on more concepts?" } else { "Another round?" }}
                    </h3>
                    <p class="mt-1 text-callout text-text-muted">
                        "Claude writes 5 fresh questions. Practice only: it never changes your Gate."
                    </p>
                    <div class="mt-4 flex flex-wrap gap-3">
                        <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| start(MoreScope::ThisLesson)>
                            "Yes, on this lesson"
                        </button>
                        {move || {
                            let n = earlier().len();
                            (n > 0)
                                .then(|| {
                                    view! {
                                        <button type="button" class=PILL_GHOST disabled=move || busy.get() on:click=move |_| start(MoreScope::SoFar)>
                                            {format!("Yes, mix in {n} earlier lesson{}", if n == 1 { "" } else { "s" })}
                                        </button>
                                    }
                                })
                        }}
                        <button type="button" class="press min-h-11 px-3 text-callout text-text-muted" on:click=move |_| asking.set(false)>
                            "No thanks"
                        </button>
                    </div>
                    {move || busy.get().then(|| view! { <p class="mt-3 text-callout" role="status">"Claude is writing your questions…"</p> })}
                    {move || error.get().map(|e| view! { <p class="mt-3 text-callout text-down" role="alert">"⚠ " {e}</p> })}
                </section>
            </Show>
        </div>
    }
}
