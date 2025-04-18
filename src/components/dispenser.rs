use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

const DISPENSER_SVG: Asset = asset!("/assets/images/dispenser.svg");

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
                    style: {style},
                    img { 
                        src: DISPENSER_SVG,
                        style: "position: absolute; margin: 0 auto; width: 18rem",
                    }

                    div {
                        style: "background-color: {background_color}; position: absolute; top: 8.5rem; padding: 2rem; 
                        left: 3.2rem; width: 7.5rem; border-radius: 1rem; text-align: center",
                        Math {
                            style: "font-size: 7rem; color: {text_color}",
                            tex: {tex},
                        }
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