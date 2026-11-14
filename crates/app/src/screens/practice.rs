//! Practice tab: Chart Drills. Phase 3 ships the "Bullish or bearish?" drill; the rest come in Phase 7.

use crate::catalog::Tx;
use crate::i18n::{t, ts};
use crate::screens::shell::{LargeTitle, SubBar};
use crate::state::now;
use crate::ui::chart::CandleChart;
use crate::ui::controls::PILL;
use crate::router::DrillKind;
use telosgard_core::drills::{Candle, CandleDirection, Rng, check_direction, generate};
use telosgard_core::stage1_drills::{BidAskAsk, BidAskQ, DistanceQ, OrderKind, OrderQ, PipQ};
use telosgard_core::money::{DollarMove, Price};
use leptos::prelude::*;

struct DrillInfo {
    kind: DrillKind,
    title: Tx,
    blurb: Tx,
    lesson: Tx,
}

const DRILLS: [DrillInfo; 13] = [
    DrillInfo { kind: DrillKind::Calibration, title: Tx("Can you call the next candle?", "Peux-tu deviner la prochaine bougie ?"), blurb: Tx("Ten charts: predict up or down, then see your hit rate.", "Dix graphiques : prédis hausse ou baisse, puis découvre ton taux de réussite."), lesson: Tx("Lesson 2", "Leçon 2") },
    DrillInfo { kind: DrillKind::Distance, title: Tx("Measure the move", "Mesure le mouvement"), blurb: Tx("Turn two prices into a Dollar Move.", "Transforme deux prix en Mouvement en dollars."), lesson: Tx("Lesson 4", "Leçon 4") },
    DrillInfo { kind: DrillKind::Direction, title: Tx("Bullish or bearish?", "Haussière ou baissière ?"), blurb: Tx("Read a candle's direction from its open and close.", "Lis le sens d’une bougie à partir de son ouverture et de sa clôture."), lesson: Tx("Lesson 5", "Leçon 5") },
    DrillInfo { kind: DrillKind::BidAsk, title: Tx("Spot the spread", "Repère le spread"), blurb: Tx("Find the bid, the ask, and the cost of getting in.", "Trouve le bid, l’ask, et ce que coûte l’entrée."), lesson: Tx("Lesson 6", "Leçon 6") },
    DrillInfo { kind: DrillKind::Pips, title: Tx("Translate the pips", "Traduis les pips"), blurb: Tx("Convert someone else's “pips” into dollars.", "Convertis les « pips » des autres en dollars."), lesson: Tx("Lesson 7", "Leçon 7") },
    DrillInfo { kind: DrillKind::Orders, title: Tx("Pick the order", "Choisis l’ordre"), blurb: Tx("Market, limit or stop: match the order to the intent.", "Au marché, limite ou stop : associe l’ordre à l’intention."), lesson: Tx("Lesson 10", "Leçon 10") },
    DrillInfo { kind: DrillKind::Session, title: Tx("Which session is it?", "Quelle séance est-ce ?"), blurb: Tx("Turn a time in New York or London into the session that's open.", "Transforme une heure de New York ou de Londres en séance ouverte."), lesson: Tx("Lesson 13", "Leçon 13") },
    DrillInfo { kind: DrillKind::NoTrade, title: Tx("Is this a No-Trade Window?", "Est-ce une Fenêtre sans trade ?"), blurb: Tx("Spot the rollover, the Sunday open and the minutes around big releases.", "Repère le rollover, l’ouverture du dimanche et les minutes autour des grandes annonces."), lesson: Tx("Lesson 15", "Leçon 15") },
    DrillInfo { kind: DrillKind::Surprise, title: Tx("Hot or soft?", "Chaude ou molle ?"), blurb: Tx("Compare a release with its forecast and say which way gold usually goes.", "Compare une statistique à sa prévision et dis dans quel sens l’or va en général."), lesson: Tx("Lesson 19", "Leçon 19") },
    DrillInfo { kind: DrillKind::BuildCandle, title: Tx("Build the candle", "Construis la bougie"), blurb: Tx("Turn an open, high, low and close into the right candle, then say who won it.", "Transforme une ouverture, un plus haut, un plus bas et une clôture en la bonne bougie, puis dis qui l’a gagnée."), lesson: Tx("Lesson 21", "Leçon 21") },
    DrillInfo { kind: DrillKind::FindLevel, title: Tx("Find the level", "Trouve le niveau"), blurb: Tx("Spot the zone price has reacted to at least twice.", "Repère la zone sur laquelle le prix a réagi au moins deux fois."), lesson: Tx("Lesson 24", "Leçon 24") },
    DrillInfo { kind: DrillKind::NameStructure, title: Tx("Name the structure", "Nomme la structure"), blurb: Tx("Uptrend, downtrend or range, then BOS or CHoCH.", "Hausse, baisse ou range, puis BOS ou CHoCH."), lesson: Tx("Lesson 25", "Leçon 25") },
    DrillInfo { kind: DrillKind::NameCandle, title: Tx("Name the candle", "Nomme la bougie"), blurb: Tx("Pin bar, engulfing, inside bar, or nothing at all.", "Pin bar, avalement, inside bar, ou rien du tout."), lesson: Tx("Lesson 26", "Leçon 26") },
];

#[component]
pub fn Practice() -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    view! {
        <main class="gutter mx-auto max-w-[1040px] pb-8">
            <LargeTitle title=t("Practice,", "L’entraînement,") accent=t("without pressure.", "sans pression.") />
            <p class="mt-2 max-w-xl text-callout text-text-muted">
                {t("Quick drills on illustrative prices and charts. They train your eye and never count towards a Gate.", "Des exercices rapides sur des prix et des graphiques d’exemple. Ils entraînent ton œil et ne comptent jamais pour une validation.")}
            </p>
            <div class="mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {DRILLS
                    .iter()
                    .map(|d| {
                        let slug = d.kind.slug();
                        view! {
                            <a href=format!("#drill-{slug}") class="lift flex h-full flex-col rounded-card bg-surface p-5 shadow-card">
                                <span class="block text-eyebrow text-eyebrow-ink">{d.lesson.get()}</span>
                                <span class="mt-1 block text-headline">{d.title.get()}</span>
                                <span class="mt-1 block text-callout text-text-muted">{d.blurb.get()}</span>
                                <span class="tabular mt-auto block pt-4 text-footnote font-semibold">
                                    {move || {
                                        store
                                            .drills
                                            .with(|m| m.get(slug).map(|r| (r.correct, r.answered)))
                                            .map_or(t("Start ›", "Commencer ›").to_string(), |(c, a)| ts(format!("{c} of {a} right · Continue ›"), format!("{c} sur {a} justes · Continuer ›")))
                                    }}
                                </span>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </main>
    }
}

/// A quote without the dollar sign: "4,100.00" or, in French, "4 100,00".
fn money(p: Price) -> String {
    p.to_string().trim_start_matches('$').trim_end_matches('$').trim_end_matches('\u{a0}').to_string()
}

/// A picture above a drill question (Stage 5): two candles, or swing points with zones or a break.
#[derive(Debug, Clone, PartialEq)]
enum Visual {
    Candles(Vec<Candle>, Price, Price),
    Swings { swings: Vec<telosgard_core::stage5_drills::Swing>, zones: Vec<(Price, Price)>, broke_up: Option<bool> },
}

struct Choice {
    prompt: String,
    detail: Option<String>,
    visual: Option<Visual>,
    options: Vec<String>,
    answer: usize,
    why: String,
}

fn shuffle_choice(mut options: Vec<(String, bool)>, seed: u64) -> (Vec<String>, usize) {
    let mut rng = Rng::new(seed ^ 0x5eed);
    options.dedup_by(|a, b| a.0 == b.0);
    for i in (1..options.len()).rev() {
        options.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let answer = options.iter().position(|o| o.1).unwrap_or(0);
    (options.into_iter().map(|o| o.0).collect(), answer)
}

fn make_choice(kind: DrillKind, seed: u64) -> Choice {
    match kind {
        DrillKind::BidAsk => {
            let q = BidAskQ::new(seed);
            let spread = q.ask - q.bid;
            let (prompt, opts, why) = match q.asked {
                BidAskAsk::Spread => (
                    t("What is the spread?", "Quel est le spread ?").to_string(),
                    vec![(spread.to_string(), true), (DollarMove(spread.0 * 10).to_string(), false), (DollarMove(spread.0 * 100).to_string(), false), (DollarMove((spread.0 / 10).max(1) + spread.0).to_string(), false)],
                    format!("ask − bid = {} − {} = {spread}.", money(q.ask), money(q.bid)),
                ),
                BidAskAsk::BuyPrice => (
                    t("You buy now. Which price do you pay?", "Tu achètes maintenant. Quel prix paies-tu ?").to_string(),
                    vec![(money(q.ask), true), (money(q.bid), false), (money(Price((q.ask.0 + q.bid.0) / 2)), false)],
                    t("You buy at the ask, the higher price.", "Tu achètes à l’ask, le prix le plus haut.").to_string(),
                ),
                BidAskAsk::SellPrice => (
                    t("You sell (go short) now. Which price do you get?", "Tu vends (position courte) maintenant. Quel prix obtiens-tu ?").to_string(),
                    vec![(money(q.bid), true), (money(q.ask), false), (money(Price((q.ask.0 + q.bid.0) / 2)), false)],
                    t("You sell at the bid, the lower price.", "Tu vends au bid, le prix le plus bas.").to_string(),
                ),
            };
            let (options, answer) = shuffle_choice(opts, seed);
            Choice { visual: None, prompt, detail: Some(ts(format!("Bid (sell) {}  ·  Ask (buy) {}", money(q.bid), money(q.ask)), format!("Bid (vente) {}  ·  Ask (achat) {}", money(q.bid), money(q.ask)))), options, answer, why }
        }
        DrillKind::Distance => {
            let q = DistanceQ::new(seed);
            let a = q.answer();
            let way = if q.to > q.from { t("up", "à la hausse") } else { t("down", "à la baisse") };
            let (options, answer) = shuffle_choice(
                vec![(a.to_string(), true), (DollarMove(a.0 * 10).to_string(), false), (DollarMove((a.0 / 10).max(1)).to_string(), false), (DollarMove(a.0 + 100).to_string(), false)],
                seed,
            );
            Choice {
                visual: None,
                prompt: ts(format!("Gold moves from {} to {}. How big was the move?", money(q.from), money(q.to)), format!("L’or passe de {} à {}. De combien a-t-il bougé ?", money(q.from), money(q.to))),
                detail: None,
                options,
                answer,
                why: format!("{} − {} = {a}, {way}.", money(q.from.max(q.to)), money(q.from.min(q.to))),
            }
        }
        DrillKind::Pips => {
            let q = PipQ::new(seed);
            let size = DollarMove(q.pip_cents);
            let (options, answer) = shuffle_choice(
                [1, 10, 100].into_iter().map(|c| (DollarMove(q.pips * c).to_string(), c == q.pip_cents)).collect(),
                seed,
            );
            Choice {
                visual: None,
                prompt: ts(format!("A trader says “a {}-pip stop on gold” and, when you ask, they mean {size} per pip. What Dollar Move is that?", q.pips), format!("Un trader parle d’un « stop de {} pips sur l’or » et, quand tu demandes, il compte {size} par pip. Quel Mouvement en dollars est-ce ?", q.pips)),
                detail: None,
                options,
                answer,
                why: ts(format!("{} pips × {size} = {}. Always convert before you trust a pip number.", q.pips, q.answer()), format!("{} pips × {size} = {}. Convertis toujours avant de te fier à un nombre de pips.", q.pips, q.answer())),
            }
        }
        DrillKind::Session => {
            use telosgard_core::stage2_drills::{SessionAnswer, SessionQ};
            let q = SessionQ::new(seed);
            let (options, answer) = shuffle_choice(SessionAnswer::ALL.iter().map(|a| (a.label().to_string(), *a == q.answer)).collect(), seed);
            let hm = |m: u32| format!("{:02}:{:02}", m / 60, m % 60);
            let ny = telosgard_core::time::ny(q.utc).minute;
            let ldn = telosgard_core::time::london(q.utc).minute;
            let tokyo = ((q.utc.rem_euclid(telosgard_core::DAY_MS) / 60_000) as u32 + 9 * 60) % 1440;
            let why = ts(
                format!(
                    "At that moment it's {} in New York, {} in London and {} in Tokyo. Sessions: Asian 09:00–16:00 Tokyo, London 08:00–17:00 London, New York 08:00–17:00 New York.",
                    hm(ny), hm(ldn), hm(tokyo)
                ),
                format!(
                    "À ce moment-là, il est {} à New York, {} à Londres et {} à Tokyo. Séances : Asie 09:00–16:00 heure de Tokyo, Londres 08:00–17:00 heure de Londres, New York 08:00–17:00 heure de New York.",
                    hm(ny), hm(ldn), hm(tokyo)
                ),
            );
            Choice { visual: None, prompt: q.prompt(), detail: None, options, answer, why }
        }
        DrillKind::NoTrade => {
            use telosgard_core::stage2_drills::{NoTradeQ, WindowAnswer};
            let q = NoTradeQ::new(seed);
            let (options, answer) = shuffle_choice(WindowAnswer::ALL.iter().map(|a| (a.label().to_string(), *a == q.answer)).collect(), seed);
            let why = match q.answer {
                WindowAnswer::Rollover => t("Monday to Thursday, 5–6 pm New York time is the daily rollover: no new trades.", "Du lundi au jeudi, 17 h–18 h heure de New York, c’est le rollover quotidien : aucun nouveau trade.").to_string(),
                WindowAnswer::SundayOpen => t("Gold reopens at 6 pm New York time on Sunday; the first hour is a No-Trade Window because price can gap.", "L’or rouvre le dimanche à 18 h, heure de New York ; la première heure est une Fenêtre sans trade parce que le prix peut faire un gap.").to_string(),
                WindowAnswer::NearEvent => ts(
                    format!("{} is a tier-1 release: the window runs from 5 minutes before to 5 minutes after it.", q.event.map_or("This", |(n, _)| n)),
                    format!("{} est une grande statistique : la fenêtre va de 5 minutes avant à 5 minutes après.", q.event.map_or("Ceci", |(n, _)| telosgard_core::stage2_drills::event_fr(n))),
                ),
                WindowAnswer::Closed => t("Gold doesn't trade from Friday 5 pm to Sunday 6 pm New York time.", "L’or ne se trade pas du vendredi 17 h au dimanche 18 h, heure de New York.").to_string(),
                WindowAnswer::Clear => t("None of the three windows applies: not the rollover, not the Sunday open, and more than 5 minutes from any tier-1 release.", "Aucune des trois fenêtres ne s’applique : ni le rollover, ni l’ouverture du dimanche, et plus de 5 minutes avant ou après toute grande statistique.").to_string(),
            };
            Choice { visual: None, prompt: q.prompt(), detail: None, options, answer, why }
        }
        DrillKind::FindLevel => {
            use telosgard_core::stage5_drills::LevelQ;
            let q = LevelQ::new(seed);
            let letters = ["A", "B", "C", "D"];
            let zone = |z: (Price, Price)| format!("{}–{}", money(z.0), money(z.1));
            let options = q.zones.iter().enumerate().map(|(i, z)| format!("{}: {}", letters[i], zone(*z))).collect();
            let why = ts(
                format!("Zone {} ({}): price turned there twice. The other zones had one turn or none, so they're just places price passed.", letters[q.answer], zone(q.zones[q.answer])),
                format!("Zone {} ({}) : le prix y a tourné deux fois. Les autres zones n’ont eu qu’un retournement ou aucun : ce sont juste des endroits où le prix est passé.", letters[q.answer], zone(q.zones[q.answer])),
            );
            Choice {
                visual: Some(Visual::Swings { swings: q.swings.clone(), zones: q.zones.to_vec(), broke_up: None }),
                prompt: t("The dots are the turning points. Which zone has price reacted to at least twice?", "Les points sont les retournements. Sur quelle zone le prix a-t-il réagi au moins deux fois ?").into(),
                detail: None,
                options,
                answer: q.answer,
                why,
            }
        }
        DrillKind::NameStructure => {
            use telosgard_core::stage5_drills::{Break, Structure, StructureAsk, StructureQ};
            let q = StructureQ::new(seed);
            let read = match q.structure {
                Structure::Up => t("The highs and the lows are both rising: an uptrend.", "Les sommets et les creux montent tous les deux : une tendance haussière."),
                Structure::Down => t("The highs and the lows are both falling: a downtrend.", "Les sommets et les creux baissent tous les deux : une tendance baissière."),
                Structure::Range => t("The highs and lows aren't moving clearly either way: a range.", "Les sommets et les creux ne bougent nettement dans aucun sens : un range."),
            };
            match q.ask {
                StructureAsk::Name => Choice {
                    visual: Some(Visual::Swings { swings: q.swings.clone(), zones: Vec::new(), broke_up: None }),
                    prompt: t("Read the swings from left to right. What's the structure?", "Lis les swings de gauche à droite. Quelle est la structure ?").into(),
                    detail: None,
                    options: Structure::ALL.iter().map(|s| s.label().to_string()).collect(),
                    answer: Structure::ALL.iter().position(|s| *s == q.structure).unwrap_or(0),
                    why: read.into(),
                },
                StructureAsk::Break { up } => {
                    let ans = q.answer_break().unwrap_or(Break::Bos);
                    let why = match ans {
                        Break::Bos => ts(format!("{read} Breaking the last swing in the trend's own direction is a BOS: the trend continues."), format!("{read} Casser le dernier swing dans le sens de la tendance, c’est un BOS : la tendance continue.")),
                        Break::Choch => ts(format!("{read} Breaking the last swing against the trend is a CHoCH: a warning, not yet a new trend."), format!("{read} Casser le dernier swing contre la tendance, c’est un CHoCH : un avertissement, pas encore une nouvelle tendance.")),
                    };
                    Choice {
                        visual: Some(Visual::Swings { swings: q.swings.clone(), zones: Vec::new(), broke_up: Some(up) }),
                        prompt: if up {
                            t("Price now closes above the last swing high (dashed). Is that a BOS or a CHoCH?", "Le prix clôture maintenant au-dessus du dernier sommet de swing (pointillés). BOS ou CHoCH ?").into()
                        } else {
                            t("Price now closes below the last swing low (dashed). Is that a BOS or a CHoCH?", "Le prix clôture maintenant sous le dernier creux de swing (pointillés). BOS ou CHoCH ?").into()
                        },
                        detail: None,
                        options: Break::ALL.iter().map(|b| b.label().to_string()).collect(),
                        answer: Break::ALL.iter().position(|b| *b == ans).unwrap_or(0),
                        why,
                    }
                }
            }
        }
        DrillKind::NameCandle => {
            use telosgard_core::stage5_drills::{Pattern, PatternQ};
            let q = PatternQ::new(seed);
            let lo = Price(q.prev.low.0.min(q.last.low.0) - 200);
            let hi = Price(q.prev.high.0.max(q.last.high.0) + 200);
            let why = match q.answer {
                Pattern::BullishPin => t("A long lower wick, about two thirds of the candle or more, and a small body near the top: lower prices were rejected.", "Une longue mèche basse, environ deux tiers de la bougie ou plus, et un petit corps près du haut : les prix plus bas ont été refusés."),
                Pattern::BearishPin => t("A long upper wick and a small body near the bottom: higher prices were rejected.", "Une longue mèche haute et un petit corps près du bas : les prix plus hauts ont été refusés."),
                Pattern::BullishEngulfing => t("A bullish body that covers the whole bearish body before it.", "Un corps haussier qui recouvre tout le corps baissier précédent."),
                Pattern::BearishEngulfing => t("A bearish body that covers the whole bullish body before it.", "Un corps baissier qui recouvre tout le corps haussier précédent."),
                Pattern::InsideBar => t("Its high and low sit inside the previous candle's range: a pause.", "Son plus haut et son plus bas restent dans l’amplitude de la bougie précédente : une pause."),
                Pattern::None => t("No long wick on one side, no engulfing body, not inside the previous range: no pattern.", "Pas de longue mèche d’un côté, pas de corps qui avale, pas à l’intérieur de l’amplitude précédente : pas de figure."),
            };
            let why = ts(format!("{why} And remember: even a real pattern only counts at a location."), format!("{why} Et souviens-toi : même une vraie figure ne compte qu’à un endroit précis."));
            Choice {
                visual: Some(Visual::Candles(vec![q.prev, q.last], lo, hi)),
                prompt: t("What is the candle on the right?", "Qu’est-ce que la bougie de droite ?").into(),
                detail: None,
                options: Pattern::ALL.iter().map(|p| p.label().to_string()).collect(),
                answer: Pattern::ALL.iter().position(|p| *p == q.answer).unwrap_or(0),
                why,
            }
        }
        DrillKind::Surprise => {
            use telosgard_core::stage3_drills::{Surprise, SurpriseQ};
            let q = SurpriseQ::new(seed);
            // Fixed order (hot, soft, in line) reads better than a shuffle for a three-way scale.
            let options = Surprise::ALL.iter().map(|s| s.label().to_string()).collect();
            let answer = Surprise::ALL.iter().position(|s| *s == q.answer).unwrap_or(0);
            Choice { visual: None, prompt: q.prompt(), detail: None, options, answer, why: q.why() }
        }
        _ => {
            let q = OrderQ::new(seed);
            let want = match (q.buy, q.target) {
                (true, None) => t("buy right now", "acheter tout de suite").to_string(),
                (false, None) => t("sell right now", "vendre tout de suite").to_string(),
                (b, Some(lvl)) => ts(
                    format!("{} only if price {} {}", if b { "buy" } else { "sell" }, if lvl > q.current { "rises to" } else { "falls to" }, money(lvl)),
                    format!("{} seulement si le prix {} {}", if b { "acheter" } else { "vendre" }, if lvl > q.current { "monte à" } else { "descend à" }, money(lvl)),
                ),
            };
            let all = [(OrderKind::Market, t("Market order", "Ordre au marché")), (OrderKind::BuyLimit, "Buy limit"), (OrderKind::SellLimit, "Sell limit"), (OrderKind::BuyStop, "Buy stop"), (OrderKind::SellStop, "Sell stop")];
            let correct = q.answer();
            let (options, answer) = shuffle_choice(all.iter().map(|(k, n)| (n.to_string(), *k == correct)).collect(), seed);
            let why = match correct {
                OrderKind::Market => t("To act now, use a market order.", "Pour agir tout de suite, utilise un ordre au marché."),
                OrderKind::BuyLimit => t("Buying below the current price is a buy limit: it waits for a better price.", "Acheter sous le prix actuel, c’est un buy limit : il attend un meilleur prix."),
                OrderKind::SellLimit => t("Selling above the current price is a sell limit: it waits for a better price.", "Vendre au-dessus du prix actuel, c’est un sell limit : il attend un meilleur prix."),
                OrderKind::BuyStop => t("Buying only once price rises through a level is a buy stop.", "Acheter seulement une fois que le prix a franchi un niveau à la hausse, c’est un buy stop."),
                OrderKind::SellStop => t("Selling only once price falls through a level is a sell stop.", "Vendre seulement une fois que le prix a franchi un niveau à la baisse, c’est un sell stop."),
            };
            Choice { visual: None, prompt: ts(format!("Gold is at {}. You want to {want}. Which order?", money(q.current)), format!("L’or est à {}. Tu veux {want}. Quel ordre ?", money(q.current))), detail: None, options, answer, why: why.into() }
        }
    }
}

#[component]
pub fn ChoiceDrill(kind: DrillKind) -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    let info = DRILLS.iter().find(|d| d.kind == kind).expect("known drill");
    let seed = RwSignal::new(now() as u64);
    let choice = Memo::new(move |_| {
        let c = make_choice(kind, seed.get());
        (c.prompt, c.detail, c.options, c.answer, c.why, c.visual)
    });
    let picked = RwSignal::new(None::<usize>);
    let score = RwSignal::new((0u32, 0u32));
    let pick = move |i: usize| {
        if picked.get_untracked().is_some() {
            return;
        }
        picked.set(Some(i));
        let right = choice.with_untracked(|c| c.3 == i);
        store.record_drill(kind.slug(), right);
        score.update(|(r, t)| {
            *t += 1;
            *r += right as u32;
        });
    };
    let next = move |_| {
        seed.update(|s| *s = s.wrapping_add(7919));
        picked.set(None);
    };
    view! {
        <SubBar back="#practice" back_label=t("Practice", "Entraînement") title=info.title.get() />
        <main class="gutter mx-auto max-w-[640px] pt-6 pb-10">
            <div class="flex items-baseline justify-between gap-4">
                <h1 class="text-title1">{info.title.get()}</h1>
                <p class="tabular shrink-0 whitespace-nowrap text-callout text-text-muted">{move || { let (r, t) = score.get(); ts(format!("{r} of {t} right"), format!("{r} sur {t} justes")) }}</p>
            </div>
            <p class="mt-2 text-callout text-text-muted">{info.blurb.get()} {t(" Prices are illustrative.", " Prix donnés à titre d’exemple.")}</p>
            <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                {move || {
                    let (prompt, detail, options, answer, why, visual) = choice.get();
                    let picture = visual.map(|v| match v {
                        Visual::Candles(cs, lo, hi) => crate::ui::chart::candle_strip(&cs, lo, hi, t("The two latest candles; the question is about the one on the right", "Les deux dernières bougies ; la question porte sur celle de droite").into()).into_any(),
                        Visual::Swings { swings, zones, broke_up } => crate::ui::chart::swing_chart(&swings, &zones, broke_up, t("Swing highs and lows joined by a line", "Sommets et creux de swing reliés par une ligne").into()).into_any(),
                    });
                    view! {
                        {picture.map(|p| view! { <div class="mb-3">{p}</div> })}
                        {detail.map(|d| view! { <p class="tabular mb-3 rounded-btn bg-bg p-3 text-headline">{d}</p> })}
                        <h2 class="text-headline">{prompt}</h2>
                        <div class="mt-4 grid gap-2">
                            {options
                                .into_iter()
                                .enumerate()
                                .map(|(i, o)| {
                                    let is_answer = i == answer;
                                    view! {
                                        <button
                                            type="button"
                                            disabled=move || picked.get().is_some()
                                            on:click=move |_| pick(i)
                                            class=move || {
                                                let base = "press tabular flex min-h-12 w-full items-center rounded-btn bg-bg px-4 py-3 text-left text-body";
                                                match picked.get() {
                                                    None => format!("{base} hover:bg-raised"),
                                                    Some(_) if is_answer => format!("{base} ring-2 ring-up"),
                                                    Some(p) if p == i => format!("{base} ring-2 ring-down"),
                                                    Some(_) => format!("{base} opacity-55"),
                                                }
                                            }
                                        >
                                            {o}
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                        <Show when=move || picked.get().is_some()>
                            <div class="anim-fade mt-4 rounded-btn bg-bg p-4" aria-live="polite">
                                {move || if picked.get() == Some(answer) {
                                    view! { <p class="text-headline text-up">{t("✓ Right", "✓ Juste")}</p> }.into_any()
                                } else {
                                    view! { <p class="text-headline text-down">{t("✗ Not quite", "✗ Pas tout à fait")}</p> }.into_any()
                                }}
                                <p class="mt-1 text-callout">{why.clone()}</p>
                            </div>
                            <button type="button" class=format!("{PILL} mt-4") on:click=next>{t("Next question ›", "Question suivante ›")}</button>
                        </Show>
                    }
                }}
            </section>
        </main>
    }
}

/// A chart whose last candle is clearly bullish or bearish (dojis are skipped for this drill).
fn round(seed: u64) -> Vec<Candle> {
    (seed..)
        .map(|s| generate(s, 24, Price(235_000), DollarMove(600), DollarMove(350)))
        .find(|cs| cs.last().is_some_and(|c| c.direction() != CandleDirection::Doji))
        .expect("a non-doji chart exists within a few seeds")
}

#[component]
pub fn DirectionDrill() -> impl IntoView {
    let seed = RwSignal::new(now() as u64);
    let candles = Memo::new(move |_| round(seed.get()));
    let answer = RwSignal::new(None::<CandleDirection>);
    let score = RwSignal::new((0u32, 0u32));
    let store = expect_context::<crate::store::Store>();
    let last = move || candles.with(|cs| *cs.last().expect("non-empty"));

    let pick = move |dir: CandleDirection| {
        if answer.get_untracked().is_some() {
            return;
        }
        answer.set(Some(dir));
        let right = check_direction(&last(), dir);
        store.record_drill("direction", right);
        score.update(|(r, t)| {
            *t += 1;
            if right {
                *r += 1;
            }
        });
    };
    let next = move |_| {
        seed.update(|s| *s = s.wrapping_add(7919));
        answer.set(None);
    };

    let choice = move |dir: CandleDirection, label: &'static str, glyph: &'static str, tone: &'static str| {
        view! {
            <button
                type="button"
                disabled=move || answer.get().is_some()
                on:click=move |_| pick(dir)
                class=move || {
                    let ring = match answer.get() {
                        Some(_) if last().direction() == dir => "ring-2 ring-up",
                        Some(a) if a == dir => "ring-2 ring-down",
                        Some(_) => "opacity-55",
                        None => "",
                    };
                    format!("press flex min-h-14 flex-1 items-center justify-center gap-2 rounded-btn bg-surface shadow-card text-headline {ring}")
                }
            >
                <span class=tone aria-hidden="true">{glyph}</span>
                {label}
            </button>
        }
    };

    view! {
        <SubBar back="#practice" back_label=t("Practice", "Entraînement") title=t("Bullish or bearish?", "Haussière ou baissière ?") />
        <main class="gutter mx-auto max-w-[760px] pt-6 pb-10">
            <div class="flex items-baseline justify-between gap-4">
                <h1 class="text-title1">{t("Bullish or bearish?", "Haussière ou baissière ?")}</h1>
                <p class="tabular shrink-0 whitespace-nowrap text-callout text-text-muted">{move || { let (r, n) = score.get(); ts(format!("{r} of {n} right"), format!("{r} sur {n} justes")) }}</p>
            </div>
            <p class="mt-2 text-callout text-text-muted">
                {t("Look at the highlighted candle. Did buyers or sellers win it? Hollow candles closed higher; filled candles closed lower.", "Regarde la bougie mise en avant. Qui l’a emporté, acheteurs ou vendeurs ? Les bougies creuses ont clôturé plus haut ; les bougies pleines, plus bas.")}
            </p>
            {move || {
                store
                    .drills
                    .with(|m| m.get("direction").cloned())
                    .filter(|d| d.answered > 0)
                    .map(|d| view! { <p class="tabular mt-1 text-footnote text-text-muted">{ts(format!("All time: {} of {} right", d.correct, d.answered), format!("Depuis le début : {} sur {} justes", d.correct, d.answered))}</p> })
            }}
            <section class="mt-5 rounded-card bg-surface p-4 shadow-card md:p-5">
                <CandleChart
                    candles=Signal::derive(move || candles.get())
                    highlight=Signal::derive(move || Some(candles.with(|c| c.len() - 1)))
                    label=t("Illustrative candlestick chart with the last candle highlighted", "Graphique en chandeliers d’exemple, dernière bougie mise en avant")
                />
            </section>
            <div class="mt-5 flex gap-3">
                {choice(CandleDirection::Bullish, t("Bullish", "Haussière"), "▲", "text-up")}
                {choice(CandleDirection::Bearish, t("Bearish", "Baissière"), "▼", "text-down")}
            </div>
            {move || {
                answer
                    .get()
                    .map(|a| {
                        let c = last();
                        let moved = (c.close - c.open).abs();
                        let truth = c.direction();
                        let (verdict, tone) = if a == truth { (t("✓ Right", "✓ Juste"), "text-up") } else { (t("✗ Not quite", "✗ Pas tout à fait"), "text-down") };
                        let explain = match truth {
                            CandleDirection::Bullish => ts(format!("It closed {moved} above where it opened, so buyers won that candle."), format!("Elle a clôturé {moved} au-dessus de son ouverture : les acheteurs ont gagné cette bougie.")),
                            _ => ts(format!("It closed {moved} below where it opened, so sellers won that candle."), format!("Elle a clôturé {moved} sous son ouverture : les vendeurs ont gagné cette bougie.")),
                        };
                        view! {
                            <div class="anim-fade mt-5 rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                                <p class=format!("text-headline {tone}")>{verdict}</p>
                                <p class="mt-1 text-callout">{explain}</p>
                                <button type="button" class=format!("{PILL} mt-4") on:click=next>{t("Next chart ›", "Graphique suivant ›")}</button>
                            </div>
                        }
                    })
            }}
        </main>
    }
}

/// Calibration drill (Phase 11): ten predictions of the next candle on a random walk. The hit rate
/// lands near 50% for everyone; the lesson is to manage risk over many trades, not to predict.
#[component]
pub fn CalibrationDrill() -> impl IntoView {
    use telosgard_core::calibration::{ROUND_LEN, calibration_round, hit_rate};
    let store = expect_context::<crate::store::Store>();
    let seed = RwSignal::new(now() as u64);
    let charts = Memo::new(move |_| calibration_round(seed.get()));
    let idx = RwSignal::new(0usize);
    let guess = RwSignal::new(None::<bool>);
    let right = RwSignal::new(0u32);
    let done = RwSignal::new(false);

    let chart = move || charts.with(|c| c[idx.get()].clone());
    let shown = Signal::derive(move || {
        let c = chart();
        let mut v = c.shown.clone();
        if guess.get().is_some() {
            v.push(c.next);
        }
        v
    });
    let highlight = Signal::derive(move || guess.get().map(|_| shown.with(|v| v.len() - 1)));
    let predict = move |up: bool| {
        if guess.get_untracked().is_some() {
            return;
        }
        guess.set(Some(up));
        let hit = chart().went_up() == up;
        right.update(|r| *r += hit as u32);
        store.record_drill("calibration", hit);
    };
    let next = move |_| {
        if idx.get_untracked() + 1 >= ROUND_LEN {
            done.set(true);
        } else {
            idx.update(|i| *i += 1);
            guess.set(None);
        }
    };
    let again = move |_| {
        seed.update(|s| *s = s.wrapping_add(7919));
        idx.set(0);
        guess.set(None);
        right.set(0);
        done.set(false);
    };
    let choice = move |up: bool, label: &'static str, glyph: &'static str, tone: &'static str| {
        view! {
            <button
                type="button"
                disabled=move || guess.get().is_some()
                on:click=move |_| predict(up)
                class=move || {
                    let ring = match guess.get() {
                        Some(_) if chart().went_up() == up => "ring-2 ring-up",
                        Some(g) if g == up => "ring-2 ring-down",
                        Some(_) => "opacity-55",
                        None => "",
                    };
                    format!("press flex min-h-14 flex-1 items-center justify-center gap-2 rounded-btn bg-surface shadow-card text-headline {ring}")
                }
            >
                <span class=tone aria-hidden="true">{glyph}</span>
                {label}
            </button>
        }
    };

    view! {
        <SubBar back="#practice" back_label=t("Practice", "Entraînement") title=t("Call the next candle", "Devine la prochaine bougie") />
        <main class="gutter mx-auto max-w-[760px] pt-6 pb-10">
            <h1 class="text-title1">{t("Can you call the next candle?", "Peux-tu deviner la prochaine bougie ?")}</h1>
            <p class="mt-2 text-callout text-text-muted">
                {t("Ten illustrative charts. For each, predict whether the next candle closes higher or lower. Give it your best read.", "Dix graphiques d’exemple. Pour chacun, prédis si la prochaine bougie clôture plus haut ou plus bas. Fais de ton mieux.")}
            </p>
            <Show
                when=move || done.get()
                fallback=move || view! {
                    <p class="tabular mt-4 text-eyebrow text-eyebrow-ink">{move || ts(format!("Chart {} of {ROUND_LEN}", idx.get() + 1), format!("Graphique {} sur {ROUND_LEN}", idx.get() + 1))}</p>
                    <section class="mt-2 rounded-card bg-surface p-4 shadow-card md:p-5">
                        <CandleChart candles=shown highlight=highlight label=t("Illustrative candlestick chart; predict the next candle", "Graphique en chandeliers d’exemple ; prédis la prochaine bougie") />
                    </section>
                    <div class="mt-5 flex gap-3">
                        {choice(true, t("Higher", "Plus haut"), "▲", "text-up")}
                        {choice(false, t("Lower", "Plus bas"), "▼", "text-down")}
                    </div>
                    {move || guess.get().map(|g| {
                        let c = chart();
                        let up = c.went_up();
                        let moved = (c.next.close - c.next.open).abs();
                        let (verdict, tone) = if g == up { (t("✓ Called it", "✓ Bien vu"), "text-up") } else { (t("✗ Missed", "✗ Raté"), "text-down") };
                        view! {
                            <div class="anim-fade mt-5 rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                                <p class=format!("text-headline {tone}")>{verdict}</p>
                                <p class="mt-1 text-callout">{ts(format!("The next candle closed {moved} {}.", if up { "higher" } else { "lower" }), format!("La bougie suivante a clôturé {moved} plus {}.", if up { "haut" } else { "bas" }))}</p>
                                <button type="button" class=format!("{PILL} mt-4") on:click=next>
                                    {move || if idx.get() + 1 >= ROUND_LEN { t("See your hit rate", "Voir ton taux de réussite") } else { t("Next chart ›", "Graphique suivant ›") }}
                                </button>
                            </div>
                        }
                    })}
                }
            >
                <section class="anim-fade mt-5 rounded-card bg-surface p-5 shadow-card" aria-live="polite">
                    <p class="text-eyebrow text-eyebrow-ink">{t("Your hit rate", "Ton taux de réussite")}</p>
                    <p class="tabular mt-2 text-large">{move || view! { <crate::ui::moments::CountUp value=hit_rate(right.get(), ROUND_LEN as u32).0 as i64 fmt=|v| telosgard_core::money::Percent(v as u32).to_string() /> }}</p>
                    <p class="tabular text-callout text-text-muted">{move || ts(format!("{} of {ROUND_LEN} called right", right.get()), format!("{} sur {ROUND_LEN} bien vus", right.get()))}</p>
                    <p class="mt-4 text-callout">
                        {t("These charts move at random, so anyone lands around 50% over many rounds; a high or low score here is luck. Real charts aren't much kinder: nobody can reliably call the next candle.", "Ces graphiques bougent au hasard : sur beaucoup de séries, tout le monde tombe autour de 50 % ; un score haut ou bas, ici, c’est de la chance. Les vrais graphiques ne sont guère plus tendres : personne ne peut deviner la prochaine bougie de façon fiable.")}
                    </p>
                    <p class="mt-2 text-callout">
                        {t("That's why a trader doesn't need to predict. As in Lesson 2, you win by keeping every loss small and planned and letting wins be bigger, over many trades.", "C’est pour ça qu’un trader n’a pas besoin de prédire. Comme à la leçon 2, on gagne en gardant chaque perte petite et prévue, et en laissant les gains être plus gros, sur beaucoup de trades.")}
                    </p>
                    <button type="button" class=format!("{PILL} mt-4") on:click=again>{t("New round", "Nouvelle série")}</button>
                </section>
            </Show>
        </main>
    }
}

/// "Build the candle" (Wave 4): four drawings on one price scale, one matching the numbers; then
/// who won it. It counts as right only when both answers are.
#[component]
pub fn BuildCandleDrill() -> impl IntoView {
    use telosgard_core::stage4_drills::{CandleQ, Winner};
    let store = expect_context::<crate::store::Store>();
    let seed = RwSignal::new(now() as u64);
    let q = Memo::new(move |_| CandleQ::new(seed.get()));
    let shape = RwSignal::new(None::<usize>);
    let winner = RwSignal::new(None::<Winner>);
    let score = RwSignal::new((0u32, 0u32));
    let pick_winner = move |w: Winner| {
        if winner.get_untracked().is_some() || shape.get_untracked().is_none() {
            return;
        }
        winner.set(Some(w));
        let qq = q.get_untracked();
        let right = shape.get_untracked() == Some(qq.answer) && w == qq.winner;
        store.record_drill("build-candle", right);
        score.update(|(r, n)| {
            *n += 1;
            *r += right as u32;
        });
    };
    let next = move |_| {
        seed.update(|s| *s = s.wrapping_add(7919));
        shape.set(None);
        winner.set(None);
    };
    const LETTERS: [&str; 4] = ["A", "B", "C", "D"];
    view! {
        <SubBar back="#practice" back_label=t("Practice", "Entraînement") title=t("Build the candle", "Construis la bougie") />
        <main class="gutter mx-auto max-w-[680px] pt-6 pb-10">
            <div class="flex items-baseline justify-between gap-4">
                <h1 class="text-title1">{t("Build the candle", "Construis la bougie")}</h1>
                <p class="tabular shrink-0 whitespace-nowrap text-callout text-text-muted">{move || { let (r, n) = score.get(); ts(format!("{r} of {n} right"), format!("{r} sur {n} justes")) }}</p>
            </div>
            <p class="mt-2 text-callout text-text-muted">{t("Read the four prices, pick the candle that draws them, then say who won it. Prices are illustrative.", "Lis les quatre prix, choisis la bougie qui les dessine, puis dis qui l’a gagnée. Prix donnés à titre d’exemple.")}</p>
            {move || {
                let qq = q.get();
                let c = qq.candle;
                let lo = qq.options.iter().map(|o| o.low.0).min().unwrap_or(c.low.0) - 200;
                let hi = qq.options.iter().map(|o| o.high.0).max().unwrap_or(c.high.0) + 200;
                let numbers = ts(
                    format!("Open {} · High {} · Low {} · Close {}", money(c.open), money(c.high), money(c.low), money(c.close)),
                    format!("Ouverture {} · Plus haut {} · Plus bas {} · Clôture {}", money(c.open), money(c.high), money(c.low), money(c.close)),
                );
                let describe = move |o: &telosgard_core::drills::Candle| ts(
                    format!("opens {}, high {}, low {}, closes {}", money(o.open), money(o.high), money(o.low), money(o.close)),
                    format!("ouvre à {}, plus haut {}, plus bas {}, clôture à {}", money(o.open), money(o.high), money(o.low), money(o.close)),
                );
                let options = qq.options;
                view! {
                    <section class="mt-5 rounded-card bg-surface p-5 shadow-card">
                        <p class="tabular rounded-btn bg-bg p-3 text-headline">{numbers}</p>
                        <h2 class="mt-4 text-headline">{t("1. Which candle is it?", "1. Quelle bougie est-ce ?")}</h2>
                        <div class="mt-3">
                            {crate::ui::chart::candle_strip(&options, Price(lo), Price(hi), t("Four candles labelled A to D, on one price scale", "Quatre bougies notées de A à D, sur une même échelle de prix").into())}
                            // One button under each candle: the strip is 366 units wide and its right 86 are the price axis.
                            <div class="grid max-w-[520px] grid-cols-4 gap-2" style="padding-right:23.5%">
                                {(0..4).map(|i| {
                                    let label = format!("{}: {}", LETTERS[i], describe(&options[i]));
                                    view! {
                                        <button
                                            type="button"
                                            aria-label=label
                                            disabled=move || shape.get().is_some()
                                            on:click=move |_| shape.set(Some(i))
                                            class=move || {
                                                let ring = match shape.get() {
                                                    Some(_) if i == q.with(|q| q.answer) => "ring-2 ring-up",
                                                    Some(p) if p == i => "ring-2 ring-down",
                                                    Some(_) => "opacity-55",
                                                    None => "",
                                                };
                                                format!("press min-h-11 rounded-btn bg-bg text-headline shadow-card {ring}")
                                            }
                                        >{LETTERS[i]}</button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                        <Show when=move || shape.get().is_some()>
                            <h2 class="mt-5 text-headline">{t("2. Who won it?", "2. Qui l’a gagnée ?")}</h2>
                            <div class="mt-3 grid gap-2">
                                {Winner::ALL.into_iter().map(|w| view! {
                                    <button
                                        type="button"
                                        disabled=move || winner.get().is_some()
                                        on:click=move |_| pick_winner(w)
                                        class=move || {
                                            let ring = match winner.get() {
                                                Some(_) if w == q.with(|q| q.winner) => "ring-2 ring-up",
                                                Some(p) if p == w => "ring-2 ring-down",
                                                Some(_) => "opacity-55",
                                                None => "",
                                            };
                                            format!("press min-h-12 rounded-btn bg-bg px-4 text-left text-body shadow-card {ring}")
                                        }
                                    >{w.label()}</button>
                                }).collect_view()}
                            </div>
                        </Show>
                        {move || winner.get().map(|_| {
                            let qq = q.get();
                            let c = qq.candle;
                            let right = shape.get() == Some(qq.answer) && winner.get() == Some(qq.winner);
                            let why = ts(
                                format!("Candle {}: the body runs from the open {} to the close {}, and the wicks reach the high {} and the low {}. The close decides who won.", LETTERS[qq.answer], money(c.open), money(c.close), money(c.high), money(c.low)),
                                format!("Bougie {} : le corps va de l’ouverture {} à la clôture {}, et les mèches atteignent le plus haut {} et le plus bas {}. C’est la clôture qui décide qui a gagné.", LETTERS[qq.answer], money(c.open), money(c.close), money(c.high), money(c.low)),
                            );
                            view! {
                                <div class="anim-fade mt-5" role="status">
                                    <p class=if right { "text-headline text-up" } else { "text-headline text-down" }>{if right { t("Right on both.", "Juste sur les deux.") } else { t("Not quite.", "Pas tout à fait.") }}</p>
                                    <p class="mt-1 text-callout">{why}</p>
                                    <button type="button" class=format!("{PILL} mt-4") on:click=next>{t("Next candle ›", "Bougie suivante ›")}</button>
                                </div>
                            }
                        })}
                    </section>
                }
            }}
        </main>
    }
}
