use crate::components::new_tab_link::NewTabLink;

use super::Project;
use leptos::prelude::*;

pub const PROJECT: Project = Project {
    name: "Savey",
    slug: "savey",
    desctiption: "Telegram bot that lets users download and receive media from popular platforms",
    icon: "assets/images/savey_icon.webp",
    view_link: Some("https://t.me/saveyfrombot"),
    read_component: || SaveyRead().into_any().into_view(),
};

#[component]
pub fn SaveyRead() -> impl IntoView {
    view! {
        <p>
            "Savey is a Telegram bot that lets users download and receive audio or video from popular platforms such as YouTube, TikTok, SoundCloud, and more."
        </p>
        <p>
            "It can be added to a group chat to send media from any link, or used in any chat in inline mode by typing @saveyfrombot in text input."
        </p>

        <img src="/assets/images/savey_screenshot.png" />

        <br />

        <NewTabLink href="https://github.com/nar1nari/savey-tg-bot">"GitHub"</NewTabLink>

        <br />

        <NewTabLink href="https://t.me/saveyfrombot">"Telegram"</NewTabLink>
    }
}
