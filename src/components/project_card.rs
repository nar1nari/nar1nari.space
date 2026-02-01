use leptos::prelude::*;

use crate::components::new_tab_link::NewTabLink;
use crate::projects::Project;

#[component]
pub fn ProjectCard(#[prop()] project: &'static Project) -> impl IntoView {
    view! {
        <section class="project-card">
            <img
                src=format!("assets/images/{}_icon.webp", project.slug)
                alt=format!("{} icon", project.slug)
            />

            <div class="project-info">
                <div>
                    <h2>{project.name}</h2>
                    <p>{project.desctiption}</p>
                </div>
                <nav>
                    {project
                        .view_link
                        .map(|link| view! { <NewTabLink href=link>"View"</NewTabLink> })}
                    <a href=format!("/projects/{}/read", project.slug)>"Read"</a>
                </nav>
            </div>
        </section>
    }
}
