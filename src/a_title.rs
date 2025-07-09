#![allow(non_snake_case)]
use dioxus::prelude::*;

const PORTRAIT_PICTURE: Asset = asset!("assets/portrait.png", ImageAssetOptions::new().with_avif());

#[component]
pub fn title() -> Element {
    rsx! {
        div { id: "title_name",
            img {
                id: "portrait",
                alt: "Linux Tux profile picture.",
                src: PORTRAIT_PICTURE,
            }
            span { style: "font-size: 34px;",
                b { "Linux Tux" }
            }
        }
        br {}
        div { id: "title_contacts",
            b { "Antarctica" }
            " | "
            a { href: "mailto:support@example.com", "support@example.com"}
            " | "
            "+1 555 360-1234"
            " | "
            a { href: "https://gitlab.com/yournamehere", "GitLab" }
        }
    }
}
