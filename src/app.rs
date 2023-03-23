use std::{fs, panic};
use std::path::PathBuf;
use std::str::FromStr;

use js_sys::eval;
use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::*;
use markdown::to_html;
use notify::{RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
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

#[derive(Serialize, Deserialize)]
struct ReadMarkdownArgs<'a> {
    path_str: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    view! { cx,
            <HomePage />
    }
}


#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (md_src, set_md_src) = create_signal(cx, String::from("# Markdown Renderer"));

    let get_md_src = move || {
        spawn_local(async move {
            let path = "/home/vincent/github/trooper/README.md";
            let args = to_value(&ReadMarkdownArgs { path_str: &path }).unwrap();
            log!("{:?}", args);
            let greeting = invoke("read_markdown_source", args).await.as_string();

            //set_md_src.set(greeting)
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
            //Ok(_) => set_md_src.set(fs::read_to_string(&p).unwrap()),
