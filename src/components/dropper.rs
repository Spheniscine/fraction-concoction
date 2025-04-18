use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState, OptionColorExt, NEUTRAL_CONTRAST_COLOR, NEUTRAL_HTML_COLOR}, utils::Fraction};

const DROPPER_BACK_SVG: Asset = asset!("/assets/images/test-tube.svg");

#[component]
pub fn Dropper(entity: Entity, game_state: Signal<GameState>, style: String) -> Element {
    let state = game_state();
    match entity {
        Entity::Dropper { index } => {
            let dropper = state.droppers[index];
            if dropper.capacity == Fraction::zero() {
                rsx!{}
            } else {
                let background_color = dropper.fill.to_html_color();
                let text_color = dropper.fill.contrast_html_color();
                let tex = dropper.capacity.to_tex();

                let selected_background = if state.selected == Some(entity) {
                    "filter: drop-shadow(0 0 2rem #ff0);"
                } else {
                    ""
                };
                rsx! {
                    div {
                        onclick: move |_| game_state.write().click_entity(entity),
                        style: "{style}",

                        img { 
                            src: DROPPER_BACK_SVG,
                            style: "position: absolute; margin: 0 auto; width: 15rem; height: 28rem; {selected_background}",
                        }

                        div {
                            style: "background-color: {background_color}; position: absolute; top: 6.1rem; left: 3.4rem; 
                            height: 18.2rem; width: 8.3rem; border-radius: 0 0 5rem 5rem; 
                            line-height: 18.2rem; text-align: center;",
                            Math {
                                style: "font-size: 4.5rem; color: {text_color}",
                                tex: {tex},
                            }
                        }
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