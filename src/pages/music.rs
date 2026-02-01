use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::music_card::MusicCard;
use crate::music::Music;

#[component]
pub fn Music() -> impl IntoView {
    view! {
        <Title text="Music" />

        <p>
            "Back in 2024, I tried making music in LMMS." <br />
            "It's not exactly something you'd wanna put on repeat, but making it was fun."
        </p>
        <p>"Maybe I'll get back into it someday." <br /> "You can check out my tracks here."</p>
        <div class="section-list">
            {Music::all().iter().map(|music| view! { <MusicCard music=music /> }).collect_view()}
        </div>
    }
}
