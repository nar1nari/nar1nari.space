mod sylvie;
mod zenith;

use leptos::prelude::*;

pub struct Project {
    pub name: &'static str,
    pub slug: &'static str,
    pub desctiption: &'static str,
    pub view_link: Option<&'static str>,
    pub read_component: fn() -> View<AnyView>,
}

impl Project {
    pub const ALL: &'static [Project] = &[zenith::PROJECT, sylvie::PROJECT];
}
