//! Bottom sheet (phone) / centred panel (desktop), and the glass toast.

use crate::state::Toasts;
use leptos::prelude::*;

#[component]
pub fn Sheet(
    open: RwSignal<bool>,
    title: &'static str,
    /// When false, the backdrop and Escape don't close it (the first-launch disclaimer).
    #[prop(default = true)]
    dismissible: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let _ = window_event_listener(leptos::ev::keydown, move |e| {
        if dismissible && e.key() == "Escape" && open.get_untracked() {
            open.set(false);
        }
    });
    // Move keyboard focus into the sheet when it opens (first field, else the first button).
    Effect::new(move |_| {
        if open.get() {
            request_animation_frame(|| {
                let doc = document();
                let target = doc
                    .query_selector("[role=dialog] textarea, [role=dialog] input")
                    .ok()
                    .flatten()
                    .or_else(|| doc.query_selector("[role=dialog] button").ok().flatten());
                if let Some(el) = target.and_then(|e| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(e).ok()) {
                    let _ = el.focus();
                }
            });
        }
    });
    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 z-40 flex items-end justify-center md:items-center md:p-6">
                <div
                    class="anim-fade absolute inset-0 bg-scrim"
                    aria-hidden="true"
                    on:click=move |_| {
                        if dismissible {
                            open.set(false)
                        }
                    }
                ></div>
                <div
                    role="dialog"
                    aria-modal="true"
                    aria-label=title
                    class="anim-sheet gutter relative max-h-[88dvh] w-full overflow-y-auto rounded-t-card bg-surface pt-2.5 pb-[max(24px,env(safe-area-inset-bottom,0px))] shadow-sheet md:max-w-lg md:rounded-card md:pt-5"
                >
                    <div class="mx-auto mb-3 h-1.5 w-10 rounded-full bg-line/60 md:hidden" aria-hidden="true"></div>
                    <div class="mb-4 flex items-center justify-between gap-4">
                        <h2 class="text-title2">{title}</h2>
                        {dismissible
                            .then(|| {
                                view! {
                                    <button
                                        type="button"
                                        aria-label="Close"
                                        class="press grid size-9 shrink-0 place-items-center rounded-full bg-bg text-text-muted"
                                        on:click=move |_| open.set(false)
                                    >
                                        "✕"
                                    </button>
                                }
                            })}
                    </div>
                    {children()}
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn Toast() -> impl IntoView {
    let toasts = expect_context::<Toasts>();
    view! {
        <div aria-live="polite" class="pointer-events-none fixed inset-x-0 top-[calc(env(safe-area-inset-top,0px)+12px)] z-50 flex justify-center gutter">
            {move || {
                toasts
                    .0
                    .get()
                    .map(|(_, msg)| {
                        view! { <p class="anim-toast glass max-w-md rounded-full border border-line/30 px-4 py-2 text-center text-callout shadow-sheet">{msg}</p> }
                    })
            }}
        </div>
    }
}
