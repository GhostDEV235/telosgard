//! SVG candle chart and ring progress.

use telosgard_core::drills::{Candle, ILLUSTRATIVE_LABEL};
use telosgard_core::money::Price;
use leptos::prelude::*;

const W: f64 = 640.0;
const H: f64 = 280.0;
const PLOT_L: f64 = 8.0;
const PLOT_R: f64 = W - 92.0;
const PLOT_T: f64 = 14.0;
const PLOT_B: f64 = H - 14.0;

fn axis_label(p: Price) -> String {
    p.to_string().trim_start_matches('$').to_string()
}

/// Bullish candles are hollow and bearish ones filled, so direction never depends on colour alone.
#[component]
pub fn CandleChart(
    #[prop(into)] candles: Signal<Vec<Candle>>,
    #[prop(into)] highlight: Signal<Option<usize>>,
    label: &'static str,
) -> impl IntoView {
    let body = move || {
        let cs = candles.get();
        if cs.is_empty() {
            return Vec::new();
        }
        let hi = cs.iter().map(|c| c.high.0).max().unwrap_or(0) as f64;
        let lo = cs.iter().map(|c| c.low.0).min().unwrap_or(0) as f64;
        let pad = ((hi - lo) * 0.06).max(50.0);
        let (hi, lo) = (hi + pad, lo - pad);
        let y = move |p: Price| PLOT_T + (hi - p.0 as f64) / (hi - lo) * (PLOT_B - PLOT_T);
        let slot = (PLOT_R - PLOT_L) / cs.len() as f64;
        let bw = (slot * 0.62).max(2.0);

        let mut out = Vec::new();
        for i in 0..=4 {
            let price = Price((lo + (hi - lo) * i as f64 / 4.0).round() as i64);
            let yy = y(price);
            out.push(
                view! {
                    <line x1=PLOT_L x2=PLOT_R y1=yy y2=yy class="stroke-line/30" stroke-width="1" />
                    <text x=PLOT_R + 8.0 y=yy + 6.0 class="fill-text-muted tabular" font-size="17">
                        {axis_label(price)}
                    </text>
                }
                .into_any(),
            );
        }
        let hl = highlight.get();
        for (i, c) in cs.iter().enumerate() {
            let cx = PLOT_L + slot * (i as f64 + 0.5);
            if hl == Some(i) {
                out.push(
                    view! { <rect x=cx - slot / 2.0 y=PLOT_T - 6.0 width=slot height=PLOT_B - PLOT_T + 12.0 rx="6" class="fill-text/10 stroke-text/40" stroke-width="1" /> }
                        .into_any(),
                );
            }
            let up = c.close >= c.open;
            let top = y(c.open.max(c.close));
            let h = (y(c.open.min(c.close)) - top).max(1.5);
            let (wick, rect) = if up { ("stroke-up", "fill-bg stroke-up") } else { ("stroke-down", "fill-down stroke-down") };
            out.push(
                view! {
                    <line x1=cx x2=cx y1=y(c.high) y2=y(c.low) class=wick stroke-width="1.5" />
                    <rect x=cx - bw / 2.0 y=top width=bw height=h rx="1.5" class=rect stroke-width="1.5" />
                }
                .into_any(),
            );
        }
        out
    };
    view! {
        <figure>
            <svg viewBox=format!("0 0 {W} {H}") class="h-auto w-full" role="img" aria-label=label>
                {body}
            </svg>
            <figcaption class="mt-2 text-footnote text-text-muted">{ILLUSTRATIVE_LABEL}</figcaption>
        </figure>
    }
}

const CIRC: f64 = 2.0 * std::f64::consts::PI * 42.0;

#[component]
pub fn Ring(
    /// 0.0–1.0
    #[prop(into)]
    value: Signal<f64>,
    label: &'static str,
    /// The figure under the ring (a CountUp, usually).
    children: Children,
) -> impl IntoView {
    // The arc fills from empty once it arrives (a CSS transition; off under reduced motion).
    let arrived = RwSignal::new(false);
    request_animation_frame(move || {
        let _ = arrived.try_set(true);
    });
    let v = move || if arrived.get() { value.get().clamp(0.0, 1.0) } else { 0.0 };
    view! {
        <div class="flex flex-col items-center gap-1.5 text-center">
            <svg viewBox="0 0 100 100" class="size-22 -rotate-90" aria-hidden="true">
                <circle cx="50" cy="50" r="42" fill="none" class="stroke-bg" stroke-width="11" />
                <circle
                    cx="50"
                    cy="50"
                    r="42"
                    fill="none"
                    stroke-width="11"
                    stroke-linecap="round"
                    class=move || { if v() >= 1.0 { "stroke-gold-fill" } else { "stroke-text-muted" } }
                    opacity=move || { if value.get() > 0.0 { "1" } else { "0" } }
                    stroke-dasharray=format!("{CIRC} {CIRC}")
                    stroke-dashoffset=move || CIRC * (1.0 - v())
                    style="transition: stroke-dashoffset 900ms var(--ease-enter)"
                />
            </svg>
            <p class="tabular text-headline">{children()}</p>
            <p class="text-footnote text-text-muted">{label}</p>
        </div>
    }
}
