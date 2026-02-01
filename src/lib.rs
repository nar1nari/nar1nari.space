use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod blogs;
mod components;
mod music;
mod pages;
mod projects;

use crate::components::new_tab_link::NewTabLink;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Script src="https://cdn.jsdelivr.net/npm/gitalk@1/dist/gitalk.min.js" />

        <Router>
            <header>
                <nav>
                    <A href="/">"Home"</A>
                    <A href="/projects">"Projects"</A>
                    <A href="/blog">"Blog"</A>
                    <A href="/music">"Music"</A>
                </nav>
                <div class="github-icon icon">
                    <NewTabLink href="https://github.com/nar1nari/nar1nari.space">
                        " "
                    </NewTabLink>
                </div>
            </header>
            <main>
                <Routes fallback=|| not_found::NotFound>
                    <Route path=path!("/") view=home::Home />
                    <Route path=path!("/projects") view=pages::projects::Projects />
                    <Route
                        path=path!("/projects/:slug/read")
                        view=pages::project_page::ProjectPage
                    />
                    <Route path=path!("/blog") view=pages::blog::Blog />
                    <Route path=path!("/blog/:slug") view=pages::blog_page::BlogPage />
                    <Route path=path!("/music") view=pages::music::Music />
                </Routes>
            </main>
        </Router>
    }
}
