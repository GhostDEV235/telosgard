//! Interface language (docs/FRENCH.md). The current language lives in `telosgard_core::i18n`;
//! the app keeps it in a signal too, and re-renders the whole shell when it changes, so text is
//! picked once per render with `t("English", "Français")`.

use leptos::prelude::*;
pub use telosgard_core::i18n::{is_fr, t, ts, Lang};

const KEY: &str = "gs-lang";

/// The language signal, provided by `App`.
pub type LangSignal = RwSignal<Lang>;

fn storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

/// Per-device hint for first paint (the sign-in screen); the profile is the source of truth.
pub fn load() -> Lang {
    storage().and_then(|s| s.get_item(KEY).ok().flatten()).map(|c| Lang::from_code(&c)).unwrap_or_default()
}

pub fn save(lang: Lang) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY, lang.code());
    }
}

/// Makes `lang` current for rendering and for screen readers (`<html lang>`, WCAG 3.1.1).
pub fn apply(lang: Lang) {
    telosgard_core::i18n::set(lang);
    if let Some(root) = document().document_element() {
        let _ = root.set_attribute("lang", lang.code());
    }
}

/// A written lesson in the current language (English where no French twin exists yet).
pub fn lesson(id: u8) -> Option<&'static telosgard_content::Lesson> {
    telosgard_content::lesson_in(is_fr(), id)
}

/// Every written lesson in the current language.
pub fn lessons() -> Vec<&'static telosgard_content::Lesson> {
    telosgard_content::lessons_in(is_fr())
}

/// "(en anglais)" after a video title in French: the course's videos are all in English.
pub fn english_only_note() -> &'static str {
    t("", " (en anglais)")
}
