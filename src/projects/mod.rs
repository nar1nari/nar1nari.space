mod hlebcraft;
mod savey;
mod sylvie;
mod trackpresence;
mod zenith;

use leptos::prelude::*;

pub struct Project {
    pub name: &'static str,
    pub slug: &'static str,
    pub desctiption: &'static str,
    pub icon: &'static str,
    pub view_link: Option<&'static str>,
    pub read_component: fn() -> View<AnyView>,
}

impl Project {
    pub const ALL: &'static [Project] = &[
        savey::PROJECT,
        hlebcraft::PROJECT,
        trackpresence::PROJECT,
        zenith::PROJECT,
        sylvie::PROJECT,
    ];
}
