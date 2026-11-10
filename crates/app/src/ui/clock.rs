//! Session clock (FR-12.2): current session and the next overlap and rollover, in local time.

use crate::journal_data::local_hm;
use crate::state::now;
use gold_core::Millis;
use gold_core::clock::{in_overlap, next_overlap_start, next_rollover};
use gold_core::plan::Session;
use gold_core::time::{NoTrade, no_trade_window, sessions_at};
use leptos::prelude::*;
use std::time::Duration;

fn until(target: Millis, from: Millis) -> String {
    let mins = ((target - from) / 60_000).max(0);
    let (d, h, m) = (mins / 1440, (mins % 1440) / 60, mins % 60);
    match (d, h) {
        (0, 0) => format!("in {m} min"),
        (0, _) => format!("in {h} h {m:02} min"),
        _ => format!("in {d} d {h} h"),
    }
}

fn session_label(s: &[Session]) -> String {
    let name = |s: &Session| match s {
        Session::Asian => "Asian",
        Session::London => "London",
        Session::NewYork => "New York",
    };
    match s {
        [] => "Between sessions".into(),
        [one] => format!("{} session", name(one)),
        many => format!("{} overlap", many.iter().map(name).collect::<Vec<_>>().join("–")),
    }
}

#[component]
pub fn SessionClock() -> impl IntoView {
    let tick = RwSignal::new(now());
    if let Ok(h) = set_interval_with_handle(move || tick.set(now()), Duration::from_secs(30)) {
        on_cleanup(move || h.clear());
    }
    let state = move || {
        let t = tick.get();
        let blocked = no_trade_window(t, &[]);
        let headline = match &blocked {
            Some(NoTrade::MarketClosed) => "Market closed".to_string(),
            Some(NoTrade::Rollover) => "Daily rollover · No-Trade Window".to_string(),
            Some(NoTrade::SundayOpen) => "Sunday open · No-Trade Window".to_string(),
            _ => session_label(&sessions_at(t)),
        };
        let overlap = if in_overlap(t) {
            "London–New York overlap: now".to_string()
        } else {
            next_overlap_start(t).map_or("—".into(), |o| format!("Next London–New York overlap: {} ({})", local_hm(o), until(o, t)))
        };
        let rollover = next_rollover(t).map_or("—".into(), |r| format!("Next rollover: {}–{} ({})", local_hm(r), local_hm(r + 3_600_000), until(r, t)));
        (headline, blocked.is_some(), local_hm(t), overlap, rollover)
    };
    view! {
        <section class="rounded-card bg-surface p-5 shadow-card" aria-label="Session clock">
            {move || {
                let (headline, blocked, time, overlap, rollover) = state();
                view! {
                    <div class="flex items-baseline justify-between gap-3">
                        <p class="text-eyebrow uppercase text-text-muted">"Gold right now"</p>
                        <p class="tabular text-footnote text-text-muted">"Your time " {time}</p>
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
