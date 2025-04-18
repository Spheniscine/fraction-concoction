use dioxus::prelude::*;

const FA_QUESTION_SVG: Asset = asset!("/assets/images/fa-question.svg");

#[component]
pub fn HelpIcon(style: String) -> Element {
    rsx! {
        div {
            style,
            img {
                src: FA_QUESTION_SVG,
            }
        }
    }
}