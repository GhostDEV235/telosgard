//! Lesson Widgets (FR-1.4): move one number, watch one result. Readouts state both prices and the
//! difference, never just a verdict. Anything that opens a trade asks for the stop first.

use crate::i18n::{t, ts};
use crate::ui::controls::{PILL, Segmented};
use telosgard_core::money::{Lots, Price, Usd};
use leptos::prelude::*;

const BOX: &str = "mt-1 rounded-card border border-line/40 bg-bg p-4";
const RANGE: &str = "mt-3 w-full accent-[var(--color-text)]";

/// A quote without the dollar sign: "4,100.00" or, in French, "4 100,00".
fn price(cents: i64) -> String {
    Price(cents).to_string().trim_start_matches('$').trim_end_matches('$').trim_end_matches('\u{a0}').to_string()
}

/// Two decimals with the language's decimal mark.
fn dec2(v: f64) -> String {
    let s = format!("{v:.2}");
    if crate::i18n::is_fr() { s.replace('.', ",") } else { s }
}

/// A percentage with one decimal: "4.9%" or "4,9 %".
fn pct1(v: f64) -> String {
    let s = format!("{v:.1}");
    if crate::i18n::is_fr() { format!("{}\u{a0}%", s.replace('.', ",")) } else { format!("{s}%") }
}

fn signed(cents: i64) -> (String, &'static str, &'static str) {
    if cents > 0 {
        (format!("+{}", Usd(cents)), "text-up", "▲")
    } else if cents < 0 {
        (Usd(cents).to_string(), "text-down", "▼")
    } else {
        (Usd(0).to_string(), "text-text-muted", "•")
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
        "surprise_meter" => surprise_meter().into_any(),
        "candle_builder" => candle_builder().into_any(),
        "candle_merge" => candle_merge().into_any(),
        _ => ().into_any(),
    };
    view! {
        <figure class=BOX>
            <figcaption class="text-eyebrow text-eyebrow-ink">{if kind == "session_clock" { t("Try it · drag the time", "Essaie · fais glisser l’heure") } else { t("Try it · illustrative prices", "Essaie · prix d’exemple") }}</figcaption>
            {body}
        </figure>
    }
}

fn price_drag() -> impl IntoView {
    const BUY: i64 = 410_000;
    let sell = RwSignal::new(412_000i64);
    let text = Signal::derive(move || ts(format!("Sell at {}", price(sell.get())), format!("Vente à {}", price(sell.get()))));
    view! {
        <p class="mt-2 text-callout">{t("You bought gold at ", "Tu as acheté de l’or à ")} <strong class="tabular">{price(410_000)}</strong> {t(". Drag to where you sell:", ". Fais glisser jusqu’à ton prix de vente :")}</p>
        <Slider label=t("Sell price", "Prix de vente") value=sell min=405_000 max=415_000 step=50 valuetext=text />
        <p class="tabular mt-2 text-headline" aria-live="polite">
            {move || {
                let (d, tone, arrow) = signed(sell.get() - BUY);
                view! { {t("Buy ", "Achat ")} {price(BUY)} {t(" → sell ", " → vente ")} {price(sell.get())} " = " <span class=tone>{arrow} " " {d} {t(" per ounce", " par once")}</span> }
            }}
        </p>
    }
}

fn lots_money() -> impl IntoView {
    let lots = RwSignal::new(20i64); // hundredths
    let text = Signal::derive(move || ts(format!("{} lots", Lots(lots.get())), format!("{} lot", Lots(lots.get()))));
    view! {
        <p class="mt-2 text-callout">{t("Gold moves ", "L’or bouge de ")} <strong>{t("$4", "4 $")}</strong> {t(" your way. Choose your size:", " dans ton sens. Choisis ta taille :")}</p>
        <Slider label=t("Size in lots", "Taille en lots") value=lots min=1 max=100 step=1 valuetext=text />
        <p class="tabular mt-2 text-headline" aria-live="polite">
            {move || {
                let l = lots.get();
                let money = Usd(l * 100 * 400 / 100);
                ts(format!("{} lots × 100 oz × $4.00 = {}", Lots(l), money), format!("{} lot × 100 oz × 4,00 $ = {}", Lots(l), money))
            }}
        </p>
        <p class="mt-1 text-footnote text-text-muted">{t("Same move, bigger size: bigger result, in both directions.", "Même mouvement, plus grosse taille : plus gros résultat, dans les deux sens.")}</p>
    }
}

fn one_percent() -> impl IntoView {
    let bal = RwSignal::new(10_000i64);
    let text = Signal::derive(move || Usd(bal.get() * 100).to_string());
    view! {
        <p class="mt-2 text-callout">{t("Your account balance:", "Le solde de ton compte :")}</p>
        <Slider label=t("Account balance", "Solde du compte") value=bal min=500 max=50_000 step=500 valuetext=text />
        <p class="tabular mt-2 text-headline" aria-live="polite">
            {move || ts(format!("1% of {} = {}", Usd(bal.get() * 100), Usd(bal.get())), format!("1 % de {} = {}", Usd(bal.get() * 100), Usd(bal.get())))}
        </p>
        <p class="mt-1 text-footnote text-text-muted">{t("The most you'd plan to lose on one trade. Size the trade so your stop costs about this much.", "Le maximum que tu prévois de perdre sur un trade. Choisis la taille pour que ton stop coûte à peu près ça.")}</p>
    }
}

fn stop_first() -> impl IntoView {
    const ENTRY: i64 = 410_000;
    const OZ: i64 = 10; // 0.10 lots
    let stop = RwSignal::new(409_500i64);
    let set = RwSignal::new(false);
    let entered = RwSignal::new(false);
    let text = Signal::derive(move || ts(format!("Stop at {}", price(stop.get())), format!("Stop à {}", price(stop.get()))));
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
        <p class="mt-2 text-callout">{t("Buy 0.10 lots at ", "Achat de 0,10 lot à ")} <strong class="tabular">{price(ENTRY)}</strong> {t(". First, set your stop-loss:", ". D’abord, place ton stop-loss :")}</p>
        <Slider label=t("Stop-loss price", "Prix du stop-loss") value=stop min=407_000 max=409_950 step=50 valuetext=text />
        <p class="tabular mt-2 text-callout" aria-live="polite">
            {move || ts(format!("Stop at {}: if it's hit you lose {} per ounce, {} on this trade.", price(stop.get()), Usd(ENTRY - stop.get()), Usd(loss())), format!("Stop à {} : s’il est touché, tu perds {} par once, soit {} sur ce trade.", price(stop.get()), Usd(ENTRY - stop.get()), Usd(loss())))}
        </p>
        <button type="button" class=format!("{PILL} mt-3") disabled=move || !set.get() on:click=move |_| entered.set(true)>
            {move || if set.get() { t("Enter the trade", "Entrer dans le trade") } else { t("Move the stop first", "Déplace d’abord le stop") }}
        </button>
        {move || entered.get().then(|| view! {
            <p class="anim-fade mt-3 text-callout text-up" role="status">{ts(format!("✓ Trade open with a stop at {}. The most it can lose is {}.", price(stop.get()), Usd(loss())), format!("✓ Trade ouvert avec un stop à {}. Il peut perdre au maximum {}.", price(stop.get()), Usd(loss())))}</p>
        })}
    }
}

fn short_risk() -> impl IntoView {
    const SELL: i64 = 410_000;
    const OZ: i64 = 10;
    const ACCOUNT: i64 = 100_000; // $1,000
    let now = RwSignal::new(410_000i64);
    let text = Signal::derive(move || ts(format!("Price {}", price(now.get())), format!("Prix {}", price(now.get()))));
    view! {
        <p class="mt-2 text-callout">{t("You sold (went short) 0.10 lots at ", "Tu as vendu (position courte) 0,10 lot à ")} <strong class="tabular">{price(SELL)}</strong> {t(" with no stop, on a $1,000 account. Drag the price up:", " sans stop, sur un compte de 1 000 $. Fais monter le prix :")}</p>
        <Slider label=t("Current price", "Prix actuel") value=now min=410_000 max=425_000 step=100 valuetext=text />
        {move || {
            let loss = (now.get() - SELL) * OZ;
            let gone = loss >= ACCOUNT;
            view! {
                <p class="tabular mt-2 text-headline" aria-live="polite">
                    {t("Price ", "Prix ")} {price(now.get())} {t(": ", " : ")} <span class="text-down">"▼ " {Usd(-loss.min(ACCOUNT)).to_string()}</span>
                </p>
                <p class=format!("mt-1 text-footnote {}", if gone { "text-down" } else { "text-text-muted" })>
                    {if gone {
                        t("Your whole $1,000 is gone. The broker's margin close-out ends it here; with negative balance protection you can't lose more than the account.", "Tes 1 000 $ sont partis en entier. La clôture forcée du courtier s’arrête là ; avec la protection contre le solde négatif, tu ne peux pas perdre plus que le compte.")
                    } else {
                        t("Price has no ceiling, so with no stop the loss keeps growing as long as price rises.", "Le prix n’a pas de plafond : sans stop, la perte continue de grossir tant que le prix monte.")
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
        <p class="mt-2 text-callout">{t("A $1,000 account with gold at 4,100. Choose your leverage:", "Un compte de 1 000 $ avec l’or à 4 100. Choisis ton effet de levier :")}</p>
        <div class="mt-3"><Segmented label=t("Leverage", "Effet de levier") value=lev options=vec![(1, "1×"), (10, "10×"), (50, "50×")] /></div>
        {move || {
            let oz = ACCOUNT * lev.get() as f64 / PRICE;
            let per20 = oz * 20.0;
            let gone_after = ACCOUNT / oz;
            view! {
                <dl class="tabular mt-3 grid gap-1 text-callout" aria-live="polite">
                    <div class="flex justify-between gap-3"><dt>"Position"</dt><dd>{format!("{} oz (≈ {})", dec2(oz), Usd((oz * PRICE * 100.0).round() as i64))}</dd></div>
                    <div class="flex justify-between gap-3"><dt>{t("Gold moves $20 your way", "L’or bouge de 20 $ dans ton sens")}</dt><dd class="text-up">{format!("▲ +{} ({})", Usd((per20 * 100.0).round() as i64), pct1(per20 / ACCOUNT * 100.0))}</dd></div>
                    <div class="flex justify-between gap-3"><dt>{t("Gold moves $20 against you", "L’or bouge de 20 $ contre toi")}</dt><dd class="text-down">{format!("▼ {} ({})", Usd(-(per20 * 100.0).round() as i64), pct1(per20 / ACCOUNT * 100.0))}</dd></div>
                    <div class="flex justify-between gap-3"><dt>{t("Account wiped out after a move against you of", "Compte vidé après un mouvement contre toi de")}</dt><dd class="text-down">{Usd((gone_after * 100.0).round() as i64).to_string()}</dd></div>
                </dl>
            }
        }}
        <p class="mt-2 text-footnote text-text-muted">{t("Leverage changes the size you can open, so it magnifies losses exactly as much as gains.", "L’effet de levier change la taille que tu peux ouvrir : il grossit donc les pertes exactement autant que les gains.")}</p>
    }
}

fn bid_ask() -> impl IntoView {
    let buy = RwSignal::new(true);
    view! {
        <p class="tabular mt-2 text-headline">{ts(format!("Bid (sell) {} · Ask (buy) {}", price(415_020), price(415_050)), format!("Bid (vente) {} · Ask (achat) {}", price(415_020), price(415_050)))}</p>
        <div class="mt-3"><Segmented label=t("Action", "Action") value=buy options=vec![(true, t("▲ Buy", "▲ Acheter")), (false, t("▼ Sell", "▼ Vendre"))] /></div>
        <p class="tabular mt-3 text-callout" aria-live="polite">
            {move || if buy.get() {
                t("You buy at the ask: 4,150.50. Selling straight back at the bid (4,150.20) would lose $0.30 per ounce: the spread.", "Tu achètes à l’ask : 4 150,50. Revendre aussitôt au bid (4 150,20) ferait perdre 0,30 $ par once : c’est le spread.")
            } else {
                t("You sell at the bid: 4,150.20. Buying straight back at the ask (4,150.50) would lose $0.30 per ounce: the spread.", "Tu vends au bid : 4 150,20. Racheter aussitôt à l’ask (4 150,50) ferait perdre 0,30 $ par once : c’est le spread.")
            }}
        </p>
    }
}

fn win_rate() -> impl IntoView {
    let rate = RwSignal::new(40i64);
    let text = Signal::derive(move || ts(format!("{}% of trades win", rate.get()), format!("{} % de trades gagnants", rate.get())));
    view! {
        <p class="mt-2 text-callout">{t("Every win is +2R (twice the risk), every loss −1R. Risk $100 a trade. Set your win rate:", "Chaque gain vaut +2R (deux fois le risque), chaque perte −1R. Risque 100 $ par trade. Règle ton taux de réussite :")}</p>
        <Slider label=t("Win rate", "Taux de réussite") value=rate min=20 max=80 step=5 valuetext=text />
        {move || {
            let w = rate.get();
            // Expectancy per trade in hundredths of R: w×2 − (1−w)×1 = 3w − 1.
            let exp_r100 = 3 * w - 100;
            let (total, tone, arrow) = signed(exp_r100 * 100 * 100); // 100 trades × $100 per R, in cents
            view! {
                <p class="tabular mt-2 text-headline" aria-live="polite">
                    {ts(format!("Win {w}% → {}{}R per trade → over 100 trades: ", if exp_r100 >= 0 { "+" } else { "−" }, dec2(exp_r100.abs() as f64 / 100.0)), format!("Réussite {w} % → {}{}R par trade → sur 100 trades : ", if exp_r100 >= 0 { "+" } else { "−" }, dec2(exp_r100.abs() as f64 / 100.0)))}
                    <span class=tone>{arrow} " " {total}</span>
                </p>
            }
        }}
        <p class="mt-1 text-footnote text-text-muted">{t("With wins twice the size of losses, the break-even win rate is about 33%.", "Avec des gains deux fois plus gros que les pertes, le taux de réussite d’équilibre est d’environ 33 %.")}</p>
    }
}

fn pip_convert() -> impl IntoView {
    let size = RwSignal::new(10i64); // cents per pip
    view! {
        <p class="mt-2 text-callout">{t("Someone says “a 50-pip stop on gold”. What do they mean by a pip?", "Quelqu’un parle d’un « stop de 50 pips sur l’or ». Que veut-il dire par pip ?")}</p>
        <div class="mt-3"><Segmented label=t("Their pip size", "Sa taille de pip") value=size options=vec![(1, t("$0.01", "0,01 $")), (10, t("$0.10", "0,10 $")), (100, t("$1", "1 $"))] /></div>
        <p class="tabular mt-3 text-headline" aria-live="polite">
            {move || ts(format!("50 pips × {} = a {} Dollar Move", Usd(size.get()), Usd(50 * size.get())), format!("50 pips × {} = un Mouvement en dollars de {}", Usd(size.get()), Usd(50 * size.get())))}
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
    let text = Signal::derive(move || ts(format!("{} New York time", hm(minute.get() as u32)), format!("{}, heure de New York", hm(minute.get() as u32))));
    view! {
        <p class="mt-2 text-callout">{t("A Wednesday. Drag through the New York day:", "Un mercredi. Fais défiler la journée de New York :")}</p>
        <div class="mt-3"><Segmented label=t("Season", "Saison") value=summer options=vec![(false, t("November", "Novembre")), (true, t("July", "Juillet"))] /></div>
        <Slider label=t("New York time", "Heure de New York") value=minute min=0 max=1410 step=30 valuetext=text />
        {move || {
            let u = utc();
            let tokyo = ((u.rem_euclid(telosgard_core::DAY_MS) / 60_000) as u32 + 9 * 60) % 1440;
            let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(u as f64));
            let mine = format!("{:02}:{:02}", d.get_hours(), d.get_minutes());
            let session = SessionAnswer::from_sessions(&sessions_at(u));
            let window = no_trade_window(u, &[]);
            let (status, tone) = match (&window, session) {
                (Some(telosgard_core::time::NoTrade::Rollover), _) => (t("No-Trade Window: the daily rollover (5–6 pm New York)", "Fenêtre sans trade : le rollover quotidien (17 h–18 h à New York)"), "bg-bad-bg text-bad-ink"),
                (_, SessionAnswer::Overlap) => (t("London and New York: the overlap", "Londres et New York : le chevauchement"), "bg-ok-bg text-ok-ink"),
                (_, SessionAnswer::NoneOpen) => (t("Between sessions: quiet", "Entre deux séances : calme"), "bg-raised text-text-muted"),
                (_, s) => (s.label(), "bg-warn-bg text-warn-ink"),
            };
            view! {
                <dl class="tabular mt-3 grid grid-cols-2 gap-x-4 gap-y-1 text-callout sm:grid-cols-4" aria-live="polite">
                    <div><dt class="text-footnote text-text-muted">"New York"</dt><dd class="font-semibold">{hm(ny(u).minute)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">{t("London", "Londres")}</dt><dd class="font-semibold">{hm(london(u).minute)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">"Tokyo"</dt><dd class="font-semibold">{hm(tokyo)}</dd></div>
                    <div><dt class="text-footnote text-text-muted">{t("You", "Toi")}</dt><dd class="font-semibold">{mine}</dd></div>
                </dl>
                <p class=format!("mt-3 inline-block rounded-full px-3 py-1 text-footnote font-semibold {tone}")>{status}</p>
            }
        }}
    }
}

/// A CPI release with a fixed forecast: move the actual number and see hot, soft or in line, and
/// gold's usual reaction. Same rule as the "Hot or soft?" drill.
fn surprise_meter() -> impl IntoView {
    use telosgard_core::stage3_drills::{Release, Surprise, judge};
    const FORECAST: i64 = 30; // 0.3%, in hundredths
    let actual = RwSignal::new(50i64);
    let r = Release::CpiMonthly;
    let text = Signal::derive(move || ts(format!("Actual {}", r.show(actual.get())), format!("Réel {}", r.show(actual.get()))));
    view! {
        <p class="mt-2 text-callout">{ts(format!("{}. Forecast: {}. Move the actual number:", r.name(), r.show(FORECAST)), format!("{}. Prévision : {}. Fais varier le chiffre réel :", r.name(), r.show(FORECAST)))}</p>
        <Slider label=t("Actual", "Réel") value=actual min=0 max=60 step=10 valuetext=text />
        {move || {
            let a = actual.get();
            let (verdict, tone, reaction) = match judge(r, FORECAST, a) {
                Surprise::Hot => (t("Hot", "Chaude"), "bg-bad-bg text-bad-ink", t("Traders expect higher rates for longer: gold usually falls.", "Les traders anticipent des taux plus élevés plus longtemps : l’or baisse en général.")),
                Surprise::Soft => (t("Soft", "Molle"), "bg-ok-bg text-ok-ink", t("Traders expect lower rates: gold usually rises.", "Les traders anticipent des taux plus bas : l’or monte en général.")),
                Surprise::InLine => (t("In line", "Conforme"), "bg-raised text-text-muted", t("Already expected: no clear push either way.", "Déjà attendu : pas de poussée nette dans un sens ou dans l’autre.")),
            };
            view! {
                <p class="tabular mt-2 text-headline" aria-live="polite">
                    {ts(format!("Actual {} vs forecast {}: ", r.show(a), r.show(FORECAST)), format!("Réel {} contre prévision {} : ", r.show(a), r.show(FORECAST)))}
                    <span class=format!("rounded-full px-2.5 py-0.5 text-callout font-semibold {tone}")>{verdict}</span>
                </p>
                <p class="mt-1 text-callout">{reaction}</p>
            }
        }}
        <p class="mt-1 text-footnote text-text-muted">{t("Usually, not always: the details and revisions can flip the reaction. And never trade the release candle.", "En général, pas toujours : les détails et les révisions peuvent inverser la réaction. Et ne trade jamais la bougie de la publication.")}</p>
    }
}

/// Lesson 21: the open, high and low are fixed; drag the close and watch the body and the winner change.
fn candle_builder() -> impl IntoView {
    use telosgard_core::drills::Candle;
    use telosgard_core::stage4_drills::{Winner, wicks};
    const OPEN: i64 = 410_000;
    const HIGH: i64 = 411_200;
    const LOW: i64 = 409_400;
    let close = RwSignal::new(410_800i64);
    let text = Signal::derive(move || ts(format!("Close {}", price(close.get())), format!("Clôture {}", price(close.get()))));
    view! {
        <p class="mt-2 text-callout">{ts(format!("One 1-hour candle: open {}, high {}, low {}. Move the close:", price(OPEN), price(HIGH), price(LOW)), format!("Une bougie 1 heure : ouverture {}, plus haut {}, plus bas {}. Fais varier la clôture :", price(OPEN), price(HIGH), price(LOW)))}</p>
        <Slider label=t("Close", "Clôture") value=close min=LOW max=HIGH step=100 valuetext=text />
        {move || {
            let c = Candle { open: Price(OPEN), high: Price(HIGH), low: Price(LOW), close: Price(close.get()) };
            let (up, down) = wicks(&c);
            let body = c.close.0 - c.open.0;
            let (who, tone) = match Winner::of(&c) {
                Winner::Buyers => (t("Buyers won the hour", "Les acheteurs ont gagné l’heure"), "bg-ok-bg text-ok-ink"),
                Winner::Sellers => (t("Sellers won the hour", "Les vendeurs ont gagné l’heure"), "bg-bad-bg text-bad-ink"),
                Winner::Draw => (t("A draw (doji)", "Match nul (doji)"), "bg-raised text-text-muted"),
            };
            let (b, _, _) = signed(body);
            view! {
                <div class="mt-3 grid items-center gap-4 sm:grid-cols-[minmax(0,180px)_minmax(0,1fr)]">
                    {crate::ui::chart::candle_strip(&[c], Price(LOW - 100), Price(HIGH + 100), t("The candle you built", "La bougie que tu as construite").into())}
                    <div aria-live="polite">
                        <p><span class=format!("rounded-full px-2.5 py-0.5 text-callout font-semibold {tone}")>{who}</span></p>
                        <p class="tabular mt-2 text-callout">{ts(format!("Body: {} → {} ({b})", price(OPEN), price(c.close.0)), format!("Corps : {} → {} ({b})", price(OPEN), price(c.close.0)))}</p>
                        <p class="tabular text-callout">{ts(format!("Upper wick {} · lower wick {}", Usd(up), Usd(down)), format!("Mèche haute {} · mèche basse {}", Usd(up), Usd(down)))}</p>
                    </div>
                </div>
            }
        }}
        <p class="mt-1 text-footnote text-text-muted">{t("The high and low stay put: only the close decides who won.", "Le plus haut et le plus bas ne bougent pas : seule la clôture décide qui a gagné.")}</p>
    }
}

/// Lesson 22: four 15-minute candles, or the one 1-hour candle they make.
fn candle_merge() -> impl IntoView {
    use telosgard_core::drills::Candle;
    use telosgard_core::stage4_drills::merge;
    let c = |o: i64, h: i64, l: i64, cl: i64| Candle { open: Price(o * 100), high: Price(h * 100), low: Price(l * 100), close: Price(cl * 100) };
    let quarter = [c(4100, 4105, 4098, 4104), c(4104, 4112, 4102, 4109), c(4109, 4110, 4099, 4101), c(4101, 4106, 4100, 4105)];
    let hour = merge(&quarter).expect("four candles");
    #[derive(Clone, Copy, PartialEq)]
    enum View {
        Quarter,
        Hour,
    }
    let view_as = RwSignal::new(View::Quarter);
    let (lo, hi) = (Price(409_700), Price(411_300));
    view! {
        <div class="mt-3">
            <Segmented label=t("Timeframe", "Unité de temps") value=view_as options=vec![(View::Quarter, t("4 × M15", "4 × M15")), (View::Hour, t("1 × H1", "1 × H1"))] />
        </div>
        <div class="mt-3">
            {move || match view_as.get() {
                View::Quarter => crate::ui::chart::candle_strip(&quarter, lo, hi, t("Four 15-minute candles", "Quatre bougies de 15 minutes").into()).into_any(),
                View::Hour => crate::ui::chart::candle_strip(&[hour], lo, hi, t("The 1-hour candle they make", "La bougie de 1 heure qu’elles forment").into()).into_any(),
            }}
        </div>
        <p class="tabular mt-2 text-callout" aria-live="polite">
            {ts(
                format!("The hour: open {} (first open), high {} (highest), low {} (lowest), close {} (last close).", price(hour.open.0), price(hour.high.0), price(hour.low.0), price(hour.close.0)),
                format!("L’heure : ouverture {} (première ouverture), plus haut {} (le plus haut), plus bas {} (le plus bas), clôture {} (dernière clôture).", price(hour.open.0), price(hour.high.0), price(hour.low.0), price(hour.close.0)),
            )}
        </p>
        <p class="mt-1 text-footnote text-text-muted">{t("Same trades, grouped differently: the dip inside the hour disappears into the 1-hour candle's lower wick.", "Les mêmes échanges, regroupés autrement : le creux à l’intérieur de l’heure disparaît dans la mèche basse de la bougie de 1 heure.")}</p>
    }
}
