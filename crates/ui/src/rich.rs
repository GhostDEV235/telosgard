//! The small text format the dashboard writes and both apps show (the Disclaimer, D3): paragraphs
//! separated by a blank line, and `**bold**`. Nothing else is markup, so text from the settings table
//! can never become HTML.

use leptos::prelude::*;

/// Paragraphs, each a run of (bold, text) pieces. Blank paragraphs are dropped; an unclosed `**`
/// stays as plain text.
pub fn paragraphs(text: &str) -> Vec<Vec<(bool, String)>> {
    let normalized = text.replace("\r\n", "\n");
    let mut out = Vec::new();
    for block in normalized.split("\n\n") {
        let block = block.split('\n').map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ");
        if block.is_empty() {
            continue;
        }
        let parts: Vec<&str> = block.split("**").collect();
        // An even number of parts means an odd number of `**`: the last one has no partner.
        let closed = if parts.len() % 2 == 0 { parts.len() - 1 } else { parts.len() };
        let mut runs: Vec<(bool, String)> = Vec::new();
        for (i, p) in parts.iter().enumerate() {
            let bold = i % 2 == 1 && i < closed;
            let piece = if i >= closed { format!("**{p}") } else { (*p).to_string() };
            if piece.is_empty() {
                continue;
            }
            match runs.last_mut() {
                Some((b, s)) if *b == bold => s.push_str(&piece),
                _ => runs.push((bold, piece)),
            }
        }
        out.push(runs);
    }
    out
}

/// Renders [`paragraphs`] as `<p>` elements inside a grid with the given classes.
#[component]
pub fn Rich(#[prop(into)] text: String, #[prop(default = "grid gap-3 text-body")] class: &'static str) -> impl IntoView {
    let paras = paragraphs(&text);
    view! {
        <div class=class>
            {paras.into_iter().map(|runs| view! {
                <p>{runs.into_iter().map(|(bold, s)| if bold { view! { <strong>{s}</strong> }.into_any() } else { s.into_any() }).collect_view()}</p>
            }).collect_view()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::paragraphs;

    #[test]
    fn splits_paragraphs_and_bold() {
        let p = paragraphs("It is **education, not advice**.\n\nSecond\nline.\n\n\n");
        assert_eq!(p.len(), 2);
        assert_eq!(p[0], vec![(false, "It is ".into()), (true, "education, not advice".into()), (false, ".".into())]);
        assert_eq!(p[1], vec![(false, "Second line.".into())]);
    }

    #[test]
    fn leaves_an_unclosed_marker_as_text_and_never_html() {
        let p = paragraphs("a **b** c ** d <script>");
        assert_eq!(p[0], vec![(false, "a ".into()), (true, "b".into()), (false, " c ** d <script>".into())]);
    }
}
