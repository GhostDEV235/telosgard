//! Weekly Review (FR-11.4): this week's numbers from the journal (Expectancy in R, rules broken,
//! Emotion Tags against broken plans, post-trade lessons), plus a note.

use crate::journal_data::{JournalData, today, utc_offset_min};
use crate::screens::journal::pnl_text;
use crate::screens::shell::SubBar;
use crate::screens::trade::rule_name;
use crate::state::Toasts;
use crate::ui::moments::CountUp;
use crate::ui::controls::{PILL, Segmented};
use telosgard_core::clock::week_start;
use telosgard_core::journal::local_day;
use telosgard_core::review::{ReviewEntry, weekly_review};
use telosgard_core::time::date_string;
use leptos::prelude::*;

#[component]
pub fn Review() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let this_week = week_start(today());
    let weeks_back = RwSignal::new(0i64);
    let week = move || this_week - 7 * weeks_back.get();
    let note = RwSignal::new(String::new());
    Effect::new(move |_| {
        let w = week();
        note.set(journal.reviews.with(|m| m.get(&w).cloned()).unwrap_or_default());
    });

    let stats = move || {
        let w = week();
        let offset = utc_offset_min();
        let (entries, lessons): (Vec<ReviewEntry>, Vec<String>) = journal.trades.with(|v| {
            let week: Vec<_> = v.iter().filter(|(_, t)| (w..w + 7).contains(&local_day(t.entry.at, offset))).collect();
            let entries = week
                .iter()
                .map(|(_, t)| ReviewEntry {
                    skipped: !t.taken(),
                    pnl: t.result.as_ref().map(|r| r.pnl),
                    risk: t.risk(),
                    failed_rules: t.hard_results.iter().filter(|h| !h.pass).map(|h| h.rule).collect(),
                    adherent: t.adherence.final_adherent(),
                    emotion: t.result.as_ref().and_then(|r| r.emotion),
                })
                .collect();
            let lessons = week.iter().filter_map(|(_, t)| t.result.as_ref()?.lesson.clone()).collect();
            (entries, lessons)
        });
        (weekly_review(&entries), lessons)
    };

    let tile = |label: &'static str, value: AnyView, tone: &'static str| {
        view! {
            <div class="rounded-btn bg-bg p-3">
                <p class="text-footnote text-text-muted">{label}</p>
                <p class=format!("tabular mt-0.5 text-title2 {tone}")>{value}</p>
            </div>
        }
    };
    let count = |v: i64| view! { <CountUp value=v fmt=|v| v.to_string() /> }.into_any();

    view! {
        <SubBar back="#journal" back_label="Journal" title="Weekly Review" />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">"Weekly Review"</h1>
            <p class="mt-2 text-callout text-text-muted">"Once a week: what did you do, and did you follow your plan? Your journal is where your edge (and your leaks) show up."</p>
            <div class="mt-5">
                <Segmented label="Week" value=weeks_back options=vec![(0, "This week"), (1, "Last week"), (2, "2 weeks ago")] />
            </div>
            <p class="mt-3 text-footnote text-text-muted">{move || format!("Week starting Monday {}", date_string(week()))}</p>
            {move || {
                let (r, lessons) = stats();
                let (net, tone) = pnl_text(r.net);
                let (exp, exp_tone) = match r.expectancy {
                    Some(x) => (
                        view! { <CountUp value=x.0 fmt=|v| telosgard_core::journal::RMultiple(v).to_string() /> }.into_any(),
                        if x.0 > 0 { "text-up" } else if x.0 < 0 { "text-down" } else { "" },
                    ),
                    None => ("—".into_any(), ""),
                };
                let dash = || "—".into_any();
                view! {
                    <section class="mt-4 grid grid-cols-2 gap-3 rounded-card bg-surface p-5 shadow-card sm:grid-cols-3">
                        {tile("Trades taken", count(r.taken as i64), "")}
                        {tile("Skipped setups", count(r.skipped as i64), "")}
                        {tile("Broke the plan", count(r.not_adherent as i64), if r.not_adherent > 0 { "text-down" } else { "" })}
                        {tile("Win rate", if r.closed == 0 { dash() } else { view! { <CountUp value=r.win_rate.0 as i64 fmt=|v| telosgard_core::money::Percent(v as u32).to_string() /> }.into_any() }, "")}
                        {tile("Expectancy (per trade)", exp, exp_tone)}
                        {tile("Net (closed)", if r.closed == 0 { dash() } else { net.into_any() }, if r.closed == 0 { "" } else { tone })}
                    </section>
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <h2 class="text-headline">"Rules broken"</h2>
                        {if r.rule_breaks.is_empty() {
                            view! { <p class="mt-2 text-callout text-text-muted">"No Hard Rule broken this week."</p> }.into_any()
                        } else {
                            view! {
                                <ul class="mt-2 grid gap-1 text-callout">
                                    {r.rule_breaks.into_iter().map(|(rule, n)| view! { <li class="flex justify-between"><span>{rule_name(rule)}</span><span class="tabular text-down">{format!("× {n}")}</span></li> }).collect_view()}
                                </ul>
                            }.into_any()
                        }}
                    </section>
                    <p class="mt-3 text-footnote text-text-muted">"Expectancy is your average result per trade in R. Over many trades, above 0R means the way you trade makes money; one week is too few to judge."</p>
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <h2 class="text-headline">"How you felt"</h2>
                        {if r.emotions.is_empty() {
                            view! { <p class="mt-2 text-callout text-text-muted">"No Emotion Tags this week. You set one when you record a trade's result."</p> }.into_any()
                        } else {
                            view! {
                                <ul class="mt-2 grid gap-1 text-callout">
                                    {r.emotions.into_iter().map(|c| view! {
                                        <li class="flex justify-between gap-3">
                                            <span>{c.emotion.label()}</span>
                                            <span class="tabular">
                                                {format!("{} trade{}", c.trades, if c.trades == 1 { "" } else { "s" })}
                                                <span class={if c.broke_plan > 0 { "text-down" } else { "text-text-muted" }}>{format!(" · {} broke the plan", c.broke_plan)}</span>
                                            </span>
                                        </li>
                                    }).collect_view()}
                                </ul>
                            }.into_any()
                        }}
                    </section>
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <h2 class="text-headline">"Your post-trade lessons"</h2>
                        {if lessons.is_empty() {
                            view! { <p class="mt-2 text-callout text-text-muted">"None written this week."</p> }.into_any()
                        } else {
                            view! { <ul class="mt-2 grid list-disc gap-1 pl-5 text-callout">{lessons.into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}</ul> }.into_any()
                        }}
                    </section>
                }
            }}
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">"Your note"</h2>
                <p class="mt-1 text-callout text-text-muted">"One thing to keep doing, one thing to stop."</p>
                <textarea rows="4" aria-label="Your weekly review note" class="mt-3 w-full rounded-field bg-bg p-3.5 text-callout" prop:value=move || note.get() on:input=move |ev| note.set(event_target_value(&ev))></textarea>
                <button type="button" class=format!("{PILL} mt-3") on:click=move |_| {
                    journal.save_review(week(), note.get_untracked());
                    toasts.show("Review note saved");
                }>"Save note"</button>
            </section>
        </main>
    }
}
