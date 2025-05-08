use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::game::Audio;

#[component]
pub fn AudioPreloader() -> Element {
    rsx! {
        for value in Audio::iter() {
            audio {
                preload: "auto",
                src: {value.asset()},
            }
        }
    }
}