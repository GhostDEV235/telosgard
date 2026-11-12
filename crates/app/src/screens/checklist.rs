//! Pre-Session Checklist (FR-11.1): today's routine, saved per local day. Trading without one breaks the plan.

use crate::i18n::{t, ts};
use crate::journal_data::{JournalData, local_hm, local_time_to_utc, today};
use crate::screens::shell::SubBar;
use crate::state::Toasts;
use crate::ui::clock::SessionClock;
use crate::ui::controls::{NumberField, PILL, PILL_GHOST, Segmented};
use telosgard_core::checklist::{Bias, CalendarEvent, ChecklistError, PreSessionChecklist, RiskCommitment, Tier, Zone};
use telosgard_core::money::{DEFAULT_CONTRACT_OZ, DollarMove, Percent, Price, Usd};
use telosgard_core::sizing::Sizing;
use leptos::prelude::*;

const INPUT: &str = "min-h-11 min-w-0 rounded-field bg-bg px-3 text-callout text-text placeholder:text-text-muted";

fn checklist_error(e: ChecklistError) -> &'static str {
    match e {
        ChecklistError::NoScenario => t("Write at least one Scenario: “If price does X at level Y, I do Z.”", "Écris au moins un Scénario : « Si le prix fait X au niveau Y, je fais Z. »"),
        ChecklistError::NoZone => t("Mark at least one key zone.", "Note au moins une zone clé."),
        ChecklistError::ZoneInverted => t("A zone's low must be below its high.", "Le bas d’une zone doit être sous son haut."),
        ChecklistError::AsianRangeInverted => t("The Asian low must be below the Asian high.", "Le plus bas asiatique doit être sous le plus haut asiatique."),
        ChecklistError::RiskMissing => t("Fill in your risk per trade and daily loss limit.", "Remplis ton risque par trade et ta perte maximale du jour."),
    }
}

fn tier_label(tier: Tier) -> &'static str {
    match tier {
        Tier::One => t("Tier 1", "Niveau 1"),
        Tier::Two => t("Tier 2", "Niveau 2"),
        Tier::Three => t("Tier 3", "Niveau 3"),
    }
}

/// A price or amount without the dollar sign, as shown and as typed back ("4,100.00" / "4 100,00").
fn price_label(p: Price) -> String {
    p.to_string().trim_start_matches('$').trim_end_matches('$').trim_end_matches('\u{a0}').to_string()
}

/// An amount for an input field: no sign, no thousands separator ("10000.00" / "10000,00").
fn plain_amount(cents: i64) -> String {
    let s = format!("{}.{:02}", cents / 100, cents.rem_euclid(100));
    if crate::i18n::is_fr() { s.replace('.', ",") } else { s }
}

/// Planned stops are a list; French decimals use commas, so the list separator is ";" there.
fn stop_separator() -> char {
    if crate::i18n::is_fr() { ';' } else { ',' }
}

#[component]
pub fn Checklist() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let day = today();

    let events = RwSignal::new(Vec::<CalendarEvent>::new());
    let ev_time = RwSignal::new(String::new());
    let ev_name = RwSignal::new(String::new());
    let ev_tier = RwSignal::new(Tier::One);
    let asian_hi = RwSignal::new(String::new());
    let asian_lo = RwSignal::new(String::new());
    let mood = RwSignal::new(String::new());
    let bias = RwSignal::new(Bias::Up);
    let zones = RwSignal::new(Vec::<Zone>::new());
    let z_lo = RwSignal::new(String::new());
    let z_hi = RwSignal::new(String::new());
    let scenarios = RwSignal::new(Vec::<String>::new());
    let sc_text = RwSignal::new(String::new());
    let per_trade = RwSignal::new(String::new());
    let daily = RwSignal::new(String::new());
    let stops = RwSignal::new(t("5, 8", "5 ; 8").to_string());
    let balance = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let editing = RwSignal::new(false);

    // Prefill from the saved checklist (if any) and the plan, once data is loaded.
    Effect::new(move |done: Option<bool>| {
        if done == Some(true) || !journal.loaded.get() {
            return done.unwrap_or(false);
        }
        let plan = journal.current_plan();
        let pct = |p: Percent| p.to_string().trim_end_matches('%').trim_end_matches('\u{a0}').to_string();
        per_trade.set(plan.as_ref().map_or("1".into(), |p| pct(p.hard.max_risk)));
        daily.set(plan.as_ref().map_or("3".into(), |p| pct(p.hard.daily_loss_limit)));
        if let Some(b) = journal.latest_balance() {
            balance.set(plain_amount(b.0));
        }
        if let Some(c) = journal.checklist(day) {
            events.set(c.events.clone());
            asian_hi.set(c.asian_high.map(price_label).unwrap_or_default());
            asian_lo.set(c.asian_low.map(price_label).unwrap_or_default());
            mood.set(c.mood.clone());
            bias.set(c.bias);
            zones.set(c.zones.clone());
            scenarios.set(c.scenarios.clone());
            per_trade.set(pct(c.risk.per_trade));
            daily.set(pct(c.risk.daily_limit));
            stops.set(c.risk.planned_stops.iter().map(|d| price_label(Price(d.0))).collect::<Vec<_>>().join(t(", ", " ; ")));
        } else {
            editing.set(true);
        }
        true
    });

    let add_event = move |_| {
        let name = ev_name.get_untracked().trim().to_string();
        match local_time_to_utc(day, &ev_time.get_untracked()) {
            Some(at) if !name.is_empty() => {
                events.update(|v| {
                    v.push(CalendarEvent { name, tier: ev_tier.get_untracked(), at });
                    v.sort_by_key(|e| e.at);
                });
                ev_name.set(String::new());
                ev_time.set(String::new());
            }
            _ => error.set(Some(t("Give the event a name and a time like 13:30.", "Donne un nom à l’événement et une heure comme 13:30.").into())),
        }
    };
    let add_zone = move |_| match (Price::parse(&z_lo.get_untracked()), Price::parse(&z_hi.get_untracked())) {
        (Some(low), Some(high)) => {
            zones.update(|v| v.push(Zone { low, high }));
            z_lo.set(String::new());
            z_hi.set(String::new());
        }
        _ => error.set(Some(t("Enter a zone as two prices, like 4,090 and 4,095.", "Saisis une zone sous forme de deux prix, par exemple 4090 et 4095.").into())),
    };
    let add_scenario = move |_| {
        let text = sc_text.get_untracked().trim().to_string();
        if text.split_whitespace().count() >= 5 {
            scenarios.update(|v| v.push(text));
            sc_text.set(String::new());
        } else {
            error.set(Some(t("Write the Scenario as a full sentence: “If price does X at level Y, I do Z.”", "Écris le Scénario en phrase complète : « Si le prix fait X au niveau Y, je fais Z. »").into()));
        }
    };

    let planned = move || stops.get().split(stop_separator()).filter_map(DollarMove::parse).filter(|d| d.0 > 0).collect::<Vec<_>>();
    let build = move || PreSessionChecklist {
        day,
        events: events.get_untracked(),
        asian_high: Price::parse(&asian_hi.get_untracked()),
        asian_low: Price::parse(&asian_lo.get_untracked()),
        mood: mood.get_untracked().trim().to_string(),
        bias: bias.get_untracked(),
        zones: zones.get_untracked(),
        scenarios: scenarios.get_untracked(),
        risk: RiskCommitment {
            per_trade: Percent::parse(&per_trade.get_untracked()).unwrap_or_default(),
            daily_limit: Percent::parse(&daily.get_untracked()).unwrap_or_default(),
            planned_stops: planned(),
        },
    };
    let save = move |_| {
        let c = build();
        match c.validate() {
            Ok(()) => {
                journal.save_checklist(c);
                error.set(None);
                editing.set(false);
                toasts.show(t("Checklist saved for today", "Checklist enregistrée pour aujourd’hui"));
            }
            Err(e) => error.set(Some(checklist_error(e).into())),
        }
    };

    let sizes = move || {
        let bal = Usd::parse(&balance.get()).filter(|b| b.0 > 0)?;
        let risk = Percent::parse(&per_trade.get()).filter(|p| p.0 > 0)?;
        let c = PreSessionChecklist { risk: RiskCommitment { per_trade: risk, daily_limit: Percent(1), planned_stops: planned() }, ..build() };
        Some(c.planned_sizes(bal, DEFAULT_CONTRACT_OZ))
    };

    let remove_btn = |on: Box<dyn Fn() + Send + Sync>| {
        view! { <button type="button" aria-label=t("Remove", "Retirer") class="press grid size-9 shrink-0 place-items-center rounded-full text-text-muted" on:click=move |_| on()>"✕"</button> }
    };

    let form = move || {
        view! {
            <section class="grid gap-3 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">{t("1 · Today's high-impact USD news", "1 · Les grandes annonces en USD aujourd’hui")}</h2>
                <p class="text-callout text-text-muted">
                    {t("Check the official calendars, then add each event in your local time. Tier-1 events (CPI, jobs report, FOMC) create a 5-minute No-Trade Window either side.", "Consulte les calendriers officiels, puis ajoute chaque événement dans ton heure locale. Les événements de niveau 1 (CPI, emploi, FOMC) créent une Fenêtre sans trade de 5 minutes de chaque côté.")}
                </p>
                <p class="flex flex-wrap gap-x-4 gap-y-1 text-callout">
                    <a class="underline underline-offset-2" href="https://www.bls.gov/schedule/news_release/" target="_blank" rel="noopener noreferrer">{t("BLS release calendar (CPI, jobs) ↗", "Calendrier du BLS (CPI, emploi, en anglais) ↗")}</a>
                    <a class="underline underline-offset-2" href="https://www.federalreserve.gov/monetarypolicy/fomccalendars.htm" target="_blank" rel="noopener noreferrer">{t("Fed FOMC calendar ↗", "Calendrier du FOMC de la Fed (en anglais) ↗")}</a>
                </p>
                <ul class="grid gap-1">
                    {move || {
                        events
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, e)| {
                                view! {
                                    <li class="flex items-center gap-3 rounded-btn bg-bg px-3 py-1.5 text-callout">
                                        <span class="tabular w-12">{local_hm(e.at)}</span>
                                        <span class="min-w-0 flex-1">{e.name.clone()}</span>
                                        <span class="text-footnote text-text-muted">{tier_label(e.tier)}</span>
                                        {remove_btn(Box::new(move || events.update(|v| { v.remove(i); })))}
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <div class="flex flex-wrap gap-2">
                    <input class=format!("{INPUT} w-24") placeholder="13:30" aria-label=t("Event time", "Heure de l’événement") prop:value=move || ev_time.get() on:input=move |ev| ev_time.set(event_target_value(&ev)) />
                    <input class=format!("{INPUT} flex-1") placeholder=t("US CPI", "CPI américain") aria-label=t("Event name", "Nom de l’événement") prop:value=move || ev_name.get() on:input=move |ev| ev_name.set(event_target_value(&ev)) />
                    <select class=INPUT aria-label=t("Tier", "Niveau") on:change=move |ev| ev_tier.set(match event_target_value(&ev).as_str() { "2" => Tier::Two, "3" => Tier::Three, _ => Tier::One })>
                        <option value="1">{t("Tier 1", "Niveau 1")}</option>
                        <option value="2">{t("Tier 2", "Niveau 2")}</option>
                        <option value="3">{t("Tier 3", "Niveau 3")}</option>
                    </select>
                    <button type="button" class=PILL_GHOST on:click=add_event>{t("Add", "Ajouter")}</button>
                </div>
                <p class="text-footnote text-text-muted">{t("No big news today? Leave this empty.", "Pas de grosse annonce aujourd’hui ? Laisse vide.")}</p>
            </section>

            <section class="grid gap-3 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">{t("2 · Overnight and mood", "2 · La nuit et l’ambiance")}</h2>
                <div class="grid gap-3 sm:grid-cols-2">
                    <NumberField label=t("Asian session high", "Plus haut de la séance asiatique") value=asian_hi />
                    <NumberField label=t("Asian session low", "Plus bas de la séance asiatique") value=asian_lo />
                </div>
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">{t("Macro mood, one line each: dollar, yields, risk on/off", "Ambiance macro, en une ligne : dollar, taux, appétit pour le risque")}</span>
                    <input class=format!("{INPUT} mt-1.5 w-full") placeholder=t("Dollar weaker, yields flat, risk on", "Dollar plus faible, taux stables, appétit pour le risque") prop:value=move || mood.get() on:input=move |ev| mood.set(event_target_value(&ev)) />
                </label>
            </section>

            <section class="grid gap-3 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">{t("3 · Chart preparation", "3 · Préparation du graphique")}</h2>
                <Segmented label=t("Higher-timeframe bias", "Biais de l’unité de temps supérieure") value=bias options=vec![(Bias::Up, t("▲ Up", "▲ Hausse")), (Bias::Down, t("▼ Down", "▼ Baisse")), (Bias::Range, t("↔ Range", "↔ Range"))] />
                <p class="text-footnote font-semibold text-text-muted">{t("Key zones (3–5)", "Zones clés (3 à 5)")}</p>
                <ul class="grid gap-1">
                    {move || {
                        zones
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, z)| {
                                view! {
                                    <li class="flex items-center gap-3 rounded-btn bg-bg px-3 py-1.5 text-callout">
                                        <span class="tabular flex-1">{format!("{} – {}", price_label(z.low), price_label(z.high))}</span>
                                        {remove_btn(Box::new(move || zones.update(|v| { v.remove(i); })))}
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <div class="flex flex-wrap gap-2">
                    <input class=format!("{INPUT} w-32 tabular") inputmode="decimal" placeholder=t("Low 4,090", "Bas 4090") aria-label=t("Zone low", "Bas de la zone") prop:value=move || z_lo.get() on:input=move |ev| z_lo.set(event_target_value(&ev)) />
                    <input class=format!("{INPUT} w-32 tabular") inputmode="decimal" placeholder=t("High 4,095", "Haut 4095") aria-label=t("Zone high", "Haut de la zone") prop:value=move || z_hi.get() on:input=move |ev| z_hi.set(event_target_value(&ev)) />
                    <button type="button" class=PILL_GHOST on:click=add_zone>{t("Add zone", "Ajouter la zone")}</button>
                </div>
                <p class="mt-2 text-footnote font-semibold text-text-muted">{t("Scenarios: “If price does X at level Y, I do Z.”", "Scénarios : « Si le prix fait X au niveau Y, je fais Z. »")}</p>
                <ul class="grid gap-1">
                    {move || {
                        scenarios
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, s)| {
                                view! {
                                    <li class="flex items-start gap-3 rounded-btn bg-bg px-3 py-2 text-callout">
                                        <span class="tabular text-text-muted">{format!("{}.", i + 1)}</span>
                                        <span class="min-w-0 flex-1">{s}</span>
                                        {remove_btn(Box::new(move || scenarios.update(|v| { v.remove(i); })))}
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <textarea rows="2" class=format!("{INPUT} w-full py-2.5") placeholder=t("If price pulls back to 4,090–4,095 and an H1 bullish engulfing closes there, I buy with the stop below the wick.", "Si le prix revient vers 4090–4095 et qu’une englobante haussière H1 y clôture, j’achète avec le stop sous la mèche.") prop:value=move || sc_text.get() on:input=move |ev| sc_text.set(event_target_value(&ev))></textarea>
                <div><button type="button" class=PILL_GHOST on:click=add_scenario>{t("Add Scenario", "Ajouter le Scénario")}</button></div>
            </section>

            <section class="grid gap-3 rounded-card bg-surface p-5 shadow-card">
                <h2 class="text-headline">{t("4 · Risk for today", "4 · Le risque du jour")}</h2>
                <div class="grid gap-3 sm:grid-cols-2">
                    <NumberField label=t("Risk per trade", "Risque par trade") suffix="%" value=per_trade />
                    <NumberField label=t("Daily loss limit", "Perte maximale du jour") suffix="R" value=daily />
                    <NumberField label=t("Planned stop distances", "Distances de stop prévues (séparées par ;)") prefix=t("$", "") suffix=t("", "$") value=stops />
                    <NumberField label=t("Account balance", "Solde du compte") prefix=t("$", "") suffix=t("", "$") value=balance />
                </div>
                {move || {
                    sizes()
                        .map(|rows| {
                            view! {
                                <ul class="grid gap-1 text-callout">
                                    {rows
                                        .into_iter()
                                        .map(|(stop, s)| {
                                            let text = match s {
                                                Some(Sizing::Ok { lots, risk, .. }) => ts(format!("{stop} stop → {lots} lots (risking {risk})"), format!("Stop de {stop} → {lots} lot (risque de {risk})")),
                                                Some(Sizing::TooSmall { .. }) => ts(format!("{stop} stop → too small to trade safely"), format!("Stop de {stop} → trop petit pour trader sans danger")),
                                                None => ts(format!("{stop} stop → check the numbers"), format!("Stop de {stop} → vérifie les nombres")),
                                            };
                                            view! { <li class="tabular">"• " {text}</li> }
                                        })
                                        .collect_view()}
                                </ul>
                            }
                        })
                }}
            </section>

            {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">"⚠ " {e}</p> })}
            <div class="flex justify-end">
                <button type="button" class=PILL on:click=save>{t("Save today's checklist", "Enregistrer la checklist du jour")}</button>
            </div>
        }
    };

    let summary = move || {
        journal.checklist(day).map(|c| {
            view! {
                <section class="rounded-card bg-surface p-5 shadow-card">
                    <p class="text-headline text-up">{t("✓ Today's checklist is saved", "✓ La checklist du jour est enregistrée")}</p>
                    <dl class="mt-3 grid gap-2 text-callout">
                        <div><dt class="text-footnote text-text-muted">{t("News", "Annonces")}</dt><dd>{if c.events.is_empty() { t("None", "Aucune").to_string() } else { c.events.iter().map(|e| format!("{} {} ({})", local_hm(e.at), e.name, tier_label(e.tier))).collect::<Vec<_>>().join(" · ") }}</dd></div>
                        <div><dt class="text-footnote text-text-muted">{t("Bias · mood", "Biais · ambiance")}</dt><dd>{format!("{} · {}", match c.bias { Bias::Up => t("▲ Up", "▲ Hausse"), Bias::Down => t("▼ Down", "▼ Baisse"), Bias::Range => "↔ Range" }, if c.mood.is_empty() { "—" } else { &c.mood })}</dd></div>
                        <div><dt class="text-footnote text-text-muted">{t("Zones", "Zones")}</dt><dd class="tabular">{c.zones.iter().map(|z| format!("{}–{}", price_label(z.low), price_label(z.high))).collect::<Vec<_>>().join(" · ")}</dd></div>
                        <div><dt class="text-footnote text-text-muted">{t("Scenarios", "Scénarios")}</dt><dd><ol class="list-decimal pl-5">{c.scenarios.iter().map(|s| view! { <li>{s.clone()}</li> }).collect_view()}</ol></dd></div>
                    </dl>
                    <button type="button" class=format!("{PILL_GHOST} mt-4") on:click=move |_| editing.set(true)>{t("Edit", "Modifier")}</button>
                </section>
            }
        })
    };

    view! {
        <SubBar back="#journal" back_label="Journal" title=t("Pre-Session Checklist", "Checklist avant la séance") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">{t("Pre-Session Checklist", "Checklist avant la séance")}</h1>
            <p class="mt-2 text-callout text-text-muted">{t("20–30 minutes before you look for any trade. Any trade on a day without one breaks your plan.", "20 à 30 minutes avant de chercher le moindre trade. Tout trade un jour sans checklist enfreint ton plan.")}</p>
            <div class="mt-5"><SessionClock /></div>
            <div class="mt-5 grid gap-5">
                {move || if editing.get() { form().into_any() } else { summary().into_any() }}
            </div>
        </main>
    }
}
