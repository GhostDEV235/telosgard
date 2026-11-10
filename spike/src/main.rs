use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bridge.js")]
extern "C" {
    async fn available(name: &str) -> JsValue;
    #[wasm_bindgen(js_name = samplePing)]
    async fn sample_ping() -> JsValue;
    #[wasm_bindgen(js_name = dbPing)]
    async fn db_ping() -> JsValue;
}

fn js_str(v: JsValue) -> String {
    v.as_string().unwrap_or_else(|| format!("{v:?}"))
}

#[component]
fn Check(label: &'static str, run: fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = String>>>) -> impl IntoView {
    let (out, set_out) = signal(String::from("not run"));
    view! {
        <div class="card">
            <strong>{label}</strong>
            <p><code>{move || out.get()}</code></p>
            <button on:click=move |_| {
                set_out.set("running…".into());
                spawn_local(async move { set_out.set(run().await) });
            }>"Run"</button>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <main>
            <h1>"Gold School: proof of concept"</h1>
            <div class="card">
                <strong>"1. WASM + Leptos"</strong>
                <p>"If you can click this and the number goes up, Rust is running in the viewer."</p>
                <button on:click=move |_| set_count.update(|n| *n += 1)>{move || format!("Clicked {}", count.get())}</button>
            </div>
            <Check label="2a. Capabilities available" run=|| Box::pin(async {
                let mut s = String::new();
                for n in ["sample", "db", "user", "assets"] { s += &format!("{n}={}  ", js_str(available(n).await)); }
                s
            }) />
            <Check label="2b. Claude (sample) round trip" run=|| Box::pin(async { js_str(sample_ping().await) }) />
            <Check label="3. Database write + read (run twice; second run shows previous)" run=|| Box::pin(async { js_str(db_ping().await) }) />
            <div class="card">
                <strong>"4. Video link-out (iframes are blocked by the platform)"</strong>
                <p><a href="https://www.youtube.com/results?search_query=bid+ask+spread+explained" target="_blank" rel="noopener">"Open YouTube in a new tab →"</a></p>
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
