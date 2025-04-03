use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

#[component]
pub fn Blender(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    match entity {
        Entity::Blender => {
            rsx! {
                div {
                    onclick: move |_| game_state.write().click_entity(entity),
                    style,
                    p {
                        style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 5rem;",
                        "BLENDER"
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