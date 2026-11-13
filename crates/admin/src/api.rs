//! Calls to the Worker (same origin). Every answer is a status and a JSON body.

use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/admin.js")]
extern "C" {
    #[wasm_bindgen(js_name = call)]
    async fn js_call(method: &str, path: &str, body: Option<String>) -> JsValue;
    #[wasm_bindgen(js_name = copyText)]
    async fn js_copy(text: &str) -> JsValue;
}

#[derive(Debug, Clone)]
pub struct Resp {
    pub status: u16,
    pub body: Value,
}

impl Resp {
    pub fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }
    /// The Worker's error code (`not_staff`, `two_factor_required`, …), when there is one.
    pub fn code(&self) -> Option<&str> {
        self.body.get("code").and_then(Value::as_str)
    }
    /// A message fit to show: the server's own, or a plain fallback.
    pub fn message(&self) -> String {
        match self.body.get("message").and_then(Value::as_str) {
            Some(m) if !m.is_empty() => m.to_string(),
            _ if self.status == 0 => "Couldn't reach the server. Check your connection and try again.".into(),
            _ => "Something went wrong. Try again.".into(),
        }
    }
    pub fn parse<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        serde_json::from_value(self.body.clone()).ok()
    }
}

pub async fn call(method: &str, path: &str, body: Option<Value>) -> Resp {
    let raw = js_call(method, path, body.map(|b| b.to_string())).await.as_string().unwrap_or_default();
    let outer: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
    let status = outer.get("status").and_then(Value::as_u64).unwrap_or(0) as u16;
    let text = outer.get("body").and_then(Value::as_str).unwrap_or("");
    Resp { status, body: serde_json::from_str(text).unwrap_or(Value::Null) }
}

pub async fn get(path: &str) -> Resp {
    call("GET", path, None).await
}

pub async fn post(path: &str, body: Value) -> Resp {
    call("POST", path, Some(body)).await
}

pub async fn copy(text: &str) -> bool {
    js_copy(text).await.as_bool().unwrap_or(false)
}

/// GET /api/admin/me
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    pub role: String,
    pub email: Option<String>,
    pub two_factor_enabled: bool,
    pub ready: bool,
    pub blocked: Option<String>,
    pub permissions: Vec<String>,
    pub session_ends_at: Option<f64>,
}

impl Me {
    pub fn can(&self, p: &str) -> bool {
        self.permissions.iter().any(|x| x == p)
    }
    pub fn role_label(&self) -> &'static str {
        match self.role.as_str() {
            "owner" => "Owner",
            "instructor" => "Instructor",
            "support" => "Support",
            _ => "Learner",
        }
    }
}

/// A local date and time, such as "13 Nov, 14:05".
pub fn when(ms: f64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(ms));
    const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!("{} {}, {:02}:{:02}", d.get_date(), MONTHS[d.get_month() as usize % 12], d.get_hours(), d.get_minutes())
}

pub fn time_only(ms: f64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(ms));
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}
