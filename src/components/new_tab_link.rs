use leptos::prelude::*;

#[component]
pub fn NewTabLink(#[prop()] href: &'static str, children: Children) -> impl IntoView {
    view! {
        <a href=href target="_blank" rel="noopener noreferrer">
            {children()}
        </a>
    }
}
