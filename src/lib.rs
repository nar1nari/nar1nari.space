use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod blogs;
mod components;
mod music;
mod pages;
mod projects;

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <Title text="nar1nari space" />

        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <header>
            <a href="/"> "Home" </a>
            <a href="/projects"> "Projects" </a>
            <a href="/blog"> "Blog" </a>
            <a href="/music"> "Music" </a>
            <a href="/findme"> "Find me" </a>
        </header>
        <main>
            <Router>
                <Routes fallback=|| not_found::NotFound>
                    <Route path=path!("/") view=home::Home />
                    <Route path=path!("/projects") view=pages::projects::Projects />
                    <Route path=path!("/projects/:slug/read") view=pages::project_page::ProjectPage />
                    <Route path=path!("/blog") view=pages::blog::Blog />
                    <Route path=path!("/blog/:slug") view=pages::blog_page::BlogPage />
                    <Route path=path!("/music") view=pages::music::Music />
                    <Route path=path!("/findme") view=findme::FindMe />
                </Routes>
            </Router>
        </main>
    }
}
