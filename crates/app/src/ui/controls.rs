//! Buttons, segmented control, number field.

use leptos::prelude::*;

pub const PILL: &str = "press inline-flex min-h-11 items-center justify-center gap-2 rounded-btn bg-fill px-5 text-callout font-semibold text-on-fill disabled:opacity-40 disabled:active:scale-100";
pub const PILL_GHOST: &str = "press inline-flex min-h-11 items-center justify-center gap-2 rounded-btn border border-line px-5 text-callout font-semibold text-text";

#[component]
pub fn PillLink(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! { <a href=href class=PILL>{children()}</a> }
}

#[component]
pub fn Segmented<T>(options: Vec<(T, &'static str)>, value: RwSignal<T>, label: &'static str) -> impl IntoView
where
    T: Copy + PartialEq + Send + Sync + 'static,
{
    view! {
        <div role="radiogroup" aria-label=label class="flex w-full gap-1 rounded-btn bg-bg p-1">
            {options
                .into_iter()
                .map(|(v, text)| {
                    view! {
                        <button
                            type="button"
                            role="radio"
                            aria-checked=move || (value.get() == v).to_string()
                            on:click=move |_| value.set(v)
                            class=move || {
                                if value.get() == v {
                                    "press min-h-10 flex-1 rounded-[10px] bg-fill px-3 text-callout font-semibold text-on-fill"
                                } else {
                                    "press min-h-10 flex-1 rounded-[10px] px-3 text-callout text-text-muted"
                                }
                            }
                        >
                            {text}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn NumberField(
    label: &'static str,
    value: RwSignal<String>,
    #[prop(default = "")] prefix: &'static str,
    #[prop(default = "")] suffix: &'static str,
    #[prop(into, default = Signal::stored(None))] error: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <label class="block">
            <span class="text-footnote font-semibold text-text-muted">{label}</span>
            <span class=move || {
                format!(
                    "mt-1.5 flex min-h-13 items-center gap-1.5 rounded-field bg-bg px-3.5 focus-within:outline-2 focus-within:outline-offset-2 {}",
                    if error.get().is_some() { "focus-within:outline-down ring-1 ring-down" } else { "focus-within:outline-text" },
                )
            }>
                <span class="text-title2 text-text-muted" aria-hidden="true">{prefix}</span>
                <input
                    type="text"
                    inputmode="decimal"
                    autocomplete="off"
                    class="tabular min-w-0 flex-1 bg-transparent py-2 text-title2 text-text outline-none"
                    aria-invalid=move || error.get().is_some().to_string()
                    prop:value=move || value.get()
                    on:input=move |ev| value.set(event_target_value(&ev))
                />
                <span class="text-callout text-text-muted">{suffix}</span>
            </span>
            {move || error.get().map(|e| view! { <span class="mt-1.5 block text-footnote text-down">"⚠ " {e}</span> })}
        </label>
    }
}
