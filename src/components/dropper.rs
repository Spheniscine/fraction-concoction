use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState, OptionColorExt, NEUTRAL_CONTRAST_COLOR, NEUTRAL_HTML_COLOR}};

#[component]
pub fn Dropper(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    let state = game_state();
    match entity {
        Entity::Dropper { index } => {
            let dropper = state.droppers[index];
            let background_color = dropper.fill.to_html_color();
            let text_color = dropper.fill.contrast_html_color();
            let tex = dropper.capacity.to_tex();
            let selected_border = if state.selected == Some(entity) {
                "border-style: solid; border-width: 0.7rem; border-color: #fc0;"
            } else {
                ""
            };
            rsx! {
                div {
                    onclick: move |_| game_state.write().click_entity(entity),
                    style: "background-color: {background_color}; {selected_border} {style}",
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