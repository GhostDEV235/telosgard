//! Tools tab: position-size calculator (live) and the Trading Plan placeholder.

use crate::i18n::t;
use crate::screens::shell::LargeTitle;
use crate::ui::controls::NumberField;
use telosgard_core::money::{DEFAULT_CONTRACT_OZ, DollarMove, Percent, Usd};
use telosgard_core::sizing::{Sizing, SizingInput, size};
use leptos::prelude::*;

#[component]
pub fn Tools() -> impl IntoView {
    let balance = RwSignal::new("10000".to_string());
    let risk = RwSignal::new("1".to_string());
    let stop = RwSignal::new("5".to_string());
    let contract = RwSignal::new(DEFAULT_CONTRACT_OZ.to_string());

    let balance_v = move || Usd::parse(&balance.get()).filter(|u| u.0 > 0);
    let risk_v = move || Percent::parse(&risk.get()).filter(|p| p.0 > 0 && p.0 <= 10_000);
    let stop_v = move || DollarMove::parse(&stop.get()).filter(|d| d.0 > 0);
    let contract_v = move || contract.get().trim().parse::<u32>().ok().filter(|c| *c > 0);

    let err = |bad: bool, msg: &str| bad.then(|| msg.to_string());
    let balance_err = Signal::derive(move || err(balance_v().is_none(), t("Enter your account balance, like 10000.", "Saisis le solde de ton compte, par exemple 10000.")));
    let risk_err = Signal::derive(move || err(risk_v().is_none(), t("Enter a risk between 0.01% and 100%.", "Saisis un risque entre 0,01 % et 100 %.")));
    let stop_err = Signal::derive(move || err(stop_v().is_none(), t("Enter how far away your stop is, in dollars, like 5.", "Saisis la distance de ton stop, en dollars, par exemple 5.")));
    let contract_err = Signal::derive(move || err(contract_v().is_none(), t("Enter ounces per lot (usually 100).", "Saisis le nombre d’onces par lot (en général 100).")));

    let result = move || match (balance_v(), risk_v(), stop_v(), contract_v()) {
        (Some(balance), Some(risk), Some(stop), Some(contract_oz)) => size(SizingInput { balance, risk, stop, contract_oz }).ok(),
        _ => None,
    };

    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title=t("Tools for", "Des outils pour") accent=t("the plan.", "le plan.") />
            <h2 class="mt-7 text-title2">{t("Position size", "Taille de position")}</h2>
            <p class="mt-1 max-w-xl text-callout text-text-muted">
                {t("How many lots can you trade so that, if your stop-loss is hit, you lose only the share of your account you chose?", "Combien de lots peux-tu trader pour que, si ton stop-loss est touché, tu ne perdes que la part du compte que tu as choisie ?")}
            </p>
            <div class="mt-4 grid items-start gap-5 md:grid-cols-2">
                <section class="grid gap-4 rounded-card bg-surface p-5 shadow-card" aria-label=t("Inputs", "Données")>
                    <NumberField label=t("Account balance", "Solde du compte") prefix=t("$", "") suffix=t("", "$") value=balance error=balance_err />
                    <NumberField label=t("Risk per trade", "Risque par trade") suffix="%" value=risk error=risk_err />
                    <NumberField label=t("Stop distance (Dollar Move)", "Distance du stop (Mouvement en dollars)") prefix=t("$", "") suffix=t("", "$") value=stop error=stop_err />
                    <details class="group">
                        <summary class="press min-h-11 cursor-pointer list-none py-2 text-callout text-text-muted">
                            {t("Contract size: ", "Taille du contrat : ")} {move || contract.get()} {t(" oz per lot ", " oz par lot ")}
                            <span class="inline-block transition-transform group-open:rotate-180" aria-hidden="true">"▾"</span>
                        </summary>
                        <div class="pt-2">
                            <NumberField label=t("Ounces per lot", "Onces par lot") suffix="oz" value=contract error=contract_err />
                            <p class="mt-2 text-footnote text-text-muted">{t("Most brokers use 100 oz. Check your MT5 symbol spec if unsure.", "La plupart des courtiers utilisent 100 oz. En cas de doute, vérifie la fiche du symbole dans MT5.")}</p>
                        </div>
                    </details>
                </section>

                <section class="rounded-card bg-surface p-5 shadow-card" aria-live="polite" aria-label=t("Result", "Résultat")>
                    {move || match result() {
                        Some(Sizing::Ok { lots, risk, risk_pct }) => {
                            let stop = stop_v().unwrap_or_default();
                            view! {
                                <p class="text-eyebrow text-eyebrow-ink">{t("Trade size", "Taille du trade")}</p>
                                <p class="tabular mt-1 text-[56px] leading-[1.05] font-bold tracking-tight">
                                    {lots.to_string()} <span class="text-title2 font-semibold text-text-muted">{t(" lots", " lot")}</span>
                                </p>
                                <dl class="mt-5 grid grid-cols-2 gap-4">
                                    <div>
                                        <dt class="text-footnote text-text-muted">{t("Money at risk", "Argent en jeu")}</dt>
                                        <dd class="tabular text-title2">{risk.to_string()}</dd>
                                    </div>
                                    <div>
                                        <dt class="text-footnote text-text-muted">{t("Share of balance", "Part du solde")}</dt>
                                        <dd class="tabular text-title2">{risk_pct.to_string()}</dd>
                                    </div>
                                </dl>
                                <p class="mt-5 rounded-btn bg-bg p-3.5 text-callout">
                                    {t("If gold moves ", "Si l’or bouge de ")} {stop.to_string()} {t(" against you and your stop is hit, you lose about ", " contre toi et que ton stop est touché, tu perds environ ")}
                                    <strong>{risk.to_string()}</strong> {t(". Rounded down, so never more than you planned.", ". Arrondi vers le bas : jamais plus que prévu.")}
                                </p>
                            }
                                .into_any()
                        }
                        Some(Sizing::TooSmall { min_lot_risk, min_lot_risk_pct }) => {
                            view! {
                                <p class="text-eyebrow text-eyebrow-ink">{t("Trade size", "Taille du trade")}</p>
                                <p class="mt-1 text-title1 text-down">{t("▼ Too small to trade safely", "▼ Trop petit pour trader sans danger")}</p>
                                <p class="mt-3 text-callout">
                                    {t("Even the smallest size, 0.01 lots, would lose ", "Même la plus petite taille, 0,01 lot, perdrait ")} <strong>{min_lot_risk.to_string()}</strong> " ("
                                    {min_lot_risk_pct.to_string()} {t(") if your stop is hit. That's more than you chose to risk.", ") si ton stop est touché. C’est plus que ce que tu as choisi de risquer.")}
                                </p>
                                <p class="mt-3 text-callout text-text-muted">
                                    {t("Options: find a setup with a closer stop, or skip this trade. Skipping is a valid trade decision.", "Possibilités : trouver une configuration avec un stop plus proche, ou laisser passer ce trade. Laisser passer est une vraie décision de trading.")}
                                </p>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <p class="text-eyebrow text-eyebrow-ink">{t("Trade size", "Taille du trade")}</p>
                                <p class="mt-2 text-callout text-text-muted">{t("Fix the highlighted field to see your size.", "Corrige le champ en surbrillance pour voir ta taille.")}</p>
                            }
                                .into_any()
                        }
                    }}
                    <p class="mt-5 text-footnote text-text-muted">
                        {t("lots = balance × risk% ÷ (stop distance × ounces per lot), rounded down to 0.01", "lots = solde × risque % ÷ (distance du stop × onces par lot), arrondi vers le bas à 0,01")}
                    </p>
                </section>
            </div>

            <h2 class="mt-10 text-title2">{t("Your Trading Plan", "Ton Plan de trading")}</h2>
            <a href="#plan" class="lift mt-3 flex items-center gap-4 rounded-card bg-surface p-5 shadow-card">
                <span class="min-w-0 flex-1">
                    <span class="block text-headline">{t("Trading Plan", "Plan de trading")}</span>
                    <span class="block text-callout text-text-muted">{t("Your written rules, versioned. You'll build it properly at Lesson 39.", "Tes règles écrites, avec leurs versions. Tu le construiras vraiment à la leçon 39.")}</span>
                </span>
                <span class="text-title2 text-text-muted" aria-hidden="true">"›"</span>
            </a>
        </main>
    }
}
