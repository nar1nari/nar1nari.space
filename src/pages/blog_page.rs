use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_params_map;

use crate::{blogs::fetch_all_blogs, components::comments::Comments, pages::not_found::NotFound};

#[component]
pub fn BlogPage() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|p| p.get("slug").unwrap_or_default());

    let post = RwSignal::new(None);

    spawn_local(async move {
        let blogs = fetch_all_blogs().await;
        let found = blogs.into_iter().find(|p| p.slug == slug);
        post.set(found);
    });

    view! {
        {move || match post.get() {
            Some(p) => {
                view! {
                    <div class="blog-post">
                        <div class="blog-title">
                            <h1>{p.title}</h1>
                            <p>{p.date}</p>
                        </div>
                        <hr />
                        <div inner_html=p.content_html />
                    </div>

                    <Comments />
                }
                    .into_any()
                    .into_view()
            }
            None => view! { <NotFound /> }.into_any().into_view(),
        }}
    }
}
