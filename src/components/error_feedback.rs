use std::time::Duration;

use dioxus::{html::geometry::{ClientPoint}, prelude::*};

use crate::components::{ReadTrigger, ReadTriggerExt};

const FA_XMARK_SVG: Asset = asset!("/assets/images/fa-xmark.svg");

#[component]
pub fn ErrorFeedback(trigger: ReadTrigger<ClientPoint>) -> Element {
    let duration = Duration::from_secs_f32(0.5);

    let (class, x, y) = trigger.check(duration, |point| {
        ("fading", point.x as f32, point.y as f32)
    }).unwrap_or_default();
    rsx! {
        img {
            class,
            style: "position: fixed; left: {x}px; top: {y}px; width: 5rem; height: 5rem; 
            opacity: 0; transform: translate(-50%, -50%);",
            src: FA_XMARK_SVG,
        }
    }
}