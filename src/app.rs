use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use std::{fs, panic, future};

use js_sys::eval;
use leptos::ev::InputEvent;
use leptos::leptos_dom::ev::SubmitEvent;
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

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "cli"], js_name = getMatches)]
    async fn get_matches() -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ReadMarkdownArgs<'a> {
    path: &'a str,
}

#[derive(Serialize, Deserialize)]
struct GetMatchingPathsArgs<'a> {
    path: &'a str,
}


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    view! { cx,
        <HomePage />
    }
}

async fn get_cli_path() -> String {
    let res = get_matches().await;
    let v: serde_json::Value =
        serde_json::from_str(&js_sys::JSON::stringify(&res).unwrap().as_string().unwrap())
            .unwrap();
    if let Some(path_str) = v["args"]["source"]["value"].as_str() {
        log!("{:?}", path_str);
        return String::from(path_str);
    }
    return String::new();
}


#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (md_src, set_md_src) = create_signal(cx, String::from(""));
    let (md_path, set_md_path) = create_signal(cx, String::from(""));
    let (matching_paths, set_matching_paths) = create_signal(cx, Vec::<String>::new());

    let get_md_src = move || {
        spawn_local(async move {
            let args = to_value(&ReadMarkdownArgs {
                path: &md_path.get(),
            })
            .unwrap();
            if let Some(markdown_src) = invoke("read_markdown_source", args).await.as_string() {
                set_md_src.set(markdown_src);
            }
        })
    };

    // Trigger on input
    let get_matching_paths = move || {
        spawn_local(async move {
            let args = to_value(&GetMatchingPathsArgs {
                path: &md_path.get(),
            })
            .unwrap();
            if let Some(path_str) = invoke("get_matching_paths", args).await.as_string() {
                let paths = path_str.split("\n");
                set_matching_paths.update(|v| {
                    v.clear();
                    for p in paths {
                        v.push(p.to_string());
                    }
                })
            }
        });
    };

    // Initial fetch if cli arg was passed
    spawn_local(async move {
        let p = get_cli_path().await;
        set_md_path.set(p);
        let args = to_value(&ReadMarkdownArgs {
            path: &md_path.get(),
        })
        .unwrap();
        if let Some(markdown_src) = invoke("read_markdown_source", args).await.as_string() {
            set_md_src.set(markdown_src);
        }
    });

    let md_rendered = move || md_src.with(|md_src| to_html(&md_src));
    let on_path_input = move |ev| {
        get_matching_paths();
        set_md_path.set(event_target_value(&ev));
    };

    create_effect(cx, move |_| {
        md_src.get();
        eval("reRenderMath()").unwrap_or(JsValue::null());
    });

    set_interval_with_handle(
        move || {
            get_md_src();
            log!("Hello");
        },
        Duration::from_secs(1),
    )
    .unwrap();


    view! { cx,
        <main class="theme-dark">
            <input class="path-input no-print" prop:value=move || md_path.get() on:input=on_path_input />
            <div class="md-container" inner_html=md_rendered />
            <div class="match-overlay">
                <For
                    each=move || matching_paths.get()
                    key=|p| p.clone()
                    view=move |cx, path: String| {
                        view! {
                            cx,
                            <p>{path}</p>
                        }
                }/>
            </div>
            <script type_="text/x-mathjax-config" src="public/mathjaxconf.js" />
            <script id="MathJax-script" async src="public/tex-mml-svg.js" />
        </main>
    }
}
