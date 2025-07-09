#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn core_skills() -> Element {
    rsx! {
        h3 { style: "font-size: 32px; line-height: 0.5;", "Core Skills" }
        body {
            div { id: "container",
                div { id: "left",
                li { "Swimming (fast, stealthy)" }
                li { "Feather Care (cleaning)" }
                li { "Hunting Skills (fish)" }
                li { "Chirping, Hooting, Vocalizing" }
                }
                div { id: "middle",
                    li { "Burrowing" }
                    li { "Cold Weather Survival" }
                    li { "Adapting to Different Climates" }
                    li { "Building Snow Homes" }
                }
                div { id: "right",
                    li { "English ●●●●●" } 
                    li { "Penguinean ●●●●●" } 
                    li { "Penguinian ●●○○○" } 
                    li { "Penguinesque ●○○○○" }
                }
            }
        }
    }
}
