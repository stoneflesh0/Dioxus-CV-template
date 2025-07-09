#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn education() -> Element {
    rsx! {
        h3 { style: "font-size: 32px; line-height: 0.5;", "Penguin Education" }
        dl { id: "education",
            dt { id: "date_bottom_gap", "2019 - 2021" }
            dd {
                p {
                    b { "AP Degree in Ice Technology" }
                    " at "
                    "Arctic University College"
                    " - Antarctica, Frostburg "
                    br {}
                    div { id: "bold_info",
                        "Specialized in: Pebble Programming, Ice-Embedded Systems, Blizzard Networking, Igloo Electronics, Waddle Project Management"
                        div { "Final Project: Krill Population Datascraping (Top Fish Grade)" }
                    }
                }
            }
            dt { id: "date_bottom_gap", "2015 - 2017" }
            dd {
                p {
                    b { "Bachelor's in Penguin Mobility Systems" }
                    " at "
                    "VAPM (Very Antarctic Penguin Mechanics)"
                    " - Antarctica, Iceberg City "
                }
            }
            dt { id: "date_bottom_gap", "2005 - 2016" }
            dd {
                p {
                    b { "Upper waddle education " }
                    " - Antarctica, South Pole Colony "
                    br {}
                }
            }
        }
    }
}
