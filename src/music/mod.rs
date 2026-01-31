pub struct Music {
    pub name: &'static str,
    pub filename: &'static str,
    pub description: &'static str,
}

impl Music {
    const fn new(name: &'static str, filename: &'static str, description: &'static str) -> Music {
        Self {
            name,
            filename,
            description,
        }
    }

    pub fn all() -> &'static [Music] {
        &ALL_MUSIC[..]
    }
}

const ALL_MUSIC: [Music; 5] = [
Music::new("sunny butter", "sunny_butter.mp3",
    "This was the first track I ever made.
    It was supposed to sound calm and gentle, but after a few listens I realized that I used the same melody as Sayo-nara from DDLC 
    - so it kinda ended up with a darker vibe :_)"),
Music::new("coconut soup", "coconut_soup.mp3", 
"This melody has kind of a beach vibe - maybe it could even work as a soundtrack for some game."),
Music::new("papa's secret technique", "papas_secret_technique.mp3", 
"A battle track inspired by Mortal Kombat and LISA: The Painful."),
Music::new("auspicious hamlet", "auspicious_hamlet.mp3", 
"A Zelda-like soundtrack for a humble village."),
Music::new("blur", "blur.mp3", 
"Calm and peaceful sound.")
];
