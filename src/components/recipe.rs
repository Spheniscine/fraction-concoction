use dioxus::prelude::*;

use crate::{components::Math, game::{Entity, GameState}};

#[component]
pub fn Recipe(game_state: Signal<GameState>, style: String) -> Element {
    let state = game_state();
    let name = state.recipe.name.as_str();
    let ingredients_tex = state.recipe.ingredients.iter().map(|ingredient| {
        let tex = format!("{} {}", ingredient.amount.to_tex(), ingredient.color.to_tex_symbol());
        let color = if ingredient.done {"#ccc"} else {"#000"};
        (tex, color)
    });
    let recipe_index = state.recipe.index + 1;
    rsx! {
        div {
            style,
            p {
                style: "margin-top: 1.5rem; margin-bottom: 1.5rem; font-size: 4rem;",
                "Recipe for:", br {},
                {name}
            },

            for (tex, color) in ingredients_tex {
                Math {
                    style: "margin-top: 1.5rem; margin-bottom: 1.5rem; font-size: 5rem; color: {color}",
                    tex,
                },
            }

            p {
                style: "position: absolute; top: 1.5rem; right: 1.5rem; font-size: 3rem; font-style: italic",
                "page {recipe_index}"
            },
        }
    }
}