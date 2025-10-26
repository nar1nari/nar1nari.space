use leptos::prelude::*;
use leptos::reactive::spawn_local;

use crate::{
    blogs::{fetch_all_blogs, BlogPost},
    components::blog_card::BlogCrad,
};

#[component]
pub fn Blog() -> impl IntoView {
    let blogs = RwSignal::new(Vec::new());
    spawn_local(async move {
        blogs.set(fetch_all_blogs().await);
    });

    view! {
        <For
            each=move || blogs.get()
            key=|post| post.slug.clone()
            children=move |post: BlogPost| {
                view! {
                    <BlogCrad post=&post/>
                }
            }
        />
    }
}
