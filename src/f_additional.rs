//#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn additional() -> Element {
    rsx! {
        h3 { style: "font-size: 32px; line-height: 0.5;", "Penguin Skills & Interests" }
        body { id: "additional",
            li {
                b { "Programming Languages" }
                ": Krattish, Sealish, Whale-song, Human (broken English)"
            }
            li {
                b { "Platforms" }
                ": Icebergs, Pack Ice, Open Water, Tourist Boats"
            }
            li {
                b { "Tools" }
                ": Beak, Flippers, Waddle™ 2.0, Icepick (for stubborn fish)"
            }
            li {
                b { "Frameworks" }
                ": "
                div { id: "padding_additional_items", "• Tobogganing: FastSlide, SnowPlow, BellyFirst" }
                div { id: "padding_additional_items",
                    "• Social: HuddlePro™, PebbleGift 3.0"
                }
            }
            li {
                b { "Interests" }
                ": Ice sculpting, synchronized swimming, fish tasting on,"
                "pebble collecting, climate change awareness"
            }
        }
    }
}
