use leptos::prelude::*;

#[component]
pub fn NewTabLink(#[prop()] href: &'static str, #[prop()] text: &'static str) -> impl IntoView {
    view! {
        <a href=href target="_blank" rel="noopener noreferrer"> {text} </a>
    }
}
