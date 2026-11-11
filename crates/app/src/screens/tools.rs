//! Tools tab: position-size calculator (live) and the Trading Plan placeholder.

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
    let balance_err = Signal::derive(move || err(balance_v().is_none(), "Enter your account balance, like 10000."));
    let risk_err = Signal::derive(move || err(risk_v().is_none(), "Enter a risk between 0.01% and 100%."));
    let stop_err = Signal::derive(move || err(stop_v().is_none(), "Enter how far away your stop is, in dollars, like 5."));
    let contract_err = Signal::derive(move || err(contract_v().is_none(), "Enter ounces per lot (usually 100)."));

    let result = move || match (balance_v(), risk_v(), stop_v(), contract_v()) {
        (Some(balance), Some(risk), Some(stop), Some(contract_oz)) => size(SizingInput { balance, risk, stop, contract_oz }).ok(),
        _ => None,
    };

    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title="Tools" />
            <h2 class="mt-7 text-title2">"Position size"</h2>
            <p class="mt-1 max-w-xl text-callout text-text-muted">
                "How many lots can you trade so that, if your stop-loss is hit, you lose only the share of your account you chose?"
            </p>
            <div class="mt-4 grid items-start gap-5 md:grid-cols-2">
                <section class="grid gap-4 rounded-card bg-surface p-5 shadow-card" aria-label="Inputs">
                    <NumberField label="Account balance" prefix="$" value=balance error=balance_err />
                    <NumberField label="Risk per trade" suffix="%" value=risk error=risk_err />
                    <NumberField label="Stop distance (Dollar Move)" prefix="$" value=stop error=stop_err />
                    <details class="group">
                        <summary class="press min-h-11 cursor-pointer list-none py-2 text-callout text-text-muted">
                            "Contract size: " {move || contract.get()} " oz per lot "
                            <span class="inline-block transition-transform group-open:rotate-180" aria-hidden="true">"▾"</span>
                        </summary>
                        <div class="pt-2">
                            <NumberField label="Ounces per lot" suffix="oz" value=contract error=contract_err />
                            <p class="mt-2 text-footnote text-text-muted">"Most brokers use 100 oz. Check your MT5 symbol spec if unsure."</p>
                        </div>
                    </details>
                </section>

                <section class="rounded-card bg-surface p-5 shadow-card" aria-live="polite" aria-label="Result">
                    {move || match result() {
                        Some(Sizing::Ok { lots, risk, risk_pct }) => {
                            let stop = stop_v().unwrap_or_default();
                            view! {
                                <p class="text-eyebrow text-text-muted">"Trade size"</p>
                                <p class="tabular mt-1 text-[56px] leading-[1.05] font-bold tracking-tight">
                                    {lots.to_string()} <span class="text-title2 font-semibold text-text-muted">" lots"</span>
                                </p>
                                <dl class="mt-5 grid grid-cols-2 gap-4">
                                    <div>
                                        <dt class="text-footnote text-text-muted">"Money at risk"</dt>
                                        <dd class="tabular text-title2">{risk.to_string()}</dd>
                                    </div>
                                    <div>
                                        <dt class="text-footnote text-text-muted">"Share of balance"</dt>
                                        <dd class="tabular text-title2">{risk_pct.to_string()}</dd>
                                    </div>
                                </dl>
                                <p class="mt-5 rounded-btn bg-bg p-3.5 text-callout">
                                    "If gold moves " {stop.to_string()} " against you and your stop is hit, you lose about "
                                    <strong>{risk.to_string()}</strong> ". Rounded down, so never more than you planned."
                                </p>
                            }
                                .into_any()
                        }
                        Some(Sizing::TooSmall { min_lot_risk, min_lot_risk_pct }) => {
                            view! {
                                <p class="text-eyebrow text-text-muted">"Trade size"</p>
                                <p class="mt-1 text-title1 text-down">"▼ Too small to trade safely"</p>
                                <p class="mt-3 text-callout">
                                    "Even the smallest size, 0.01 lots, would lose " <strong>{min_lot_risk.to_string()}</strong> " ("
                                    {min_lot_risk_pct.to_string()} ") if your stop is hit. That's more than you chose to risk."
                                </p>
                                <p class="mt-3 text-callout text-text-muted">
                                    "Options: find a setup with a closer stop, or skip this trade. Skipping is a valid trade decision."
                                </p>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <p class="text-eyebrow text-text-muted">"Trade size"</p>
                                <p class="mt-2 text-callout text-text-muted">"Fix the highlighted field to see your size."</p>
                            }
                                .into_any()
                        }
                    }}
                    <p class="mt-5 text-footnote text-text-muted">
                        "lots = balance × risk% ÷ (stop distance × ounces per lot), rounded down to 0.01"
                    </p>
                </section>
            </div>

            <h2 class="mt-10 text-title2">"Your Trading Plan"</h2>
            <a href="#plan" class="lift mt-3 flex items-center gap-4 rounded-card bg-surface p-5 shadow-card">
                <span class="min-w-0 flex-1">
                    <span class="block text-headline">"Trading Plan"</span>
                    <span class="block text-callout text-text-muted">"Your written rules, versioned. You'll build it properly at Lesson 39."</span>
                </span>
                <span class="text-title2 text-text-muted" aria-hidden="true">"›"</span>
            </a>
        </main>
    }
}
