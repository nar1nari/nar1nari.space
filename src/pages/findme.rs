use leptos::prelude::*;

use crate::components::new_tab_link::NewTabLink;

#[component]
pub fn FindMe() -> impl IntoView {
    view! {
        <p> "󰊤 " <NewTabLink href="https://github.com/nar1nari/" text="nar1nari"/> </p>
        <p> " good_vibrations" </p>
        <p> "󰺻 " <a href="mailto:nar1nariq@proton.me"> "nar1nariq@proton.me" </a> </p>
    }
}
