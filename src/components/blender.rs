use std::time::Duration;
use async_std::stream::StreamExt;

use dioxus::prelude::*;

use crate::{components::{ErrorFeedback, Math, use_error_trigger}, game::{Entity, GameState}};

const BLENDER_SVG: Asset = asset!("/assets/images/blender.svg");

#[component]
pub fn Blender(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    let advance_timer = use_coroutine(move |mut rx: UnboundedReceiver<usize>| async move {
        let duration = Duration::from_secs(3);
        while let Some(index) = rx.next().await {
            if index != game_state.read().recipe.index { continue; }
            async_std::task::sleep(duration).await;
            game_state.write().advance();
        }
    });

    let error_trigger = use_error_trigger();
    let mut cloned = error_trigger.clone();
    let onclick = move |evt: Event<MouseData>| {
        if game_state.write().click_entity(entity).is_err() {
            cloned.trigger(evt.client_coordinates())
        }
        if game_state.read().is_won() {
            advance_timer.send(game_state.read().recipe.index);
        }
    };

    match entity {
        Entity::Blender => {
            let shake = if game_state().is_won() {
                "shake "
            } else {""};
            rsx! {
                div {
                    onclick,
                    style,
                    img {
                        class: shake,
                        src: BLENDER_SVG,
                        style: "height: 90%;"
                    },
                    ErrorFeedback { 
                        trigger: error_trigger.signal,
                    },
                },
            }
        },
        _ => {
            rsx! {
                div {
                    style: {style},
                    "Error: this component must be linked with an entity of type Blender."
                }
            }
        }
    }
}