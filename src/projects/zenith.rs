use crate::components::new_tab_link::NewTabLink;

use super::Project;
use leptos::prelude::*;

pub const PROJECT: Project = Project {
    name: "Zenith",
    slug: "zenith",
    desctiption: "simple web planetarium",
    view_link: Some("https://nar1nari.space/zenith"),
    read_component: || ZenithRead().into_any().into_view(),
};

#[component]
pub fn ZenithRead() -> impl IntoView {
    view! {
        <p> "Zenith is a web-based planetarium where you can see which planets are currently visible in the sky.
        The center point on the circle represents the zenith, the highest point on the sky, with north at the top, west on the left, and so on - basically, 
        it's like the sky stretched out over a map." </p>
        <p>" Zenith includes a large database of cities, so you can easily find your own and see what's happening above you in real time. "</p>
        <img src="/assets/images/zenith_screenshot.png"/>
        <p>
            "You can find the source code "
            <NewTabLink href="https://github.com/nar1nari/zenith/" text="here"/>
            " and try it out "
            <NewTabLink href="https://nar1nari.space/zenith" text="here"/>
            "."
        </p>
    }
}
