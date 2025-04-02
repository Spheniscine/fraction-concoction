use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

#[component]
pub fn Recipe(game_state: Signal<GameState>, style: String) -> Element {
    let state = game_state();
    let name = state.recipe.name.as_str();
    let ingredients_tex = state.recipe.ingredients.iter().map(|ingredient| {
        format!("{} {}", ingredient.amount.to_tex(), ingredient.color.to_tex_symbol())
    });
    rsx! {
        div {
            style,
            p {
                style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 4rem;",
                "Recipe for:", br {},
                {name}
            },

            for tex in ingredients_tex {
                Math {
                    style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 5rem;",
                    tex,
                },
            }
        }
    }
}