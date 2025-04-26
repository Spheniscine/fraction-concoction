use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

const TRASH_SVG: Asset = asset!("/assets/images/sink.svg");

#[component]
pub fn Trash(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    match entity {
        Entity::Trash => {
            rsx! {
                div {
                    onclick: move |_| game_state.write().click_entity(entity),
                    style,
                    img {
                        style: "width: 40rem; height: 40rem;",
                        src: TRASH_SVG,
                    }
                },
            }
        },
        _ => {
            rsx! {
                div {
                    style: {style},
                    "Error: this component must be linked with an entity of type Trash."
                }
            }
        }
    }
}