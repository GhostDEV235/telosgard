//! Extra practice questions written by Claude after the lesson quiz. Practice only: they never
//! count towards a Gate. The prompt carries the lesson material; the reply is checked strictly and
//! malformed questions are dropped (too few good ones = an error, never a made-up quiz).

use serde_json::Value;

pub const MIN_GOOD: usize = 3;
const MAX_Q_CHARS: usize = 300;
const MAX_OPTION_CHARS: usize = 160;
const MAX_WHY_CHARS: usize = 400;

/// One lesson's material for question writing.
#[derive(Debug, Clone, Copy)]
pub struct PracticeSource<'a> {
    pub lesson: u8,
    pub title: &'a str,
    pub objective: &'a str,
    pub takeaways: &'a [&'a str],
    pub key_ideas: &'a [&'a str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticeQ {
    pub q: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub why: String,
    /// Which lesson it practises.
    pub lesson: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticeParseError {
    NotAList,
    TooFewGood { good: usize },
}

/// `avoid` = the questions the Learner has already seen, so Claude writes new ones.
pub fn build_practice_prompt(sources: &[PracticeSource], count: usize, avoid: &[&str]) -> String {
    let material = sources
        .iter()
        .map(|s| {
            format!(
                "LESSON {} — {}\nObjective: the learner can {}\nKey ideas: {}\nTakeaways:\n{}",
                s.lesson,
                s.title,
                s.objective,
                s.key_ideas.join("; "),
                s.takeaways.iter().map(|t| format!("- {t}")).collect::<Vec<_>>().join("\n")
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let avoid = if avoid.is_empty() { "none".to_string() } else { avoid.iter().map(|a| format!("- {a}")).collect::<Vec<_>>().join("\n") };
    format!(
        r#"You write practice questions for a complete beginner learning to trade gold (XAUUSD). Use ONLY the lesson material below; don't introduce ideas it doesn't cover.

{material}

Write exactly {count} NEW multiple-choice questions, spread across the lessons above. At least half must be applied: a small calculation or a realistic scenario, not just a definition. Each has 4 options with exactly one correct answer, and plausible wrong options built from common beginner mistakes.

Rules:
- Use illustrative gold prices near 4,100 (e.g. 4,100.00 to 4,250.00) and describe distances as a Dollar Move in dollars ($5, $12.50). Never present a price as real market data.
- Only use the word "pip" if a lesson above is about pips; if so, state the pip size in dollars.
- No trade recommendations and no "easy money".
- "why" is one or two plain sentences explaining the right answer.
- Don't repeat or lightly reword any of these questions the learner has already seen:
{avoid}

Reply with only this JSON object:
{{"questions": [{{"q": string, "options": [string, string, string, string], "answer": index of the correct option (0-3), "why": string, "lesson": lesson number}}]}}"#
    )
}

fn clean(v: &Value, key: &str, max: usize) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty() && s.chars().count() <= max).map(str::to_string)
}

fn one(v: &Value, lessons: &[u8], avoid: &[String]) -> Option<PracticeQ> {
    let q = clean(v, "q", MAX_Q_CHARS)?;
    if avoid.contains(&q.to_lowercase()) {
        return None;
    }
    let options: Vec<String> = v.get("options")?.as_array()?.iter().map(|o| o.as_str().map(str::trim).filter(|s| !s.is_empty() && s.chars().count() <= MAX_OPTION_CHARS).map(str::to_string)).collect::<Option<_>>()?;
    let mut unique: Vec<String> = options.iter().map(|o| o.to_lowercase()).collect();
    unique.sort();
    unique.dedup();
    if !(3..=5).contains(&options.len()) || unique.len() != options.len() {
        return None;
    }
    let answer = v.get("answer")?.as_u64()? as usize;
    let lesson = v.get("lesson")?.as_u64()? as u8;
    if answer >= options.len() || !lessons.contains(&lesson) {
        return None;
    }
    let why = clean(v, "why", MAX_WHY_CHARS)?;
    Some(PracticeQ { q, options, answer, why, lesson })
}

pub fn parse_practice(v: &Value, count: usize, lessons: &[u8], avoid: &[&str]) -> Result<Vec<PracticeQ>, PracticeParseError> {
    let list = v.get("questions").unwrap_or(v).as_array().ok_or(PracticeParseError::NotAList)?;
    let avoid: Vec<String> = avoid.iter().map(|a| a.trim().to_lowercase()).collect();
    let mut seen: Vec<String> = Vec::new();
    let mut out = Vec::new();
    for item in list {
        if let Some(q) = one(item, lessons, &avoid) {
            let key = q.q.to_lowercase();
            if !seen.contains(&key) {
                seen.push(key);
                out.push(q);
            }
        }
        if out.len() == count {
            break;
        }
    }
    if out.len() < MIN_GOOD.min(count) {
        return Err(PracticeParseError::TooFewGood { good: out.len() });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn src(lesson: u8, title: &'static str) -> PracticeSource<'static> {
        PracticeSource {
            lesson,
            title,
            objective: "explain what a trader profits from",
            takeaways: &["A trader profits from a change in price."],
            key_ideas: &["you buy and sell at different prices"],
        }
    }

    fn q(text: &str, lesson: u8) -> Value {
        json!({"q": text, "options": ["$15", "$1.50", "$150", "$1,500"], "answer": 0, "why": "4,115 − 4,100 = $15.", "lesson": lesson})
    }

    #[test]
    fn prompt_carries_material_count_avoid_list_and_format() {
        let p = build_practice_prompt(&[src(1, "What is trading?"), src(3, "Reading “XAUUSD”")], 5, &["What is a trader trying to profit from?"]);
        for needle in ["What is trading?", "Reading “XAUUSD”", "A trader profits from a change in price.", "exactly 5", "What is a trader trying to profit from?", "\"answer\"", "Dollar Move", "4,1"] {
            assert!(p.contains(needle), "missing {needle}");
        }
    }

    #[test]
    fn parses_good_questions() {
        let v = json!({"questions": [q("Gold moves from 4,100 to 4,115. How far?", 1), q("Gold moves from 4,200 to 4,215. How far?", 3), q("A $15 move: how many dollars?", 1)]});
        let out = parse_practice(&v, 5, &[1, 3], &[]).unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(out[1].lesson, 3);
        assert_eq!(out[0].options.len(), 4);
    }

    #[test]
    fn accepts_a_bare_array_and_caps_at_count() {
        let v = json!([q("One?", 1), q("Two?", 1), q("Three?", 1), q("Four?", 1)]);
        assert_eq!(parse_practice(&v, 3, &[1], &[]).unwrap().len(), 3);
    }

    #[test]
    fn drops_malformed_and_repeated_questions() {
        let v = json!([
            q("Good one?", 1),
            {"q": "Answer out of range?", "options": ["a", "b"], "answer": 5, "why": "x", "lesson": 1},
            {"q": "Duplicate options?", "options": ["a", "a", "b"], "answer": 0, "why": "x", "lesson": 1},
            {"q": "No why?", "options": ["a", "b", "c"], "answer": 1, "lesson": 1},
            q("Unknown lesson?", 9),
            q("What is a trader trying to profit from?", 1),
            q("Another good one?", 1),
            q("Third good one?", 1),
        ]);
        let out = parse_practice(&v, 8, &[1], &["what is a trader trying to profit from?"]).unwrap();
        let texts: Vec<_> = out.iter().map(|q| q.q.as_str()).collect();
        assert_eq!(texts, ["Good one?", "Another good one?", "Third good one?"]);
    }

    #[test]
    fn too_few_good_questions_is_an_error() {
        let v = json!([q("Only one?", 1), {"q": "bad"}]);
        assert_eq!(parse_practice(&v, 5, &[1], &[]), Err(PracticeParseError::TooFewGood { good: 1 }));
        assert_eq!(parse_practice(&json!("nope"), 5, &[1], &[]), Err(PracticeParseError::NotAList));
    }
}
