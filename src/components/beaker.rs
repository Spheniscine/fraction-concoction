use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState, OptionColorExt, NEUTRAL_CONTRAST_COLOR, NEUTRAL_HTML_COLOR}};

#[component]
pub fn Beaker(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    match entity {
        Entity::Beaker { index } => {
            if let Some(beaker) = game_state().beakers[index] {
                let background_color = beaker.fill.to_html_color();
                let text_color = beaker.fill.contrast_html_color();
                let frac_tex = beaker.amount.to_tex();
                let color_tex = beaker.fill.as_ref().map(|c| c.to_tex_symbol()).unwrap_or("");
                rsx! {
                    div {
                        style: "background-color: {background_color}; {style}",
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