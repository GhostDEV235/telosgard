//! Weekly Review (FR-11.4): this week's numbers from the journal, plus a note.

use crate::journal_data::{JournalData, today, utc_offset_min};
use crate::screens::journal::pnl_text;
use crate::screens::shell::SubBar;
use crate::screens::trade::rule_name;
use crate::state::Toasts;
use crate::ui::controls::{PILL, Segmented};
use gold_core::clock::week_start;
use gold_core::journal::local_day;
use gold_core::review::{ReviewEntry, weekly_review};
use gold_core::time::date_string;
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
        let entries: Vec<ReviewEntry> = journal.trades.with(|v| {
            v.iter()
                .filter(|(_, t)| (w..w + 7).contains(&local_day(t.entry.at, offset)))
                .map(|(_, t)| ReviewEntry {
                    skipped: !t.taken(),
                    pnl: t.result.as_ref().map(|r| r.pnl),
                    risk: t.risk(),
                    failed_rules: t.hard_results.iter().filter(|h| !h.pass).map(|h| h.rule).collect(),
                    adherent: t.adherence.final_adherent(),
                })
                .collect()
        });
        weekly_review(&entries)
    };

    let tile = |label: &'static str, value: String, tone: &'static str| {
        view! {
            <div class="rounded-btn bg-bg p-3">
                <p class="text-footnote text-text-muted">{label}</p>
                <p class=format!("tabular mt-0.5 text-title2 {tone}")>{value}</p>
            </div>
        }
    };

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
                let r = stats();
                let (net, tone) = pnl_text(r.net);
                let avg = format!("{}{:.2}R", if r.avg_r >= 0 { "+" } else { "−" }, r.avg_r.abs() as f64 / 100.0);
                view! {
                    <section class="mt-4 grid grid-cols-2 gap-3 rounded-card bg-surface p-5 shadow-card sm:grid-cols-3">
                        {tile("Trades taken", r.taken.to_string(), "")}
                        {tile("Skipped setups", r.skipped.to_string(), "")}
                        {tile("Broke the plan", r.not_adherent.to_string(), if r.not_adherent > 0 { "text-down" } else { "" })}
                        {tile("Win rate", if r.closed == 0 { "—".into() } else { r.win_rate.to_string() }, "")}
                        {tile("Average R", if r.closed == 0 { "—".into() } else { avg }, "")}
                        {tile("Net (closed)", if r.closed == 0 { "—".into() } else { net }, if r.closed == 0 { "" } else { tone })}
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
                }
            }}
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">"Your note"</h2>
                <p class="mt-1 text-callout text-text-muted">"One thing to keep doing, one thing to stop."</p>
                <textarea rows="4" class="mt-3 w-full rounded-field bg-bg p-3.5 text-callout" prop:value=move || note.get() on:input=move |ev| note.set(event_target_value(&ev))></textarea>
                <button type="button" class=format!("{PILL} mt-3") on:click=move |_| {
                    journal.save_review(week(), note.get_untracked());
                    toasts.show("Review note saved");
                }>"Save note"</button>
            </section>
        </main>
    }
}
