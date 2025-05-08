use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::game::Audio;

#[component]
pub fn AudioPreloader() -> Element {
    rsx! {
        for value in Audio::iter() {
            link {
                rel: "preload",
                href: {value.asset()},
                r#as: "audio",
            }
        }
    }
}