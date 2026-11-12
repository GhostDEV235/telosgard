//! One question at a time; questions and options reshuffle on every attempt (FR-2.4).
//! Used for the lesson quiz (counts towards the Gate), Recall, Spaced Review and Claude-written
//! practice rounds (never do). Questions with a key ask for Confidence before revealing the answer.

use crate::i18n::{t, ts};
use crate::state::now;
use crate::ui::controls::{PILL, PILL_GHOST};
use telosgard_core::drills::Rng;
use telosgard_core::gate::{QUIZ_PASS_PERCENT, quiz_score};
use telosgard_core::retention::{Confidence, QuestionKey};
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuizItem {
    pub q: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub why: String,
    /// Set for course questions: their answers feed Question Records.
    pub key: Option<QuestionKey>,
}

impl QuizItem {
    pub fn course(lesson: u8, q: &telosgard_content::QuizQ) -> QuizItem {
        QuizItem {
            q: q.q.into(),
            options: q.options.iter().map(|o| o.to_string()).collect(),
            answer: q.answer,
            why: q.why.into(),
            key: Some(QuestionKey::new(lesson, q.id)),
        }
    }
}

impl From<telosgard_core::practice::PracticeQ> for QuizItem {
    fn from(q: telosgard_core::practice::PracticeQ) -> QuizItem {
        QuizItem { q: q.q, options: q.options, answer: q.answer, why: q.why, key: None }
    }
}

type Order = Vec<(usize, Vec<usize>)>;

fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i as u64 + 1) as usize);
    }
}

fn new_order(qs: &[QuizItem]) -> Order {
    let mut rng = Rng::new(now() as u64);
    let mut order: Order = qs.iter().enumerate().map(|(i, q)| (i, (0..q.options.len()).collect())).collect();
    shuffle(&mut order, &mut rng);
    for (_, opts) in order.iter_mut() {
        shuffle(opts, &mut rng);
    }
    order
}

#[component]
pub fn QuizCard(
    questions: Vec<QuizItem>,
    #[prop(into)] on_done: Callback<u8>,
    /// A practice round: no pass mark, and it says so.
    #[prop(default = false)]
    practice: bool,
    /// Heading shown instead of "Practice" / nothing.
    #[prop(optional)]
    label: Option<&'static str>,
) -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    let n = questions.len();
    let questions = StoredValue::new(questions);
    let order = RwSignal::new(questions.with_value(|qs| new_order(qs)));
    let step = RwSignal::new(0usize);
    // `picked` is a provisional choice awaiting Sure/Guessing; `chosen` is the revealed answer.
    let picked = RwSignal::new(None::<usize>);
    let chosen = RwSignal::new(None::<usize>);
    let correct = RwSignal::new(0u32);
    let finished = RwSignal::new(false);

    let restart = move |_| {
        order.set(questions.with_value(|qs| new_order(qs)));
        step.set(0);
        picked.set(None);
        chosen.set(None);
        correct.set(0);
        finished.set(false);
    };
    let advance = move |_| {
        if step.get_untracked() + 1 >= n {
            finished.set(true);
            store.flush_qrecs();
            on_done.run(quiz_score(correct.get_untracked(), n as u32));
        } else {
            step.update(|s| *s += 1);
            picked.set(None);
            chosen.set(None);
        }
    };

    let question = move || {
        let (qi, opts) = order.with(|o| o[step.get()].clone());
        let q = questions.with_value(|qs| qs[qi].clone());
        let answer = q.answer;
        let asks = q.key.is_some();
        let key = StoredValue::new(q.key.clone());
        let reveal = move |oi: usize, conf: Option<Confidence>| {
            chosen.set(Some(oi));
            if oi == answer {
                correct.update(|c| *c += 1);
            }
            if let (Some(k), Some(c)) = (key.get_value(), conf) {
                store.record_answer(&k, oi == answer, c);
            }
        };
        let options = opts
            .into_iter()
            .map(|oi| {
                let is_answer = oi == answer;
                let class = move || {
                    let base = "press flex w-full items-center gap-3 rounded-btn px-4 py-3 min-h-12 text-left text-body";
                    match chosen.get() {
                        None if picked.get() == Some(oi) => format!("{base} bg-raised ring-2 ring-text"),
                        None => format!("{base} bg-bg hover:bg-raised"),
                        Some(_) if is_answer => format!("{base} bg-bg ring-2 ring-up"),
                        Some(c) if c == oi => format!("{base} bg-bg ring-2 ring-down"),
                        Some(_) => format!("{base} bg-bg opacity-55"),
                    }
                };
                let mark = move || match chosen.get() {
                    Some(_) if is_answer => "✓",
                    Some(c) if c == oi => "✗",
                    _ => "",
                };
                view! {
                    <button
                        type="button"
                        class=class
                        disabled=move || chosen.get().is_some()
                        aria-pressed=move || (asks && chosen.get().is_none()).then(|| (picked.get() == Some(oi)).to_string())
                        on:click=move |_| if asks { picked.set(Some(oi)) } else { reveal(oi, None) }
                    >
                        <span class="flex-1">{q.options[oi].clone()}</span>
                        <span class=move || { if is_answer { "text-up" } else { "text-down" } } aria-hidden="true">{mark}</span>
                    </button>
                }
            })
            .collect_view();
        let why = q.why.clone();
        view! {
            <div class="flex items-center justify-between gap-3">
                <p class="text-eyebrow text-eyebrow-ink">
                    {match (label, practice) {
                        (Some(l), _) => format!("{l} · "),
                        (None, true) => t("Practice · ", "Entraînement · ").into(),
                        _ => String::new(),
                    }}
                    {ts(format!("Question {} of {n}", step.get() + 1), format!("Question {} sur {n}", step.get() + 1))}
                </p>
                <div class="flex gap-1.5" aria-hidden="true">
                    {(0..n)
                        .map(|i| {
                            let c = if i < step.get() { "bg-text-muted" } else if i == step.get() { "bg-text" } else { "bg-line/50" };
                            view! { <span class=format!("size-1.5 rounded-full {c}")></span> }
                        })
                        .collect_view()}
                </div>
            </div>
            <h3 class="mt-2 text-headline text-pretty">{q.q.clone()}</h3>
            <div class="mt-4 grid gap-2">{options}</div>
            <Show when=move || asks && chosen.get().is_none() && picked.get().is_some()>
                <div class="anim-fade mt-4">
                    <p class="text-callout text-text-muted">{t("How sure are you? Either way, a right answer counts.", "Quel est ton degré de certitude ? Dans les deux cas, une bonne réponse compte.")}</p>
                    <div class="mt-2 flex flex-wrap gap-2">
                        <button type="button" class=PILL on:click=move |_| if let Some(oi) = picked.get_untracked() { reveal(oi, Some(Confidence::Sure)) }>{t("I'm sure", "Aucun doute")}</button>
                        <button type="button" class=PILL_GHOST on:click=move |_| if let Some(oi) = picked.get_untracked() { reveal(oi, Some(Confidence::Guessing)) }>{t("I'm guessing", "Je devine")}</button>
                    </div>
                </div>
            </Show>
            <Show when=move || chosen.get().is_some()>
                <div class="anim-fade mt-4 rounded-btn bg-bg p-4" aria-live="polite">
                    {move || {
                        if chosen.get() == Some(answer) {
                            view! { <p class="text-headline text-up">{t("✓ Right", "✓ Juste")}</p> }.into_any()
                        } else {
                            view! { <p class="text-headline text-down">{t("✗ Not quite", "✗ Pas tout à fait")}</p> }.into_any()
                        }
                    }}
                    <p class="mt-1 text-callout">{why.clone()}</p>
                </div>
                <div class="mt-4 flex justify-end">
                    <button type="button" class=PILL on:click=advance>
                        {move || if step.get() + 1 >= n { t("See score", "Voir le score") } else { t("Next ›", "Suivant ›") }}
                    </button>
                </div>
            </Show>
        }
    };

    let result = move || {
        let score = quiz_score(correct.get(), n as u32);
        let passed = score >= QUIZ_PASS_PERCENT;
        view! {
            <p class="text-eyebrow text-eyebrow-ink">{if practice { t("Practice result", "Résultat de l’entraînement") } else { t("Quiz result", "Résultat du quiz") }}</p>
            <p class="tabular mt-2 text-large"><crate::ui::moments::CountUp value=score as i64 fmt=|v| ts(format!("{v}%"), format!("{v} %")) /></p>
            <p class="text-callout text-text-muted">{ts(format!("{} of {n} right", correct.get()), format!("{} sur {n} justes", correct.get()))}</p>
            {if practice {
                view! { <p class="mt-3 text-callout">{t("Practice only: it doesn't change your Gate.", "Pour s’entraîner seulement : ça ne change pas ta validation.")}</p> }.into_any()
            } else if passed {
                view! { <p class="mt-3 text-headline text-gold">{t("✓ Quiz passed", "✓ Quiz réussi")}</p> }.into_any()
            } else {
                view! {
                    <p class="mt-3 text-callout">
                        {ts(format!("You need {QUIZ_PASS_PERCENT}%. Have another go: the questions reshuffle, and there's no penalty."), format!("Il te faut {QUIZ_PASS_PERCENT} %. Réessaie : les questions sont remélangées, et il n’y a aucune pénalité."))}
                    </p>
                }
                    .into_any()
            }}
            <div class="mt-4">
                <button type="button" class=PILL_GHOST on:click=restart>{if practice { t("Retry these", "Refaire ces questions") } else { t("Retake quiz", "Refaire le quiz") }}</button>
            </div>
        }
    };

    view! {
        <section class="rounded-card bg-surface p-5 shadow-card">
            {move || if finished.get() { result().into_any() } else { question().into_any() }}
        </section>
    }
}
