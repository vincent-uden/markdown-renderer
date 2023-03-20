use js_sys::eval;
use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::*;
use markdown::to_html;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
            <HomePage />
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (md_src, set_md_src) = create_signal(cx, String::from("# Markdown Renderer"));

    let get_md_src = move || {
        spawn_local(async move {
            let file_contents = invoke("read_markdown_source", JsValue::null()).await.as_string().unwrap();
            set_md_src.set(file_contents)
        })
    };

    let md_rendered = move || {
        md_src.with(|md_src| to_html(&md_src))
    };

    create_effect(cx, move |_| {
        md_src.get();
        eval("reRenderMath()").unwrap_or(JsValue::null());
    });
    
    view! { cx,
        <main class="theme-dark">
            <div class="md-container" inner_html=md_rendered />
            <script type_="text/x-mathjax-config" src="public/mathjaxconf.js" />
            <script id="MathJax-script" async src="public/tex-mml-svg.js" />
            <button on:click=move |_| get_md_src()>"Reload markdown source"</button>
        </main>
    }
}