use std::time::Duration;
use async_std::stream::StreamExt;

use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

const BLENDER_SVG: Asset = asset!("/assets/images/blender.svg");

#[component]
pub fn Blender(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    let advance_timer = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
        let duration = Duration::from_secs(3);
        while let Some(_) = rx.next().await {
            async_std::task::sleep(duration).await;
            game_state.write().advance();
        }
    });

    match entity {
        Entity::Blender => {
            let shake = if game_state().is_won() {
                "animation: shake 1s; animation-iteration-count: infinite;"
            } else {""};
            rsx! {
                div {
                    onclick: move |_| {
                        game_state.write().click_entity(entity);
                        if game_state.read().is_won() {
                            advance_timer.send(());
                        }
                    },
                    style,
                    img {
                        src: BLENDER_SVG,
                        style: "height: 90%; {shake}"
                    }
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