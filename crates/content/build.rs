//! Parses `lessons/NN-*.md` (TOML front matter between `+++` lines, Markdown body), enforces the
//! CURRICULUM authoring rules, and writes `generated.rs`. Any rule broken fails the build.
//! `lessons/fr/NN-*.md` are the French twins: same rules (with room for French running about a
//! fifth longer), and the same structure as the English lesson: ids, answers, widgets, drills,
//! videos and sources must match, so progress and Question Records are shared across languages.

use serde::Deserialize;
use std::fmt::Write as _;
use std::{env, fs, path::Path};

const READ_WPM: usize = 130;
const MAX_STEP_WORDS: usize = 60;
/// French takes roughly 20% more words for the same text.
const MAX_STEP_WORDS_FR: usize = 75;
const WIDGETS: &[&str] = &["price_drag", "lots_money", "one_percent", "stop_first", "short_risk", "leverage_toggle", "bid_ask", "win_rate", "pip_convert", "session_clock", "surprise_meter", "candle_builder", "candle_merge"];
/// The lesson about pips, the only one allowed to use the word.
const PIPS_LESSON: u8 = 7;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Front {
    id: u8,
    stage: u8,
    title: String,
    objective: String,
    hook: String,
    steps: Vec<Step>,
    analogy: String,
    analogy_tag: String,
    example: String,
    mistake: String,
    takeaways: Vec<String>,
    #[serde(default)]
    videos: Vec<Video>,
    sources: Vec<Source>,
    quiz: Vec<Quiz>,
    recall: Vec<Quiz>,
    explain_back: Explain,
    #[serde(default)]
    drills: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Step {
    heading: String,
    text: String,
    why: Option<String>,
    widget: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Video {
    id: String,
    title: String,
    channel: String,
    duration: String,
    watch_for: String,
    fallback_search: String,
    checked: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    claim: String,
    name: String,
    url: String,
    checked: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Quiz {
    id: String,
    q: String,
    options: Vec<String>,
    answer: usize,
    why: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Explain {
    prompt: String,
    must_include: Vec<String>,
    misconceptions: Vec<String>,
    fixtures: Vec<Fixture>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    answer: String,
    expect: String,
}

const DRILLS: &[&str] = &["candle_direction", "bid_ask", "dollar_distance", "pip_convert", "order_type", "session", "no_trade", "surprise", "build_candle"];

fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    s.len() == 10 && b[4] == b'-' && b[7] == b'-' && s.chars().enumerate().all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn mentions_pips(s: &str) -> bool {
    s.to_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).any(|w| w == "pip" || w == "pips")
}

fn validate(file: &str, f: &Front, body: &str, fr: bool) -> (Vec<String>, usize) {
    let max_step = if fr { MAX_STEP_WORDS_FR } else { MAX_STEP_WORDS };
    let max_minutes = if fr { 6 } else { 5 };
    let mut e = Vec::new();
    let mut err = |m: String| e.push(format!("{file}: {m}"));
    let prefix: u8 = file.rsplit('/').next().unwrap_or(file).split('-').next().and_then(|n| n.parse().ok()).unwrap_or(0);
    if prefix != f.id {
        err(format!("file number {prefix} doesn't match id {}", f.id));
    }
    if f.id <= 12 && f.stage != 1 {
        err("lessons 1–12 are Stage 1".into());
    }
    if !body.trim().is_empty() {
        err("the explanation lives in [[steps]]; leave the Markdown body empty".into());
    }
    if !(3..=6).contains(&f.steps.len()) {
        err(format!("needs 3–6 steps, has {}", f.steps.len()));
    }
    for (i, st) in f.steps.iter().enumerate() {
        if st.heading.trim().is_empty() || words(&st.heading) > if fr { 12 } else { 10 } {
            err(format!("step {}: a heading of 1–10 words", i + 1));
        }
        if words(&st.text) == 0 || words(&st.text) > max_step {
            err(format!("step {} has {} words of main text; keep it to {max_step} and move detail into `why`", i + 1, words(&st.text)));
        }
        if let Some(w) = &st.widget {
            if !WIDGETS.contains(&w.as_str()) {
                err(format!("step {}: unknown widget {w}", i + 1));
            }
        }
    }
    if f.steps.iter().filter(|s| s.widget.is_some()).count() > 2 {
        err("at most 2 widgets per lesson".into());
    }
    if f.recall.len() != 3 {
        err(format!("the Recall Set has exactly 3 questions, not {}", f.recall.len()));
    }
    let mut qids: Vec<&str> = f.quiz.iter().chain(&f.recall).map(|q| q.id.as_str()).collect();
    let nq = qids.len();
    qids.sort();
    qids.dedup();
    if qids.len() != nq || f.quiz.iter().any(|q| !q.id.starts_with('q')) || f.recall.iter().any(|q| !q.id.starts_with('r')) {
        err("question ids must be unique, quiz ids start with q and recall ids with r".into());
    }
    for (i, q) in f.recall.iter().enumerate() {
        if !(2..=5).contains(&q.options.len()) || q.answer >= q.options.len() || q.why.trim().is_empty() {
            err(format!("recall {} is malformed", i + 1));
        }
    }
    for (name, v) in [("title", &f.title), ("objective", &f.objective), ("hook", &f.hook), ("analogy", &f.analogy), ("analogy_tag", &f.analogy_tag), ("example", &f.example), ("mistake", &f.mistake)] {
        if v.trim().is_empty() {
            err(format!("{name} is empty"));
        }
    }
    if !(3..=5).contains(&f.takeaways.len()) {
        err(format!("needs 3–5 takeaways, has {}", f.takeaways.len()));
    }
    if !(5..=8).contains(&f.quiz.len()) {
        err(format!("needs 5–8 quiz questions, has {}", f.quiz.len()));
    }
    for (i, q) in f.quiz.iter().enumerate() {
        if !(2..=5).contains(&q.options.len()) || q.answer >= q.options.len() || q.why.trim().is_empty() {
            err(format!("quiz {} is malformed (2–5 options, answer in range, a why)", i + 1));
        }
    }
    let x = &f.explain_back;
    if !(2..=4).contains(&x.must_include.len()) || !(1..=3).contains(&x.misconceptions.len()) {
        err("explain_back needs 2–4 must_include and 1–3 misconceptions".into());
    }
    let expects: Vec<&str> = x.fixtures.iter().map(|fx| fx.expect.as_str()).collect();
    if expects != ["got_it", "almost", "almost_or_not_yet"] {
        err("fixtures must be got_it, almost, almost_or_not_yet in that order".into());
    }
    if f.videos.len() > 2 {
        err("at most 2 videos".into());
    }
    for v in &f.videos {
        let ok_id = v.id.len() == 11 && v.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !ok_id || !is_date(&v.checked) || v.watch_for.trim().is_empty() || v.fallback_search.trim().is_empty() {
            err(format!("video {} needs an 11-char id, a checked date, watch_for and fallback_search", v.id));
        }
    }
    if f.sources.is_empty() {
        err("at least one source (CURRICULUM rule 8)".into());
    }
    for s in &f.sources {
        if !s.url.starts_with("https://") || !is_date(&s.checked) {
            err(format!("source {} needs an https url and a checked date", s.name));
        }
    }
    for d in &f.drills {
        if !DRILLS.contains(&d.as_str()) {
            err(format!("unknown drill {d}"));
        }
    }
    let step_text: Vec<&str> = f.steps.iter().map(|s| s.text.as_str()).collect();
    let why_text: Vec<&str> = f.steps.iter().filter_map(|s| s.why.as_deref()).collect();
    let main = [vec![f.hook.as_str(), &f.analogy, &f.example, &f.mistake], step_text].concat().join(" ");
    if f.id != PIPS_LESSON && mentions_pips(&format!("{main} {}", why_text.join(" "))) {
        err(format!("the word \"pip\" belongs in Lesson {PIPS_LESSON} only; use Dollar Moves"));
    }
    let total = words(&main) + f.takeaways.iter().map(|t| words(t)).sum::<usize>();
    let minutes = (total + READ_WPM / 2) / READ_WPM;
    if !(2..=max_minutes).contains(&minutes) {
        err(format!("the main path reads in ~{minutes} min ({total} words); target 2–{max_minutes}"));
    }
    let with_why = total + words(&why_text.join(" "));
    if (with_why + READ_WPM / 2) / READ_WPM > if fr { 10 } else { 8 } {
        err(format!("with every \"Why?\" opened it reads in over 8 minutes ({with_why} words)"));
    }
    (e, minutes)
}

/// A French lesson must have the English lesson's shape, so one set of progress serves both.
fn parity(file: &str, en: &Front, fr: &Front) -> Vec<String> {
    let mut e = Vec::new();
    let mut err = |m: &str| e.push(format!("fr/{file}: {m} differs from the English lesson"));
    if en.stage != fr.stage {
        err("stage");
    }
    let steps = |f: &Front| f.steps.iter().map(|s| (s.widget.clone(), s.why.is_some())).collect::<Vec<_>>();
    if steps(en) != steps(fr) {
        err("the steps (count, widgets or which have a Why?)");
    }
    let qs = |v: &[Quiz]| v.iter().map(|q| (q.id.clone(), q.options.len(), q.answer)).collect::<Vec<_>>();
    if qs(&en.quiz) != qs(&fr.quiz) {
        err("the quiz (ids, option counts or answers)");
    }
    if qs(&en.recall) != qs(&fr.recall) {
        err("the Recall Set (ids, option counts or answers)");
    }
    if en.takeaways.len() != fr.takeaways.len() {
        err("the number of takeaways");
    }
    let x = |f: &Front| (f.explain_back.must_include.len(), f.explain_back.misconceptions.len(), f.explain_back.fixtures.iter().map(|x| x.expect.clone()).collect::<Vec<_>>());
    if x(en) != x(fr) {
        err("the Explain-Back (key ideas, misconceptions or fixtures)");
    }
    let vids = |f: &Front| f.videos.iter().map(|v| (v.id.clone(), v.title.clone(), v.channel.clone(), v.duration.clone())).collect::<Vec<_>>();
    if vids(en) != vids(fr) {
        err("the videos (id, title, channel, duration stay as published)");
    }
    let srcs = |f: &Front| f.sources.iter().map(|s| s.url.clone()).collect::<Vec<_>>();
    if srcs(en) != srcs(fr) {
        err("the source urls");
    }
    if en.drills != fr.drills {
        err("drills");
    }
    e
}

fn s(v: &str) -> String {
    format!("{v:?}")
}

fn list(v: &[String]) -> String {
    format!("&[{}]", v.iter().map(|x| s(x)).collect::<Vec<_>>().join(", "))
}

struct Parsed {
    name: String,
    front: Front,
    body: String,
}

fn read_dir(dir: &Path, errors: &mut Vec<String>, label: &str) -> Vec<Parsed> {
    println!("cargo:rerun-if-changed={}", dir.display());
    let Ok(rd) = fs::read_dir(dir) else { return Vec::new() };
    let mut files: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "md")).collect();
    files.sort();
    let mut out = Vec::new();
    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let text = fs::read_to_string(path).unwrap();
        let Some(rest) = text.strip_prefix("+++\n") else {
            errors.push(format!("{label}{name}: must start with +++ front matter"));
            continue;
        };
        let Some((front, body)) = rest.split_once("\n+++\n") else {
            errors.push(format!("{label}{name}: front matter isn't closed with +++"));
            continue;
        };
        match toml::from_str::<Front>(front) {
            Ok(f) => out.push(Parsed { name, front: f, body: body.to_string() }),
            Err(e) => errors.push(format!("{label}{name}: {e}")),
        }
    }
    out
}

fn emit(out: &mut String, f: &Front, minutes: usize) {
    let md = |src: &str| {
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new_ext(src.trim(), pulldown_cmark::Options::ENABLE_TABLES));
        html
    };
    let steps: Vec<String> = f
        .steps
        .iter()
        .map(|st| {
            let opt = |o: Option<String>| o.map_or("None".to_string(), |v| format!("Some({})", s(&v)));
            format!("Step {{ heading: {}, text_html: {}, why_html: {}, widget: {} }}", s(&st.heading), s(&md(&st.text)), opt(st.why.as_deref().map(md)), opt(st.widget.clone()))
        })
        .collect();
    let videos: Vec<String> = f
        .videos
        .iter()
        .map(|v| format!("Video {{ id: {}, title: {}, channel: {}, duration: {}, watch_for: {}, fallback_search: {}, checked: {} }}", s(&v.id), s(&v.title), s(&v.channel), s(&v.duration), s(&v.watch_for), s(&v.fallback_search), s(&v.checked)))
        .collect();
    let sources: Vec<String> = f.sources.iter().map(|x| format!("Source {{ claim: {}, name: {}, url: {}, checked: {} }}", s(&x.claim), s(&x.name), s(&x.url), s(&x.checked))).collect();
    let qz = |q: &Quiz| format!("QuizQ {{ id: {}, q: {}, options: {}, answer: {}, why: {} }}", s(&q.id), s(&q.q), list(&q.options), q.answer, s(&q.why));
    let quiz: Vec<String> = f.quiz.iter().map(qz).collect();
    let recall: Vec<String> = f.recall.iter().map(qz).collect();
    let fixtures: Vec<String> = f
        .explain_back
        .fixtures
        .iter()
        .map(|fx| {
            let e = match fx.expect.as_str() {
                "got_it" => "GotIt",
                "almost" => "Almost",
                _ => "AlmostOrNotYet",
            };
            format!("Fixture {{ answer: {}, expect: Expect::{e} }}", s(&fx.answer))
        })
        .collect();
    let x = &f.explain_back;
    write!(
        out,
        "  Lesson {{ id: {}, stage: {}, title: {}, objective: {}, hook: {}, steps: &[{}], analogy: {}, analogy_tag: {}, example: {}, mistake: {}, takeaways: {}, read_minutes: {minutes}, videos: &[{}], sources: &[{}], quiz: &[{}], recall: &[{}], explain: ExplainBack {{ prompt: {}, must_include: {}, misconceptions: {}, fixtures: &[{}] }}, drills: {} }},\n",
        f.id,
        f.stage,
        s(&f.title),
        s(&f.objective),
        s(&f.hook),
        steps.join(", "),
        s(&f.analogy),
        s(&f.analogy_tag),
        s(&f.example),
        s(&f.mistake),
        list(&f.takeaways),
        videos.join(", "),
        sources.join(", "),
        quiz.join(", "),
        recall.join(", "),
        s(&x.prompt),
        list(&x.must_include),
        list(&x.misconceptions),
        fixtures.join(", "),
        list(&f.drills),
    )
    .unwrap();
}

fn main() {
    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("lessons");
    let mut errors = Vec::new();
    let en = read_dir(&dir, &mut errors, "");
    let fr = read_dir(&dir.join("fr"), &mut errors, "fr/");

    let mut out = String::from("pub static LESSONS: &[Lesson] = &[\n");
    let mut seen = Vec::new();
    for p in &en {
        if seen.contains(&p.front.id) {
            errors.push(format!("{}: duplicate id {}", p.name, p.front.id));
        }
        seen.push(p.front.id);
        let (errs, minutes) = validate(&p.name, &p.front, &p.body, false);
        errors.extend(errs);
        emit(&mut out, &p.front, minutes);
    }
    out.push_str("];\n\npub static LESSONS_FR: &[Lesson] = &[\n");
    for p in &fr {
        let Some(twin) = en.iter().find(|e| e.front.id == p.front.id) else {
            errors.push(format!("fr/{}: no English lesson {}", p.name, p.front.id));
            continue;
        };
        if twin.name != p.name {
            errors.push(format!("fr/{}: name it {} like the English lesson", p.name, twin.name));
        }
        let (errs, minutes) = validate(&format!("fr/{}", p.name), &p.front, &p.body, true);
        errors.extend(errs);
        errors.extend(parity(&p.name, &twin.front, &p.front));
        emit(&mut out, &p.front, minutes);
    }
    out.push_str("];\n");
    if !errors.is_empty() {
        panic!("\nLesson content failed validation:\n  - {}\n", errors.join("\n  - "));
    }
    fs::write(Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs"), out).unwrap();
}
