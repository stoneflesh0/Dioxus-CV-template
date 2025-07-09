use dioxus::prelude::*;
mod a_title;
mod b_professional_profile;
mod c_core_skills;
mod d_work_experience;
mod e_education;
mod f_additional;
mod g_flexing_tools;

const MAIN_CSS: Asset = asset!("./assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { id: "whole_page",
            document::Stylesheet { href: MAIN_CSS }

            a_title::title {}
            b_professional_profile::professional_profile {}
            c_core_skills::core_skills {}
            d_work_experience::work_experience {}

            //div { class: "page-number", "Page 1/2" }
            //div { style: "page-break-after: always;" }

            e_education::education {}
            f_additional::additional {}
            hr {}
            g_flexing_tools::flexing_tools {}
        }
    }
}
