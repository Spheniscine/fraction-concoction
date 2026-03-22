use std::time::Duration;

use dioxus::{html::geometry::{ClientPoint}, prelude::*};

use crate::components::{Trigger, TriggerExt};

const FA_XMARK_SVG: Asset = asset!("/assets/images/fa-xmark.svg");

#[component]
pub fn ErrorFeedback(mut trigger: Trigger<ClientPoint>) -> Element {
    let duration = Duration::from_secs_f32(0.5);

    let events = trigger.read();
    let iter = events.iter().map(|evt| {
        (evt.id, evt.message.x as f32, evt.message.y as f32)
    });
    rsx! {
        for (key, x, y) in iter {
            img {
                key,
                id: "error-feedback-{key}",
                class: "fading",
                style: "position: fixed; left: {x}px; top: {y}px; width: 5rem; height: 5rem; 
                opacity: 0; transform: translate(-50%, -50%);",
                src: FA_XMARK_SVG,
            }
        }
    }
}