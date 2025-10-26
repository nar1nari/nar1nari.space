use leptos::prelude::*;

use crate::music::Music;

#[component]
pub fn MusicCard(#[prop()] music: &'static Music) -> impl IntoView {
    view! {
        <div class="music-card">
            <h2>{music.name}</h2>
            <p>{music.description}</p>
            <audio controls loop>
                <source src=format!("assets/music/{}", music.filename) type="audio/wav"/>
                "Your browser does not support the audio element."
            </audio>
        </div>
    }
}
