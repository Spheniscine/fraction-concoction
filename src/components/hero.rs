use dioxus::prelude::*;

use crate::{components::{math::Math, Dispenser, Recipe}, game::{random_name, Color, Entity, GameState}, utils::Fraction};


const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TEST_SVG: Asset = asset!("/assets/test.svg");

#[component]
pub fn Hero() -> Element {
    let test_tex = (Fraction::new(3, 16) + Fraction::new(5, 16)).to_tex();
    let name = random_name();

    let state = use_signal(|| GameState::new_test());

    rsx! {
        div {
            id: "hero",
            class: "select-none",
            Dispenser { 
                style: "position: absolute; left: 1.25rem; top: 2rem; width: 17.5rem; height: 40rem; 
                    display: flex; justify-content: center; align-items: center;",
                entity: Entity::Dispenser { color: Color::Red },
                game_state: state,
            },
            Dispenser { 
                style: "position: absolute; left: 21.25rem; top: 2rem; width: 17.5rem; height: 40rem; 
                    display: flex; justify-content: center; align-items: center;",
                entity: Entity::Dispenser { color: Color::Green },
                game_state: state,
            },
            Dispenser { 
                style: "position: absolute; left: 41.25rem; top: 2rem; width: 17.5rem; height: 40rem; 
                    display: flex; justify-content: center; align-items: center;",
                entity: Entity::Dispenser { color: Color::Blue },
                game_state: state,
            },
            Dispenser { 
                style: "position: absolute; left: 61.25rem; top: 2rem; width: 17.5rem; height: 40rem; 
                    display: flex; justify-content: center; align-items: center;",
                entity: Entity::Dispenser { color: Color::Yellow },
                game_state: state,
            },
            Dispenser {
                style: "position: absolute; left: 81.25rem; top: 2rem; width: 17.5rem; height: 40rem;
                     display: flex; justify-content: center; align-items: center;",
                entity: Entity::Dispenser { color: Color::Cyan },
                game_state: state,
            },

            // Recipe
            Recipe {
                style: "position: absolute; left: 1.25rem; top: 43rem; width: 53.5rem; height: 50rem; padding: 2rem; background-color: #ffc;
                     display: flex; flex-direction: column; justify-content: center; align-items: center; 
                     text-align: center;",
                game_state: state,
            },

            // div {
            //     style: "position: absolute; left: 1.25rem; top: 43rem; width: 53.5rem; height: 50rem; padding: 2rem; background-color: #ffc;
            //          display: flex; flex-direction: column; justify-content: center; align-items: center; 
            //          text-align: center;",
            //     p {
            //         style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 4rem;",
            //         "Recipe for:", br {},
            //         "{name}"
            //     },
            //     Math {
            //         style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 5rem;",
            //         tex: r#"\large\frac{{1}}{{2}} \Omega"#,
            //     },
            //     Math {
            //         style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 5rem;",
            //         tex: r#"{test_tex} \Delta"#,
            //     },
            //     Math {
            //         style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 5rem;",
            //         tex: r#"\large\frac{{7}}{{13}} \Psi"#,
            //     },
            // },


            // Blender
            div {
                style: "position: absolute; left: 61.25rem; top: 43rem; width: 33rem; height: 50rem; padding: 2rem; background-color: #ccc;
                     display: flex; flex-direction: column; justify-content: center; align-items: center; 
                     text-align: center;",
                p {
                    style: "margin-top: 2rem; margin-bottom: 2rem; font-size: 5rem;",
                    "BLENDER"
                }
            },

            div {
                style: "position: absolute; left: 2.5rem; top: 99rem; width: 23rem; height: 22.5rem; background-color: #0ff;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{1}}{{2}} \Omega"#,
                }
            },
            div {
                style: "position: absolute; left: 37.5rem; top: 99rem; width: 23rem; height: 22.5rem; background-color: #f00;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem; color: #fff",
                    tex: r#"2 \large\frac{{1113}}{{2224}} \Delta"#,
                }
            },
            div {
                style: "position: absolute; left: 74.5rem; top: 99rem; width: 23rem; height: 22.5rem; background-color: #ff0;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{7}}{{13}} \Psi"#,
                }
            },


            div {
                style: "position: absolute; left: 2rem; top: 123rem; width: 15rem; height: 24rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{2}}{{13}}"#,
                }
            },
            div {
                style: "position: absolute; left: 19rem; top: 123rem; width: 15rem; height: 24rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{1}}{{2}}"#,
                }
            },
            div {
                style: "position: absolute; left: 36rem; top: 123rem; width: 15rem; height: 24rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{1}}{{4}}"#,
                }
            },
            div {
                style: "position: absolute; left: 2rem; top: 151rem; width: 15rem; height: 24rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"1"#,
                }
            },
            div {
                style: "position: absolute; left: 19rem; top: 151rem; width: 15rem; height: 24rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{5}}{{13}}"#,
                }
            },
            div {
                style: "position: absolute; left: 36rem; top: 151rem; width: 15rem; height: 24rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center;",
                Math {
                    style: "font-size: 5rem;",
                    tex: r#"\large\frac{{3}}{{13}}"#,
                }
            },

            div {
                style: "position: absolute; left: 55rem; top: 127rem; width: 40rem; height: 45rem; background-color: #ccc;
                     display: flex; justify-content: center; align-items: center; font-size: 5rem;",
                "TRASH"
            },
        }
    }
}