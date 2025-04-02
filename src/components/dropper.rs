use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState, OptionColorExt, NEUTRAL_CONTRAST_COLOR, NEUTRAL_HTML_COLOR}};

#[component]
pub fn Dropper(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    match entity {
        Entity::Dropper { index } => {
            let dropper = game_state().droppers[index];
            let background_color = dropper.fill.to_html_color();
            let text_color = dropper.fill.contrast_html_color();
            let tex = dropper.capacity.to_tex();
            rsx! {
                div {
                    style: "background-color: {background_color}; {style}",
                    Math {
                        style: "font-size: 5rem; color: {text_color}",
                        tex: {tex},
                    }
                }
            }
        },
        _ => {
            rsx! {
                div {
                    style: {style},
                    "Error: this component must be linked with an entity of type Dropper."
                }
            }
        }
    }
}