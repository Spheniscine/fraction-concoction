use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

#[component]
pub fn Dispenser(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    match entity {
        Entity::Dispenser { color } => {
            let background_color = color.to_html_color();
            let text_color = color.contrast_html_color();
            let tex = color.to_tex_symbol();
            rsx! {
                div {
                    onclick: move |_| game_state.write().click_entity(entity),
                    style: "background-color: {background_color}; {style}",
                    Math {
                        style: "font-size: 7rem; color: {text_color}",
                        tex: {tex},
                    }
                }
            }
        },
        _ => {
            rsx! {
                div {
                    style: {style},
                    "Error: this component must be linked with an entity of type Dispenser."
                }
            }
        }
    }
}