//! Lesson Widgets (FR-1.4): move one number, watch one result. Readouts state both prices and the
//! difference, never just a verdict. Anything that opens a trade asks for the stop first.

use crate::ui::controls::{PILL, Segmented};
use telosgard_core::money::{Price, Usd};
use leptos::prelude::*;

const BOX: &str = "mt-1 rounded-card border border-line/40 bg-bg p-4";
const RANGE: &str = "mt-3 w-full accent-[var(--color-text)]";

fn price(cents: i64) -> String {
    Price(cents).to_string().trim_start_matches('$').to_string()
}

fn signed(cents: i64) -> (String, &'static str, &'static str) {
    if cents > 0 {
        (format!("+{}", Usd(cents)), "text-up", "▲")
    } else if cents < 0 {
        (Usd(cents).to_string(), "text-down", "▼")
    } else {
        ("$0.00".into(), "text-text-muted", "•")
    }
}

/// A labelled range slider bound to an integer signal (`scale` turns the integer into the shown number).
#[component]
fn Slider(label: &'static str, value: RwSignal<i64>, min: i64, max: i64, step: i64, #[prop(into)] valuetext: Signal<String>) -> impl IntoView {
    view! {
        <input
            type="range"
            class=RANGE
            aria-label=label
            min=min
            max=max
            step=step
            aria-valuetext=move || valuetext.get()
            prop:value=move || value.get().to_string()
            on:input=move |ev| {
                if let Ok(v) = event_target_value(&ev).parse::<i64>() {
                    value.set(v)
                }
            }
        />
    }
}

#[component]
pub fn Widget(kind: &'static str) -> impl IntoView {
    let body = match kind {
        "price_drag" => price_drag().into_any(),
        "lots_money" => lots_money().into_any(),
        "one_percent" => one_percent().into_any(),
        "stop_first" => stop_first().into_any(),
        "short_risk" => short_risk().into_any(),
        "leverage_toggle" => leverage_toggle().into_any(),
        "bid_ask" => bid_ask().into_any(),
        "win_rate" => win_rate().into_any(),
        "pip_convert" => pip_convert().into_any(),
        "session_clock" => session_clock().into_any(),
        _ => ().into_any(),
    };
    view! {
        <figure class=BOX>
            <figcaption class="text-eyebrow text-eyebrow-ink">{if kind == "session_clock" { "Try it · drag the time" } else { "Try it · illustrative prices" }}</figcaption>
            {body}
        </figure>
    }
}

fn price_drag() -> impl IntoView {
    const BUY: i64 = 410_000;
    let sell = RwSignal::new(412_000i64);
    let text = Signal::derive(move || format!("Sell at {}", price(sell.get())));
    view! {
        <p class="mt-2 text-callout">"You bought gold at " <strong class="tabular">"4,100.00"</strong> ". Drag to where you sell:"</p>
        <Slider label="Sell price" value=sell min=405_000 max=415_000 step=50 valuetext=text />
        <p class="tabular mt-2 text-headline" aria-live="polite">
            {move || {
                let (d, tone, arrow) = signed(sell.get() - BUY);
                view! { "Buy 4,100.00 → sell " {price(sell.get())} " = " <span class=tone>{arrow} " " {d} " per ounce"</span> }
            }}
        </p>
    }
}

fn lots_money() -> impl IntoView {
    let lots = RwSignal::new(20i64); // hundredths
    let text = Signal::derive(move || format!("{:.2} lots", lots.get() as f64 / 100.0));
    view! {
        <p class="mt-2 text-callout">"Gold moves " <strong>"$4"</strong> " your way. Choose your size:"</p>
        <Slider label="Size in lots" value=lots min=1 max=100 step=1 valuetext=text />
        <p class="tabular mt-2 text-headline" aria-live="polite">
            {move || {
                let l = lots.get();
                let money = Usd(l * 100 * 400 / 100);
                format!("{:.2} lots × 100 oz × $4.00 = {}", l as f64 / 100.0, money)
            }}
        </p>
        <p class="mt-1 text-footnote text-text-muted">"Same move, bigger size: bigger result, in both directions."</p>
    }
}

fn one_percent() -> impl IntoView {
    let bal = RwSignal::new(10_000i64);
    let text = Signal::derive(move || Usd(bal.get() * 100).to_string());
    view! {
        <p class="mt-2 text-callout">"Your account balance:"</p>
        <Slider label="Account balance" value=bal min=500 max=50_000 step=500 valuetext=text />
        <p class="tabular mt-2 text-headline" aria-live="polite">
            {move || format!("1% of {} = {}", Usd(bal.get() * 100), Usd(bal.get()))}
        </p>
        <p class="mt-1 text-footnote text-text-muted">"The most you'd plan to lose on one trade. Size the trade so your stop costs about this much."</p>
    }
}

fn stop_first() -> impl IntoView {
    const ENTRY: i64 = 410_000;
    const OZ: i64 = 10; // 0.10 lots
    let stop = RwSignal::new(409_500i64);
    let set = RwSignal::new(false);
    let entered = RwSignal::new(false);
    let text = Signal::derive(move || format!("Stop at {}", price(stop.get())));
    Effect::new(move |prev: Option<i64>| {
        let s = stop.get();
        if prev.is_some_and(|p| p != s) {
            set.set(true);
            entered.set(false);
        }
        s
    });
    let loss = move || (ENTRY - stop.get()) * OZ;
    view! {
        <p class="mt-2 text-callout">"Buy 0.10 lots at " <strong class="tabular">"4,100.00"</strong> ". First, set your stop-loss:"</p>
        <Slider label="Stop-loss price" value=stop min=407_000 max=409_950 step=50 valuetext=text />
        <p class="tabular mt-2 text-callout" aria-live="polite">
            {move || format!("Stop at {}: if it's hit you lose {} per ounce, {} on this trade.", price(stop.get()), Usd(ENTRY - stop.get()), Usd(loss()))}
        </p>
        <button type="button" class=format!("{PILL} mt-3") disabled=move || !set.get() on:click=move |_| entered.set(true)>
            {move || if set.get() { "Enter the trade" } else { "Move the stop first" }}
        </button>
        {move || entered.get().then(|| view! {
            <p class="anim-fade mt-3 text-callout text-up" role="status">{format!("✓ Trade open with a stop at {}. The most it can lose is {}.", price(stop.get()), Usd(loss()))}</p>
        })}
    }
}

fn short_risk() -> impl IntoView {
    const SELL: i64 = 410_000;
    const OZ: i64 = 10;
    const ACCOUNT: i64 = 100_000; // $1,000
    let now = RwSignal::new(410_000i64);
    let text = Signal::derive(move || format!("Price {}", price(now.get())));
    view! {
        <p class="mt-2 text-callout">"You sold (went short) 0.10 lots at " <strong class="tabular">"4,100.00"</strong> " with no stop, on a $1,000 account. Drag the price up:"</p>
        <Slider label="Current price" value=now min=410_000 max=425_000 step=100 valuetext=text />
        {move || {
            let loss = (now.get() - SELL) * OZ;
            let gone = loss >= ACCOUNT;
            view! {
                <p class="tabular mt-2 text-headline" aria-live="polite">
                    "Price " {price(now.get())} ": " <span class="text-down">"▼ " {Usd(-loss.min(ACCOUNT)).to_string()}</span>
                </p>
                <p class=format!("mt-1 text-footnote {}", if gone { "text-down" } else { "text-text-muted" })>
                    {if gone {
                        "Your whole $1,000 is gone. The broker's margin close-out ends it here; with negative balance protection you can't lose more than the account."
                    } else {
                        "Price has no ceiling, so with no stop the loss keeps growing as long as price rises."
                    }}
                </p>
            }
        }}
    }
}

fn leverage_toggle() -> impl IntoView {
    const ACCOUNT: f64 = 1_000.0;
    const PRICE: f64 = 4_100.0;
    let lev = RwSignal::new(10u32);
    view! {
        <p class="mt-2 text-callout">"A $1,000 account with gold at 4,100. Choose your leverage:"</p>
        <div class="mt-3"><Segmented label="Leverage" value=lev options=vec![(1, "1×"), (10, "10×"), (50, "50×")] /></div>
        {move || {
            let oz = ACCOUNT * lev.get() as f64 / PRICE;
            let per20 = oz * 20.0;
            let gone_after = ACCOUNT / oz;
            view! {
                <dl class="tabular mt-3 grid gap-1 text-callout" aria-live="polite">
                    <div class="flex justify-between gap-3"><dt>"Position"</dt><dd>{format!("{oz:.2} oz (≈ {})", Usd((oz * PRICE * 100.0).round() as i64))}</dd></div>
                    <div class="flex justify-between gap-3"><dt>"Gold moves $20 your way"</dt><dd class="text-up">{format!("▲ +${per20:.2} ({:.1}%)", per20 / ACCOUNT * 100.0)}</dd></div>
                    <div class="flex justify-between gap-3"><dt>"Gold moves $20 against you"</dt><dd class="text-down">{format!("▼ −${per20:.2} ({:.1}%)", per20 / ACCOUNT * 100.0)}</dd></div>
                    <div class="flex justify-between gap-3"><dt>"Account wiped out after a move against you of"</dt><dd class="text-down">{format!("${gone_after:.2}")}</dd></div>
                </dl>
            }
        }}
        <p class="mt-2 text-footnote text-text-muted">"Leverage changes the size you can open, so it magnifies losses exactly as much as gains."</p>
    }
}

fn bid_ask() -> impl IntoView {
    let buy = RwSignal::new(true);
    view! {
        <p class="tabular mt-2 text-headline">"Bid (sell) 4,150.20 · Ask (buy) 4,150.50"</p>
        <div class="mt-3"><Segmented label="Action" value=buy options=vec![(true, "▲ Buy"), (false, "▼ Sell")] /></div>
        <p class="tabular mt-3 text-callout" aria-live="polite">
            {move || if buy.get() {
                "You buy at the ask: 4,150.50. Selling straight back at the bid (4,150.20) would lose $0.30 per ounce: the spread."
            } else {
                "You sell at the bid: 4,150.20. Buying straight back at the ask (4,150.50) would lose $0.30 per ounce: the spread."
            }}
        </p>
    }
}

fn win_rate() -> impl IntoView {
    let rate = RwSignal::new(40i64);
    let text = Signal::derive(move || format!("{}% of trades win", rate.get()));
    view! {
        <p class="mt-2 text-callout">"Every win is +2R (twice the risk), every loss −1R. Risk $100 a trade. Set your win rate:"</p>
        <Slider label="Win rate" value=rate min=20 max=80 step=5 valuetext=text />
        {move || {
            let w = rate.get();
            // Expectancy per trade in hundredths of R: w×2 − (1−w)×1 = 3w − 1.
            let exp_r100 = 3 * w - 100;
            let (total, tone, arrow) = signed(exp_r100 * 100 * 100); // 100 trades × $100 per R, in cents
            view! {
                <p class="tabular mt-2 text-headline" aria-live="polite">
                    {format!("Win {w}% → {}{:.2}R per trade → over 100 trades: ", if exp_r100 >= 0 { "+" } else { "−" }, exp_r100.abs() as f64 / 100.0)}
                    <span class=tone>{arrow} " " {total}</span>
                </p>
            }
        }}
        <p class="mt-1 text-footnote text-text-muted">"With wins twice the size of losses, the break-even win rate is about 33%."</p>
    }
}

fn pip_convert() -> impl IntoView {
    let size = RwSignal::new(10i64); // cents per pip
    view! {
        <p class="mt-2 text-callout">"Someone says “a 50-pip stop on gold”. What do they mean by a pip?"</p>
        <div class="mt-3"><Segmented label="Their pip size" value=size options=vec![(1, "$0.01"), (10, "$0.10"), (100, "$1")] /></div>
        <p class="tabular mt-3 text-headline" aria-live="polite">
            {move || format!("50 pips × {} = a {} Dollar Move", Usd(size.get()), Usd(50 * size.get()))}
        </p>
    }
}

/// Drag a time across one New York weekday and see every city's clock, the sessions open and the
/// rollover window. Uses the same session rules as the app's clock and Hard Rules.
fn session_clock() -> impl IntoView {
    use telosgard_core::stage2_drills::{City, SessionAnswer, city_to_utc};
    use telosgard_core::time::{london, no_trade_window, ny, sessions_at};
    let minute = RwSignal::new(10 * 60i64); // New York minutes since midnight
    let summer = RwSignal::new(false);
    let utc = move || if summer.get() { city_to_utc(City::NewYork, 2026, 7, 15, minute.get() as u32) } else { city_to_utc(City::NewYork, 2026, 11, 18, minute.get() as u32) };
    let hm = |m: u32| format!("{:02}:{:02}", m / 60, m % 60);
    let text = Signal::derive(move || format!("{} New York time", hm(minute.get() as u32)));
    view! {
        <p class="mt-2 text-callout">"A Wednesday. Drag through the New York day:"</p>
        <div class="mt-3"><Segmented label="Season" value=summer options=vec![(false, "November"), (true, "July")] /></div>
        <Slider label="New York time" value=minute min=0 max=1410 step=30 valuetext=text />
        {move || {
            let u = utc();
            let tokyo = ((u.rem_euclid(telosgard_core::DAY_MS) / 60_000) as u32 + 9 * 60) % 1440;
            let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(u as f64));
            let mine = format!("{:02}:{:02}", d.get_hours(), d.get_minutes());
            let session = SessionAnswer::from_sessions(&sessions_at(u));
            let window = no_trade_window(u, &[]);
            let (status, tone) = match (&window, session) {
                (Some(telosgard_core::time::NoTrade::Rollover), _) => ("No-Trade Window: the daily rollover (5–6 pm New York)", "bg-bad-bg text-bad-ink"),
                (_, SessionAnswer::Overlap) => ("London and New York: the overlap", "bg-ok-bg text-ok-ink"),
                (_, SessionAnswer::NoneOpen) => ("Between sessions: quiet", "bg-raised text-text-muted"),
                (_, s) => (s.label(), "bg-warn-bg text-warn-ink"),
            };
            view! {
                <dl class="tabular mt-3 grid grid-cols-2 gap-x-4 gap-y-1 text-callout sm:grid-cols-4" aria-live="polite">
                    <div><dt class="text-footnote text-text-muted">"New York"</dt><dd class="font-semibold">{hm(ny(u).minute)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">"London"</dt><dd class="font-semibold">{hm(london(u).minute)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">"Tokyo"</dt><dd class="font-semibold">{hm(tokyo)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">"You"</dt><dd class="font-semibold">{mine}</dd></div>
                </dl>
                <p class=format!("mt-3 inline-block rounded-full px-3 py-1 text-footnote font-semibold {tone}")>{status}</p>
            }
        }}
    }
}
