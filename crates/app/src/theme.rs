//! Theme: follows the system (or the viewer's own claude.ai choice) unless the Learner picks one.
//! Saved in localStorage as a per-device hint; Phase 4 also stores it in `profile.theme`.

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeChoice {
    System,
    Dark,
    Light,
}

const KEY: &str = "gs-theme";

fn storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

pub fn load() -> ThemeChoice {
    match storage().and_then(|s| s.get_item(KEY).ok().flatten()).as_deref() {
        Some("dark") => ThemeChoice::Dark,
        Some("light") => ThemeChoice::Light,
        _ => ThemeChoice::System,
    }
}

pub fn save(choice: ThemeChoice) {
    if let Some(s) = storage() {
        let _ = match choice {
            ThemeChoice::System => s.remove_item(KEY),
            ThemeChoice::Dark => s.set_item(KEY, "dark"),
            ThemeChoice::Light => s.set_item(KEY, "light"),
        };
    }
}

pub fn apply(choice: ThemeChoice) {
    let Some(root) = document().document_element() else { return };
    let _ = match choice {
        ThemeChoice::Dark => root.set_attribute("data-theme", "dark"),
        ThemeChoice::Light => root.set_attribute("data-theme", "light"),
        ThemeChoice::System => {
            // Hand back to whatever the viewer set before we touched it (recorded in index.html).
            let host = js_sys::Reflect::get(&window(), &"__gsHostTheme".into()).ok().and_then(|v| v.as_string());
            match host {
                Some(h) => root.set_attribute("data-theme", &h),
                None => root.remove_attribute("data-theme"),
            }
        }
    };
}
