use leptos::prelude::*;

use crate::components::new_tab_link::NewTabLink;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <img src="assets/images/hatman.gif" alt="hatman" class="hatman"/>
        <h2> "Hi and welcome to "<mark> " nar1nari.space " </mark> </h2>
        <p> "Welcome to my website. Here you can find my projects, blog posts, and other things." </p>

        <h2> "About me" </h2>
        <ul>
            <li> "nari" </li>
            <li> "19 y.o" </li>
            <li> "IT&AI student" </li>
            <li class="heart-li"> "Rust" </li>
            <li class="heart-li"> "FGSFDS" </li>
            <li class="heart-li"> <NewTabLink href="https://akrosha.neocities.org/" text="Akro"/>  </li>
        </ul>

        <h2>"Say hi here"</h2>
        <div inner_html="
        <iframe src=\"https://www5.cbox.ws/box/?boxid=955415&boxtag=15Wv1V\" width=\"100%\" height=\"450\" allow=\"autoplay\" frameborder=\"0\" marginheight=\"0\" marginwidth=\"0\" scrolling=\"auto\"></iframe>"
        >
        </div>

        <footer class="footer">
            <p>
                "This website was made with "
                <NewTabLink href="https://rust-lang.org/" text="Rust"/>
                " and "
                <NewTabLink href="https://leptos.dev/" text="Leptos"/>
                "."
            </p>
            <p>
                "With "
                <NewTabLink href="https://lospec.com/palette-list/paper-8" text="Paper 8 Palette"/>
                "."
            </p>
        </footer>
    }
}
