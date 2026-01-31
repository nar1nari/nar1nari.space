use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Gitalk;

    #[wasm_bindgen(constructor)]
    fn new(config: JsValue) -> Gitalk;

    #[wasm_bindgen(method)]
    fn render(this: &Gitalk, container_id: &str);
}

#[component]
pub fn Comments() -> impl IntoView {
    let params = use_params_map();
    let container_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let slug = params.with(|p| p.get("slug").unwrap_or_else(|| "home".to_string()));

        if let Some(div) = container_ref.get() {
            let render_gitalk = {
                let slug = slug.clone();
                let div = div.clone();
                move || {
                    div.set_inner_html("");

                    let config = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &config,
                        &"clientID".into(),
                        &"Ov23liW1HhfUVvB8uvAR".into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(&config, &"clientSecret".into(), &"".into()).unwrap();
                    js_sys::Reflect::set(
                        &config,
                        &"proxy".into(),
                        &"https://gitalk-proxy.levizortv.workers.dev/".into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(&config, &"repo".into(), &"nar1nari.space".into())
                        .unwrap();
                    js_sys::Reflect::set(&config, &"owner".into(), &"nar1nari".into()).unwrap();
                    js_sys::Reflect::set(
                        &config,
                        &"admin".into(),
                        &serde_wasm_bindgen::to_value(&vec!["nar1nari"]).unwrap(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(&config, &"id".into(), &slug.into()).unwrap();
                    js_sys::Reflect::set(&config, &"distractionFreeMode".into(), &false.into())
                        .unwrap();
                    js_sys::Reflect::set(&config, &"createIssueManually".into(), &true.into())
                        .unwrap();

                    let gitalk = Gitalk::new(config.into());
                    gitalk.render("gitalk-container");
                }
            };

            let window = web_sys::window().unwrap();
            if js_sys::Reflect::has(&window, &"Gitalk".into()).unwrap_or(false) {
                render_gitalk();
            } else {
                let handle = gloo_timers::callback::Timeout::new(250, move || {
                    render_gitalk();
                });
                handle.forget();
            }
        }
    });

    view! { <div id="gitalk-container" node_ref=container_ref></div> }
}
