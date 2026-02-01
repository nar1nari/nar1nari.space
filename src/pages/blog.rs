use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_meta::Title;

use crate::{
    blogs::{fetch_all_blogs, BlogPost},
    components::blog_card::BlogCard,
};

#[component]
pub fn Blog() -> impl IntoView {
    let blogs = RwSignal::new(Vec::new());
    spawn_local(async move {
        blogs.set(fetch_all_blogs().await);
    });

    view! {
        <Title text="Blog" />

        <div class="section-list">
            <For
                each=move || blogs.get()
                key=|post| post.slug.clone()
                children=move |post: BlogPost| {
                    view! { <BlogCard post=&post /> }
                }
            />
        </div>
    }
}
