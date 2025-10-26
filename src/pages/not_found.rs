use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <p>"Hic sunt dracones"</p>
        <img src="/assets/images/dragon.png"/>
    }
}
