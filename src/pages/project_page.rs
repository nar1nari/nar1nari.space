use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::projects::Project;

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|p| p.get("slug").unwrap_or_default());

    let project = Project::ALL.iter().find(|p| p.slug == slug);

    view! {
        {match project {
            Some(p) => view!{
                <h1> {p.name} </h1>
                {(p.read_component)()}
            }.into_any().into_view(),
            None => view! {
                <p>"Project not found."</p>
            }.into_any().into_view(),
        }}
    }
}
