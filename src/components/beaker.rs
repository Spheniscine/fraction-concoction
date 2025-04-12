use dioxus::prelude::*;

use crate::{components::Math, game::{self, Entity, GameState, OptionColorExt, NEUTRAL_CONTRAST_COLOR, NEUTRAL_HTML_COLOR}};

const BEAKER_BACK_SVG: Asset = asset!("/assets/images/beaker-back.svg");

#[component]
pub fn Beaker(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    let state = game_state();
    match entity {
        Entity::Beaker { index } => {
            if let Some(beaker) = state.beakers[index] {
                let background_color = beaker.fill.to_html_color();
                let text_color = beaker.fill.contrast_html_color();
                let frac_tex = beaker.amount.to_tex();
                let color_tex = beaker.fill.as_ref().map(|c| c.to_tex_symbol()).unwrap_or("");
                let selected_background = if state.selected == Some(entity) {
                    "filter: drop-shadow(0 0 2rem #ff0);"
                } else {
                    ""
                };
                rsx! {
                    div {
                        onclick: move |_| game_state.write().click_entity(entity),
                        style: {style},

                        img { 
                            src: BEAKER_BACK_SVG,
                            style: "position: absolute; margin: 0 auto; width: 29rem; height: 28rem; {selected_background}",
                        }

                        div {
                            style: "background-color: {background_color}; position: absolute; top: 3.5rem; left: 5.2rem; 
                            height: 18.5rem; width: 16.4rem; border-radius: 0 0 3rem 3rem; 
                            line-height: 18.5rem; text-align: center;",
                            Math {
                                style: "font-size: 4.5rem; color: {text_color}",
                                tex: "{frac_tex} {color_tex}",
                            }
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