//! StageCard, LessonRow, GateStatus, VideoCard, Callout.

use crate::catalog::{self, Section, VideoRef};
use crate::router::lesson_href;
use crate::state::Progress;
use crate::ui::controls::{PILL, PillLink};
use gold_core::course::LessonId;
use leptos::prelude::*;

fn required(section: &'static Section) -> impl Iterator<Item = LessonId> {
    section.lessons.iter().map(|l| l.0).filter(|id| *id != LessonId::Indicators)
}

#[component]
pub fn StageCard(section: &'static Section) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let open = RwSignal::new(false);
    let total = required(section).count();
    let done = move || required(section).filter(|id| progress.passed(*id)).count();
    let locked = move || required(section).next().is_some_and(|first| !progress.unlocked(first));
    let complete = move || done() == total;
    let next = move || required(section).find(|id| progress.unlocked(*id) && !progress.passed(*id));
    // Muted text is below 4.5:1 on the locked fill, so locked cards use full-strength text.
    let soft = move || if locked() { "text-text" } else { "text-text-muted" };

    view! {
        <article class=move || {
            format!("overflow-hidden rounded-card {}", if locked() { "bg-locked" } else { "bg-surface shadow-card" })
        }>
            <div class="p-5 pb-4">
                <div class="flex items-center justify-between gap-3">
                    <p class=move || format!("text-eyebrow uppercase {}", soft())>{section.eyebrow}</p>
                    {move || {
                        if complete() {
                            view! { <span class="rounded-full bg-gold-fill px-2.5 py-1 text-eyebrow text-on-gold uppercase">"✓ Complete"</span> }
                                .into_any()
                        } else if locked() {
                            view! { <span class="text-eyebrow uppercase">"🔒 Locked"</span> }.into_any()
                        } else {
                            ().into_any()
                        }
                    }}
                </div>
                <h2 class="mt-1.5 text-title1">{section.title}</h2>
                <p class=move || format!("mt-1.5 text-callout {}", soft())>{section.blurb}</p>
                {(!section.pace.is_empty())
                    .then(|| {
                        view! {
                            <p class=move || format!("mt-2 text-footnote font-semibold {}", soft())>
                                <span aria-hidden="true">"◷ "</span>
                                {section.pace}
                            </p>
                        }
                    })}
                {move || {
                    if locked() {
                        view! { <p class="mt-4 text-callout font-semibold">{section.locked_hint}</p> }.into_any()
                    } else {
                        let pct = done() * 100 / total.max(1);
                        view! {
                            <div class="mt-4">
                                <div
                                    class="h-1.5 overflow-hidden rounded-full bg-bg"
                                    role="progressbar"
                                    aria-label=format!("{} progress", section.title)
                                    aria-valuemin="0"
                                    aria-valuemax=total
                                    aria-valuenow=done()
                                >
                                    <div
                                        class={ if complete() { "h-full rounded-full bg-gold-fill" } else { "h-full rounded-full bg-text-muted" } }
                                        style=format!("width:{pct}%")
                                    ></div>
                                </div>
                                <p class="tabular mt-2 text-footnote text-text-muted">{format!("{} of {total} lessons", done())}</p>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
            <div class="flex flex-wrap items-center justify-between gap-3 border-t border-line/30 px-5 py-3">
                <button
                    type="button"
                    class=move || format!("press min-h-11 text-callout {}", soft())
                    aria-expanded=move || open.get().to_string()
                    on:click=move |_| open.update(|o| *o = !*o)
                >
                    {move || if open.get() { "Hide lessons ▴" } else { "Show lessons ▾" }}
                </button>
                {move || {
                    next()
                        .filter(|_| !locked())
                        .map(|id| {
                            let label = if progress.get(id) == Default::default() { "Start" } else { "Continue" };
                            view! { <PillLink href=lesson_href(id)>{label} " ›"</PillLink> }
                        })
                }}
            </div>
            <Show when=move || open.get()>
                <ul class="anim-fade px-2 pb-3">
                    {section.lessons.iter().map(|(id, title)| view! { <LessonRow id=*id title=*title /> }).collect_view()}
                </ul>
            </Show>
        </article>
    }
}

#[component]
pub fn LessonRow(id: LessonId, title: &'static str) -> impl IntoView {
    let progress = expect_context::<Progress>();
    move || {
        let unlocked = progress.unlocked(id);
        let passed = progress.passed(id);
        let (glyph, glyph_class, sr) = if passed {
            ("✓", "text-gold", "Passed")
        } else if unlocked {
            ("●", "text-text", "Up next")
        } else {
            ("🔒", "", "Locked")
        };
        let optional = (id == LessonId::Indicators).then(|| {
            view! { <span class="ml-2 rounded-full border border-line px-2 py-0.5 text-footnote text-text-muted">"Optional"</span> }
        });
        let inner = view! {
            <span class="tabular w-7 shrink-0 text-callout text-text-muted">{catalog::label(id)}</span>
            <span class="min-w-0 flex-1 text-body">{title} {optional}</span>
            <span class=format!("shrink-0 {glyph_class}") aria-hidden="true">{glyph}</span>
            <span class="sr-only">{sr}</span>
        };
        let row = "flex min-h-12 items-center gap-3 rounded-btn px-3 py-2";
        if unlocked {
            view! { <li><a href=lesson_href(id) class=format!("{row} press hover:bg-bg/60")>{inner}</a></li> }.into_any()
        } else {
            view! { <li><div class=format!("{row} opacity-70") aria-disabled="true">{inner}</div></li> }.into_any()
        }
    }
}

#[component]
pub fn GateStatus(id: LessonId) -> impl IntoView {
    let progress = expect_context::<Progress>();
    let pill = |met: bool| {
        if met {
            "rounded-full bg-gold-fill px-3 py-1.5 text-footnote font-semibold text-on-gold"
        } else {
            "rounded-full border border-line px-3 py-1.5 text-footnote font-semibold text-text-muted"
        }
    };
    let next = match id {
        LessonId::Numbered(n) if n < gold_core::course::LAST_LIVE => Some(LessonId::Numbered(n + 1)),
        LessonId::Advanced(n) if n < gold_core::course::LAST_ADVANCED => Some(LessonId::Advanced(n + 1)),
        _ => None,
    };
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card" aria-labelledby="gate-h">
            <h2 id="gate-h" class="text-headline">"To unlock the next lesson"</h2>
            <div class="mt-3 flex flex-wrap gap-2">
                <span class=move || pill(progress.get(id).quiz_met())>
                    {move || if progress.get(id).quiz_met() { "✓ " } else { "○ " }}
                    "Quiz ≥ 80%"
                </span>
                <span class=move || pill(progress.get(id).explain_back_met())>
                    {move || if progress.get(id).explain_back_met() { "✓ " } else { "○ " }}
                    "Explain-Back: Got it"
                </span>
            </div>
            <div class="mt-4">
                {move || match next {
                    Some(n) if progress.passed(id) => view! { <PillLink href=lesson_href(n)>"Next lesson ›"</PillLink> }.into_any(),
                    Some(_) => view! { <button type="button" disabled class=PILL>"Next lesson ›"</button> }.into_any(),
                    None => ().into_any(),
                }}
            </div>
        </section>
    }
}

#[component]
pub fn VideoCard(video: &'static VideoRef) -> impl IntoView {
    view! {
        <a
            href=video.url
            target="_blank"
            rel="noopener noreferrer"
            class="press flex items-start gap-4 rounded-card bg-surface p-4 shadow-card hover:bg-raised"
        >
            <span class="grid size-14 shrink-0 place-items-center rounded-btn bg-fill text-title2 text-on-fill" aria-hidden="true">"▶"</span>
            <span class="min-w-0 flex-1">
                <span class="block text-headline">{video.title}</span>
                <span class="block text-footnote text-text-muted">{video.channel} " · opens YouTube"</span>
                <span class="mt-1.5 block text-callout"><span class="font-semibold">"Watch for: "</span>{video.watch_for}</span>
            </span>
            <span class="text-text-muted" aria-hidden="true">"↗"</span>
        </a>
    }
}

#[derive(Clone, Copy)]
pub enum CalloutKind {
    Analogy,
    Example,
    Mistake,
}

#[component]
pub fn Callout(kind: CalloutKind, text: &'static str) -> impl IntoView {
    let label = match kind {
        CalloutKind::Analogy => "💡 Analogy",
        CalloutKind::Example => "🌍 Real life",
        CalloutKind::Mistake => "⚠️ Common mistake",
    };
    view! {
        <aside class="rounded-card border-l-4 border-fill bg-surface p-5 shadow-card">
            <p class="text-eyebrow uppercase text-text-muted">{label}</p>
            <p class="mt-2 text-callout">{text}</p>
        </aside>
    }
}
