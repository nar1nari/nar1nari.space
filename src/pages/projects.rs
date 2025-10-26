use leptos::prelude::*;

use crate::components::project_card::ProjectCard;
use crate::projects::Project;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <h1> "Projects" </h1>
        <div class="project-list">
        {Project::ALL.iter().map(|p| {
            view! {
                <ProjectCard project=&p/>
            }
        }).collect_view()}
        </div>
    }
}
