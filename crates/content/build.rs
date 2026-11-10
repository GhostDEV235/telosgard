//! Parses `lessons/NN-*.md` (TOML front matter between `+++` lines, Markdown body), enforces the
//! CURRICULUM authoring rules, and writes `generated.rs`. Any rule broken fails the build.

use serde::Deserialize;
use std::fmt::Write as _;
use std::{env, fs, path::Path};

const READ_WPM: usize = 130;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Front {
    id: u8,
    stage: u8,
    title: String,
    objective: String,
    hook: String,
    analogy: String,
    analogy_tag: String,
    example: String,
    mistake: String,
    takeaways: Vec<String>,
    #[serde(default)]
    videos: Vec<Video>,
    sources: Vec<Source>,
    quiz: Vec<Quiz>,
    explain_back: Explain,
    #[serde(default)]
    drills: Vec<String>,
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

const DRILLS: &[&str] = &["candle_direction", "bid_ask", "dollar_distance", "pip_convert", "order_type"];

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

fn validate(file: &str, f: &Front, body: &str) -> (Vec<String>, usize) {
    let mut e = Vec::new();
    let mut err = |m: String| e.push(format!("{file}: {m}"));
    let prefix: u8 = file.split('-').next().and_then(|n| n.parse().ok()).unwrap_or(0);
    if prefix != f.id {
        err(format!("file number {prefix} doesn't match id {}", f.id));
    }
    if f.id <= 11 && f.stage != 1 {
        err("lessons 1–11 are Stage 1".into());
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
    let prose = [f.hook.as_str(), body, &f.analogy, &f.example, &f.mistake].join(" ");
    if f.id != 6 && mentions_pips(&prose) {
        err("the word \"pip\" belongs in Lesson 6 only; use Dollar Moves".into());
    }
    let total = words(&prose) + f.takeaways.iter().map(|t| words(t)).sum::<usize>();
    let minutes = (total + READ_WPM / 2) / READ_WPM;
    if !(5..=8).contains(&minutes) {
        err(format!("reads in ~{minutes} min ({total} words); target 5–8"));
    }
    (e, minutes)
}

fn s(v: &str) -> String {
    format!("{v:?}")
}

fn list(v: &[String]) -> String {
    format!("&[{}]", v.iter().map(|x| s(x)).collect::<Vec<_>>().join(", "))
}

fn main() {
    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("lessons");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut files: Vec<_> = fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "md")).collect();
    files.sort();

    let mut errors = Vec::new();
    let mut out = String::from("pub static LESSONS: &[Lesson] = &[\n");
    let mut seen = Vec::new();
    for path in &files {
        println!("cargo:rerun-if-changed={}", path.display());
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let text = fs::read_to_string(path).unwrap();
        let Some(rest) = text.strip_prefix("+++\n") else {
            errors.push(format!("{name}: must start with +++ front matter"));
            continue;
        };
        let Some((front, body)) = rest.split_once("\n+++\n") else {
            errors.push(format!("{name}: front matter isn't closed with +++"));
            continue;
        };
        let f: Front = match toml::from_str(front) {
            Ok(f) => f,
            Err(e) => {
                errors.push(format!("{name}: {e}"));
                continue;
            }
        };
        if seen.contains(&f.id) {
            errors.push(format!("{name}: duplicate id {}", f.id));
        }
        seen.push(f.id);
        let (errs, minutes) = validate(&name, &f, body);
        errors.extend(errs);

        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new_ext(body.trim(), pulldown_cmark::Options::ENABLE_TABLES));

        let videos: Vec<String> = f
            .videos
            .iter()
            .map(|v| format!("Video {{ id: {}, title: {}, channel: {}, duration: {}, watch_for: {}, fallback_search: {}, checked: {} }}", s(&v.id), s(&v.title), s(&v.channel), s(&v.duration), s(&v.watch_for), s(&v.fallback_search), s(&v.checked)))
            .collect();
        let sources: Vec<String> = f.sources.iter().map(|x| format!("Source {{ claim: {}, name: {}, url: {}, checked: {} }}", s(&x.claim), s(&x.name), s(&x.url), s(&x.checked))).collect();
        let quiz: Vec<String> = f.quiz.iter().map(|q| format!("QuizQ {{ q: {}, options: {}, answer: {}, why: {} }}", s(&q.q), list(&q.options), q.answer, s(&q.why))).collect();
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
            "  Lesson {{ id: {}, stage: {}, title: {}, objective: {}, hook: {}, body_html: {}, analogy: {}, analogy_tag: {}, example: {}, mistake: {}, takeaways: {}, read_minutes: {minutes}, videos: &[{}], sources: &[{}], quiz: &[{}], explain: ExplainBack {{ prompt: {}, must_include: {}, misconceptions: {}, fixtures: &[{}] }}, drills: {} }},\n",
            f.id,
            f.stage,
            s(&f.title),
            s(&f.objective),
            s(&f.hook),
            s(&html),
            s(&f.analogy),
            s(&f.analogy_tag),
            s(&f.example),
            s(&f.mistake),
            list(&f.takeaways),
            videos.join(", "),
            sources.join(", "),
            quiz.join(", "),
            s(&x.prompt),
            list(&x.must_include),
            list(&x.misconceptions),
            fixtures.join(", "),
            list(&f.drills),
        )
        .unwrap();
    }
    out.push_str("];\n");
    if !errors.is_empty() {
        panic!("\nLesson content failed validation:\n  - {}\n", errors.join("\n  - "));
    }
    fs::write(Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs"), out).unwrap();
}
