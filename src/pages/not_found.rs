use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="404 Not found" />

        <div class="not-found">
            <h1>"Page not found."</h1>
            <p>"404"</p>
        </div>
    }
}
