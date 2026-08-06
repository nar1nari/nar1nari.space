use leptos::prelude::*;

use crate::components::new_tab_link::NewTabLink;
use crate::games::Game;

#[component]
pub fn GameCard(#[prop()] game: Game) -> impl IntoView {
    view! {
        <section class="game-card">
            {game.cover_url.clone().map(|src| view! {
                <NewTabLink href=game.url.clone()>
                    <img src=src />
                </NewTabLink>
            })}
            <h3>{game.title.clone()}</h3>
            {game.short_text.clone().map(|t| view! { <p>{t}</p> })}
            <NewTabLink href=game.url.clone()> <i class="icon">" "</i> "Play on itch.io"</NewTabLink>
        </section>
    }
}
