//! One question at a time; questions and options reshuffle on every attempt (FR-2.4).

use gold_content::QuizQ;
use crate::state::now;
use crate::ui::controls::{PILL, PILL_GHOST};
use gold_core::drills::Rng;
use gold_core::gate::{QUIZ_PASS_PERCENT, quiz_score};
use leptos::prelude::*;

type Order = Vec<(usize, Vec<usize>)>;

fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i as u64 + 1) as usize);
    }
}

fn new_order(qs: &[QuizQ]) -> Order {
    let mut rng = Rng::new(now() as u64);
    let mut order: Order = qs.iter().enumerate().map(|(i, q)| (i, (0..q.options.len()).collect())).collect();
    shuffle(&mut order, &mut rng);
    for (_, opts) in order.iter_mut() {
        shuffle(opts, &mut rng);
    }
    order
}

#[component]
pub fn QuizCard(questions: &'static [QuizQ], #[prop(into)] on_done: Callback<u8>) -> impl IntoView {
    let n = questions.len();
    let order = RwSignal::new(new_order(questions));
    let step = RwSignal::new(0usize);
    let chosen = RwSignal::new(None::<usize>);
    let correct = RwSignal::new(0u32);
    let finished = RwSignal::new(false);

    let restart = move |_| {
        order.set(new_order(questions));
        step.set(0);
        chosen.set(None);
        correct.set(0);
        finished.set(false);
    };
    let advance = move |_| {
        if step.get_untracked() + 1 >= n {
            finished.set(true);
            on_done.run(quiz_score(correct.get_untracked(), n as u32));
        } else {
            step.update(|s| *s += 1);
            chosen.set(None);
        }
    };

    let question = move || {
        let (qi, opts) = order.with(|o| o[step.get()].clone());
        let q = &questions[qi];
        let options = opts
            .into_iter()
            .map(|oi| {
                let is_answer = oi == q.answer;
                let class = move || {
                    let base = "press flex w-full items-center gap-3 rounded-btn px-4 py-3 min-h-12 text-left text-body";
                    match chosen.get() {
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
                        on:click=move |_| {
                            chosen.set(Some(oi));
                            if is_answer {
                                correct.update(|c| *c += 1);
                            }
                        }
                    >
                        <span class="flex-1">{q.options[oi]}</span>
                        <span class=move || { if is_answer { "text-up" } else { "text-down" } } aria-hidden="true">{mark}</span>
                    </button>
                }
            })
            .collect_view();
        view! {
            <div class="flex items-center justify-between gap-3">
                <p class="text-eyebrow uppercase text-text-muted">{format!("Question {} of {n}", step.get() + 1)}</p>
                <div class="flex gap-1.5" aria-hidden="true">
                    {(0..n)
                        .map(|i| {
                            let c = if i < step.get() { "bg-text-muted" } else if i == step.get() { "bg-text" } else { "bg-line/50" };
                            view! { <span class=format!("size-1.5 rounded-full {c}")></span> }
                        })
                        .collect_view()}
                </div>
            </div>
            <h3 class="mt-2 text-headline text-pretty">{q.q}</h3>
            <div class="mt-4 grid gap-2">{options}</div>
            <Show when=move || chosen.get().is_some()>
                <div class="anim-fade mt-4 rounded-btn bg-bg p-4" aria-live="polite">
                    {move || {
                        if chosen.get() == Some(q.answer) {
                            view! { <p class="text-headline text-up">"✓ Right"</p> }.into_any()
                        } else {
                            view! { <p class="text-headline text-down">"✗ Not quite"</p> }.into_any()
                        }
                    }}
                    <p class="mt-1 text-callout">{q.why}</p>
                </div>
                <div class="mt-4 flex justify-end">
                    <button type="button" class=PILL on:click=advance>
                        {move || if step.get() + 1 >= n { "See score" } else { "Next ›" }}
                    </button>
                </div>
            </Show>
        }
    };

    let result = move || {
        let score = quiz_score(correct.get(), n as u32);
        let passed = score >= QUIZ_PASS_PERCENT;
        view! {
            <p class="text-eyebrow uppercase text-text-muted">"Quiz result"</p>
            <p class="tabular mt-2 text-large">{format!("{score}%")}</p>
            <p class="text-callout text-text-muted">{format!("{} of {n} right", correct.get())}</p>
            {if passed {
                view! { <p class="mt-3 text-headline text-gold">"✓ Quiz passed"</p> }.into_any()
            } else {
                view! {
                    <p class="mt-3 text-callout">
                        "You need " {QUIZ_PASS_PERCENT} "%. Have another go: the questions reshuffle, and there's no penalty."
                    </p>
                }
                    .into_any()
            }}
            <div class="mt-4">
                <button type="button" class=PILL_GHOST on:click=restart>"Retake quiz"</button>
            </div>
        }
    };

    view! {
        <section class="rounded-card bg-surface p-5 shadow-card">
            {move || if finished.get() { result().into_any() } else { question().into_any() }}
        </section>
    }
}
