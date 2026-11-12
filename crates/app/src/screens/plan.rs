//! Trading Plan builder (FR-5): Hard Rules + Soft Rules, versioned. Saving always creates a new version.

use crate::i18n::{t, ts};
use crate::journal_data::{JournalData, local_label};
use crate::screens::shell::SubBar;
use crate::state::Toasts;
use crate::ui::controls::{NumberField, PILL, PILL_GHOST};
use telosgard_core::money::Percent;
use telosgard_core::plan::{HardRules, PlanError, Session, SoftRules, TradingPlan};
use leptos::prelude::*;

const CLASSIC_SETUP: &str = "Classic Price Action: trade only in the direction of the H4 trend (read from market structure), at a key support or resistance zone marked in my checklist, after a confirmation candle (pin bar or engulfing) closes at that zone.";
const CLASSIC_SETUP_FR: &str = "Price Action classique : ne trader que dans le sens de la tendance H4 (lue dans la structure du marché), sur une zone clé de support ou de résistance notée dans ma checklist, après la clôture d’une bougie de confirmation (pin bar ou englobante) sur cette zone.";

fn plan_error(e: PlanError) -> &'static str {
    match e {
        PlanError::MaxRiskOutOfRange => t("Max risk per trade must be between 0.01% and 100%.", "Le risque maximal par trade doit être entre 0,01 % et 100 %."),
        PlanError::MaxTotalLossOutOfRange => t("Max total loss must be between 0.01% and 100%.", "La perte totale maximale doit être entre 0,01 % et 100 %."),
        PlanError::MinRewardRiskNotPositive => t("Minimum reward-to-risk must be above 0.", "Le rapport gain/risque minimal doit être supérieur à 0."),
        PlanError::NoSessions => t("Choose at least one session.", "Choisis au moins une séance."),
        PlanError::NoTradesAllowed => t("Allow at least one trade per day.", "Autorise au moins un trade par jour."),
        PlanError::DailyLossLimitNotPositive => t("The daily loss limit must be above 0R.", "La perte maximale du jour doit être supérieure à 0R."),
        PlanError::SetupMissing => t("Describe your setup.", "Décris ta configuration."),
    }
}

fn ratio(p: Percent) -> String {
    p.to_string().trim_end_matches('%').trim_end_matches('\u{a0}').to_string()
}

pub fn session_name(s: Session) -> &'static str {
    match s {
        Session::Asian => t("Asian", "Asie"),
        Session::London => t("London", "Londres"),
        Session::NewYork => "New York",
    }
}

#[component]
fn PlanSummary(plan: TradingPlan) -> impl IntoView {
    let h = &plan.hard;
    let rows = vec![
        (t("Max risk per trade", "Risque maximal par trade"), h.max_risk.to_string()),
        (t("Max total loss", "Perte totale maximale"), ts(format!("{} of the run's starting balance", h.max_total_loss), format!("{} du solde de départ de la série", h.max_total_loss))),
        (t("Minimum reward-to-risk", "Rapport gain/risque minimal"), format!("{}R", ratio(h.min_reward_risk))),
        (t("Sessions", "Séances"), h.sessions.iter().map(|s| session_name(*s)).collect::<Vec<_>>().join(", ")),
        (t("Max trades per day", "Trades maximum par jour"), h.max_trades_per_day.to_string()),
        (t("Daily loss limit", "Perte maximale du jour"), format!("{}R", ratio(h.daily_loss_limit))),
        (t("Stop-loss on every trade", "Stop-loss sur chaque trade"), t("Always", "Toujours").into()),
        (t("No-Trade Windows", "Fenêtres sans trade"), t("Always: rollover, Sunday open, ±5 min of tier-1 news", "Toujours : rollover, ouverture du dimanche, ±5 min autour des grandes statistiques").into()),
        (t("Pre-Session Checklist", "Checklist avant la séance"), t("Always: required on any day you trade", "Toujours : obligatoire chaque jour où tu trades").into()),
    ];
    let extra = plan.soft.extra.clone();
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card">
            <p class="text-eyebrow text-eyebrow-ink">{ts(format!("Version {} · saved {}", plan.version, local_label(plan.created_at)), format!("Version {} · enregistrée le {}", plan.version, local_label(plan.created_at)))}</p>
            <h2 class="mt-3 text-headline">{t("Hard Rules · checked by calculation", "Règles strictes · vérifiées par calcul")}</h2>
            <dl class="mt-2 divide-y divide-line/30">
                {rows
                    .into_iter()
                    .map(|(k, v)| {
                        view! {
                            <div class="flex flex-wrap justify-between gap-x-4 gap-y-1 py-2.5">
                                <dt class="text-callout text-text-muted">{k}</dt>
                                <dd class="tabular text-callout font-semibold">{v}</dd>
                            </div>
                        }
                    })
                    .collect_view()}
            </dl>
            <h2 class="mt-5 text-headline">{t("Soft Rules · checked by Professor Aurel", "Règles souples · vérifiées par le Professeur Aurel")}</h2>
            <p class="mt-2 text-callout">{plan.soft.setup.clone()}</p>
            <ul class="mt-2 grid gap-1 text-callout">
                {extra.into_iter().map(|r| view! { <li>"• " {r}</li> }).collect_view()}
                <li>{t("• The trade is one of that day's written Scenarios.", "• Le trade fait partie des Scénarios écrits ce jour-là.")}</li>
            </ul>
        </section>
    }
}

#[component]
pub fn Plan() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let editing = RwSignal::new(false);

    let max_risk = RwSignal::new("1".to_string());
    let max_loss = RwSignal::new("10".to_string());
    let min_rr = RwSignal::new("1.5".to_string());
    let per_day = RwSignal::new("2".to_string());
    let daily = RwSignal::new("3".to_string());
    let sessions = RwSignal::new(vec![Session::London, Session::NewYork]);
    let setup = RwSignal::new(t(CLASSIC_SETUP, CLASSIC_SETUP_FR).to_string());
    let extra = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);

    let start_editing = move || {
        if let Some(p) = journal.current_plan() {
            max_risk.set(ratio(p.hard.max_risk));
            max_loss.set(ratio(p.hard.max_total_loss));
            min_rr.set(ratio(p.hard.min_reward_risk));
            per_day.set(p.hard.max_trades_per_day.to_string());
            daily.set(ratio(p.hard.daily_loss_limit));
            sessions.set(p.hard.sessions.clone());
            setup.set(p.soft.setup.clone());
            extra.set(p.soft.extra.join("\n"));
        }
        error.set(None);
        editing.set(true);
    };

    let save = move |_| {
        let pct = |s: RwSignal<String>| Percent::parse(&s.get_untracked());
        let (Some(r), Some(l), Some(rr), Some(d)) = (pct(max_risk), pct(max_loss), pct(min_rr), pct(daily)) else {
            error.set(Some(t("Check the numbers: use plain values like 1, 10, 1.5 or 3.", "Vérifie les nombres : utilise des valeurs simples comme 1, 10, 1,5 ou 3.").into()));
            return;
        };
        let Ok(n) = per_day.get_untracked().trim().parse::<u32>() else {
            error.set(Some(t("Max trades per day must be a whole number.", "Le nombre maximal de trades par jour doit être un nombre entier.").into()));
            return;
        };
        let hard = HardRules { max_risk: r, max_total_loss: l, min_reward_risk: rr, sessions: sessions.get_untracked(), max_trades_per_day: n, daily_loss_limit: d };
        let soft = SoftRules {
            setup: setup.get_untracked().trim().to_string(),
            extra: extra.get_untracked().lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect(),
        };
        match journal.save_plan(hard, soft) {
            Ok(p) => {
                toasts.show(ts(format!("Trading Plan v{} saved", p.version), format!("Plan de trading v{} enregistré", p.version)));
                editing.set(false);
            }
            Err(e) => error.set(Some(plan_error(e).into())),
        }
    };

    let session_toggle = move |s: Session| {
        view! {
            <button
                type="button"
                role="checkbox"
                aria-checked=move || sessions.with(|v| v.contains(&s)).to_string()
                on:click=move |_| sessions.update(|v| if let Some(i) = v.iter().position(|x| *x == s) { v.remove(i); } else { v.push(s); })
                class=move || {
                    if sessions.with(|v| v.contains(&s)) {
                        "press min-h-11 rounded-full bg-fill px-4 text-callout font-semibold text-on-fill"
                    } else {
                        "press min-h-11 rounded-full border border-line px-4 text-callout text-text-muted"
                    }
                }
            >
                {session_name(s)}
            </button>
        }
    };

    let form = move || {
        view! {
            <section class="grid gap-4 rounded-card bg-surface p-5 shadow-card">
                <p class="text-eyebrow text-eyebrow-ink">{t("Hard Rules", "Règles strictes")}</p>
                <div class="grid gap-4 sm:grid-cols-2">
                    <NumberField label=t("Max risk per trade", "Risque maximal par trade") suffix="%" value=max_risk />
                    <NumberField label=t("Max total loss (of the run's start)", "Perte totale maximale (du départ de la série)") suffix="%" value=max_loss />
                    <NumberField label=t("Minimum reward-to-risk", "Rapport gain/risque minimal") suffix="R" value=min_rr />
                    <NumberField label=t("Daily loss limit", "Perte maximale du jour") suffix="R" value=daily />
                    <NumberField label=t("Max trades per day", "Trades maximum par jour") value=per_day />
                </div>
                <div>
                    <p class="text-footnote font-semibold text-text-muted">{t("Sessions you trade", "Les séances où tu trades")}</p>
                    <div class="mt-2 flex flex-wrap gap-2">
                        {session_toggle(Session::Asian)}
                        {session_toggle(Session::London)}
                        {session_toggle(Session::NewYork)}
                    </div>
                </div>
                <p class="text-footnote text-text-muted">{t("Always on, not editable: a stop-loss on every trade, the No-Trade Windows, and a Pre-Session Checklist on any day you trade.", "Toujours actives, non modifiables : un stop-loss sur chaque trade, les Fenêtres sans trade, et une Checklist avant la séance chaque jour où tu trades.")}</p>
                <p class="mt-2 text-eyebrow text-eyebrow-ink">{t("Soft Rules", "Règles souples")}</p>
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">{t("Your setup", "Ta configuration")}</span>
                    <textarea rows="4" class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout" prop:value=move || setup.get() on:input=move |ev| setup.set(event_target_value(&ev))></textarea>
                </label>
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">{t("Any other rules (one per line)", "Autres règles (une par ligne)")}</span>
                    <textarea rows="3" placeholder=t("e.g. No trading in the 30 minutes before US CPI", "ex. : Pas de trading dans les 30 minutes avant le CPI américain") class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout placeholder:text-text-muted" prop:value=move || extra.get() on:input=move |ev| extra.set(event_target_value(&ev))></textarea>
                </label>
                {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">"⚠ " {e}</p> })}
                <div class="flex flex-wrap justify-end gap-3">
                    <Show when=move || journal.current_plan().is_some()>
                        <button type="button" class=PILL_GHOST on:click=move |_| editing.set(false)>{t("Cancel", "Annuler")}</button>
                    </Show>
                    <button type="button" class=PILL on:click=save>
                        {move || journal.current_plan().map_or(t("Save plan v1", "Enregistrer le plan v1").to_string(), |p| ts(format!("Save as v{}", p.version + 1), format!("Enregistrer en v{}", p.version + 1)))}
                    </button>
                </div>
            </section>
        }
    };

    view! {
        <SubBar back="#size" back_label=t("Tools", "Outils") title=t("Trading Plan", "Plan de trading") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">{t("Trading Plan", "Plan de trading")}</h1>
            <p class="mt-2 text-callout text-text-muted">
                {t("Your written rules. You'll build it properly at Lesson 39; every Demo Trade is checked against the version in force when you took it.", "Tes règles écrites. Tu le construiras vraiment à la leçon 39 ; chaque Trade démo est vérifié selon la version en vigueur au moment où tu l’as pris.")}
            </p>
            <div class="mt-6 grid gap-5">
                {move || match (journal.current_plan(), editing.get()) {
                    (Some(p), false) => {
                        let next = p.version + 1;
                        view! {
                            <PlanSummary plan=p />
                            <div>
                                <button type="button" class=PILL_GHOST on:click=move |_| start_editing()>
                                    {ts(format!("Change the plan (creates v{next})"), format!("Modifier le plan (crée la v{next})"))}
                                </button>
                            </div>
                        }
                            .into_any()
                    }
                    _ => form().into_any(),
                }}
                {move || {
                    let older: Vec<TradingPlan> = journal.plans.with(|h| h.versions().iter().rev().skip(1).cloned().collect());
                    (!older.is_empty())
                        .then(|| {
                            view! {
                                <details class="rounded-card bg-surface p-5 shadow-card">
                                    <summary class="press min-h-11 cursor-pointer text-headline">{ts(format!("Earlier versions ({})", older.len()), format!("Versions précédentes ({})", older.len()))}</summary>
                                    <div class="mt-3 grid gap-4">{older.into_iter().map(|p| view! { <PlanSummary plan=p /> }).collect_view()}</div>
                                </details>
                            }
                        })
                }}
            </div>
        </main>
    }
}
