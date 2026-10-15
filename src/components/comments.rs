use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Gitalk;

    #[wasm_bindgen(constructor)]
    fn new(config: JsValue) -> Gitalk;

    #[wasm_bindgen(method)]
    fn render(this: &Gitalk, container_id: &str);
}

fn render_gitalk(slug: &str) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Some(div) = document.get_element_by_id("gitalk-container") else {
        return;
    };
    div.set_inner_html("");

    let config = js_sys::Object::new();
    let set = |k: &str, v: JsValue| {
        js_sys::Reflect::set(&config, &k.into(), &v).unwrap();
    };
    set("clientID", "Ov23liW1HhfUVvB8uvAR".into());
    set("clientSecret", "".into());
    set(
        "proxy",
        "https://gitalk-proxy.levizortv.workers.dev/".into(),
    );
    set("repo", "nar1nari.space".into());
    set("owner", "nar1nari".into());
    set(
        "admin",
        serde_wasm_bindgen::to_value(&vec!["nar1nari"]).unwrap(),
    );
    set("id", slug.into());
    set("distractionFreeMode", false.into());
    set("createIssueManually", true.into());

    let gitalk = Gitalk::new(config.into());
    gitalk.render("gitalk-container");
}

fn gitalk_ready() -> bool {
    web_sys::window()
        .map(|w| js_sys::Reflect::has(&w, &"Gitalk".into()).unwrap_or(false))
        .unwrap_or(false)
}

#[component]
pub fn Comments() -> impl IntoView {
    let params = use_params_map();
    let container_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let slug = params.with(|p| p.get("slug").unwrap_or_else(|| "home".to_string()));

        if container_ref.get().is_none() {
            return;
        }

        if gitalk_ready() {
            render_gitalk(&slug);
            return;
        }

        let attempts = Rc::new(Cell::new(0u32));
        let slug = slug.clone();
        let interval = Rc::new(Cell::new(None::<gloo_timers::callback::Interval>));
        let interval_inner = interval.clone();

        let handle = gloo_timers::callback::Interval::new(100, move || {
            attempts.set(attempts.get() + 1);

            if gitalk_ready() {
                render_gitalk(&slug);
                interval_inner.take();
            } else if attempts.get() >= 100 {
                interval_inner.take();
            }
        });
        interval.set(Some(handle));
    });

    view! { <div id="gitalk-container" node_ref=container_ref></div> }
}
