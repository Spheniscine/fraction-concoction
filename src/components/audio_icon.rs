use dioxus::prelude::*;

use crate::game::{Feedback, GameState};

const VOLUME_OFF_SVG: Asset = asset!("/assets/images/volume-off.svg");
const VOLUME_ON_SVG: Asset = asset!("/assets/images/volume-on.svg");

#[component]
pub fn AudioIcon(style: String, game_state: Signal<GameState>) -> Element {
    let src = if game_state.read().feedback.get_audio_state() == 0. {VOLUME_OFF_SVG} else {VOLUME_ON_SVG};
    rsx! {
        div {
            style,
            img { // preload
                style: "width: 0; height: 0;",
                src: VOLUME_OFF_SVG,
            },
            img { // preload
                style: "width: 0; height: 0;",
                src: VOLUME_ON_SVG,
            },
            img {
                style: "width: 14rem; height: 11rem; cursor: pointer;",
                src: src,
                onclick: move |_| {game_state.write().toggle_audio();}
            }
        }
    }
}