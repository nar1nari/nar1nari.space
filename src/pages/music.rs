use leptos::prelude::*;

use crate::components::music_card::MusicCard;
use crate::music::Music;

#[component]
pub fn Music() -> impl IntoView {
    view! {
        <p> "Back in 2024, I tried making music in LMMS." </p>
        <p> "It's not exactly something you'd wanna put on repeat, but making it was fun." </p>
        <p> "Maybe I'll get back into it someday." </p>
        <p> "You can check out my tracks here." </p>
        {Music::all().iter()
            .map(|music| view!{
                <MusicCard music=music/>
            }).collect_view()
        }
    }
}
