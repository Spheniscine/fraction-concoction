use dioxus::prelude::*;

use crate::{components::Math, game::{self, Entity, GameState, OptionColorExt, NEUTRAL_CONTRAST_COLOR, NEUTRAL_HTML_COLOR}};

#[component]
pub fn Beaker(entity: Entity, mut game_state: Signal<GameState>, style: String) -> Element {
    let state = game_state();
    match entity {
        Entity::Beaker { index } => {
            if let Some(beaker) = state.beakers[index] {
                let background_color = beaker.fill.to_html_color();
                let text_color = beaker.fill.contrast_html_color();
                let frac_tex = beaker.amount.to_tex();
                let color_tex = beaker.fill.as_ref().map(|c| c.to_tex_symbol()).unwrap_or("");
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
                            tex: "{frac_tex} {color_tex}",
                        }
                    }
                }
            } else {
                rsx! {}
            }
        },
        _ => {
            rsx! {
                div {
                    style: {style},
                    "Error: this component must be linked with an entity of type Beaker."
                }
            }
        }
    }
}