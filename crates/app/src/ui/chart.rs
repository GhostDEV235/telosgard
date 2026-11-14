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

/// A few candles drawn large on a fixed price scale, for widgets and the "Build the candle" drill.
/// `lo` and `hi` set the scale, so several drawings can be compared; the price axis is on the right.
pub fn candle_strip(cs: &[Candle], lo: Price, hi: Price, label: String) -> impl IntoView + use<> {
    const SLOT: f64 = 70.0;
    const TOP: f64 = 10.0;
    const BOTTOM: f64 = 190.0;
    let plot_r = SLOT * cs.len() as f64;
    let w = plot_r + 86.0;
    let (lo_f, hi_f) = (lo.0 as f64, hi.0 as f64);
    let y = move |p: Price| TOP + (hi_f - p.0 as f64) / (hi_f - lo_f).max(1.0) * (BOTTOM - TOP);
    let grid = (0..=3)
        .map(|i| {
            // Whole dollars, so the axis reads like a chart, not a calculator.
            let price = Price(((lo_f + (hi_f - lo_f) * i as f64 / 3.0) / 100.0).round() as i64 * 100);
            let yy = y(price);
            view! {
                <line x1="0" x2=plot_r y1=yy y2=yy class="stroke-line/30" stroke-width="1" />
                <text x=plot_r + 8.0 y=yy + 5.0 class="fill-text-muted tabular" font-size="15">{axis_label(price)}</text>
            }
        })
        .collect_view();
    let candles = cs
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let cx = SLOT * (i as f64 + 0.5);
            let up = c.close >= c.open;
            let top = y(c.open.max(c.close));
            let h = (y(c.open.min(c.close)) - top).max(2.0);
            let (wick, rect) = if up { ("stroke-up", "fill-bg stroke-up") } else { ("stroke-down", "fill-down stroke-down") };
            view! {
                <line x1=cx x2=cx y1=y(c.high) y2=y(c.low) class=wick stroke-width="2" />
                <rect x=cx - 14.0 y=top width="28" height=h rx="2" class=rect stroke-width="2" />
            }
        })
        .collect_view();
    view! {
        // At most 1.5× its own size, so a strip of one or two candles doesn't blow up its labels.
        <svg viewBox=format!("0 0 {w} 200") class="h-auto w-full" style=format!("max-width:{:.0}px", w * 1.5) role="img" aria-label=label>
            {grid}
            {candles}
        </svg>
    }
}

/// Swing points joined by a line, with optional labelled zones (A–D) and an optional final break
/// (dashed, above the last swing high when `broke_up`, below the last swing low otherwise).
/// For the Stage 5 and 6 drills; prices are illustrative. `extra` adds letters on swing points and
/// labelled Fibonacci lines (Stage 6).
pub fn swing_chart(swings: &[telosgard_core::stage5_drills::Swing], zones: &[(Price, Price)], broke_up: Option<bool>, extra: &SwingExtra, label: String) -> impl IntoView + use<> {
    const W: f64 = 520.0;
    const H: f64 = 220.0;
    const PLOT_L: f64 = 34.0;
    const PLOT_R: f64 = W - 84.0;
    const TOP: f64 = 14.0;
    const BOTTOM: f64 = H - 14.0;
    let mut lo = swings.iter().map(|s| s.price.0).min().unwrap_or(0);
    let mut hi = swings.iter().map(|s| s.price.0).max().unwrap_or(1);
    for z in zones {
        lo = lo.min(z.0.0);
        hi = hi.max(z.1.0);
    }
    let (lo, hi) = ((lo - 400) as f64, (hi + 400) as f64);
    let y = move |p: i64| TOP + (hi - p as f64) / (hi - lo).max(1.0) * (BOTTOM - TOP);
    let n = swings.len() + usize::from(broke_up.is_some());
    let x = move |i: usize| PLOT_L + (PLOT_R - PLOT_L) * i as f64 / (n.max(2) - 1) as f64;
    let points: Vec<(f64, f64)> = swings.iter().enumerate().map(|(i, s)| (x(i), y(s.price.0))).collect();
    let path = points.iter().map(|(a, b)| format!("{a:.1},{b:.1}")).collect::<Vec<_>>().join(" ");
    let breakout = broke_up.and_then(|up| {
        let last = swings.iter().rev().find(|s| s.high == up)?;
        let (lx, ly) = *points.last()?;
        let target = if up { last.price.0 + 600 } else { last.price.0 - 600 };
        Some(view! {
            <line x1=PLOT_L x2=PLOT_R y1=y(last.price.0) y2=y(last.price.0) class="stroke-text-muted" stroke-width="1" stroke-dasharray="4 4" />
            <line x1=lx x2=x(n - 1) y1=ly y2=y(target) class="stroke-text" stroke-width="2.5" stroke-dasharray="6 5" stroke-linecap="round" />
        })
    });
    let letters = ["A", "B", "C", "D"];
    let bands = zones
        .iter()
        .enumerate()
        .map(|(i, z)| {
            let (top, bottom) = (y(z.1.0), y(z.0.0));
            view! {
                <rect x=PLOT_L - 30.0 y=top width=PLOT_R - PLOT_L + 30.0 height=(bottom - top).max(3.0) class="fill-gold-fill/25 stroke-gold-fill/60" stroke-width="1" />
                <text x=PLOT_L - 24.0 y=(top + bottom) / 2.0 + 5.0 class="fill-text" font-size="15" font-weight="600">{letters[i % 4]}</text>
            }
        })
        .collect_view();
    let grid = (0..=3)
        .map(|i| {
            let price = Price(((lo + (hi - lo) * i as f64 / 3.0) / 100.0).round() as i64 * 100);
            let yy = y(price.0);
            view! {
                <line x1=PLOT_L x2=PLOT_R y1=yy y2=yy class="stroke-line/30" stroke-width="1" />
                <text x=PLOT_R + 8.0 y=yy + 5.0 class="fill-text-muted tabular" font-size="15">{axis_label(price)}</text>
            }
        })
        .collect_view();
    let dots = points.iter().map(|(a, b)| view! { <circle cx=*a cy=*b r="4" class="fill-text" /> }).collect_view();
    // Letters sit above a swing high and below a swing low.
    let letters_on = extra
        .points
        .iter()
        .filter_map(|(i, l)| {
            let (px, py) = *points.get(*i)?;
            let high = swings[*i].high;
            Some(view! { <text x=px y=if high { py - 10.0 } else { py + 22.0 } text-anchor="middle" class="fill-text" font-size="15" font-weight="700">{*l}</text> })
        })
        .collect_view();
    let fib_lines = extra
        .fibs
        .iter()
        .map(|(p, l)| {
            let yy = y(p.0);
            view! {
                <line x1=PLOT_L x2=PLOT_R y1=yy y2=yy class="stroke-gold" stroke-width="1.5" stroke-dasharray="5 4" />
                <text x=PLOT_R + 8.0 y=yy + 5.0 class="fill-gold tabular" font-size="14" font-weight="600">{l.clone()}</text>
            }
        })
        .collect_view();
    // With Fibonacci lines, their labels take the right-hand margin instead of the price grid.
    let grid = if extra.fibs.is_empty() { Some(grid) } else { None };
    view! {
        <svg viewBox=format!("0 0 {W} {H}") class="h-auto w-full max-w-[560px]" role="img" aria-label=label>
            {grid}
            {bands}
            {fib_lines}
            <polyline points=path fill="none" class="stroke-text" stroke-width="2.5" stroke-linejoin="round" stroke-linecap="round" />
            {dots}
            {letters_on}
            {breakout}
        </svg>
    }
}

/// Stage 6 additions to [`swing_chart`]: letters on swing points (by index) and Fibonacci lines.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwingExtra {
    pub points: Vec<(usize, &'static str)>,
    pub fibs: Vec<(Price, String)>,
}
