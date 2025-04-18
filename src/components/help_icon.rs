use dioxus::prelude::*;

const FA_QUESTION_SVG: Asset = asset!("/assets/images/fa-question.svg");
const TUTORIAL_VIDEO: Asset = asset!("/assets/video/fraction-concoction-tutorial.mp4");

#[component]
pub fn HelpIcon(style: String) -> Element {
    rsx! {
        div {
            style,
            a {
                href: TUTORIAL_VIDEO,
                target: "_blank",
                img {
                    src: FA_QUESTION_SVG,
                }
            }
        }
    }
}