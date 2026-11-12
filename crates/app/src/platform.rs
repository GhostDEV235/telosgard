//! Rust side of `bridge.js`: typed async calls over JSON strings (ARCHITECTURE §4).

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bridge.js")]
extern "C" {
    #[wasm_bindgen(js_name = init)]
    async fn js_init() -> JsValue;
    #[wasm_bindgen(js_name = dbGet)]
    async fn js_get(doc: &str) -> JsValue;
    #[wasm_bindgen(js_name = dbSet)]
    async fn js_set(doc: &str, json: &str) -> JsValue;
    #[wasm_bindgen(js_name = dbDelete)]
    async fn js_delete(doc: &str) -> JsValue;
    #[wasm_bindgen(js_name = dbList)]
    async fn js_list(kind: &str) -> JsValue;
    #[wasm_bindgen(js_name = dbSetShared)]
    async fn js_set_shared(path: &str, json: &str) -> JsValue;
    #[wasm_bindgen(js_name = uploadFromInput)]
    async fn js_upload(input_id: &str) -> JsValue;
    #[wasm_bindgen(js_name = sampleJson)]
    async fn js_sample(prompt: &str, tier: &str) -> JsValue;
    #[wasm_bindgen(js_name = sampleText)]
    async fn js_sample_text(prompt: &str, tier: &str) -> JsValue;
    #[wasm_bindgen(js_name = authSignIn)]
    async fn js_auth_sign_in(email: &str, password: &str) -> JsValue;
    #[wasm_bindgen(js_name = authSignUp)]
    async fn js_auth_sign_up(name: &str, email: &str, password: &str, invite: &str) -> JsValue;
    #[wasm_bindgen(js_name = authSignOut)]
    async fn js_auth_sign_out() -> JsValue;
    #[wasm_bindgen(js_name = inviteCreate)]
    async fn js_invite_create() -> JsValue;
    #[wasm_bindgen(js_name = copyText)]
    async fn js_copy_text(text: &str) -> JsValue;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformError {
    /// The capability isn't granted or isn't available in this view (preview, signed out, declined).
    NotGranted,
    /// Transient: network or platform. Already retried once in the bridge.
    Unavailable,
    /// Access withdrawn while the page was open. Terminal for this page load.
    Revoked,
    /// A storage cap is full.
    Quota(String),
    /// Too many calls; slow down.
    RateLimited,
    /// A bad request or a stored document that doesn't match the expected shape. A bug, not the Learner's fault.
    Invalid(String),
    /// Claude declined or couldn't answer.
    Claude(String),
    /// Sign-in or sign-up refused; the message is the server's own.
    Auth(String),
}

impl PlatformError {
    fn from_code(code: &str, message: String) -> PlatformError {
        match code {
            "not_granted" | "capability_disabled" | "capability_removed" | "not_declared" | "sampling_disabled" | "session_expired" => PlatformError::NotGranted,
            "revoked" => PlatformError::Revoked,
            "quota_exceeded" => PlatformError::Quota(message),
            "resource_exhausted" | "rate_limited" => PlatformError::RateLimited,
            "invalid_argument" | "transform_error" | "invalid_request" | "prompt_too_large" => PlatformError::Invalid(message),
            "refused" => PlatformError::Claude(message),
            "auth" => PlatformError::Auth(message),
            "invalid_json" => PlatformError::Claude("its reply couldn't be read".into()),
            _ => PlatformError::Unavailable,
        }
    }

    /// Short, plain-language text for a toast or banner.
    pub fn describe(&self) -> String {
        match self {
            PlatformError::NotGranted => "This isn't available in this view (permission not granted, or signed out).".into(),
            PlatformError::Unavailable => "Couldn't reach Telosgard's server. Check your connection and try again.".into(),
            PlatformError::Revoked => "Access to this page changed. Reload to continue.".into(),
            PlatformError::Quota(_) => "Storage is full. Nothing new can be saved.".into(),
            PlatformError::RateLimited => "Too many requests. Wait a moment and try again.".into(),
            PlatformError::Invalid(m) => format!("Something went wrong saving ({m})."),
            PlatformError::Claude(m) => format!("Professor Aurel couldn't answer ({m})."),
            PlatformError::Auth(m) => m.clone(),
        }
    }
}

#[derive(Deserialize)]
struct Envelope {
    ok: bool,
    value: Option<Value>,
    code: Option<String>,
    message: Option<String>,
}

fn unwrap(v: JsValue) -> Result<Value, PlatformError> {
    let text = v.as_string().ok_or_else(|| PlatformError::Invalid("bridge returned a non-string".into()))?;
    let env: Envelope = serde_json::from_str(&text).map_err(|e| PlatformError::Invalid(e.to_string()))?;
    if env.ok {
        Ok(env.value.unwrap_or(Value::Null))
    } else {
        Err(PlatformError::from_code(env.code.as_deref().unwrap_or(""), env.message.unwrap_or_default()))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Caps {
    /// Storage usable: db granted AND a signed-in viewer to own the private folder.
    pub db: bool,
    pub sample: bool,
    /// The `user` capability answered (so a missing id means signed out, not "no platform").
    pub user: bool,
    pub signed_in: bool,
    #[serde(default)]
    pub email: Option<String>,
    /// The signed-in account is the owner (may create invites).
    #[serde(default)]
    pub owner: bool,
    /// Every lesson is open for this account (a reviewer), whatever its progress.
    #[serde(default)]
    pub full_access: bool,
}

pub async fn auth_sign_in(email: &str, password: &str) -> Result<(), PlatformError> {
    unwrap(js_auth_sign_in(email, password).await).map(|_| ())
}

pub async fn auth_sign_up(name: &str, email: &str, password: &str, invite: &str) -> Result<(), PlatformError> {
    unwrap(js_auth_sign_up(name, email, password, invite).await).map(|_| ())
}

pub async fn auth_sign_out() -> Result<(), PlatformError> {
    unwrap(js_auth_sign_out().await).map(|_| ())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub url: String,
}

pub async fn invite_create() -> Result<Invite, PlatformError> {
    serde_json::from_value(unwrap(js_invite_create().await)?).map_err(|e| PlatformError::Invalid(e.to_string()))
}

pub async fn copy_text(text: &str) -> bool {
    unwrap(js_copy_text(text).await).ok().and_then(|v| v.as_bool()).unwrap_or(false)
}

pub async fn init() -> Result<Caps, PlatformError> {
    let v = unwrap(js_init().await)?;
    serde_json::from_value(v).map_err(|e| PlatformError::Invalid(e.to_string()))
}

pub async fn get<T: DeserializeOwned>(doc: &str) -> Result<Option<T>, PlatformError> {
    match unwrap(js_get(doc).await)? {
        Value::Null => Ok(None),
        v => serde_json::from_value(v).map(Some).map_err(|e| PlatformError::Invalid(format!("{doc}: {e}"))),
    }
}

pub async fn set<T: Serialize>(doc: &str, body: &T) -> Result<(), PlatformError> {
    let json = serde_json::to_string(body).map_err(|e| PlatformError::Invalid(e.to_string()))?;
    unwrap(js_set(doc, &json).await).map(|_| ())
}

pub async fn delete(doc: &str) -> Result<(), PlatformError> {
    unwrap(js_delete(doc).await).map(|_| ())
}

#[derive(Deserialize)]
struct Listed {
    id: String,
    data: Value,
}

/// Every document of one kind. Documents that don't match `T` are skipped (and logged), never fatal.
pub async fn list<T: DeserializeOwned>(kind: &str) -> Result<Vec<(String, T)>, PlatformError> {
    let rows: Vec<Listed> = serde_json::from_value(unwrap(js_list(kind).await)?).map_err(|e| PlatformError::Invalid(e.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|r| match serde_json::from_value(r.data) {
            Ok(t) => Some((r.id, t)),
            Err(e) => {
                leptos::logging::warn!("skipping {}: {e}", r.id);
                None
            }
        })
        .collect())
}

/// Grader lab only: writes under the shared `lab/` prefix.
pub async fn set_shared<T: Serialize>(path: &str, body: &T) -> Result<(), PlatformError> {
    let json = serde_json::to_string(body).map_err(|e| PlatformError::Invalid(e.to_string()))?;
    unwrap(js_set_shared(path, &json).await).map(|_| ())
}

pub async fn sample_json(prompt: &str, tier: &str) -> Result<Value, PlatformError> {
    unwrap(js_sample(prompt, tier).await)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SampledText {
    pub text: String,
    pub truncated: bool,
}

pub async fn sample_text(prompt: &str, tier: &str) -> Result<SampledText, PlatformError> {
    serde_json::from_value(unwrap(js_sample_text(prompt, tier).await)?).map_err(|e| PlatformError::Invalid(e.to_string()))
}

#[derive(Debug, Clone, Deserialize)]
pub struct Uploaded {
    pub id: String,
}

/// Uploads the file chosen in an `<input type=file>`; `None` when nothing was chosen.
pub async fn upload_from_input(input_id: &str) -> Result<Option<Uploaded>, PlatformError> {
    match unwrap(js_upload(input_id).await)? {
        Value::Null => Ok(None),
        v => serde_json::from_value(v).map(Some).map_err(|e| PlatformError::Invalid(e.to_string())),
    }
}

/// Where a stored image is served (the Worker's R2 route; only to its owner).
pub fn asset_url(id: &str) -> String {
    format!("/api/files/{id}")
}
