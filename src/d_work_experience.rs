#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn work_experience() -> Element {
    rsx! {
        h3 { style: "font-size: 32px; line-height: 0.5;", "Work Experience" }
        dl { id: "work_experience_indent",  
            dt { id: "date_bottom_gap", "2023-01 - 2024-01 " }  
            dd {  
                p {  
                    b { "Iceberg Systems Manager & Fish Code Developer" }  
                    " at "  
                    b { "Snowflake Data Waddle" }  
                    " - Antarctica, Frosty Shores "  
                    br {}  
                    div { id: "bold_info",  
                        div { "• Developed and maintained fish-tracking integrations, automated krill distribution tasks." }  
                        div {  
                            "• Collaborated with seal and whale teams to define hunting strategies and migration timelines."  
                        }  
                        div { "• Maintained and supported Blizzard Cloud (AWS for penguins) and Iceberg Backup Systems." }  
                        div {  
                            "• Created pebble-based documentation for standardized nest-building techniques."  
                        }  
                    }  
                }  
            }  
            dt { id: "date_bottom_gap", "2022-01 - 2023-01 " }  
            dd {  
                p {  
                    b { "Professional Ice Tech Innovator" }  
                    " at "  
                    b { "Snowflake Data Waddle" }  
                    " - Antarctica, Frosty Shores "  
                    br {}  
                    div { id: "bold_info",  
                        div { "• Developed automated fish-counting systems and optimized waddle routes." }  
                        div { "• Designed igloo cloud architecture for better heat retention." }  
                        div { "• Provided technical support for lost baby penguins and confused seals." }  
                        div {  
                            "• Diagnosed and resolved critical iceberg stability issues for sliding competitions."  
                        }  
                    }  
                }  
            }  
            dt { id: "date_bottom_gap", "2021-01 - 2022-01 " }  
            dd {  
                p {  
                    b { "Junior Snow & Ice Engineer" }  
                    " at "  
                    b { "Snowflake Data Waddle" }  
                    " - Antarctica, Frosty Shores "  
                    br {}  
                    div { id: "bold_info",  
                        div { "• Developed a custom IceOS for Penguin Interaction Kiosks." }  
                        div {  
                            "• Automated fish delivery systems, reducing hunting time with zero pebble configuration."  
                        }  
                        div { "• Designed and implemented Penguin-to-Iceberg IoT tracking devices." }  
                        div { "• Prototyped sled-based transportation for elderly penguins." }  
                    }  
                }  
            }  
            dt { id: "date_bottom_gap", "2015-01 - 2022-01 " }  
            dd {  
                p {  
                    b { "Part-Time Pebble Entrepreneur" }  
                    " - Antarctica, Pebble Beach"  
                    br {}  
                    div { id: "bold_info",  
                        "• Repaired broken ice slides, fixed malfunctioning fish alarms, and traded shiny pebbles."  
                    }  
                }  
            }  
        }  
    }
}
