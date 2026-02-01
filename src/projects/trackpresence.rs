use crate::components::new_tab_link::NewTabLink;

use super::Project;
use leptos::prelude::*;

pub const PROJECT: Project = Project {
    name: "Track presence",
    slug: "trackpresence",
    desctiption: "Lightweight Discord music status",
    view_link: Some("https://crates.io/crates/track_presence"),
    read_component: || TrackPresenceRead().into_any().into_view(),
};

#[component]
pub fn TrackPresenceRead() -> impl IntoView {
    view! {
        <img src="/assets/images/trackpresence_cover.webp" />
        <p>
            "Track Presence is a lightweight Discord music status application that tracks what you’re listening to and displays it on Discord. It’s modular, fully local, and extremely resource-efficient."
        </p>
        <p>
            "Track Presence reads information about the currently playing music in your system media player and displays it as Discord Rich Presence."
        </p>
        <p>
            "At the moment, it only supports Linux via MPRIS, but there are plans to port it to Windows and possibly MacOS in the future."
        </p>

        <p>
            "Track Presence is available on crates.io, so you can install it using Cargo: "
            <NewTabLink href="https://crates.io/crates/track_presence">"click"</NewTabLink> "."
        </p>
    }
}
