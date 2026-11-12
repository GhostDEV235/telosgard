//! Logging a Demo Trade or a skipped setup, and one trade's page: Hard Rules, Claude's Adherence
//! Verdict, one Dispute, and the result once closed (FR-6). Trade fields are locked once saved.

use crate::i18n::{t, ts};
use crate::grading;
use crate::journal_data::{JournalData, local_label, local_time_to_utc, utc_offset_min};
use crate::platform::{self, asset_url};
use crate::screens::journal::{pnl_text, verdict_chip};
use crate::screens::shell::SubBar;
use crate::state::{Toasts, now};
use crate::store::{TradeRecord, TradeResult, new_id};
use crate::ui::controls::{NumberField, PILL, PILL_GHOST, PillLink, Segmented};
use telosgard_core::adherence::{AdherenceInput, Dispute};
use telosgard_core::journal::{Adherence, Direction, Emotion, HardRule, HardRuleResult, TradeEntry, combine, evaluate_hard_rules, local_day, pnl};
use telosgard_core::money::{DEFAULT_CONTRACT_OZ, Lots, Price, Usd};
use telosgard_core::sizing::{Sizing, SizingInput, size};
use telosgard_core::time::{civil_from_days, days_from_civil};
use leptos::prelude::*;
use leptos::task::spawn_local;

pub fn rule_name(r: HardRule) -> &'static str {
    match r {
        HardRule::StopLoss => "Stop-loss",
        HardRule::RiskPerTrade => t("Risk per trade", "Risque par trade"),
        HardRule::MinRewardRisk => t("Reward-to-risk", "Rapport gain/risque"),
        HardRule::Session => t("Session", "Séance"),
        HardRule::TradesPerDay => t("Trades per day", "Trades par jour"),
        HardRule::DailyLossLimit => t("Daily loss limit", "Perte maximale du jour"),
        HardRule::NoTradeWindow => t("No-Trade Windows", "Fenêtres sans trade"),
        HardRule::ChecklistDone => t("Pre-Session Checklist", "Checklist avant la séance"),
    }
}

fn hard_list(results: Vec<HardRuleResult>) -> impl IntoView {
    view! {
        <ul class="grid gap-2">
            {results
                .into_iter()
                .map(|r| {
                    let (mark, tone) = if r.pass { ("✓", "text-up") } else { ("✗", "text-down") };
                    view! {
                        <li class="flex gap-3 text-callout">
                            <span class=format!("w-4 shrink-0 font-semibold {tone}")>{mark}</span>
                            <span><span class="font-semibold">{rule_name(r.rule)} {t(": ", " : ")}</span>{r.detail}</span>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

/// An amount for an input field: no sign, no thousands separator ("10000.00" / "10000,00").
fn plain_amount(cents: i64) -> String {
    let s = format!("{}.{:02}", cents / 100, cents.rem_euclid(100));
    if crate::i18n::is_fr() { s.replace('.', ",") } else { s }
}

/// "YYYY-MM-DDTHH:MM" (a datetime-local value) for a UTC instant, in local time.
fn to_input(ms: i64) -> String {
    let local = ms + utc_offset_min() as i64 * 60_000;
    let day = local.div_euclid(telosgard_core::DAY_MS);
    let (y, m, d) = civil_from_days(day);
    let mins = local.rem_euclid(telosgard_core::DAY_MS) / 60_000;
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}", mins / 60, mins % 60)
}

fn from_input(s: &str) -> Option<i64> {
    let (date, hm) = s.split_once('T')?;
    let mut parts = date.split('-').map(|p| p.parse::<i64>().ok());
    let (y, m, d) = (parts.next()??, parts.next()?? as u32, parts.next()?? as u32);
    local_time_to_utc(days_from_civil(y, m, d), hm)
}

/// Asks Claude for the verdict (or the dispute re-judgement) and stores the result. Never assumes a pass.
pub async fn request_verdict(journal: JournalData, id: String) -> Result<(), String> {
    let rec = journal.trade(&id).ok_or(t("Trade not found.", "Trade introuvable."))?;
    let plan = journal.plans.with_untracked(|h| h.get(rec.entry.plan_version).cloned()).ok_or(t("The plan version for this trade is missing.", "La version du plan de ce trade est introuvable."))?;
    let scenarios = journal.checklist(rec.checklist_day).map(|c| c.scenarios).unwrap_or_default();
    let disputing = matches!(rec.adherence, Adherence::DisputePending { .. });
    let first = rec.first_summary.clone().unwrap_or_default();
    let argument = rec.dispute_text.clone().unwrap_or_default();
    let label = local_label(rec.entry.at);
    let input = AdherenceInput {
        entry: &rec.entry,
        local_time: &label,
        scenario: rec.scenario.as_deref(),
        setup_rule: &plan.soft.setup,
        extra_rules: &plan.soft.extra,
        scenarios: &scenarios,
        hard: &rec.hard_results,
        has_screenshot: false,
        dispute: disputing.then(|| Dispute { first_summary: &first, argument: &argument }),
    };
    let res = grading::judge(&input).await?;
    let adherent = combine(&rec.hard_results, res.adherent);
    let mut updated = rec.clone();
    updated.adherence = if disputing { rec.adherence.resolve_dispute(adherent) } else { Adherence::Judged { adherent } };
    if !disputing {
        updated.first_summary = Some(res.summary.clone());
    }
    updated.verdict_summary = Some(res.summary);
    updated.soft_results = res.soft;
    journal.put_trade(id, updated);
    Ok(())
}

#[component]
pub fn TradeNew() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let when = RwSignal::new(to_input(now()));
    let direction = RwSignal::new(Direction::Buy);
    let entry = RwSignal::new(String::new());
    let stop = RwSignal::new(String::new());
    let tp = RwSignal::new(String::new());
    let lots = RwSignal::new(String::new());
    let balance = RwSignal::new(journal.latest_balance().map(|b| plain_amount(b.0)).unwrap_or_default());
    let scenario = RwSignal::new(String::new());
    let reason = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    let at = move || from_input(&when.get());
    let built = move || -> Option<TradeEntry> {
        let plan = journal.current_plan()?;
        Some(TradeEntry {
            at: at()?,
            direction: direction.get(),
            entry: Price::parse(&entry.get())?,
            stop: Price::parse(&stop.get()),
            take_profit: Price::parse(&tp.get()),
            lots: Lots((lots.get().trim().replace(',', ".").parse::<f64>().ok()? * 100.0).round() as i64).into_positive()?,
            balance_before: Usd::parse(&balance.get()).filter(|b| b.0 > 0)?,
            contract_oz: DEFAULT_CONTRACT_OZ,
            setup_reason: reason.get().trim().to_string(),
            plan_version: plan.version,
        })
    };
    let day_ctx = move || at().map(|t| journal.day_context_for(t, None));
    let preview = move || {
        let e = built()?;
        let plan = journal.current_plan()?;
        let (count, realized, checklist) = day_ctx()?;
        Some(evaluate_hard_rules(&e, &plan.hard, &JournalData::day_context(count, realized, checklist.as_ref())))
    };
    let suggestion = move || {
        let plan = journal.current_plan()?;
        let dist = (Price::parse(&entry.get())? - Price::parse(&stop.get())?).abs();
        let bal = Usd::parse(&balance.get())?;
        size(SizingInput { balance: bal, risk: plan.hard.max_risk, stop: dist, contract_oz: DEFAULT_CONTRACT_OZ }).ok()
    };
    let day_scenarios = move || day_ctx().and_then(|(_, _, c)| c).map(|c| c.scenarios).unwrap_or_default();
    let breached = move || journal.readiness_progress().is_some_and(|(_, p)| p.breached);

    let submit = move |_| {
        let Some(e) = built() else {
            error.set(Some(t("Fill in the time, entry price, lots and balance before (use numbers like 4,100.50 and 0.20).", "Remplis l’heure, le prix d’entrée, les lots et le solde avant (avec des nombres comme 4100,50 et 0,20).").into()));
            return;
        };
        if e.setup_reason.split_whitespace().count() < 5 {
            error.set(Some(t("Describe why you took the trade in at least a sentence: Professor Aurel judges your plan from it.", "Explique en au moins une phrase pourquoi tu as pris le trade : le Professeur Aurel juge ton plan à partir de ça.").into()));
            return;
        }
        if breached() {
            error.set(Some(t("This Readiness Run breached its max loss. Start a new run on the Live Readiness screen first.", "Cette Série de préparation a dépassé sa perte maximale. Commence d’abord une nouvelle série sur l’écran « Prêt pour le réel ».").into()));
            return;
        }
        let plan = journal.current_plan().expect("checked in built()");
        let (count, realized, checklist) = journal.day_context_for(e.at, None);
        let hard = evaluate_hard_rules(&e, &plan.hard, &JournalData::day_context(count, realized, checklist.as_ref()));
        let run = journal.active_run().unwrap_or_else(|| journal.start_run(e.balance_before));
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            let shot = match platform::upload_from_input("trade-shot").await {
                Ok(u) => u.map(|u| u.id),
                Err(err) => {
                    toasts.show(ts(format!("Screenshot not saved: {}", err.describe()), format!("Capture non enregistrée : {}", err.describe())));
                    None
                }
            };
            let id = new_id("trade");
            let sc = scenario.get_untracked();
            let rec = TradeRecord {
                status: "taken".into(),
                checklist_day: local_day(e.at, utc_offset_min()),
                entry: e,
                run: run.run.number,
                scenario: (!sc.is_empty()).then_some(sc),
                hard_results: hard,
                adherence: Adherence::Pending,
                soft_results: Vec::new(),
                verdict_summary: None,
                first_summary: None,
                dispute_text: None,
                screenshot: shot,
                result: None,
            };
            journal.put_trade(id.clone(), rec);
            let _ = window().location().set_hash(&id);
            if let Err(msg) = request_verdict(journal, id).await {
                toasts.show(msg);
            }
            busy.set(false);
        });
    };

    view! {
        <SubBar back="#journal" back_label="Journal" title=t("Log a trade", "Noter un trade") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">{t("Log a Demo Trade", "Noter un Trade démo")}</h1>
            {move || {
                if journal.current_plan().is_none() {
                    return view! {
                        <section class="mt-6 rounded-card bg-surface p-5 shadow-card">
                            <p class="text-headline">{t("Write your Trading Plan first", "Écris d’abord ton Plan de trading")}</p>
                            <p class="mt-1 text-callout text-text-muted">{t("Each trade is judged against the plan in force when you took it.", "Chaque trade est jugé selon le plan en vigueur au moment où tu l’as pris.")}</p>
                            <div class="mt-4"><PillLink href="#plan">{t("Write your plan", "Écrire ton plan")}</PillLink></div>
                        </section>
                    }
                        .into_any();
                }
                view! {
                    <section class="mt-6 grid gap-4 rounded-card bg-surface p-5 shadow-card">
                        <label class="block">
                            <span class="text-footnote font-semibold text-text-muted">{t("When you entered (your time)", "Heure d’entrée (ton heure)")}</span>
                            <input type="datetime-local" class="mt-1.5 min-h-12 w-full rounded-field bg-bg px-3.5 text-body" prop:value=move || when.get() on:input=move |ev| when.set(event_target_value(&ev)) />
                        </label>
                        <Segmented label=t("Direction", "Sens") value=direction options=vec![(Direction::Buy, t("▲ Buy (long)", "▲ Achat (long)")), (Direction::Sell, t("▼ Sell (short)", "▼ Vente (short)"))] />
                        <div class="grid gap-4 sm:grid-cols-3">
                            <NumberField label=t("Entry price", "Prix d’entrée") value=entry />
                            <NumberField label="Stop-loss" value=stop />
                            <NumberField label="Take-profit" value=tp />
                        </div>
                        <div class="grid gap-4 sm:grid-cols-2">
                            <NumberField label=t("Balance before", "Solde avant") prefix=t("$", "") suffix=t("", "$") value=balance />
                            <NumberField label=t("Size", "Taille") suffix=t("lots", "lot") value=lots />
                        </div>
                        {move || {
                            suggestion()
                                .map(|s| {
                                    let text = match s {
                                        Sizing::Ok { lots, risk, risk_pct } => ts(format!("Your plan's max size here: {lots} lots (risking {risk}, {risk_pct})."), format!("Taille maximale selon ton plan ici : {lots} lot (risque de {risk}, {risk_pct}).")),
                                        Sizing::TooSmall { .. } => t("Even 0.01 lots risks more than your plan allows with this stop.", "Même 0,01 lot risque plus que ton plan ne l’autorise avec ce stop.").into(),
                                    };
                                    view! { <p class="text-footnote text-text-muted">{text}</p> }
                                })
                        }}
                        <label class="block">
                            <span class="text-footnote font-semibold text-text-muted">{t("Which of today's Scenarios is this?", "De quel Scénario du jour s’agit-il ?")}</span>
                            <select class="mt-1.5 min-h-12 w-full rounded-field bg-bg px-3 text-callout" on:change=move |ev| scenario.set(event_target_value(&ev))>
                                <option value="">{t("None of them", "Aucun")}</option>
                                {move || day_scenarios().into_iter().map(|s| view! { <option value=s.clone()>{s.clone()}</option> }).collect_view()}
                            </select>
                        </label>
                        {move || day_ctx().is_some_and(|(_, _, c)| c.is_none()).then(|| view! {
                            <p class="rounded-btn bg-bg p-3 text-callout text-down">{t("⚠ No Pre-Session Checklist saved for this day, so this trade will break your plan.", "⚠ Aucune Checklist avant la séance enregistrée ce jour-là : ce trade enfreindra ton plan.")}</p>
                        })}
                        <label class="block">
                            <span class="text-footnote font-semibold text-text-muted">{t("Why you took it: trend, level, confirmation candle", "Pourquoi tu l’as pris : tendance, niveau, bougie de confirmation")}</span>
                            <textarea rows="4" class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout placeholder:text-text-muted" placeholder=t("H4 uptrend (higher lows). Price pulled back into the 4,090–4,095 zone and an H1 bullish engulfing closed there.", "Tendance haussière en H4 (plus bas ascendants). Le prix est revenu dans la zone 4090–4095 et une englobante haussière H1 y a clôturé.") prop:value=move || reason.get() on:input=move |ev| reason.set(event_target_value(&ev))></textarea>
                        </label>
                        <label class="block">
                            <span class="text-footnote font-semibold text-text-muted">{t("Chart screenshot (optional)", "Capture du graphique (facultatif)")}</span>
                            <input id="trade-shot" type="file" accept="image/*" class="mt-1.5 block w-full text-callout" />
                        </label>
                    </section>
                    {move || preview().map(|r| view! {
                        <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                            <h2 class="text-headline">{t("Hard Rules, checked as you type", "Règles strictes, vérifiées pendant la saisie")}</h2>
                            <div class="mt-3">{hard_list(r)}</div>
                        </section>
                    })}
                    {move || error.get().map(|e| view! { <p class="mt-4 text-callout text-down" role="alert">"⚠ " {e}</p> })}
                    <div class="mt-5 flex justify-end">
                        <button type="button" class=PILL disabled=move || busy.get() on:click=submit>
                            {move || if busy.get() { t("Saving…", "Enregistrement…") } else { t("Save and get Professor Aurel's verdict", "Enregistrer et obtenir le verdict du Professeur Aurel") }}
                        </button>
                    </div>
                }
                    .into_any()
            }}
        </main>
    }
}

#[component]
pub fn TradeSkip() -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let when = RwSignal::new(to_input(now()));
    let direction = RwSignal::new(Direction::Buy);
    let price = RwSignal::new(String::new());
    let reason = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);

    let save = move |_| {
        let (Some(at), Some(p)) = (from_input(&when.get_untracked()), Price::parse(&price.get_untracked())) else {
            error.set(Some(t("Enter the time and the price when you passed on it.", "Saisis l’heure et le prix au moment où tu as laissé passer.").into()));
            return;
        };
        let why = reason.get_untracked().trim().to_string();
        if why.split_whitespace().count() < 3 {
            error.set(Some(t("Say why you skipped it, in a few words.", "Dis en quelques mots pourquoi tu l’as écartée.").into()));
            return;
        }
        let entry = TradeEntry {
            at,
            direction: direction.get_untracked(),
            entry: p,
            stop: None,
            take_profit: None,
            lots: Lots(0),
            balance_before: journal.latest_balance().unwrap_or_default(),
            contract_oz: DEFAULT_CONTRACT_OZ,
            setup_reason: why,
            plan_version: journal.current_plan().map_or(0, |p| p.version),
        };
        let rec = TradeRecord {
            status: "skipped".into(),
            checklist_day: local_day(at, utc_offset_min()),
            entry,
            run: journal.active_run().map_or(0, |r| r.run.number),
            scenario: None,
            hard_results: Vec::new(),
            adherence: Adherence::Pending,
            soft_results: Vec::new(),
            verdict_summary: None,
            first_summary: None,
            dispute_text: None,
            screenshot: None,
            result: None,
        };
        journal.put_trade(new_id("trade"), rec);
        toasts.show(t("Skipped setup logged", "Configuration écartée notée"));
        let _ = window().location().set_hash("journal");
    };

    view! {
        <SubBar back="#journal" back_label="Journal" title=t("Skipped setup", "Configuration écartée") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">{t("Log a skipped setup", "Noter une configuration écartée")}</h1>
            <p class="mt-2 text-callout text-text-muted">{t("A setup you saw but didn't take, and why. It never counts toward Live Readiness, but your Weekly Review shows them: passing on a bad trade is a skill.", "Une configuration que tu as vue mais pas prise, et pourquoi. Elle ne compte jamais pour « Prêt pour le réel », mais ton Bilan de la semaine les montre : laisser passer un mauvais trade est une compétence.")}</p>
            <section class="mt-6 grid gap-4 rounded-card bg-surface p-5 shadow-card">
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">{t("When (your time)", "Quand (ton heure)")}</span>
                    <input type="datetime-local" class="mt-1.5 min-h-12 w-full rounded-field bg-bg px-3.5 text-body" prop:value=move || when.get() on:input=move |ev| when.set(event_target_value(&ev)) />
                </label>
                <Segmented label=t("Direction you considered", "Sens envisagé") value=direction options=vec![(Direction::Buy, t("▲ Buy", "▲ Achat")), (Direction::Sell, t("▼ Sell", "▼ Vente"))] />
                <NumberField label=t("Price at the time", "Prix à ce moment-là") value=price />
                <label class="block">
                    <span class="text-footnote font-semibold text-text-muted">{t("Why you passed", "Pourquoi tu as laissé passer")}</span>
                    <textarea rows="3" class="mt-1.5 w-full rounded-field bg-bg p-3.5 text-callout placeholder:text-text-muted" placeholder=t("No confirmation candle at the zone, and CPI was 10 minutes away.", "Pas de bougie de confirmation sur la zone, et le CPI tombait dans 10 minutes.") prop:value=move || reason.get() on:input=move |ev| reason.set(event_target_value(&ev))></textarea>
                </label>
            </section>
            {move || error.get().map(|e| view! { <p class="mt-4 text-callout text-down" role="alert">"⚠ " {e}</p> })}
            <div class="mt-5 flex justify-end"><button type="button" class=PILL on:click=save>{t("Save", "Enregistrer")}</button></div>
        </main>
    }
}

#[component]
pub fn TradeDetail(id: String) -> impl IntoView {
    let journal = expect_context::<JournalData>();
    let toasts = expect_context::<Toasts>();
    let id = StoredValue::new(id);
    let busy = RwSignal::new(false);
    let dispute_open = RwSignal::new(false);
    let dispute_text = RwSignal::new(String::new());
    let exit = RwSignal::new(String::new());
    let balance_after = RwSignal::new(String::new());
    let emotion = RwSignal::new(None::<Emotion>);
    let confirm_delete = RwSignal::new(false);
    let lesson = RwSignal::new(String::new());

    let rec = move || journal.trade(&id.get_value());

    let ask = move || {
        busy.set(true);
        spawn_local(async move {
            if let Err(msg) = request_verdict(journal, id.get_value()).await {
                toasts.show(msg);
            }
            busy.set(false);
        });
    };
    let dispute = move |_| {
        let Some(mut r) = rec() else { return };
        let text = dispute_text.get_untracked().trim().to_string();
        if text.split_whitespace().count() < 5 {
            toasts.show(t("Explain why you think the verdict is wrong, in at least a sentence.", "Explique en au moins une phrase pourquoi tu penses que le verdict est faux."));
            return;
        }
        match r.adherence.dispute(&r.hard_results) {
            Ok(next) => {
                r.adherence = next;
                r.dispute_text = Some(text);
                journal.put_trade(id.get_value(), r);
                dispute_open.set(false);
                ask();
            }
            Err(_) => toasts.show(t("This verdict can't be disputed.", "Ce verdict ne peut pas être contesté.")),
        }
    };
    let close = move |_| {
        let Some(mut r) = rec() else { return };
        let Some(x) = Price::parse(&exit.get_untracked()) else {
            toasts.show(t("Enter the exit price.", "Saisis le prix de sortie."));
            return;
        };
        let Some(em) = emotion.get_untracked() else {
            toasts.show(t("Pick how you felt during this trade.", "Choisis ce que tu as ressenti pendant ce trade."));
            return;
        };
        let p = pnl(&r.entry, x);
        let after = Usd::parse(&balance_after.get_untracked()).filter(|b| b.0 > 0).unwrap_or(r.entry.balance_before + p);
        let lesson = Some(lesson.get_untracked().trim().to_string()).filter(|l| !l.is_empty());
        r.result = Some(TradeResult { exit: x, pnl: p, balance_after: after, closed_at: now(), emotion: Some(em), lesson });
        journal.put_trade(id.get_value(), r);
        toasts.show(t("Result saved", "Résultat enregistré"));
    };

    let delete = move |_| {
        journal.delete_trade(id.get_value());
        toasts.show(t("Entry deleted", "Entrée supprimée"));
        let _ = window().location().set_hash("journal");
    };
    let deletable = move || crate::store::logged_at(&id.get_value()).is_some_and(|at| telosgard_core::journal::can_delete(at, now()));

    let body = move || {
        let Some(tr) = rec() else {
            return view! { <p class="mt-6 text-callout text-text-muted">{move || if journal.loaded.get() { t("This entry wasn't found.", "Cette entrée est introuvable.") } else { t("Loading…", "Chargement…") }}</p> }.into_any();
        };
        let (chip, chip_class) = verdict_chip(&tr);
        let e = tr.entry.clone();
        let dir = match e.direction { Direction::Buy => t("▲ Buy (long)", "▲ Achat (long)"), Direction::Sell => t("▼ Sell (short)", "▼ Vente (short)") };
        let fmt = |p: Option<Price>| p.map_or("—".to_string(), |p| p.to_string());
        let can_dispute = tr.adherence.dispute(&tr.hard_results).is_ok();
        let hard_failed = tr.hard_results.iter().any(|h| !h.pass);
        let preview_pnl = move || Price::parse(&exit.get()).map(|x| pnl_text(pnl(&e, x)));
        let e2 = tr.entry.clone();
        let r_of = tr.r();
        let risk = tr.risk();
        view! {
            <div class="mt-3 flex flex-wrap items-center gap-3">
                <span class=format!("rounded-full px-3 py-1 text-footnote font-semibold {chip_class}")>{chip}</span>
                <span class="text-footnote text-text-muted">{ts(format!("Plan v{} · Readiness Run {}", tr.entry.plan_version, tr.run), format!("Plan v{} · Série de préparation {}", tr.entry.plan_version, tr.run))}</span>
            </div>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                <dl class="grid grid-cols-2 gap-x-4 gap-y-3 sm:grid-cols-3">
                    <div><dt class="text-footnote text-text-muted">{t("Direction", "Sens")}</dt><dd class="text-callout font-semibold">{dir}</dd></div>
                    <div><dt class="text-footnote text-text-muted">{t("Entry", "Entrée")}</dt><dd class="tabular text-callout">{e2.entry.to_string()}</dd></div>
                    <div><dt class="text-footnote text-text-muted">{t("Size", "Taille")}</dt><dd class="tabular text-callout">{ts(format!("{} lots", e2.lots), format!("{} lot", e2.lots))}</dd></div>
                    <div><dt class="text-footnote text-text-muted">"Stop-loss"</dt><dd class="tabular text-callout">{fmt(e2.stop)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">"Take-profit"</dt><dd class="tabular text-callout">{fmt(e2.take_profit)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">{t("Balance before", "Solde avant")}</dt><dd class="tabular text-callout">{e2.balance_before.to_string()}</dd></div>
                </dl>
                {tr.scenario.clone().map(|s| view! { <p class="mt-4 text-callout"><span class="font-semibold">{t("Scenario: ", "Scénario : ")}</span>{s}</p> })}
                <p class="mt-3 text-callout"><span class="font-semibold">{if tr.taken() { t("Why: ", "Pourquoi : ") } else { t("Why you passed: ", "Pourquoi tu as laissé passer : ") }}</span>{e2.setup_reason.clone()}</p>
                {tr.screenshot.clone().map(|s| view! { <img class="mt-4 rounded-btn" src=asset_url(&s) alt=t("Chart screenshot for this trade", "Capture du graphique de ce trade") /> })}
            </section>
            {tr.taken().then(|| view! {
                <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                    <h2 class="text-headline">{t("Hard Rules", "Règles strictes")}</h2>
                    <div class="mt-3">{hard_list(tr.hard_results.clone())}</div>
                </section>
                <section class="mt-5 rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                    <h2 class="text-headline">{if matches!(tr.adherence, Adherence::Disputed { .. }) { t("Professor Aurel's second verdict (final)", "Second verdict du Professeur Aurel (définitif)") } else { t("Professor Aurel's verdict", "Verdict du Professeur Aurel") }}</h2>
                    {match tr.adherence {
                        Adherence::Pending | Adherence::DisputePending { .. } => view! {
                            <p class="mt-2 text-callout text-text-muted">{move || if busy.get() { t("Professor Aurel is reading your entry…", "Le Professeur Aurel lit ton entrée…") } else { t("No verdict yet. Nothing counts until one comes back.", "Pas encore de verdict. Rien ne compte tant qu’il n’est pas revenu.") }}</p>
                            <button type="button" class=format!("{PILL} mt-3") disabled=move || busy.get() on:click=move |_| ask()>{t("Get Professor Aurel's verdict", "Obtenir le verdict du Professeur Aurel")}</button>
                        }.into_any(),
                        _ => view! {
                            <p class="mt-2 text-callout">{tr.verdict_summary.clone().unwrap_or_default()}</p>
                            <ul class="mt-3 grid gap-2">
                                {tr.soft_results.clone().into_iter().map(|s| {
                                    let (mark, tone) = if s.pass { ("✓", "text-up") } else { ("✗", "text-down") };
                                    view! { <li class="flex gap-3 text-callout"><span class=format!("w-4 shrink-0 font-semibold {tone}")>{mark}</span><span><span class="font-semibold">{s.rule} {t(": ", " : ")}</span>{s.why}</span></li> }
                                }).collect_view()}
                            </ul>
                            {(matches!(tr.adherence, Adherence::Disputed { .. })).then(|| view! {
                                <details class="mt-3 text-callout text-text-muted"><summary class="cursor-pointer">{t("First verdict and your dispute", "Premier verdict et ta contestation")}</summary>
                                    <p class="mt-2">{tr.first_summary.clone().unwrap_or_default()}</p>
                                    <p class="mt-2"><span class="font-semibold">{t("You said: ", "Tu as dit : ")}</span>{tr.dispute_text.clone().unwrap_or_default()}</p>
                                </details>
                            })}
                            {(hard_failed && !tr.adherence.final_adherent().unwrap_or(true)).then(|| view! {
                                <p class="mt-3 text-footnote text-text-muted">{t("A Hard Rule failed, so this trade broke the plan whatever the Soft Rules say. Hard Rule failures can't be disputed.", "Une Règle stricte n’a pas été respectée : ce trade a donc enfreint le plan, quoi que disent les Règles souples. Un échec de Règle stricte ne peut pas être contesté.")}</p>
                            })}
                            {can_dispute.then(|| view! {
                                <Show when=move || !dispute_open.get() fallback=move || view! {
                                    <div class="mt-4 grid gap-3">
                                        <textarea rows="3" class="w-full rounded-field bg-bg p-3.5 text-callout placeholder:text-text-muted" placeholder=t("What did Professor Aurel miss? You get one dispute; the second verdict is final.", "Qu’est-ce que le Professeur Aurel n’a pas vu ? Tu as droit à une contestation ; le second verdict est définitif.") prop:value=move || dispute_text.get() on:input=move |ev| dispute_text.set(event_target_value(&ev))></textarea>
                                        <div class="flex justify-end gap-3">
                                            <button type="button" class=PILL_GHOST on:click=move |_| dispute_open.set(false)>{t("Cancel", "Annuler")}</button>
                                            <button type="button" class=PILL on:click=dispute>{t("Send dispute", "Envoyer la contestation")}</button>
                                        </div>
                                    </div>
                                }>
                                    <button type="button" class=format!("{PILL_GHOST} mt-4") on:click=move |_| dispute_open.set(true)>{t("Dispute this verdict (once)", "Contester ce verdict (une fois)")}</button>
                                </Show>
                            })}
                        }.into_any(),
                    }}
                </section>
                <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                    <h2 class="text-headline">{t("Result", "Résultat")}</h2>
                    {match tr.result.clone() {
                        Some(r) => {
                            let (txt, tone) = pnl_text(r.pnl);
                            view! {
                                <dl class="mt-3 grid grid-cols-2 gap-4 sm:grid-cols-4">
                                    <div><dt class="text-footnote text-text-muted">{t("Exit", "Sortie")}</dt><dd class="tabular text-callout">{r.exit.to_string()}</dd></div>
                                    <div><dt class="text-footnote text-text-muted">{t("Profit / loss", "Gain / perte")}</dt><dd class=format!("tabular text-callout font-semibold {tone}")>{txt}</dd></div>
                                    <div><dt class="text-footnote text-text-muted">{t("Result in R", "Résultat en R")}</dt><dd class=format!("tabular text-callout font-semibold {tone}")>{r_of.map_or("—".to_string(), |x| x.to_string())}</dd></div>
                                    <div><dt class="text-footnote text-text-muted">{t("Balance after", "Solde après")}</dt><dd class="tabular text-callout">{r.balance_after.to_string()}</dd></div>
                                </dl>
                                {risk.map(|k| view! { <p class="mt-2 text-footnote text-text-muted">{ts(format!("1R = {k}, the money at risk between entry and stop."), format!("1R = {k}, l’argent en jeu entre l’entrée et le stop."))}</p> })}
                                {r.emotion.map(|em| view! { <p class="mt-3 text-callout"><span class="font-semibold">{t("Felt: ", "Ressenti : ")}</span>{em.label()}</p> })}
                                {r.lesson.clone().map(|l| view! { <p class="mt-1 text-callout"><span class="font-semibold">{t("Lesson: ", "Leçon : ")}</span>{l}</p> })}
                            }.into_any()
                        }
                        None => view! {
                            <p class="mt-1 text-callout text-text-muted">{t("Fill this in once the trade is closed in MT5.", "Remplis ceci une fois le trade clôturé dans MT5.")}</p>
                            <div class="mt-3 grid gap-4 sm:grid-cols-2">
                                <NumberField label=t("Exit price", "Prix de sortie") value=exit />
                                <NumberField label=t("Balance after (from MT5, optional)", "Solde après (d’après MT5, facultatif)") prefix=t("$", "") suffix=t("", "$") value=balance_after />
                            </div>
                            {move || preview_pnl().map(|(txt, tone)| view! { <p class=format!("tabular mt-3 text-callout font-semibold {tone}")>{txt} {t(" before costs", " avant coûts")}</p> })}
                            <fieldset class="mt-5">
                                <legend class="text-callout font-semibold">{t("How did you feel during this trade?", "Qu’as-tu ressenti pendant ce trade ?")}</legend>
                                <div role="radiogroup" aria-label=t("Emotion Tag", "Émotion notée") class="mt-2 flex flex-wrap gap-2">
                                    {Emotion::ALL.into_iter().map(|em| view! {
                                        <button
                                            type="button"
                                            role="radio"
                                            aria-checked=move || (emotion.get() == Some(em)).to_string()
                                            on:click=move |_| emotion.set(Some(em))
                                            class=move || if emotion.get() == Some(em) {
                                                "press min-h-11 rounded-full bg-fill px-4 text-callout font-semibold text-on-fill"
                                            } else {
                                                "press min-h-11 rounded-full border border-line bg-surface px-4 text-callout text-text-muted"
                                            }
                                        >{em.label()}</button>
                                    }).collect_view()}
                                </div>
                            </fieldset>
                            <label class="mt-4 block">
                                <span class="text-callout font-semibold">{t("One lesson from this trade (optional)", "Une leçon de ce trade (facultatif)")}</span>
                                <input type="text" maxlength="200" class="mt-1.5 min-h-11 w-full rounded-field bg-bg px-3.5 text-callout placeholder:text-text-muted" placeholder=t("I moved my stop because I was scared; next time I leave it.", "J’ai déplacé mon stop parce que j’avais peur ; la prochaine fois, je le laisse.") prop:value=move || lesson.get() on:input=move |ev| lesson.set(event_target_value(&ev)) />
                            </label>
                            <button type="button" class=format!("{PILL} mt-4") on:click=close>{t("Save result", "Enregistrer le résultat")}</button>
                        }.into_any(),
                    }}
                </section>
            })}
            {deletable().then(|| view! {
                <section class="mt-8 rounded-card border border-line/40 p-5" aria-label=t("Delete this entry", "Supprimer cette entrée")>
                    <Show
                        when=move || confirm_delete.get()
                        fallback=move || view! {
                            <button type="button" class="press min-h-11 text-callout text-down underline underline-offset-4" on:click=move |_| confirm_delete.set(true)>
                                {t("Delete this entry", "Supprimer cette entrée")}
                            </button>
                            <p class="mt-1 text-footnote text-text-muted">{t("For test or mistaken entries. Possible for 24 hours after logging; after that it's part of your record.", "Pour les entrées de test ou les erreurs. Possible pendant 24 heures après l’enregistrement ; ensuite, elle fait partie de ton historique.")}</p>
                        }
                    >
                        <p class="text-callout font-semibold" role="alert">{t("Delete this entry permanently?", "Supprimer définitivement cette entrée ?")}</p>
                        <p class="mt-1 text-callout text-text-muted">
                            {t("It's removed from your journal, Readiness and Weekly Review, and can't be undone. If it's the only entry in your current Readiness Run, that run is removed too.", "Elle est retirée de ton journal, de ta préparation et de ton Bilan de la semaine, sans retour possible. Si c’est la seule entrée de ta Série de préparation actuelle, la série est retirée aussi.")}
                        </p>
                        <div class="mt-3 flex flex-wrap gap-3">
                            <button type="button" class=PILL_GHOST on:click=move |_| confirm_delete.set(false)>{t("Cancel", "Annuler")}</button>
                            <button type="button" class="press min-h-11 rounded-full bg-down px-5 text-callout font-semibold text-bg" on:click=delete>{t("Delete permanently", "Supprimer définitivement")}</button>
                        </div>
                    </Show>
                </section>
            })}
        }
        .into_any()
    };

    view! {
        <SubBar back="#journal" back_label="Journal" title=t("Journal entry", "Entrée du journal") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <h1 class="text-title1">{move || rec().map_or(t("Journal entry", "Entrée du journal").into(), |tr| local_label(tr.entry.at))}</h1>
            {body}
        </main>
    }
}

trait PositiveLots {
    fn into_positive(self) -> Option<Lots>;
}
impl PositiveLots for Lots {
    fn into_positive(self) -> Option<Lots> {
        (self.0 > 0).then_some(self)
    }
}
