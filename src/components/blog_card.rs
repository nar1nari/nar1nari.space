use crate::blogs::BlogPost;
use leptos::prelude::*;

#[component]
pub fn BlogCard<'a>(#[prop()] post: &'a BlogPost) -> impl IntoView {
    view! {
        <a href={format!("/blog/{}", post.slug)} class="blog-link">
            <div class="blog-card">
                <div class="blog-title">
                    <h3>{post.title.clone()}</h3>
                    <p>{post.date.clone()}</p>
                </div>

                <div class="tags">
                    {post.tags.iter().map(|tag| {
                        view! {
                            <p>{tag.clone()}</p>
                        }
                    }).collect_view()}
                </div>
            </div>
        </a>
    }
}
