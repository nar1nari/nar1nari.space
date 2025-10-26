use leptos::prelude::*;

use crate::components::new_tab_link::NewTabLink;
use crate::projects::Project;

#[component]
pub fn ProjectCard(#[prop()] project: &'static Project) -> impl IntoView {
    view! {
        <div class="project-card">
            <img src=format!("assets/images/{}_icon.png", project.slug) alt=format!("{} icon", project.slug)/>
            <h2>{project.name}</h2>
            <p>{project.desctiption}</p>
            <div class="project-buttons">
                {project.view_link.map(|link| view! { <NewTabLink href=link text="View"/> })}
                <a href={format!("/projects/{}/read", project.slug)}>"Read"</a>
            </div>
        </div>
    }
}
