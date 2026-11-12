//! The interface language (English or French). One current language for the whole app: changing
//! it re-renders everything, so text can be picked with `t(en, fr)` at render time instead of
//! every string being reactive. Defaults to English (and stays English in tests).

use std::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    En,
    Fr,
}

impl Lang {
    /// The BCP 47 tag, for `<html lang>`, storage and `Intl`.
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        }
    }
    pub fn from_code(s: &str) -> Lang {
        if s.trim().to_ascii_lowercase().starts_with("fr") { Lang::Fr } else { Lang::En }
    }
}

thread_local! {
    static CURRENT: Cell<Lang> = const { Cell::new(Lang::En) };
}

pub fn set(lang: Lang) {
    CURRENT.with(|c| c.set(lang));
}

pub fn current() -> Lang {
    CURRENT.with(Cell::get)
}

pub fn is_fr() -> bool {
    current() == Lang::Fr
}

/// The text for the current language.
pub fn t(en: &'static str, fr: &'static str) -> &'static str {
    if is_fr() { fr } else { en }
}

/// Owned variant, for text built with `format!` in both languages.
pub fn ts(en: String, fr: String) -> String {
    if is_fr() { fr } else { en }
}

/// Added to every prompt sent to Professor Aurel while the course is in French. The prompts stay
/// in English (they are tuned there); this asks for French text with "tu" and leaves the JSON keys
/// and fixed values (grades and the like) exactly as the prompt specifies.
pub const FRENCH_REPLY: &str = "\n\nLANGUAGE: The learner is taking this course in French, and the lesson text above may be in French. Write every piece of text meant for the learner (feedback, explanations, questions, answer options, suggestions, plain-text answers) in natural, simple French, addressing the learner as \"tu\". When you rewrite or quote the learner's own answer, keep it in their language. Keep JSON keys and fixed values (such as grade names) exactly as specified above, in English.";

/// The prompt as sent: unchanged in English, with `FRENCH_REPLY` added in French.
pub fn localize_prompt(prompt: String) -> String {
    if is_fr() { prompt + FRENCH_REPLY } else { prompt }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_english_and_switches() {
        assert_eq!(t("Today", "Aujourd’hui"), "Today");
        set(Lang::Fr);
        assert_eq!(t("Today", "Aujourd’hui"), "Aujourd’hui");
        assert!(localize_prompt("Grade this.".into()).ends_with(FRENCH_REPLY));
        set(Lang::En);
        assert_eq!(localize_prompt("Grade this.".into()), "Grade this.");
        assert_eq!(Lang::from_code("fr-CA"), Lang::Fr);
        assert_eq!(Lang::from_code("en-GB"), Lang::En);
    }
}
