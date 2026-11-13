//! The dashboard frame: a sidebar on wide screens, a bottom tab bar on phones, and a ⌘K palette.
//! Sections the role can't use are not shown; the API refuses them anyway.

use crate::api::{self, Me};
use crate::pages;
use crate::theme::ThemeChoice;
use crate::GateSignal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use telosgard_ui::controls::Segmented;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Overview,
    Learners,
    Enrollment,
    Curriculum,
    Settings,
    Audit,
    /// Phones: everything that isn't in the tab bar, plus the account.
    More,
}

impl Section {
    pub const NAV: [Section; 6] = [Section::Overview, Section::Learners, Section::Enrollment, Section::Curriculum, Section::Settings, Section::Audit];

    pub fn slug(self) -> &'static str {
        match self {
            Section::Overview => "overview",
            Section::Learners => "learners",
            Section::Enrollment => "enrollment",
            Section::Curriculum => "curriculum",
            Section::Settings => "settings",
            Section::Audit => "audit",
            Section::More => "more",
        }
    }
    pub fn title(self) -> &'static str {
        match self {
            Section::Overview => "Overview",
            Section::Learners => "Learners",
            Section::Enrollment => "Enrollment",
            Section::Curriculum => "Curriculum",
            Section::Settings => "Settings",
            Section::Audit => "Audit log",
            Section::More => "More",
        }
    }
    /// The permission that opens this section (roles.ts in the Worker).
    pub fn permission(self) -> Option<&'static str> {
        match self {
            Section::Overview => Some("overview.read"),
            Section::Learners => Some("learners.read"),
            Section::Enrollment => Some("invites.manage"),
            Section::Curriculum => Some("lessons.read"),
            Section::Settings => Some("settings.read"),
            Section::Audit => Some("audit.read"),
            Section::More => None,
        }
    }
    pub fn below_divider(self) -> bool {
        matches!(self, Section::Settings | Section::Audit)
    }
    fn parse(hash: &str) -> Section {
        let h = hash.trim_start_matches('#');
        if h.starts_with("learner-") {
            return Section::Learners;
        }
        if h.starts_with("lesson-") {
            return Section::Curriculum;
        }
        Section::NAV.into_iter().chain([Section::More]).find(|s| s.slug() == h).unwrap_or(Section::Overview)
    }
    fn icon(self) -> &'static str {
        match self {
            Section::Overview => "M4 13h6V4H4zM14 20h6V4h-6zM4 20h6v-4H4z",
            Section::Learners => "M9 11a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7zM3 20c0-3.3 2.7-6 6-6s6 2.7 6 6M16 4.5a3.5 3.5 0 0 1 0 6.5M18 14.3c1.8.8 3 2.7 3 5.7",
            Section::Enrollment => "M4 6.5h16v11H4zM4 7l8 6 8-6",
            Section::Curriculum => "M5 4.5h10a2 2 0 0 1 2 2v13H7a2 2 0 0 1-2-2zM17 8h2v11.5M8.5 9h5M8.5 12.5h5",
            Section::Settings => "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM19 12l2-1-1-3-2.2.2-1.6-1.6L16.4 4.4l-3-1-1 2h-1l-1-2-3 1 .2 2.2-1.6 1.6L3.8 8l-1 3 2 1v1l-2 1 1 3 2.2-.2 1.6 1.6-.2 2.2 3 1 1-2h1l1 2 3-1-.2-2.2 1.6-1.6 2.2.2 1-3-2-1z",
            Section::Audit => "M6 3.5h9l3 3v14H6zM9 10h6M9 13.5h6M9 17h4",
            Section::More => "M5 12h.01M12 12h.01M19 12h.01",
        }
    }
}

fn icon(s: Section, class: &'static str) -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" class=class aria-hidden="true">
            <path d=s.icon() fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    }
}

fn current() -> Section {
    Section::parse(&window().location().hash().unwrap_or_default())
}

#[component]
pub fn Shell(me: Me) -> impl IntoView {
    let section = RwSignal::new(current());
    // A Learner's profile panel is `#learner-<id>`: it opens over the list without leaving it.
    let selected = RwSignal::new(crate::learners::selected_from_hash());
    let lesson = RwSignal::new(crate::curriculum::selected_from_hash());
    let _ = window_event_listener(leptos::ev::hashchange, move |_| {
        let next = current();
        let sel = crate::learners::selected_from_hash();
        if next != section.get_untracked() {
            window().scroll_to_with_x_and_y(0.0, 0.0);
            section.set(next);
        }
        selected.set(sel);
        lesson.set(crate::curriculum::selected_from_hash());
    });
    let palette = RwSignal::new(false);
    let _ = window_event_listener(leptos::ev::keydown, move |ev| {
        if (ev.meta_key() || ev.ctrl_key()) && ev.key().eq_ignore_ascii_case("k") {
            ev.prevent_default();
            palette.update(|p| *p = !*p);
        } else if ev.key() == "Escape" {
            palette.set(false);
        }
    });
    let allowed: Vec<Section> = Section::NAV.into_iter().filter(|s| s.permission().is_some_and(|p| me.can(p))).collect();
    provide_context(StoredValue::new(me.clone()));

    let allowed_page = allowed.clone();
    let page_me = me.clone();
    let page = move || {
        let s = section.get();
        let s = if s == Section::More || allowed_page.contains(&s) { s } else { allowed_page.first().copied().unwrap_or(Section::More) };
        let me = page_me.clone();
        view! {
            <div class="anim-route">
                {match s {
                    Section::Overview => view! { <pages::Overview me=me /> }.into_any(),
                    Section::Learners => view! { <crate::learners::Learners me=me selected=selected /> }.into_any(),
                    Section::Enrollment => view! { <crate::enrollment::Enrollment /> }.into_any(),
                    Section::Curriculum => view! { <crate::curriculum::Curriculum me=me selected=lesson /> }.into_any(),
                    Section::Audit => view! { <pages::Audit me=me /> }.into_any(),
                    Section::More => view! { <pages::More sections=allowed_page.clone() /> }.into_any(),
                    other => view! { <pages::Coming section=other /> }.into_any(),
                }}
            </div>
        }
    };

    view! {
        <div class="lg:flex">
            <Sidebar sections=allowed.clone() section=section palette=palette me=me.clone() />
            <div class="min-h-dvh min-w-0 flex-1 pb-[calc(84px+env(safe-area-inset-bottom,0px))] lg:pb-10">
                <PhoneBar palette=palette />
                <main class="gutter mx-auto max-w-[1040px] pt-2 lg:pt-8">{page}</main>
            </div>
        </div>
        <TabBar sections=allowed.clone() section=section />
        <Palette open=palette sections=allowed find_learners=me.can("learners.read") />
    }
}

#[component]
fn Sidebar(sections: Vec<Section>, section: RwSignal<Section>, palette: RwSignal<bool>, me: Me) -> impl IntoView {
    let link = move |s: Section| {
        let active = move || section.get() == s;
        view! {
            <li>
                <a
                    href=format!("#{}", s.slug())
                    aria-current=move || active().then_some("page")
                    class=move || format!(
                        "press flex min-h-10 items-center gap-3 rounded-btn px-3 text-callout {}",
                        if active() { "bg-raised font-semibold text-text" } else { "text-text-muted hover:bg-raised/60" },
                    )
                >
                    {icon(s, "size-5 shrink-0")}
                    {s.title()}
                </a>
            </li>
        }
    };
    let (top, bottom): (Vec<Section>, Vec<Section>) = sections.into_iter().partition(|s| !s.below_divider());
    view! {
        <aside class="sticky top-0 hidden h-dvh w-[248px] shrink-0 flex-col border-r border-line/40 bg-surface px-3 py-5 lg:flex" aria-label="Dashboard">
            <a href="#overview" class="px-3">
                <p class="text-eyebrow text-eyebrow-ink">"Admin"</p>
                <p class="font-display text-title2">"Telosgard"</p>
            </a>
            <button
                type="button"
                class="press mt-4 flex min-h-10 items-center justify-between rounded-btn border border-line/60 bg-bg px-3 text-callout text-text-muted"
                on:click=move |_| palette.set(true)
            >
                "Jump to…"
                <kbd class="rounded border border-line/60 px-1.5 text-footnote">"⌘K"</kbd>
            </button>
            <nav class="mt-4 flex-1 overflow-y-auto" aria-label="Sections">
                <ul class="grid gap-0.5">{top.into_iter().map(link).collect_view()}</ul>
                {(!bottom.is_empty()).then(|| view! {
                    <hr class="my-3 border-line/40" />
                    <ul class="grid gap-0.5">{bottom.into_iter().map(link).collect_view()}</ul>
                })}
            </nav>
            <Account me=me />
        </aside>
    }
}

/// Who is signed in, the session's end, theme, and the ways out.
#[component]
pub fn Account(me: Me) -> impl IntoView {
    let gate = expect_context::<GateSignal>();
    let theme = expect_context::<RwSignal<ThemeChoice>>();
    let sign_out = move |_| {
        spawn_local(async move {
            let _ = api::post("/api/auth/sign-out", serde_json::json!({})).await;
            let _ = window().location().set_hash("");
            gate.0.set(crate::Gate::SignIn(None));
        });
    };
    view! {
        <div class="grid gap-3 border-t border-line/40 px-1 pt-4">
            <div class="min-w-0 px-2">
                <p class="truncate text-callout font-semibold">{me.email.clone().unwrap_or_default()}</p>
                <p class="text-footnote text-text-muted">
                    <span class="mr-1.5 rounded-full bg-raised px-2 py-0.5 font-semibold text-text">{me.role_label()}</span>
                    {me.session_ends_at.map(|t| format!("Signed in until {}", api::time_only(t)))}
                </p>
            </div>
            <Segmented label="Theme" value=theme options=vec![(ThemeChoice::System, "System"), (ThemeChoice::Dark, "Dark"), (ThemeChoice::Light, "Light")] />
            <div class="flex flex-wrap gap-x-4 px-2">
                <a href="/" class="press min-h-11 content-center text-callout underline underline-offset-4">"Open the course"</a>
                <button type="button" class="press min-h-11 text-callout underline underline-offset-4" on:click=sign_out>"Sign out"</button>
            </div>
        </div>
    }
}

#[component]
fn PhoneBar(palette: RwSignal<bool>) -> impl IntoView {
    view! {
        <header class="glass sticky top-[env(safe-area-inset-top,0px)] z-20 border-b border-line/30 lg:hidden">
            <div class="gutter mx-auto flex min-h-12 max-w-[1040px] items-center justify-between gap-3">
                <p class="font-display text-headline">"Telosgard " <span class="text-text-muted">"Admin"</span></p>
                <button type="button" aria-label="Jump to a section" class="press grid size-11 place-items-center rounded-full" on:click=move |_| palette.set(true)>
                    <svg viewBox="0 0 24 24" class="size-5" aria-hidden="true"><path d="M10.5 17a6.5 6.5 0 1 0 0-13 6.5 6.5 0 0 0 0 13zM20 20l-4.8-4.8" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" /></svg>
                </button>
            </div>
        </header>
    }
}

/// Phones: up to three sections, then More (the plan's Overview, Learners, Curriculum, More).
#[component]
fn TabBar(sections: Vec<Section>, section: RwSignal<Section>) -> impl IntoView {
    let mut tabs: Vec<Section> = [Section::Overview, Section::Learners, Section::Curriculum].into_iter().filter(|s| sections.contains(s)).collect();
    tabs.push(Section::More);
    view! {
        <nav aria-label="Sections" class="glass gutter fixed inset-x-0 bottom-0 z-30 border-t border-line/30 pb-[env(safe-area-inset-bottom,0px)] lg:hidden">
            <ul class="mx-auto flex max-w-[560px] justify-around">
                {tabs.into_iter().map(|s| {
                    let active = move || section.get() == s || (s == Section::More && !matches!(section.get(), Section::Overview | Section::Learners | Section::Curriculum));
                    view! {
                        <li>
                            <a
                                href=format!("#{}", s.slug())
                                aria-current=move || active().then_some("page")
                                class=move || format!("press flex min-h-14 min-w-16 flex-col items-center justify-center gap-0.5 px-2 text-footnote font-medium {}", if active() { "text-text" } else { "text-text-muted" })
                            >
                                {icon(s, "size-6")}
                                {s.title()}
                            </a>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </nav>
    }
}

/// ⌘K: type to jump to a section. Learners and lessons join the list in D1 and D2.
#[component]
fn Palette(open: RwSignal<bool>, sections: Vec<Section>, find_learners: bool) -> impl IntoView {
    let query = RwSignal::new(String::new());
    // Learners are fetched the first time the palette opens (only for roles that may see them).
    let people = RwSignal::new(Vec::<(String, String, String)>::new());
    let fetched = StoredValue::new(false);
    Effect::new(move |_| {
        if open.get() && find_learners && !fetched.get_value() {
            fetched.set_value(true);
            spawn_local(async move {
                if let Some(rows) = api::get("/api/admin/learners").await.parse::<Vec<crate::learners::Row>>() {
                    people.set(rows.into_iter().map(|r| (r.id, r.name, r.email)).collect());
                }
            });
        }
    });
    let people_matches = move || {
        let q = query.get().trim().to_lowercase();
        if q.len() < 2 {
            return Vec::new();
        }
        people.get().into_iter().filter(|(_, n, e)| n.to_lowercase().contains(&q) || e.to_lowercase().contains(&q)).take(8).collect::<Vec<_>>()
    };
    let input = NodeRef::<leptos::html::Input>::new();
    Effect::new(move |_| {
        if open.get() {
            query.set(String::new());
            if let Some(el) = input.get() {
                let _ = el.focus();
            }
        }
    });
    let sections = StoredValue::new(sections);
    let matches = move || {
        let q = query.get().to_lowercase();
        sections.get_value().into_iter().filter(|s| s.title().to_lowercase().contains(q.trim())).collect::<Vec<_>>()
    };
    let go = move |s: Section| {
        open.set(false);
        let _ = window().location().set_hash(s.slug());
    };
    view! {
        <Show when=move || open.get()>
            <div class="anim-fade fixed inset-0 z-50 bg-scrim" on:click=move |_| open.set(false)></div>
            <div role="dialog" aria-modal="true" aria-label="Jump to" class="anim-sheet fixed inset-x-0 top-[12vh] z-50 mx-auto w-[min(560px,calc(100%-32px))] overflow-hidden rounded-card bg-surface shadow-sheet">
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    if let Some(first) = matches().first() {
                        go(*first);
                    } else if let Some((id, _, _)) = people_matches().first() {
                        open.set(false);
                        let _ = window().location().set_hash(&format!("learner-{id}"));
                    }
                }>
                    <label class="sr-only" for="palette-q">"Jump to"</label>
                    <input
                        id="palette-q"
                        node_ref=input
                        type="text"
                        autocomplete="off"
                        placeholder=if find_learners { "Jump to a section or Learner…" } else { "Jump to a section…" }
                        class="block min-h-14 w-full border-b border-line/40 bg-transparent px-5 text-body text-text outline-none"
                        prop:value=query
                        on:input=move |ev| query.set(event_target_value(&ev))
                    />
                </form>
                <ul class="max-h-[50vh] overflow-y-auto p-2">
                    {move || people_matches().into_iter().map(|(id, name, email)| view! {
                        <li>
                            <a href=format!("#learner-{id}") on:click=move |_| open.set(false) class="press flex min-h-11 w-full items-center gap-3 rounded-btn px-3 text-left text-body hover:bg-raised">
                                {icon(Section::Learners, "size-5 text-text-muted")}
                                <span class="min-w-0 truncate">{if name.is_empty() { email.clone() } else { name }}<span class="ml-2 text-footnote text-text-muted">{email.clone()}</span></span>
                            </a>
                        </li>
                    }).collect_view()}
                    {move || {
                        let list = matches();
                        if list.is_empty() && people_matches().is_empty() {
                            view! { <li class="px-3 py-4 text-callout text-text-muted">"Nothing matches."</li> }.into_any()
                        } else {
                            list.into_iter().map(|s| view! {
                                <li>
                                    <button type="button" class="press flex min-h-11 w-full items-center gap-3 rounded-btn px-3 text-left text-body hover:bg-raised" on:click=move |_| go(s)>
                                        {icon(s, "size-5 text-text-muted")}
                                        {s.title()}
                                    </button>
                                </li>
                            }).collect_view().into_any()
                        }
                    }}
                </ul>
            </div>
        </Show>
    }
}
