use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_meta::Title;

use crate::{pages::not_found::NotFound, projects::Project};

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|p| p.get("slug").unwrap_or_default());

    let project = Project::ALL.iter().find(|p| p.slug == slug);

    view! {
        {match project {
            Some(p) => {
                view! {
                    <Title text={p.name} />

                    <h1>{p.name}</h1>
                    <hr />
                    {(p.read_component)()}
                }
                    .into_any()
                    .into_view()
            }
            None => view! { <NotFound /> }.into_any().into_view(),
        }}
    }
}
