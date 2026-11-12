//! Session clock (FR-12.2): current session and the next overlap and rollover, in local time.

use crate::i18n::{t, ts};
use crate::journal_data::local_hm;
use crate::state::now;
use telosgard_core::Millis;
use telosgard_core::clock::{in_overlap, next_overlap_start, next_rollover};
use telosgard_core::plan::Session;
use telosgard_core::time::{NoTrade, no_trade_window, sessions_at};
use leptos::prelude::*;
use std::time::Duration;

fn until(target: Millis, from: Millis) -> String {
    let mins = ((target - from) / 60_000).max(0);
    let (d, h, m) = (mins / 1440, (mins % 1440) / 60, mins % 60);
    match (d, h) {
        (0, 0) => ts(format!("in {m} min"), format!("dans {m} min")),
        (0, _) => ts(format!("in {h} h {m:02} min"), format!("dans {h} h {m:02}")),
        _ => ts(format!("in {d} d {h} h"), format!("dans {d} j {h} h")),
    }
}

fn session_label(s: &[Session]) -> String {
    let name = |s: &Session| match s {
        Session::Asian => t("Asian", "asiatique"),
        Session::London => t("London", "de Londres"),
        Session::NewYork => t("New York", "de New York"),
    };
    let city = |s: &Session| match s {
        Session::Asian => t("Asian", "Asie"),
        Session::London => t("London", "Londres"),
        Session::NewYork => "New York",
    };
    match s {
        [] => t("Between sessions", "Entre deux séances").into(),
        [one] => ts(format!("{} session", name(one)), format!("Séance {}", name(one))),
        many => ts(
            format!("{} overlap", many.iter().map(name).collect::<Vec<_>>().join("–")),
            format!("Chevauchement {}", many.iter().map(city).collect::<Vec<_>>().join("–")),
        ),
    }
}

#[component]
pub fn SessionClock() -> impl IntoView {
    let tick = RwSignal::new(now());
    if let Ok(h) = set_interval_with_handle(move || tick.set(now()), Duration::from_secs(30)) {
        on_cleanup(move || h.clear());
    }
    let state = move || {
        let at = tick.get();
        let blocked = no_trade_window(at, &[]);
        let headline = match &blocked {
            Some(NoTrade::MarketClosed) => t("Market closed", "Marché fermé").to_string(),
            Some(NoTrade::Rollover) => t("Daily rollover · No-Trade Window", "Rollover quotidien · Fenêtre sans trade").to_string(),
            Some(NoTrade::SundayOpen) => t("Sunday open · No-Trade Window", "Ouverture du dimanche · Fenêtre sans trade").to_string(),
            _ => session_label(&sessions_at(at)),
        };
        let overlap = if in_overlap(at) {
            t("London–New York overlap: now", "Chevauchement Londres–New York : maintenant").to_string()
        } else {
            next_overlap_start(at).map_or("—".into(), |o| ts(format!("Next London–New York overlap: {} ({})", local_hm(o), until(o, at)), format!("Prochain chevauchement Londres–New York : {} ({})", local_hm(o), until(o, at))))
        };
        let rollover = next_rollover(at).map_or("—".into(), |r| ts(format!("Next rollover: {}–{} ({})", local_hm(r), local_hm(r + 3_600_000), until(r, at)), format!("Prochain rollover : {}–{} ({})", local_hm(r), local_hm(r + 3_600_000), until(r, at))));
        (headline, blocked.is_some(), local_hm(at), overlap, rollover)
    };
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card" aria-label=t("Session clock", "Horloge des séances")>
            {move || {
                let (headline, blocked, time, overlap, rollover) = state();
                view! {
                    <div class="flex items-baseline justify-between gap-3">
                        <p class="text-eyebrow text-eyebrow-ink">{t("Gold right now", "L’or en ce moment")}</p>
                        <p class="tabular text-footnote text-text-muted">{t("Your time ", "Ton heure ")} {time}</p>
                    </div>
                    <p class=format!("mt-1 text-headline {}", if blocked { "text-down" } else { "" })>
                        {if blocked { "⚠ " } else { "◷ " }}
                        {headline}
                    </p>
                    <ul class="mt-2 grid gap-1 text-callout text-text-muted">
                        <li>{overlap}</li>
                        <li>{rollover}</li>
                    </ul>
                }
            }}
        </section>
    }
}
