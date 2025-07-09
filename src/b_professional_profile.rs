#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn professional_profile() -> Element {
    rsx! {
        hr {}
        h3 { style: "font-size: 32px; line-height:0.5;", "Professional Profile" }
        p { id: "professional_profile",
        "Penguin living on the icy tundra, spending its days foraging for fish and crustaceans in the frigid waters. It builds its cozy nest using boulders and seaweed to keep warm through the long nights. The penguin often rests by huddling together with other penguins for added warmth and protection from predators. It also communicates with its peers using a variety of vocalizations to maintain social cohesion and coordinate hunting strategies."
        }
    }
}
