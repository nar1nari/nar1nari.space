use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;

use crate::{
    blogs::{fetch_all_blogs, BlogPost},
    components::{blog_card::BlogCard, comments::Comments, new_tab_link::NewTabLink},
};

#[component]
pub fn Home() -> impl IntoView {
    let latest_blog = RwSignal::new(BlogPost::default());
    spawn_local(async move {
        if let Some(blog) = fetch_all_blogs().await.first() {
            latest_blog.set(blog.clone());
        }
    });

    view! {
        <Title text="nar1nari space" />

        <h2>"Welcome to " <mark>"nar1nari.space"</mark></h2>
        <p>"Hi! I\'m nar1nari, but you can just call me nari."</p>
        <p>
            "I\'m passionate about programming and other silly stuff. My main language is Rust, though I like experimenting with other technologies from time to time."
        </p>
        <p>"On this website, you\'ll find my projects, blogs and other stuff made by me."</p>
        <p>"Feel free to explore if you're curious!"</p>

        <div class="section-grid">
            <section>
                <h2>"Facts about me"</h2>
                <ul>
                    <li>"nari"</li>
                    <li>"19 y.o"</li>
                    <li>"🇰🇿"</li>
                    <li>"IT&AI student"</li>
                    <li class="heart-li">"FGSFDS"</li>
                    <li class="heart-li">
                        <NewTabLink href="https://akrosha.neocities.org/">"Akro"</NewTabLink>
                    </li>
                </ul>
            </section>

            <section>
                <h2>"Find me here"</h2>
                <p>
                    <i class="icon">"󰊤 "</i>
                    <NewTabLink href="https://github.com/nar1nari/">"GitHub"</NewTabLink>
                    <br />
                    <i class="icon">" "</i>
                    <NewTabLink href="https://t.me/nar1nari/">"Telegram"</NewTabLink>
                    <br />
                    <i class="icon">" "</i>
                    "good_vibrations"
                    <br />
                    <i class="icon">" "</i>
                    <a href="mailto:nar1nariq@proton.me">"nar1nariq@proton.me"</a>
                </p>
            </section>
        </div>

        <h2>"Latest blog post"</h2>
        {move || {
            latest_blog
                .with(|post| {
                    view! { <BlogCard post=post /> }
                })
        }}

        <h2 style="margin: 0; margin-top: 1em;">"Say hi here"</h2>
        <Comments />

        <footer class="footer">
            <p>
                "This website was made with " <NewTabLink href="https://rust-lang.org/">
                    <i class="icon">""</i>
                    " Rust"
                </NewTabLink> " and " <NewTabLink href="https://leptos.dev/">"Leptos"</NewTabLink>
                "."
            </p>
        </footer>
    }
}
