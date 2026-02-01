use crate::components::new_tab_link::NewTabLink;

use super::Project;
use leptos::prelude::*;

pub const PROJECT: Project = Project {
    name: "Sylvie",
    slug: "sylvie",
    desctiption: "multifunctional AI Discord bot",
    view_link: Some("https://discord.com/oauth2/authorize?client_id=1276848855656435722"),
    read_component: || SylvieRead().into_any().into_view(),
};

#[component]
pub fn SylvieRead() -> impl IntoView {
    view! {
        <p>
            "Sylvie became my first project to actually reach public use.
            The idea for this project came to me out of nowhere in September 2024, while I was stuck at home on sick leave."
        </p>
        <p>
            "She started out simple - just a bot that could pull up images, play music in voice chats,
            and handle a few roleplay and admin commands like auto-greetings and language settings.
            But the thing that really made her stand out was the built-in AI."
        </p>
        <img src="/assets/images/sylvie_help.webp" />
        <p>
            "Each server gets its own memory, and Sylvie herself has a bit of personality - even a backstory.
            She's more than just a bot. She feels like a character that grew along with the project."
        </p>

        <img src="/assets/images/sylvie_soap.webp" />
        <q>
            "- Sylvie, please tell me how I can stop getting angry about losing matches?" <br />
            "- Adam, I'm sorry, but I don't think I can help you with this. You know what nari said...
            that you jerked off into an orphan's liquid soap. That's... That's not right, Adam."
        </q>
        <p class="comment">"The funniest shit she ever said xd"</p>

        <p>
            "I even added Sylvie to our Minecraft SMP server.
            It’s kind of wild, looking back, how something that started as a small side project turned into a companion that people actually interact with every day.
            Seeing friends talk to her like she was one of us… that was the moment I realized Sylvie had grown beyond just lines of code."
        </p>
        <img src="/assets/images/sylvie_minecraft.webp" />

        <p>
            "You can add Sylvie to your Discord server by clicking "
            <NewTabLink href="https://discord.com/oauth2/authorize?client_id=1276848855656435722">
                "here"
            </NewTabLink> "."
        </p>
    }
}
