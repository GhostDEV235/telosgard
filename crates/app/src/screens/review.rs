//! Weekly Review (FR-11.4): this week's numbers from the journal (Expectancy in R, rules broken,
//! Emotion Tags against broken plans, post-trade lessons), plus a note.

use crate::i18n::{t, ts};
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
        <SubBar back="#journal" back_label="Journal" title=t("Weekly Review", "Bilan de la semaine") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">{t("Weekly Review", "Bilan de la semaine")}</h1>
            <p class="mt-2 text-callout text-text-muted">{t("Once a week: what did you do, and did you follow your plan? Your journal is where your edge (and your leaks) show up.", "Une fois par semaine : qu’as-tu fait, et as-tu suivi ton plan ? C’est dans ton journal qu’apparaissent tes points forts (et tes fuites).")}</p>
            <div class="mt-5">
                <Segmented label=t("Week", "Semaine") value=weeks_back options=vec![(0, t("This week", "Cette semaine")), (1, t("Last week", "Semaine dernière")), (2, t("2 weeks ago", "Il y a 2 semaines"))] />
            </div>
            <p class="mt-3 text-footnote text-text-muted">{move || ts(format!("Week starting Monday {}", date_string(week())), format!("Semaine commençant le lundi {}", date_string(week())))}</p>
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
                        {tile(t("Trades taken", "Trades pris"), count(r.taken as i64), "")}
                        {tile(t("Skipped setups", "Configurations écartées"), count(r.skipped as i64), "")}
                        {tile(t("Broke the plan", "Plan enfreint"), count(r.not_adherent as i64), if r.not_adherent > 0 { "text-down" } else { "" })}
                        {tile(t("Win rate", "Taux de réussite"), if r.closed == 0 { dash() } else { view! { <CountUp value=r.win_rate.0 as i64 fmt=|v| telosgard_core::money::Percent(v as u32).to_string() /> }.into_any() }, "")}
                        {tile(t("Expectancy (per trade)", "Espérance (par trade)"), exp, exp_tone)}
                        {tile(t("Net (closed)", "Net (clôturés)"), if r.closed == 0 { dash() } else { net.into_any() }, if r.closed == 0 { "" } else { tone })}
                    </section>
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <h2 class="text-headline">{t("Rules broken", "Règles enfreintes")}</h2>
                        {if r.rule_breaks.is_empty() {
                            view! { <p class="mt-2 text-callout text-text-muted">{t("No Hard Rule broken this week.", "Aucune Règle stricte enfreinte cette semaine.")}</p> }.into_any()
                        } else {
                            view! {
                                <ul class="mt-2 grid gap-1 text-callout">
                                    {r.rule_breaks.into_iter().map(|(rule, n)| view! { <li class="flex justify-between"><span>{rule_name(rule)}</span><span class="tabular text-down">{format!("× {n}")}</span></li> }).collect_view()}
                                </ul>
                            }.into_any()
                        }}
                    </section>
                    <p class="mt-3 text-footnote text-text-muted">{t("Expectancy is your average result per trade in R. Over many trades, above 0R means the way you trade makes money; one week is too few to judge.", "L’espérance, c’est ton résultat moyen par trade en R. Sur beaucoup de trades, au-dessus de 0R veut dire que ta façon de trader gagne de l’argent ; une semaine, c’est trop peu pour juger.")}</p>
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <h2 class="text-headline">{t("How you felt", "Ce que tu as ressenti")}</h2>
                        {if r.emotions.is_empty() {
                            view! { <p class="mt-2 text-callout text-text-muted">{t("No Emotion Tags this week. You set one when you record a trade's result.", "Aucune émotion notée cette semaine. Tu en notes une quand tu enregistres le résultat d’un trade.")}</p> }.into_any()
                        } else {
                            view! {
                                <ul class="mt-2 grid gap-1 text-callout">
                                    {r.emotions.into_iter().map(|c| view! {
                                        <li class="flex justify-between gap-3">
                                            <span>{c.emotion.label()}</span>
                                            <span class="tabular">
                                                {format!("{} trade{}", c.trades, if c.trades == 1 { "" } else { "s" })}
                                                <span class={if c.broke_plan > 0 { "text-down" } else { "text-text-muted" }}>{ts(format!(" · {} broke the plan", c.broke_plan), format!(" · {} ont enfreint le plan", c.broke_plan))}</span>
                                            </span>
                                        </li>
                                    }).collect_view()}
                                </ul>
                            }.into_any()
                        }}
                    </section>
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <h2 class="text-headline">{t("Your post-trade lessons", "Tes leçons après trade")}</h2>
                        {if lessons.is_empty() {
                            view! { <p class="mt-2 text-callout text-text-muted">{t("None written this week.", "Aucune écrite cette semaine.")}</p> }.into_any()
                        } else {
                            view! { <ul class="mt-2 grid list-disc gap-1 pl-5 text-callout">{lessons.into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}</ul> }.into_any()
                        }}
                    </section>
                }
            }}
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">{t("Your note", "Ta note")}</h2>
                <p class="mt-1 text-callout text-text-muted">{t("One thing to keep doing, one thing to stop.", "Une chose à continuer, une chose à arrêter.")}</p>
                <textarea rows="4" aria-label=t("Your weekly review note", "Ta note du bilan de la semaine") class="mt-3 w-full rounded-field bg-bg p-3.5 text-callout" prop:value=move || note.get() on:input=move |ev| note.set(event_target_value(&ev))></textarea>
                <button type="button" class=format!("{PILL} mt-3") on:click=move |_| {
                    journal.save_review(week(), note.get_untracked());
                    toasts.show(t("Review note saved", "Note du bilan enregistrée"));
                }>{t("Save note", "Enregistrer la note")}</button>
            </section>
        </main>
    }
}
