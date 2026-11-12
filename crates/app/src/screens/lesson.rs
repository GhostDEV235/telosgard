//! Lesson reader: text → videos → quiz → Explain-Back → gate (CURRICULUM authoring order).

use crate::i18n::{t, ts};
use crate::catalog;
use crate::grading;
use telosgard_content::Lesson;
use crate::state::{Progress, Toasts, now};
use crate::store::{Attempt, SaveState, Store, repo};
use telosgard_core::gate::Grade;
use telosgard_core::grading::GradeResult;
use leptos::task::spawn_local;
use crate::ui::controls::{PILL, PILL_GHOST};
use crate::ui::course::{Callout, CalloutKind, GateStatus, VideoCard};
use crate::ui::quiz::{QuizCard, QuizItem};
use crate::ui::sheet::Sheet;
use telosgard_core::course::LessonId;
use leptos::prelude::*;

#[component]
pub fn LessonScreen(id: LessonId) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let title = catalog::title(id);
    // Only the lock state decides which view shows. Reading progress directly here re-rendered the
    // whole reader on every save, which reset the quiz to question 1 and lost Explain-Back results.
    let unlocked = Memo::new(move |_| progress.unlocked(id));
    view! {
        <crate::screens::shell::SubBar back="#today" back_label=t("Today", "Aujourd’hui") title=title />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <p class="text-eyebrow text-eyebrow-ink">{catalog::eyebrow(id)}</p>
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
        Some(p) if !matches!(id.track(), telosgard_core::course::Track::Live | telosgard_core::course::Track::Advanced) => {
            ts(
                format!("Pass “{}” first. Lessons build on each other, so none can be skipped.", catalog::title(p)),
                format!("Réussis d’abord « {} ». Les leçons s’appuient les unes sur les autres : aucune ne peut être sautée.", catalog::title(p)),
            )
        }
        _ => t("This track unlocks with Live Readiness: 50 disciplined Demo Trades over at least 8 weeks.", "Ce parcours se débloque avec « Prêt pour le réel » : 50 Trades démo disciplinés sur au moins 8 semaines.").to_string(),
    };
    view! {
        <section class="mt-6 rounded-card bg-locked p-5">
            <p class="text-headline">{t("🔒 Locked", "🔒 Verrouillée")}</p>
            <p class="mt-1 text-callout">{why}</p>
        </section>
    }
}

pub fn content_for(id: LessonId) -> Option<&'static Lesson> {
    match id {
        LessonId::Numbered(n) => crate::i18n::lesson(n),
        _ => None,
    }
}

fn not_written() -> impl IntoView {
    view! {
        <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
            <p class="text-headline">{t("This lesson is written when you get close to it", "Cette leçon sera écrite quand tu en approcheras")}</p>
            <p class="mt-1 text-callout text-text-muted">
                {t("Each stage is written using what worked for you in the one before: which analogies landed, and where you got “Almost”.", "Chaque étape est écrite à partir de ce qui a marché pour toi dans la précédente : les analogies qui ont parlé, et là où tu as eu « Presque ».")}
            </p>
        </section>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Hook,
    Step(usize),
    Analogy,
    Example,
    Mistake,
    Takeaways,
    Videos,
    Recall,
}

pub fn screens(l: &Lesson) -> Vec<Screen> {
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
            <p class="text-eyebrow text-eyebrow-ink">{t("Start with a question", "Commence par une question")}</p>
            <p class="mt-2 text-title2">{l.hook}</p>
            <p class="mt-3 text-callout text-text-muted">{t("Have a guess in your head, then tap Next.", "Fais une hypothèse dans ta tête, puis touche Suivant.")}</p>
        }
        .into_any(),
        Screen::Step(i) => {
            let st = &l.steps[i];
            view! {
                <h2 class="font-display text-title2 font-medium">{st.heading}</h2>
                <div class="lesson-prose mt-3" inner_html=st.text_html></div>
                {st.widget.map(|w| view! { <div class="mt-4"><crate::ui::widgets::Widget kind=w /></div> })}
                {st.why_html.map(|w| view! {
                    <details class="mt-4 rounded-btn bg-bg p-3.5">
                        <summary class="press min-h-11 cursor-pointer py-1 text-callout font-semibold">{t("Why? Tell me more", "Pourquoi ? Dis-m’en plus")}</summary>
                        <div class="lesson-prose mt-2 text-callout" inner_html=w></div>
                    </details>
                })}
                <button type="button" class="press mt-4 min-h-11 text-callout underline underline-offset-4" on:click=move |_| {
                    if wide() {
                        if let Some(el) = document().get_element_by_id("ask-panel-q").and_then(|e| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(e).ok()) {
                            let _ = el.focus();
                        }
                    } else {
                        ask.set(Some(i))
                    }
                }>
                    {t("Ask Professor Aurel about this step", "Poser une question au Professeur Aurel sur cette partie")}
                </button>
            }
            .into_any()
        }
        Screen::Analogy => view! { <Callout kind=CalloutKind::Analogy text=l.analogy /> }.into_any(),
        Screen::Example => view! { <Callout kind=CalloutKind::Example text=l.example /> }.into_any(),
        Screen::Mistake => view! { <Callout kind=CalloutKind::Mistake text=l.mistake /> }.into_any(),
        Screen::Takeaways => view! {
            <h2 class="text-headline">{t("Key takeaways", "À retenir")}</h2>
            <ul class="mt-3 grid gap-2">
                {l.takeaways.iter().map(|tk| view! {
                    <li class="flex gap-3 text-callout"><span class="text-text-muted" aria-hidden="true">"✓"</span><span>{*tk}</span></li>
                }).collect_view()}
            </ul>
        }
        .into_any(),
        Screen::Videos => view! {
            <h2 class="text-headline">{t("Watch (optional)", "À regarder (facultatif)")}</h2>
            <div class="mt-3 grid gap-3">{l.videos.iter().map(|v| view! { <VideoCard video=v /> }).collect_view()}</div>
        }
        .into_any(),
        Screen::Recall => view! {
            <p class="text-eyebrow text-eyebrow-ink">{t("Recall · from memory", "Rappel · de mémoire")}</p>
            <p class="mt-1 text-callout text-text-muted">{t("Three quick questions before the quiz. Practice only.", "Trois petites questions avant le quiz. Pour s’entraîner seulement.")}</p>
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
    let ui = expect_context::<LessonUi>();
    let moments = expect_context::<crate::ui::moments::Moments>();
    let pos = ui.pos;
    let forward = RwSignal::new(true);
    pos.set(0);
    ui.lesson.set(Some(l.id));
    let this = l.id;
    on_cleanup(move || {
        if ui.lesson.get_untracked() == Some(this) {
            ui.lesson.set(None);
        }
    });
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
            toasts.show(t("🏅 Lesson passed", "🏅 Leçon réussie"));
            moments.lesson_passed(progress, id);
        }
    };
    let go = move |to: usize| {
        forward.set(to >= pos.get_untracked());
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
        <p class="mt-3 text-footnote text-text-muted">{ts(format!("About {} minutes, step by step", l.read_minutes), format!("Environ {} minutes, pas à pas", l.read_minutes))}</p>
        <p class="mt-2 text-callout text-text-muted">{t("By the end, you'll be able to ", "À la fin, tu sauras ")} {l.objective}</p>
        {returning.then(|| view! {
            <button type="button" class="press mt-2 min-h-11 text-callout underline underline-offset-4" on:click=move |_| to_quiz()>{t("Skip to the quiz ›", "Aller directement au quiz ›")}</button>
        })}

        <SpacedReview current=l.id />

        <section id="lesson-stepper" class="mt-6 scroll-mt-20 rounded-card bg-surface p-5 shadow-card" aria-label=t("Lesson steps", "Parties de la leçon")>
            <div class="flex items-center justify-between gap-3">
                <p class="tabular text-eyebrow text-eyebrow-ink">{move || ts(format!("Step {} of {n}", pos.get() + 1), format!("Partie {} sur {n}", pos.get() + 1))}</p>
            </div>
            <div class="mt-2 h-1.5 overflow-hidden rounded-full bg-bg" role="progressbar" aria-label=t("Lesson progress", "Progression dans la leçon") aria-valuemin="1" aria-valuemax=n aria-valuenow=move || pos.get() + 1>
                <div class="h-full rounded-full bg-fill transition-[width]" style=move || format!("width:{}%", (pos.get() + 1) * 100 / n)></div>
            </div>
            <div class="mt-5 min-h-48" aria-live="polite">
                {move || {
                    let p = pos.get();
                    let class = if forward.get_untracked() { "anim-step-next" } else { "anim-step-prev" };
                    view! { <div class=class>{all.with_value(|v| screen_view(l, v[p], ask))}</div> }
                }}
            </div>
            <div class="mt-6 flex items-center justify-between gap-3">
                <button type="button" class=PILL_GHOST disabled=move || pos.get() == 0 on:click=move |_| go(pos.get_untracked().saturating_sub(1))>{t("‹ Back", "‹ Retour")}</button>
                {move || if pos.get() + 1 < n {
                    view! { <button type="button" class=PILL on:click=move |_| go(pos.get_untracked() + 1)>{t("Next ›", "Suivant ›")}</button> }.into_any()
                } else {
                    view! { <button type="button" class=PILL on:click=move |_| to_quiz()>{t("On to the quiz ›", "Place au quiz ›")}</button> }.into_any()
                }}
            </div>
        </section>

        <Show when=move || reached_end.get() || returning>
            <div id="lesson-quiz" class="scroll-mt-20">
                <h2 class="mt-10 font-display text-title2 font-medium">{t("Check your understanding", "Vérifie ta compréhension")}</h2>
                <div class="mt-3">
                    <QuizCard questions=l.quiz.iter().map(|q| QuizItem::course(l.id, q)).collect() on_done=on_quiz />
                </div>
            </div>
            <Show when=move || quiz_done.get()>
                <MorePractice lesson=l />
            </Show>

            <h2 class="mt-10 font-display text-title2 font-medium">{t("Explain it back", "Explique-le avec tes mots")}</h2>
            <div class="mt-3">
                <ExplainBack id=id prompt=l.explain.prompt />
            </div>

            <div class="mt-10">
                <GateStatus id=id />
            </div>
        </Show>

        <AskStep lesson=l ask=ask />

        <details class="mt-10 text-footnote text-text-muted lg:hidden">
            <summary class="press min-h-11 cursor-pointer py-2">{t("Sources for this lesson", "Sources de cette leçon")}</summary>
            <ul class="mt-2 grid gap-2">
                {l.sources
                    .iter()
                    .map(|s| {
                        view! {
                            <li>
                                <a href=s.url target="_blank" rel="noopener noreferrer" class="underline underline-offset-2">{s.name}</a>
                                {ts(format!(" · checked {}", s.checked), format!(" · vérifiée le {}", s.checked))}
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
    use telosgard_core::retention::{QuestionKey, pick_review};
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
        let passed: Vec<u8> = (1..=telosgard_core::course::LAST_CORE)
            .filter(|n| progress.passed(LessonId::Numbered(*n)))
            .collect();
        let recs = store.qrecs.get_untracked();
        let questions: Vec<(QuestionKey, &'static telosgard_content::QuizQ)> = passed
            .iter()
            .filter_map(|n| crate::i18n::lesson(*n))
            .flat_map(|l| l.quiz.iter().chain(l.recall.iter()).map(move |q| (QuestionKey::new(l.id, q.id), q)))
            .collect();
        let pool: Vec<_> = questions.iter().map(|(k, _)| (k.clone(), recs.get(&k.lesson).and_then(|m| m.get(&k.id)))).collect();
        let seed = (now() / telosgard_core::DAY_MS) as u64 * 1000 + current as u64;
        pick_review(&pool, &passed, current, 3, seed)
            .into_iter()
            .filter_map(|k| questions.iter().find(|(qk, _)| *qk == k).map(|(_, q)| QuizItem::course(k.lesson, q)))
            .collect::<Vec<_>>()
        })
    });
    move || {
        let items = picks.get();
        (!items.is_empty() && !skipped.get()).then(|| view! {
            <section class="mt-6" aria-label=t("Spaced Review", "Révision espacée")>
                <div class="flex items-baseline justify-between gap-3">
                    <h2 class="text-headline">{t("A quick look back", "Petit retour en arrière")}</h2>
                    <button type="button" class="press min-h-11 text-callout underline underline-offset-4" on:click=move |_| skipped.set(true)>{t("Skip", "Passer")}</button>
                </div>
                <p class="mt-1 text-callout text-text-muted">{t("Questions from earlier lessons, mostly ones you missed or guessed. Practice only.", "Des questions des leçons précédentes, surtout celles que tu as ratées ou devinées. Pour s’entraîner seulement.")}</p>
                <div class="mt-3">
                    <QuizCard questions=items on_done=|_| {} practice=true label=t("Spaced Review", "Révision espacée") />
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

/// Which lesson is on screen and its current step, shared with the wide-screen lesson panel.
#[derive(Clone, Copy)]
pub struct LessonUi {
    pub lesson: RwSignal<Option<u8>>,
    pub pos: RwSignal<usize>,
}

impl LessonUi {
    pub fn new() -> LessonUi {
        LessonUi { lesson: RwSignal::new(None), pos: RwSignal::new(0) }
    }
}

fn wide() -> bool {
    window().inner_width().ok().and_then(|v| v.as_f64()).is_some_and(|w| w >= 1024.0)
}

/// The question form and this page load's answers for one step. Used in the phone sheet and in the
/// wide-screen lesson panel. Plain-text answers that stay on the lesson (Phase 12).
#[component]
fn AskBody(lesson: &'static Lesson, #[prop(into)] step: Signal<Option<usize>>, input_id: &'static str) -> impl IntoView {
    let log = expect_context::<TutorLog>().0;
    let question = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let send = move || {
        let Some(step) = step.get_untracked() else { return };
        let q = match telosgard_core::tutor::check_question(&question.get_untracked()) {
            Ok(q) => q,
            Err(telosgard_core::tutor::QuestionError::Empty) => return error.set(Some(t("Type a question first.", "Écris d’abord une question.").into())),
            Err(telosgard_core::tutor::QuestionError::TooLong) => {
                return error.set(Some(ts(format!("Keep it under {} characters.", telosgard_core::tutor::MAX_QUESTION_CHARS), format!("Reste sous les {} caractères.", telosgard_core::tutor::MAX_QUESTION_CHARS))));
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
    move || match step.get() {
        None => view! { <p class="text-callout text-text-muted">{t("Open one of the lesson's steps to ask Professor Aurel about it.", "Ouvre une des parties de la leçon pour poser ta question au Professeur Aurel.")}</p> }.into_any(),
        Some(i) => {
            let st = &lesson.steps[i];
            let earlier = log.with(|v| v.iter().filter(|a| a.lesson == lesson.id && a.step == i).cloned().collect::<Vec<_>>());
            view! {
                <p class="text-footnote text-text-muted">{ts(format!("About “{}”. Professor Aurel answers about this lesson only, and never tells you what to trade. He is Telosgard's AI tutor.", st.heading), format!("Sur « {} ». Le Professeur Aurel ne répond que sur cette leçon, et ne te dit jamais quoi trader. C’est le tuteur IA de Telosgard.", st.heading))}</p>
                <div class="mt-3 grid gap-3" aria-live="polite">
                    {earlier.into_iter().map(|a| view! {
                        <div class="rounded-btn bg-surface p-3.5 shadow-card">
                            <p class="text-callout font-semibold">{a.question}</p>
                            <p class="mt-1.5 whitespace-pre-line text-callout">{a.answer}</p>
                            {a.truncated.then(|| view! { <p class="mt-1 text-footnote text-text-muted">{t("The answer was cut short. Try a narrower question.", "La réponse a été coupée. Essaie une question plus précise.")}</p> })}
                        </div>
                    }).collect_view()}
                </div>
                <label class="mt-3 block">
                    <span class="text-callout font-semibold">{t("Your question", "Ta question")}</span>
                    <textarea id=input_id rows="3" maxlength="500" class="mt-1.5 w-full rounded-field bg-surface p-3.5 text-callout shadow-card placeholder:text-text-muted" placeholder=t("What part of this step is unclear?", "Quel passage de cette partie n’est pas clair ?") prop:value=move || question.get() on:input=move |ev| question.set(event_target_value(&ev))></textarea>
                </label>
                {move || error.get().map(|e| view! { <p class="mt-2 text-callout text-down" role="alert">{e}</p> })}
                <div class="mt-3 flex items-center justify-between gap-3">
                    <p class="text-footnote text-text-muted">{t("Answers stay until you close the page.", "Les réponses restent jusqu’à ce que tu fermes la page.")}</p>
                    <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| send()>
                        {move || if busy.get() { t("Asking…", "Question envoyée…") } else { t("Ask", "Demander") }}
                    </button>
                </div>
            }.into_any()
        }
    }
}

/// The phone sheet for "Ask Professor Aurel about this step".
#[component]
fn AskStep(lesson: &'static Lesson, ask: RwSignal<Option<usize>>) -> impl IntoView {
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
    view! {
        <Sheet open=open title=t("Ask Professor Aurel", "Demander au Professeur Aurel")>
            <AskBody lesson=lesson step=Signal::derive(move || ask.get()) input_id="ask-sheet-q" />
        </Sheet>
    }
}

pub fn screen_title(l: &Lesson, s: Screen) -> &'static str {
    match s {
        Screen::Hook => t("Start with a question", "Commence par une question"),
        Screen::Step(i) => l.steps[i].heading,
        Screen::Analogy => t("Analogy", "Analogie"),
        Screen::Example => t("Real life", "Dans la vraie vie"),
        Screen::Mistake => t("Common mistake", "Erreur fréquente"),
        Screen::Takeaways => t("Key takeaways", "À retenir"),
        Screen::Videos => t("Watch (optional)", "À regarder (facultatif)"),
        Screen::Recall => t("Recall", "Rappel"),
    }
}

/// Wide-screen right panel for a lesson: progress (jump to any step), the Gate, Ask Claude about the
/// current step, and sources.
#[component]
pub fn LessonPanel(id: LessonId) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let ui = expect_context::<LessonUi>();
    let unlocked = Memo::new(move |_| progress.unlocked(id));
    let body = move || {
        let l = content_for(id).filter(|_| unlocked.get())?;
        let all = screens(l);
        let n = all.len();
        let all = StoredValue::new(all);
        let step = Signal::derive(move || all.with_value(|v| match v.get(ui.pos.get()) {
            Some(Screen::Step(i)) => Some(*i),
            _ => None,
        }));
        let check = |met: bool, text: &'static str| view! {
            <li class="flex items-center gap-2 text-callout">
                <span class=if met { "text-gold" } else { "text-text-muted" } aria-hidden="true">{if met { "✓" } else { "○" }}</span>
                <span class=if met { "" } else { "text-text-muted" }>{text}</span>
                <span class="sr-only">{if met { t(", met", ", atteint") } else { t(", not met yet", ", pas encore atteint") }}</span>
            </li>
        };
        Some(view! {
            <p class="text-eyebrow text-eyebrow-ink">{t("Progress", "Progression")}</p>
            <p class="tabular mt-1 text-callout">{move || ts(format!("Step {} of {n}", ui.pos.get().min(n - 1) + 1), format!("Partie {} sur {n}", ui.pos.get().min(n - 1) + 1))}</p>
            <div class="mt-2 h-1 overflow-hidden rounded-full bg-surface" aria-hidden="true">
                <div class="h-full rounded-full bg-fill transition-[width] duration-200" style=move || format!("width:{}%", (ui.pos.get().min(n - 1) + 1) * 100 / n)></div>
            </div>
            <ol class="mt-3 grid grid-cols-1 gap-px">
                {all.with_value(|v| v.iter().enumerate().map(|(i, s)| {
                    let title = screen_title(l, *s);
                    view! {
                        <li>
                            <button
                                type="button"
                                aria-current=move || (ui.pos.get() == i).then_some("step")
                                class=move || format!(
                                    "press flex min-h-8 w-full items-center gap-2 rounded-[8px] px-2 text-left text-footnote {}",
                                    if ui.pos.get() == i { "bg-surface font-semibold text-text" } else { "text-text-muted hover:bg-surface/60 hover:text-text" },
                                )
                                on:click=move |_| ui.pos.set(i)
                            >
                                <span class="tabular w-5 shrink-0 text-right">{i + 1}</span>
                                <span class="min-w-0 flex-1 truncate">{title}</span>
                            </button>
                        </li>
                    }
                }).collect_view())}
            </ol>
            <div class="my-5 border-t border-hairline"></div>
            <p class="text-eyebrow text-eyebrow-ink">{t("To unlock the next lesson", "Pour débloquer la leçon suivante")}</p>
            {move || {
                let p = progress.get(id);
                view! {
                    <ul class="mt-2 grid gap-1.5">
                        {check(p.quiz_met(), t("Quiz ≥ 80%", "Quiz ≥ 80 %"))}
                        {check(p.explain_back_met(), t("Explain-Back: Got it", "Explique-le : Acquis"))}
                    </ul>
                    {progress.passed(id).then(|| view! { <p class="mt-2 text-callout text-gold">{t("🏅 Lesson passed", "🏅 Leçon réussie")}</p> })}
                }
            }}
            <div class="my-5 border-t border-hairline"></div>
            <p class="text-eyebrow text-eyebrow-ink">{t("Ask Professor Aurel", "Demander au Professeur Aurel")}</p>
            <div class="mt-2"><AskBody lesson=l step=step input_id="ask-panel-q" /></div>
            <div class="my-5 border-t border-hairline"></div>
            <p class="text-eyebrow text-eyebrow-ink">"Sources"</p>
            <ul class="mt-2 grid gap-2 text-footnote text-text-muted">
                {l.sources.iter().map(|s| view! {
                    <li>
                        <a href=s.url target="_blank" rel="noopener noreferrer" class="underline underline-offset-2">{s.name}</a>
                        {ts(format!(" · checked {}", s.checked), format!(" · vérifiée le {}", s.checked))}
                    </li>
                }).collect_view()}
            </ul>
        })
    };
    view! {
        <aside aria-label=t("Lesson panel", "Panneau de la leçon") class="sticky top-0 hidden h-dvh w-[280px] shrink-0 overflow-y-auto border-l border-hairline bg-bg px-4 xl:w-[320px] xl:px-5 py-6 lg:block">
            {body}
        </aside>
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
        Grade::GotIt => (t("✓ Got it", "✓ Acquis"), "bg-ok-bg text-ok-ink"),
        Grade::Almost => (t("Almost", "Presque"), "bg-warn-bg text-warn-ink"),
        Grade::NotYet => (t("Not yet", "Pas encore"), "bg-bad-bg text-bad-ink"),
    };
    view! { <span class=format!("inline-block rounded-full px-3 py-1 text-footnote font-semibold {class}")>{label}</span> }
}

/// The Learner's answer with the missing idea added, additions highlighted (the [[...]] parts).
fn suggestion_view(s: String) -> impl IntoView {
    let parts = telosgard_core::grading::marked_segments(&s);
    view! {
        <div class="mt-3 rounded-btn border-l-4 border-fill bg-surface p-3.5">
            <p class="text-eyebrow text-eyebrow-ink">{t("Your answer, with the missing piece added", "Ta réponse, avec ce qui manquait")}</p>
            <p class="mt-1.5 text-callout">
                {parts
                    .into_iter()
                    .map(|(seg, added)| {
                        if added {
                            view! { <mark class="rounded bg-fill/45 px-0.5 font-semibold text-text underline decoration-2 underline-offset-2">{seg}</mark> }.into_any()
                        } else {
                            view! { <span>{seg}</span> }.into_any()
                        }
                    })
                    .collect_view()}
            </p>
            <p class="mt-2 text-footnote text-text-muted">{t("The highlighted part is what was missing. Read it, then explain it again in your own words.", "La partie surlignée est ce qui manquait. Lis-la, puis réexplique avec tes propres mots.")}</p>
        </div>
    }
}

fn short_date(ms: i64) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64));
    const M: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    const MF: [&str; 12] = ["janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.", "oct.", "nov.", "déc."];
    let m = d.get_month() as usize % 12;
    format!("{} {} · {:02}:{:02}", d.get_date(), t(M[m], MF[m]), d.get_hours(), d.get_minutes())
}

#[component]
fn ExplainBack(id: LessonId, prompt: &'static str) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let store = expect_context::<Store>();
    let toasts = expect_context::<Toasts>();
    let moments = expect_context::<crate::ui::moments::Moments>();
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
            phase.set(Phase::Failed(t("This lesson doesn't have a grading rubric yet.", "Cette leçon n’a pas encore de grille de correction.").into()));
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
                        toasts.show(t("🏅 Lesson passed", "🏅 Leçon réussie"));
                        moments.lesson_passed(progress, id);
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
            <p class="anim-fade mt-3 rounded-btn bg-bg p-3.5 text-callout" role="status">{t("Professor Aurel is reading your answer…", "Le Professeur Aurel lit ta réponse…")}</p>
        }
        .into_any(),
        Phase::Failed(msg) => view! {
            <div class="anim-fade mt-3 rounded-btn bg-bg p-3.5" role="alert">
                <p class="text-callout">{msg}</p>
                <button type="button" class=format!("{PILL_GHOST} mt-3") on:click=submit>{t("Try again", "Réessayer")}</button>
            </div>
        }
        .into_any(),
        Phase::Done(res) => {
            let extra = match res.grade {
                Grade::GotIt => res.gap.clone().map(|tip| {
                    view! { <p class="mt-3 text-callout"><span class="font-semibold">{t("One extra tip: ", "Un conseil en plus : ")}</span>{tip}</p> }.into_any()
                }),
                Grade::Almost => res.gap.clone().map(|gap| {
                    view! {
                        <div class="mt-3 rounded-btn border-l-4 border-fill bg-surface p-3.5">
                            <p class="text-eyebrow text-eyebrow-ink">{t("What's missing", "Ce qui manque")}</p>
                            <p class="mt-1 text-callout">{gap}</p>
                        </div>
                    }
                    .into_any()
                }),
                Grade::NotYet => res.reteach.clone().map(|r| {
                    view! {
                        <div class="mt-3 rounded-btn border-l-4 border-fill bg-surface p-3.5">
                            <p class="text-eyebrow text-eyebrow-ink">{t("💡 Another way to see it", "💡 Une autre façon de voir")}</p>
                            <p class="mt-1 text-callout">{r}</p>
                        </div>
                    }
                    .into_any()
                }),
            };
            let suggested = res.suggestion.clone().map(suggestion_view);
            let next = if res.grade == Grade::GotIt {
                view! { <button type="button" class=format!("{PILL} mt-4") on:click=move |_| open.set(false)>{t("Done", "Terminé")}</button> }.into_any()
            } else {
                let label = if res.grade == Grade::Almost { t("Rewrite my answer", "Réécrire ma réponse") } else { t("Try again", "Réessayer") };
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
                {t("Write it like you're explaining to a friend. Professor Aurel reads it for meaning, not wording.", "Écris comme si tu expliquais à un ami. Le Professeur Aurel juge le sens, pas les mots.")}
            </p>
            <div class="mt-4 flex flex-wrap items-center gap-3">
                <button type="button" class=PILL on:click=move |_| open.set(true)>
                    {move || if progress.get(id).explain_back_met() { t("Answer again", "Répondre à nouveau") } else { t("Write your answer", "Écrire ta réponse") }}
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
                                <p class="text-eyebrow text-eyebrow-ink">{ts(format!("Your last answer · {}", short_date(a.at)), format!("Ta dernière réponse · {}", short_date(a.at)))}</p>
                                <p class="mt-1.5 text-callout">{a.feedback.clone()}</p>
                                {(a.grade == Grade::Almost).then_some(a.gap.clone()).flatten().map(|g| view! {
                                    <p class="mt-2 text-callout"><span class="font-semibold">{t("What's missing: ", "Ce qui manque : ")}</span>{g}</p>
                                })}
                                {reteach.map(|r| view! { <p class="mt-2 text-callout"><span class="font-semibold">{t("💡 Another way to see it: ", "💡 Une autre façon de voir : ")}</span>{r}</p> })}
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
                                {ts(format!("{n} previous attempt{} ", if n == 1 { "" } else { "s" }), format!("{n} tentative{} précédente{} ", if n == 1 { "" } else { "s" }, if n == 1 { "" } else { "s" }))}
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
        <Sheet open=open title=t("Explain-Back", "Explique-le")>
            <p class="text-callout">{prompt}</p>
            <textarea
                rows="7"
                aria-label=t("Your answer", "Ta réponse")
                placeholder=t("In my own words…", "Avec mes propres mots…")
                disabled=move || !editing()
                class="mt-3 w-full resize-y rounded-field bg-bg p-3.5 text-body text-text placeholder:text-text-muted focus:outline-2 focus:outline-text disabled:opacity-60"
                prop:value=move || text.get()
                on:input=move |ev| text.set(event_target_value(&ev))
            ></textarea>
            <p class="tabular mt-1 text-right text-footnote text-text-muted">{move || ts(format!("{} words", text.get().split_whitespace().count()), format!("{} mots", text.get().split_whitespace().count()))}</p>
            {result}
            <Show when=editing>
                <div class="mt-4 flex flex-wrap justify-end gap-3">
                    <button type="button" class=PILL_GHOST on:click=move |_| open.set(false)>{t("Close", "Fermer")}</button>
                    <button type="button" class=PILL disabled=move || text.get().split_whitespace().count() < 5 on:click=submit>
                        {t("Submit for grading", "Envoyer pour correction")}
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
        crate::i18n::lessons().into_iter().filter(|l| l.id < lesson.id && progress.passed(LessonId::Numbered(l.id))).collect()
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
                            <p class="mb-2 text-eyebrow text-eyebrow-ink">{ts(format!("Practice round {n} · written by Professor Aurel"), format!("Série d’entraînement {n} · écrite par le Professeur Aurel"))}</p>
                            <QuizCard questions=items on_done=move |_| asking.set(true) practice=true />
                        </div>
                    }
                })
            }}
            <Show when=move || asking.get()>
                <section class="anim-fade rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                    <h3 class="text-headline">
                        {move || if rounds.get() == 0 { t("Want to be quizzed on more concepts?", "Envie de questions sur d’autres notions ?") } else { t("Another round?", "Une autre série ?") }}
                    </h3>
                    <p class="mt-1 text-callout text-text-muted">
                        {t("Professor Aurel writes 5 fresh questions. Practice only: it never changes your Gate.", "Le Professeur Aurel écrit 5 nouvelles questions. Pour s’entraîner seulement : ça ne change jamais ta validation.")}
                    </p>
                    <div class="mt-4 flex flex-wrap gap-3">
                        <button type="button" class=PILL disabled=move || busy.get() on:click=move |_| start(MoreScope::ThisLesson)>
                            {t("Yes, on this lesson", "Oui, sur cette leçon")}
                        </button>
                        {move || {
                            let n = earlier().len();
                            (n > 0)
                                .then(|| {
                                    view! {
                                        <button type="button" class=PILL_GHOST disabled=move || busy.get() on:click=move |_| start(MoreScope::SoFar)>
                                            {ts(format!("Yes, mix in {n} earlier lesson{}", if n == 1 { "" } else { "s" }), format!("Oui, en mélangeant {n} leçon{} précédente{}", if n == 1 { "" } else { "s" }, if n == 1 { "" } else { "s" }))}
                                        </button>
                                    }
                                })
                        }}
                        <button type="button" class="press min-h-11 px-3 text-callout text-text-muted" on:click=move |_| asking.set(false)>
                            {t("No thanks", "Non merci")}
                        </button>
                    </div>
                    {move || busy.get().then(|| view! { <p class="mt-3 text-callout" role="status">{t("Professor Aurel is writing your questions…", "Le Professeur Aurel écrit tes questions…")}</p> })}
                    {move || error.get().map(|e| view! { <p class="mt-3 text-callout text-down" role="alert">"⚠ " {e}</p> })}
                </section>
            </Show>
        </div>
    }
}
