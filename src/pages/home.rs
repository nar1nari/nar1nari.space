use chrono::Datelike;
use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;

use crate::{
    blogs::{fetch_all_blogs, BlogPost},
    components::{blog_card::BlogCard, comments::Comments, new_tab_link::NewTabLink},
};

#[component]
pub fn Home() -> impl IntoView {
    const BIRTH_YEAR: i32 = 2006;
    const BIRTH_MONTH: u32 = 3;
    const BIRTH_DAY: u32 = 12;

    let age = {
        let now = chrono::Local::now();
        let mut age = now.year() - BIRTH_YEAR;

        if now.month() < BIRTH_MONTH || (now.month() == BIRTH_MONTH && now.day() < BIRTH_DAY) {
            age -= 1;
        }
        age
    };

    let latest_blog = RwSignal::new(BlogPost::default());
    spawn_local(async move {
        if let Some(blog) = fetch_all_blogs().await.first() {
            latest_blog.set(blog.clone());
        }
    });

    view! {
        <Title text="nar1nari.online" />

        <h2>"Hi, and welcome to " <mark>"nar1nari.online"</mark></h2>
        <p>
            "I am a software developer primarily interested in backend development and bot creation."
        </p>
        <p>
            "I mainly use Rust, Python, and JS/TS, tho I also experiment and use other technologies - for instance, I sometimes create Minecraft mods and plugins in Java and develop games in Godot."
        </p>
        <p>
            "This website serves as a little archive of my projects and work. Feel free to look around or get in touch via the links below."
        </p>

        <div class="section-grid">
            <section>
                <h2>"About me"</h2>
                <ul>
                    <li>"Viktor"</li>
                    <li>{age} " y.o"</li>
                    <li>"🇰🇿"</li>
                    <li>"IT&AI student"</li>
                </ul>
            </section>

            <section>
                <h2>"Links"</h2>
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
