use leptos::prelude::*;
use leptos_meta::Title;

use crate::{components::game_card::GameCard, games::fetch_games};

#[component]
pub fn Games() -> impl IntoView {
    let games = LocalResource::new(|| fetch_games());

    view! {
        <Title text="Games"/>

        <div class="section-grid">
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || {
                    games.get().map(|result| match result {
                        Ok(list) => view! {
                            <For
                                each=move || list.clone()
                                key=|game| game.id
                                let:game
                            >
                                <GameCard game=game />
                            </For>
                        }.into_any(),
                        Err(e) => view! { <p class="error">{format!("Error: {e}")}</p> }.into_any(),
                    })
                }}
            </Suspense>
        </div>
    }
}