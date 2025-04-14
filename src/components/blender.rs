use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

const BLENDER_SVG: Asset = asset!("/assets/images/blender.svg");

#[component]
pub fn Blender(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    match entity {
        Entity::Blender => {
            rsx! {
                div {
                    onclick: move |_| game_state.write().click_entity(entity),
                    style,
                    img {
                        src: BLENDER_SVG,
                        style: "height: 90%;"
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