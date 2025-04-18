use dioxus::prelude::*;

const FA_GEAR_SVG: Asset = asset!("/assets/images/fa-gear.svg");

#[component]
pub fn SettingsIcon(style: String) -> Element {
    rsx! {
        div {
            style,
            img {
                src: FA_GEAR_SVG,
            }
        }
    }
}