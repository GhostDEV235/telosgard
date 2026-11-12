//! Journal tab (FR-6, FR-11) and the Live Readiness dashboard (FR-7), on real data.

use crate::i18n::{t, ts};
use crate::journal_data::{JournalData, local_label, today};
use crate::screens::shell::{LargeTitle, SubBar};
use crate::state::{Progress, Toasts};
use crate::store::TradeRecord;
use crate::ui::chart::Ring;
use crate::ui::moments::CountUp;
use crate::ui::clock::SessionClock;
use crate::ui::controls::{NumberField, PILL, PILL_GHOST, PillLink};
use telosgard_core::journal::{Adherence, Direction};
use telosgard_core::money::{Percent, Usd};
use telosgard_core::readiness::{REQUIRED_ADHERENCE, REQUIRED_DAYS, REQUIRED_TRADES};
use leptos::prelude::*;

pub fn verdict_chip(tr: &TradeRecord) -> (&'static str, &'static str) {
    if !tr.taken() {
        return (t("Skipped", "Écartée"), "bg-raised text-text-muted");
    }
    match tr.adherence {
        Adherence::Pending => (t("Awaiting verdict", "Verdict en attente"), "bg-warn-bg text-warn-ink"),
        Adherence::DisputePending { .. } => (t("Dispute pending", "Contestation en cours"), "bg-warn-bg text-warn-ink"),
        _ => match tr.adherence.final_adherent() {
            Some(true) => (t("✓ Followed plan", "✓ Plan respecté"), "bg-ok-bg text-ok-ink"),
            _ => (t("✗ Broke plan", "✗ Plan enfreint"), "bg-bad-bg text-bad-ink"),
        },
    }
}

pub fn pnl_text(p: Usd) -> (String, &'static str) {
    if p.0 >= 0 { (format!("▲ +{p}"), "text-up") } else { (format!("▼ {p}"), "text-down") }
}

#[component]
fn ReadinessCard() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    move || {
        if journal.current_plan().is_none() {
            return view! {
                <a href="#plan" class="lift block rounded-card bg-surface p-5 shadow-card">
                    <p class="text-eyebrow text-eyebrow-ink">{t("Live Readiness", "Prêt pour le réel")}</p>
                    <p class="mt-1 text-headline">{t("Write your Trading Plan first ›", "Écris d’abord ton Plan de trading ›")}</p>
                    <p class="mt-1 text-callout text-text-muted">{t("Every Demo Trade is judged against it.", "Chaque Trade démo est jugé par rapport à lui.")}</p>
                </a>
            }
            .into_any();
        }
        let (counted, pct) = journal.readiness_progress().map_or((0, 0), |(_, p)| (p.counted, p.counted * 100 / REQUIRED_TRADES));
        view! {
            <a href="#readiness" class="lift block rounded-card bg-surface p-5 shadow-card">
                <div class="flex items-center justify-between gap-3">
                    <div>
                        <p class="text-eyebrow text-eyebrow-ink">{t("Live Readiness", "Prêt pour le réel")}</p>
                        <p class="tabular mt-1 text-headline">{ts(format!("{counted} of {REQUIRED_TRADES} trades counted"), format!("{counted} trades comptés sur {REQUIRED_TRADES}"))}</p>
                    </div>
                    <span class="text-title2 text-text-muted" aria-hidden="true">"›"</span>
                </div>
                <div class="mt-4 h-1.5 overflow-hidden rounded-full bg-bg">
                    <div class="h-full rounded-full bg-text-muted" style=format!("width:{}%", pct.min(100))></div>
                </div>
            </a>
        }
        .into_any()
    }
}

#[component]
pub fn Journal() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let day = today();
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title=t("Your journal,", "Ton journal,") accent=t("kept honest.", "tenu honnêtement.") />
            <div class="mt-6 grid items-start gap-5 md:grid-cols-2">
                <div class="grid gap-5">
                    <ReadinessCard />
                    <section class="rounded-card bg-surface p-5 shadow-card">
                        <p class="text-eyebrow text-eyebrow-ink">{t("Today", "Aujourd’hui")}</p>
                        {move || {
                            if journal.checklist(day).is_some() {
                                view! { <p class="mt-1 text-headline text-up">{t("✓ Pre-Session Checklist saved", "✓ Checklist avant la séance enregistrée")}</p> }.into_any()
                            } else {
                                view! { <p class="mt-1 text-headline">{t("Pre-Session Checklist not done yet", "Checklist avant la séance pas encore faite")}</p> }.into_any()
                            }
                        }}
                        <div class="mt-4 flex flex-wrap gap-3">
                            <PillLink href="#checklist">{move || if journal.checklist(day).is_some() { t("Open checklist", "Ouvrir la checklist") } else { t("Start checklist", "Commencer la checklist") }}</PillLink>
                            <a href="#review" class=PILL_GHOST>{t("Weekly review", "Bilan de la semaine")}</a>
                        </div>
                    </section>
                </div>
                <SessionClock />
            </div>

            <div class="mt-8 flex flex-wrap items-center justify-between gap-3">
                <h2 class="text-title2">{t("Entries", "Entrées")}</h2>
                <div class="flex flex-wrap gap-3">
                    <a href="#trade-skip" class=PILL_GHOST>{t("Log a skipped setup", "Noter une configuration écartée")}</a>
                    <PillLink href="#trade-new">{t("Log a trade", "Noter un trade")}</PillLink>
                </div>
            </div>
            {move || {
                let trades = journal.trades.get();
                if trades.is_empty() {
                    return view! {
                        <section class="mt-4 rounded-card bg-surface p-8 text-center shadow-card">
                            <p class="text-title2">{t("No entries yet", "Aucune entrée pour l’instant")}</p>
                            <p class="mx-auto mt-2 max-w-md text-callout text-text-muted">
                                {t("You'll journal your first Demo Trade in Lesson 40. Skipped setups are worth logging too: they teach discipline.", "Tu noteras ton premier Trade démo à la leçon 40. Les configurations écartées valent aussi la peine d’être notées : elles apprennent la discipline.")}
                            </p>
                        </section>
                    }
                        .into_any();
                }
                view! {
                    <ul class="mt-4 grid gap-2">
                        {trades
                            .into_iter()
                            .map(|(id, tr)| {
                                let (chip, chip_class) = verdict_chip(&tr);
                                let dir = match tr.entry.direction { Direction::Buy => (t("▲ Buy", "▲ Achat"), "text-up"), Direction::Sell => (t("▼ Sell", "▼ Vente"), "text-down") };
                                let pnl = tr.result.as_ref().map(|r| pnl_text(r.pnl));
                                let r_txt = tr.r().map(|r| r.to_string());
                                view! {
                                    <li>
                                        <a href=format!("#{id}") class="press flex flex-wrap items-center gap-x-4 gap-y-1 rounded-btn bg-surface px-4 py-3 shadow-card hover:bg-raised">
                                            <span class="tabular w-36 text-callout text-text-muted">{local_label(tr.entry.at)}</span>
                                            <span class=format!("w-16 text-callout font-semibold {}", dir.1)>{dir.0}</span>
                                            <span class="tabular text-callout">{tr.entry.entry.to_string()}</span>
                                            {pnl.map(|(s, c)| view! { <span class=format!("tabular text-callout font-semibold {c}")>{s}</span> })}
                                            {r_txt.map(|r| view! { <span class="tabular text-callout text-text-muted">{r}</span> })}
                                            <span class=format!("ml-auto rounded-full px-2.5 py-1 text-footnote font-semibold {chip_class}")>{chip}</span>
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                }
                    .into_any()
            }}
        </main>
    }
}

#[component]
pub fn Readiness() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let progress = expect_context::<Progress>();
    let toasts = expect_context::<Toasts>();
    let fresh = RwSignal::new(String::new());

    // Reaching readiness is permanent: write it once.
    Effect::new(move |_| {
        if journal.readiness_progress().is_some_and(|(_, p)| p.reached) && journal.mark_ready(progress) {
            expect_context::<crate::ui::moments::Moments>().readiness();
        }
    });

    let conditions = [
        ts(format!("{REQUIRED_TRADES} journaled Demo Trades with a final verdict"), format!("{REQUIRED_TRADES} Trades démo notés au journal avec un verdict définitif")),
        ts(format!("At least {REQUIRED_DAYS} days (8 weeks) from your first counted trade to your latest"), format!("Au moins {REQUIRED_DAYS} jours (8 semaines) entre ton premier trade compté et le dernier")),
        ts(format!("At least {REQUIRED_ADHERENCE} of them followed your plan"), format!("Au moins {REQUIRED_ADHERENCE} d’entre eux ont respecté ton plan")),
        t("No breach of your plan's max total loss. A breach ends the run; the next starts from zero with a fresh demo balance.", "Aucun dépassement de la perte totale maximale de ton plan. Un dépassement met fin à la série ; la suivante repart de zéro avec un nouveau solde démo.").to_string(),
    ];

    let new_run = move |_| match Usd::parse(&fresh.get_untracked()).filter(|u| u.0 > 0) {
        Some(b) => {
            journal.end_active_run("breach");
            let r = journal.start_run(b);
            toasts.show(ts(format!("Readiness Run {} started", r.run.number), format!("Série de préparation {} commencée", r.run.number)));
            fresh.set(String::new());
        }
        None => toasts.show(t("Enter your fresh demo balance first", "Saisis d’abord ton nouveau solde démo")),
    };

    let body = move || {
        if journal.current_plan().is_none() {
            return view! {
                <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
                    <p class="text-headline">{t("No Trading Plan yet", "Pas encore de Plan de trading")}</p>
                    <p class="mt-1 text-callout text-text-muted">{t("Readiness is measured against your plan's rules, so it starts once you've written one.", "La préparation se mesure par rapport aux règles de ton plan : elle commence une fois ton plan écrit.")}</p>
                    <div class="mt-4"><PillLink href="#plan">{t("Write your plan", "Écrire ton plan")}</PillLink></div>
                </section>
            }
            .into_any();
        }
        let Some((run, p)) = journal.readiness_progress() else {
            return view! {
                <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
                    <p class="text-headline">{t("Readiness Run 1 starts with your first journaled trade", "La série de préparation 1 commence avec ton premier trade noté au journal")}</p>
                    <p class="mt-1 text-callout text-text-muted">{t("Its starting balance is the demo balance you enter on that trade.", "Son solde de départ est le solde démo que tu saisis sur ce trade.")}</p>
                </section>
            }
            .into_any();
        };
        let pct_adh = p.adherence.0 as f64 / 10_000.0;
        let drawdown_pct = if p.max_loss.0 > 0 { (p.drawdown.0 * 100 / p.max_loss.0).clamp(0, 100) } else { 0 };
        view! {
            {p.reached.then(|| view! {
                <div class="anim-fade mt-6 flex items-center gap-3 rounded-card bg-gold-fill p-5 text-on-gold" role="status">
                    <span class="text-title1" aria-hidden="true">"🏅"</span>
                    <div>
                        <p class="text-headline">{t("Live Readiness reached", "« Prêt pour le réel » atteint")}</p>
                        <p class="text-callout">{t("The Live Track and the Advanced Track are now open, for good.", "Le Parcours réel et le Parcours avancé sont maintenant ouverts, pour de bon.")}</p>
                    </div>
                </div>
            })}
            {p.breached.then(|| view! {
                <section class="mt-6 rounded-card border border-down bg-surface p-5 shadow-card" role="alert">
                    <p class="text-headline text-down">{ts(format!("▼ Run {} breached your max total loss", run.run.number), format!("▼ La série {} a dépassé ta perte totale maximale", run.run.number))}</p>
                    <p class="mt-1 text-callout">{t("This run is over. Reset your demo account to a fresh balance, then start the next run. Your lessons stay unlocked.", "Cette série est terminée. Remets ton compte démo à un nouveau solde, puis commence la série suivante. Tes leçons restent débloquées.")}</p>
                    <div class="mt-4 flex flex-wrap items-end gap-3">
                        <div class="w-56"><NumberField label=t("Fresh demo balance", "Nouveau solde démo") prefix=t("$", "") suffix=t("", "$") value=fresh /></div>
                        <button type="button" class=PILL on:click=new_run>{ts(format!("Start Run {}", run.run.number + 1), format!("Commencer la série {}", run.run.number + 1))}</button>
                    </div>
                </section>
            })}
            <p class="mt-6 text-eyebrow text-eyebrow-ink">{ts(format!("Readiness Run {} · started with {}", run.run.number, run.run.starting_balance), format!("Série de préparation {} · départ avec {}", run.run.number, run.run.starting_balance))}</p>
            <section class="mt-2 grid grid-cols-3 gap-2 rounded-card bg-surface p-5 shadow-card">
                <Ring value=p.counted as f64 / REQUIRED_TRADES as f64 label=t("Trades", "Trades")><CountUp value=p.counted as i64 fmt=|v| format!("{v}/{REQUIRED_TRADES}") /></Ring>
                <Ring value=pct_adh / (REQUIRED_ADHERENCE.0 as f64 / 10_000.0) label=t("Followed plan", "Plan respecté")>{if p.counted == 0 { "—".into_any() } else { view! { <CountUp value=p.adherence.0 as i64 fmt=|v| telosgard_core::money::Percent(v as u32).to_string() /> }.into_any() }}</Ring>
                <Ring value=p.days as f64 / REQUIRED_DAYS as f64 label=t("Days", "Jours")><CountUp value=p.days fmt=|v| format!("{v}/{REQUIRED_DAYS}") /></Ring>
            </section>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow text-eyebrow-ink">{t("Max total loss", "Perte totale maximale")}</p>
                <p class="tabular mt-1 text-headline">{ts(format!("{} drawdown of {} allowed", p.drawdown, p.max_loss), format!("{} de drawdown sur {} autorisés", p.drawdown, p.max_loss))}</p>
                <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-bg">
                    <div class="h-full rounded-full bg-down" style=format!("width:{drawdown_pct}%")></div>
                </div>
                <p class="mt-2 text-footnote text-text-muted">{ts(format!("{} of the run's starting balance. Measured from closed trades.", journal.current_plan().map(|p| p.hard.max_total_loss).unwrap_or(Percent(0))), format!("{} du solde de départ de la série. Mesuré sur les trades clôturés.", journal.current_plan().map(|p| p.hard.max_total_loss).unwrap_or(Percent(0))))}</p>
            </section>
        }
        .into_any()
    };

    view! {
        <SubBar back="#journal" back_label="Journal" title=t("Live Readiness", "Prêt pour le réel") />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <h1 class="text-title1">{t("Live Readiness", "Prêt pour le réel")}</h1>
            <p class="mt-2 text-callout text-text-muted">
                {t("Discipline, not profit, unlocks the Live and Advanced Tracks. Losing trades that follow your plan still count.", "C’est la discipline, pas le gain, qui débloque les Parcours réel et avancé. Les trades perdants qui respectent ton plan comptent quand même.")}
            </p>
            {body}
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">{t("All four, at the same time", "Les quatre, en même temps")}</h2>
                <ol class="mt-3 grid gap-3">
                    {conditions
                        .into_iter()
                        .enumerate()
                        .map(|(i, c)| {
                            view! {
                                <li class="flex gap-3 text-callout">
                                    <span class="tabular grid size-6 shrink-0 place-items-center rounded-full bg-bg text-footnote font-semibold">{i + 1}</span>
                                    <span>{c}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </section>
        </main>
    }
}
