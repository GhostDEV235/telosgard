//! Accounts (ADR 0004): the sign-in screen, joining with an invite link, and the account section of
//! the Appearance sheet (sign out; for the owner, invite a tester). Telosgard is invite-only.

use crate::i18n::{t, ts};
use crate::platform::{self, PlatformError};
use crate::ui::controls::{PILL, PILL_GHOST};
use leptos::prelude::*;
use leptos::task::spawn_local;

const FIELD: &str = "mt-1.5 min-h-12 w-full rounded-field bg-surface px-3.5 text-body shadow-card placeholder:text-text-muted";

fn reload_to(hash: &str) {
    let loc = window().location();
    let _ = loc.set_hash(hash);
    let _ = loc.reload();
}

#[component]
fn Card(children: Children) -> impl IntoView {
    let lang = expect_context::<crate::i18n::LangSignal>();
    // Signed out, the Appearance sheet isn't reachable, so the language switch lives here too.
    let other = move || match lang.get() {
        crate::i18n::Lang::En => (crate::i18n::Lang::Fr, "Français", "fr"),
        crate::i18n::Lang::Fr => (crate::i18n::Lang::En, "English", "en"),
    };
    view! {
        <main class="gutter grid min-h-dvh place-items-center py-10">
            <div class="w-full max-w-sm">
                <div class="mb-6 flex items-center gap-2.5">
                    <span class="grid size-8 place-items-center rounded-[9px] bg-gold-fill pb-px font-display text-headline font-semibold text-on-gold" aria-hidden="true">"T"</span>
                    <span class="font-display text-title2">"Telosgard"</span>
                    <button
                        type="button"
                        lang=move || other().2
                        class="press ml-auto min-h-11 px-2 text-callout underline underline-offset-4"
                        on:click=move |_| {
                            let next = other().0;
                            crate::i18n::save(next);
                            lang.set(next);
                        }
                    >
                        {move || other().1}
                    </button>
                </div>
                {children()}
            </div>
        </main>
    }
}

#[component]
pub fn SignIn() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let (e, p) = (email.get_untracked().trim().to_string(), password.get_untracked());
        if e.is_empty() || p.is_empty() {
            return error.set(Some(t("Enter your email and password.", "Saisis ton e-mail et ton mot de passe.").into()));
        }
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match platform::auth_sign_in(&e, &p).await {
                Ok(()) => reload_to("today"),
                Err(err) => {
                    error.set(Some(match err {
                        PlatformError::Auth(m) if m.to_lowercase().contains("invalid") => t("That email and password don't match.", "Cet e-mail et ce mot de passe ne correspondent pas.").into(),
                        PlatformError::Auth(m) if m.to_lowercase().contains("suspended") => t("This account is suspended. Contact the person who invited you.", "Ce compte est suspendu. Contacte la personne qui t’a envoyé l’invitation.").into(),
                        other => other.describe(),
                    }));
                    busy.set(false);
                }
            }
        });
    };
    view! {
        <Card>
            <h1 class="text-title1">{t("Sign in to ", "Connecte-toi pour ")} <em>{t("learn with purpose.", "apprendre avec méthode.")}</em></h1>
            <p class="mt-2 text-callout text-text-muted">{t("Telosgard is invite-only. New here? Open the invite link you were sent, or enter your invite code below.", "Telosgard fonctionne sur invitation. Première visite ? Ouvre le lien d’invitation que tu as reçu, ou saisis ton code d’invitation plus bas.")}</p>
            <form class="mt-6 grid gap-4" on:submit=submit>
                <label class="block">
                    <span class="text-callout font-semibold">{t("Email", "E-mail")}</span>
                    <input type="email" autocomplete="email" required class=FIELD prop:value=move || email.get() on:input=move |ev| email.set(event_target_value(&ev)) />
                </label>
                <label class="block">
                    <span class="text-callout font-semibold">{t("Password", "Mot de passe")}</span>
                    <input type="password" autocomplete="current-password" required class=FIELD prop:value=move || password.get() on:input=move |ev| password.set(event_target_value(&ev)) />
                </label>
                {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">{e}</p> })}
                <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>{move || if busy.get() { t("Signing in…", "Connexion…") } else { t("Sign in", "Se connecter") }}</button>
            </form>
            <InviteCode />
            <p class="mt-6 text-footnote text-text-muted">{t("Forgot your password? Ask the person who invited you for a reset link.", "Mot de passe oublié ? Demande un lien de réinitialisation à la personne qui t’a envoyé l’invitation.")}</p>
        </Card>
    }
}

/// "Have an invite code?": opens Join with the code, which the server checks at sign-up.
#[component]
fn InviteCode() -> impl IntoView {
    let code = RwSignal::new(String::new());
    let go = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let c: String = code.get_untracked().trim().to_uppercase().split_whitespace().collect::<Vec<_>>().join("-");
        if (3..=24).contains(&c.len()) && c.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-') {
            let _ = window().location().set_hash(&format!("join-{c}"));
        }
    };
    view! {
        <details class="mt-6">
            <summary class="press min-h-11 cursor-pointer content-center text-callout underline underline-offset-4">{t("Have an invite code?", "Tu as un code d’invitation ?")}</summary>
            <form class="mt-2 flex gap-2" on:submit=go>
                <label class="sr-only" for="invite-code">{t("Invite code", "Code d’invitation")}</label>
                <input id="invite-code" type="text" autocapitalize="characters" autocomplete="off" placeholder="GOLD-NOV" class=format!("{FIELD} mt-0 flex-1 uppercase") prop:value=move || code.get() on:input=move |ev| code.set(event_target_value(&ev)) />
                <button type="submit" class=PILL_GHOST>{t("Continue", "Continuer")}</button>
            </form>
        </details>
    }
}

/// A one-time link from the Owner or Support: choose a new password (signed out).
#[component]
pub fn ResetPassword(token: String) -> impl IntoView {
    let password = RwSignal::new(String::new());
    let again = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let done = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let token = StoredValue::new(token);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let (p, a) = (password.get_untracked(), again.get_untracked());
        if p.chars().count() < 10 {
            return error.set(Some(t("Choose a password of at least 10 characters.", "Choisis un mot de passe d’au moins 10 caractères.").into()));
        }
        if p != a {
            return error.set(Some(t("The two passwords don't match.", "Les deux mots de passe ne correspondent pas.").into()));
        }
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match platform::password_reset(&token.get_value(), &p).await {
                Ok(()) => done.set(true),
                Err(PlatformError::Auth(m)) if m.to_lowercase().contains("expired") => {
                    error.set(Some(t("This link has been used or has expired. Ask for a new one.", "Ce lien a déjà servi ou a expiré. Demandes-en un nouveau.").into()))
                }
                Err(e) => error.set(Some(e.describe())),
            }
            busy.set(false);
        });
    };
    view! {
        <Card>
            <h1 class="text-title1">{t("Choose a ", "Choisis un ")} <em>{t("new password.", "nouveau mot de passe.")}</em></h1>
            {move || if done.get() {
                view! {
                    <p class="mt-3 text-callout" role="status">{t("Your password is changed, and you're signed out everywhere. Sign in with the new one.", "Ton mot de passe est changé, et toutes tes sessions sont fermées. Connecte-toi avec le nouveau.")}</p>
                    <a href="#today" on:click=move |_| reload_to("today") class=format!("{PILL} mt-6 w-full")>{t("Sign in", "Se connecter")}</a>
                }.into_any()
            } else {
                view! {
                    <p class="mt-2 text-callout text-text-muted">{t("This link works once, for 24 hours.", "Ce lien ne marche qu’une fois, pendant 24 heures.")}</p>
                    <form class="mt-6 grid gap-4" on:submit=submit>
                        <label class="block">
                            <span class="text-callout font-semibold">{t("New password", "Nouveau mot de passe")}</span>
                            <input type="password" autocomplete="new-password" required class=FIELD prop:value=move || password.get() on:input=move |ev| password.set(event_target_value(&ev)) />
                            <span class="mt-1 block text-footnote text-text-muted">{t("At least 10 characters.", "Au moins 10 caractères.")}</span>
                        </label>
                        <label class="block">
                            <span class="text-callout font-semibold">{t("Type it again", "Saisis-le encore une fois")}</span>
                            <input type="password" autocomplete="new-password" required class=FIELD prop:value=move || again.get() on:input=move |ev| again.set(event_target_value(&ev)) />
                        </label>
                        {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">{e}</p> })}
                        <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>{move || if busy.get() { t("Saving…", "Enregistrement…") } else { t("Save new password", "Enregistrer le mot de passe") }}</button>
                    </form>
                }.into_any()
            }}
        </Card>
    }
}

#[component]
pub fn Join(token: String) -> impl IntoView {
    let token = StoredValue::new(token);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let (n, e, p) = (name.get_untracked().trim().to_string(), email.get_untracked().trim().to_string(), password.get_untracked());
        if n.is_empty() || e.is_empty() {
            return error.set(Some(t("Enter your name and email.", "Saisis ton nom et ton e-mail.").into()));
        }
        if p.chars().count() < 10 {
            return error.set(Some(t("Choose a password of at least 10 characters.", "Choisis un mot de passe d’au moins 10 caractères.").into()));
        }
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match platform::auth_sign_up(&n, &e, &p, &token.get_value()).await {
                Ok(()) => reload_to("today"),
                Err(err) => {
                    error.set(Some(match err {
                        PlatformError::Auth(m) if m.to_lowercase().contains("invite") => t("This invite has been used up or has expired. Ask for a new one.", "Cette invitation a déjà servi ou a expiré. Demandes-en une nouvelle.").into(),
                        PlatformError::Auth(m) if m.to_lowercase().contains("exist") => t("There's already an account with that email. Sign in instead.", "Un compte existe déjà avec cet e-mail. Connecte-toi plutôt.").into(),
                        other => other.describe(),
                    }));
                    busy.set(false);
                }
            }
        });
    };
    view! {
        <Card>
            <h1 class="text-title1">{t("You're ", "Tu as reçu ")} <em>{t("invited.", "une invitation.")}</em></h1>
            <p class="mt-2 text-callout text-text-muted">{t("Create your Telosgard account: a patient, risk-first course in trading gold. Education, not financial advice.", "Crée ton compte Telosgard : un cours patient sur le trading de l’or, qui commence par le risque. De la formation, pas du conseil financier.")}</p>
            <form class="mt-6 grid gap-4" on:submit=submit>
                <label class="block">
                    <span class="text-callout font-semibold">{t("Your name", "Ton nom")}</span>
                    <input type="text" autocomplete="name" required class=FIELD prop:value=move || name.get() on:input=move |ev| name.set(event_target_value(&ev)) />
                </label>
                <label class="block">
                    <span class="text-callout font-semibold">{t("Email", "E-mail")}</span>
                    <input type="email" autocomplete="email" required class=FIELD prop:value=move || email.get() on:input=move |ev| email.set(event_target_value(&ev)) />
                </label>
                <label class="block">
                    <span class="text-callout font-semibold">{t("Password", "Mot de passe")}</span>
                    <input type="password" autocomplete="new-password" minlength="10" required class=FIELD prop:value=move || password.get() on:input=move |ev| password.set(event_target_value(&ev)) />
                    <span class="mt-1 block text-footnote text-text-muted">{t("At least 10 characters.", "Au moins 10 caractères.")}</span>
                </label>
                {move || error.get().map(|e| view! { <p class="text-callout text-down" role="alert">{e}</p> })}
                <button type="submit" class=format!("{PILL} w-full") disabled=move || busy.get()>{move || if busy.get() { t("Creating your account…", "Création de ton compte…") } else { t("Create account", "Créer le compte") }}</button>
            </form>
            <a href="#today" on:click=move |_| reload_to("today") class="mt-6 inline-block text-callout underline underline-offset-4">{t("I already have an account", "J’ai déjà un compte")}</a>
        </Card>
    }
}

/// Inside the Appearance sheet: who is signed in, sign out, and (owner) invite a tester.
#[component]
pub fn AccountSection() -> impl IntoView {
    let store = expect_context::<crate::store::Store>();
    let toasts = expect_context::<crate::state::Toasts>();
    let invite = RwSignal::new(None::<platform::Invite>);
    let busy = RwSignal::new(false);
    let make_invite = move |_| {
        busy.set(true);
        spawn_local(async move {
            match platform::invite_create().await {
                Ok(i) => invite.set(Some(i)),
                Err(e) => toasts.show(e.describe()),
            }
            busy.set(false);
        });
    };
    let copy = move |_| {
        if let Some(i) = invite.get_untracked() {
            spawn_local(async move {
                toasts.show(if platform::copy_text(&i.url).await { t("Invite link copied", "Lien d’invitation copié") } else { t("Select the link and copy it", "Sélectionne le lien et copie-le") });
            });
        }
    };
    let sign_out = move |_| {
        spawn_local(async move {
            let _ = platform::auth_sign_out().await;
            reload_to("today");
        });
    };
    move || {
        store.account.get().map(|(email, owner)| {
            view! {
                <hr class="my-5 border-line/30" />
                <p class="text-eyebrow text-eyebrow-ink">{t("Account", "Compte")}</p>
                <p class="mt-1 text-callout">{ts(format!("Signed in as {}", email.clone().unwrap_or_default()), format!("Compte : {}", email.clone().unwrap_or_default()))}</p>
                {owner.then(|| view! {
                    <div class="mt-4">
                        <button type="button" class=PILL_GHOST disabled=move || busy.get() on:click=make_invite>{t("Invite a tester", "Inviter une personne testeuse")}</button>
                        <p class="mt-2 text-footnote text-text-muted">{t("A single-use link, valid for 7 days. Send it however you like; testers can ask Professor Aurel 50 things a day.", "Un lien à usage unique, valable 7 jours. Envoie-le comme tu veux ; chaque personne testeuse peut poser 50 questions par jour au Professeur Aurel.")}</p>
                        {move || invite.get().map(|i| view! {
                            <div class="mt-3 rounded-btn bg-surface p-3 shadow-card">
                                <input readonly aria-label=t("Invite link", "Lien d’invitation") class="w-full truncate bg-transparent text-footnote" prop:value=i.url.clone() on:focus=move |ev| { let _ = event_target::<web_sys::HtmlInputElement>(&ev).select(); } />
                                <button type="button" class=format!("{PILL} mt-2") on:click=copy>{t("Copy link", "Copier le lien")}</button>
                            </div>
                        })}
                    </div>
                })}
                <button type="button" class="press mt-4 min-h-11 text-callout underline underline-offset-4" on:click=sign_out>{t("Sign out", "Se déconnecter")}</button>
                {(!owner).then(|| view! { <DeleteAccount /> })}
            }
        })
    }
}

/// Account Deletion by the Learner (ADR 0008): two steps, password, then a 7-day grace.
#[component]
fn DeleteAccount() -> impl IntoView {
    let open = RwSignal::new(false);
    let password = RwSignal::new(String::new());
    let busy = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let confirm = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        busy.set(true);
        error.set(None);
        spawn_local(async move {
            match platform::account_delete(&password.get_untracked()).await {
                Ok(()) => reload_to("today"),
                Err(PlatformError::Auth(m)) if m.to_lowercase().contains("password") => {
                    error.set(Some(t("That password isn't right.", "Ce mot de passe n’est pas le bon.").into()));
                    busy.set(false);
                }
                Err(e) => {
                    error.set(Some(e.describe()));
                    busy.set(false);
                }
            }
        });
    };
    view! {
        <div class="mt-2">
            {move || if !open.get() {
                view! {
                    <button type="button" class="press min-h-11 text-callout text-down underline underline-offset-4" on:click=move |_| open.set(true)>{t("Delete my account", "Supprimer mon compte")}</button>
                }.into_any()
            } else {
                view! {
                    <form class="mt-2 rounded-btn border border-down/40 p-4" on:submit=confirm>
                        <p class="text-headline">{t("Delete your account?", "Supprimer ton compte ?")}</p>
                        <p class="mt-1 text-callout text-text-muted">{t("You'll be signed out now. After 7 days, your name, email, journal, plans and answers are erased for good; only anonymous totals remain. To change your mind within 7 days, contact the person who invited you.", "Ta session se ferme tout de suite. Au bout de 7 jours, ton nom, ton e-mail, ton journal, tes plans et tes réponses sont effacés pour de bon ; seuls des totaux anonymes restent. Pour changer d’avis dans les 7 jours, contacte la personne qui t’a envoyé l’invitation.")}</p>
                        <label class="mt-3 block">
                            <span class="text-callout font-semibold">{t("Your password", "Ton mot de passe")}</span>
                            <input type="password" autocomplete="current-password" required class=FIELD prop:value=move || password.get() on:input=move |ev| password.set(event_target_value(&ev)) />
                        </label>
                        {move || error.get().map(|e| view! { <p class="mt-2 text-callout text-down" role="alert">{e}</p> })}
                        <div class="mt-3 flex flex-wrap gap-3">
                            <button type="submit" class="press inline-flex min-h-11 items-center rounded-full bg-down px-5 text-callout font-semibold text-surface" disabled=move || busy.get()>{t("Delete my account", "Supprimer mon compte")}</button>
                            <button type="button" class=PILL_GHOST on:click=move |_| { open.set(false); password.set(String::new()); }>{t("Keep it", "Le garder")}</button>
                        </div>
                    </form>
                }.into_any()
            }}
        </div>
    }
}
