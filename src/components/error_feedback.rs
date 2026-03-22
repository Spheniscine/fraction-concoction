use std::time::Duration;

use dioxus::{html::geometry::{ClientPoint}, prelude::*};

use crate::components::{Trigger, TriggerSignal, use_trigger};

pub const ERROR_SVG: Asset = asset!("/assets/images/circle-xmark-regular.svg");

pub fn use_error_trigger() -> Trigger<ClientPoint> {
    use_trigger(Duration::from_secs_f32(0.5))
}

#[component]
pub fn ErrorFeedback(mut trigger: TriggerSignal<ClientPoint>) -> Element {
    let events = trigger.read();
    let iter = events.iter().map(|evt| {
        (evt.message.x as f32, evt.message.y as f32)
    });
    rsx! {
        for (x, y) in iter {
            img {
                class: "fading",
                style: "position: fixed; left: {x}px; top: {y}px; width: 7rem; height: 7rem; 
                opacity: 0; transform: translate(-50%, -50%);",
                src: ERROR_SVG,
            }
        }
    }
}