use std::time::Duration;

use dioxus::prelude::*;

use crate::{components::{math::Math, AudioPreloader, Beaker, Blender, Dispenser, Dropper, HelpIcon, Recipe, Settings, SettingsIcon, Trash}, game::{random_name, Color, Difficulty, Entity, GameState}, utils::Fraction};

#[component]
pub fn Hero() -> Element {
    let test_tex = (Fraction::new(3, 16) + Fraction::new(5, 16)).to_tex();
    let name = random_name();

    let mut state = use_signal(|| {
        let mut state = GameState::new_test();
        state.generate(Difficulty::Easy);
        state
    });

    rsx! {
        AudioPreloader {  }
        div {
            id: "hero",
            class: "select-none",

            if !state().show_settings {
                Dispenser { 
                    style: "position: absolute; left: 1.25rem; top: 2rem; width: 17.5rem; height: 36rem; 
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dispenser { color: Color::Red },
                    game_state: state,
                },
                Dispenser { 
                    style: "position: absolute; left: 21.25rem; top: 2rem; width: 17.5rem; height: 36rem; 
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dispenser { color: Color::Green },
                    game_state: state,
                },
                Dispenser { 
                    style: "position: absolute; left: 41.25rem; top: 2rem; width: 17.5rem; height: 36rem; 
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dispenser { color: Color::Blue },
                    game_state: state,
                },
                Dispenser { 
                    style: "position: absolute; left: 61.25rem; top: 2rem; width: 17.5rem; height: 36rem; 
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dispenser { color: Color::Yellow },
                    game_state: state,
                },
                Dispenser {
                    style: "position: absolute; left: 81.25rem; top: 2rem; width: 17.5rem; height: 36rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dispenser { color: Color::Cyan },
                    game_state: state,
                },

                Recipe {
                    style: "position: absolute; left: 3rem; top: 41rem; width: 52rem; height: 50rem; padding: 2rem;
                        background-color: #ffc;
                        display: flex; flex-direction: column; justify-content: center; align-items: center; border-radius: 1rem;
                        text-align: center;",
                    game_state: state,
                },


                Blender {
                    style: "position: absolute; left: 61.25rem; top: 41rem; width: 33rem; height: 50rem; padding: 2rem;
                        display: flex; flex-direction: column; justify-content: center; align-items: center; 
                        text-align: center;",
                    entity: Entity::Blender,
                    game_state: state,
                },

                // Beakers
                Beaker {
                    style: "position: absolute; left: 2.5rem; top: 98rem; width: 25rem; height: 22.5rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Beaker { index: 0 },
                    game_state: state,
                },
                Beaker {
                    style: "position: absolute; left: 37.5rem; top: 98rem; width: 25rem; height: 22.5rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Beaker { index: 1 },
                    game_state: state,
                },
                Beaker {
                    style: "position: absolute; left: 74.5rem; top: 98rem; width: 25rem; height: 22.5rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Beaker { index: 2 },
                    game_state: state,
                },

                Dropper {
                    style: "position: absolute; left: 2rem; top: 123rem; width: 15rem; height: 24rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dropper { index: 0 },
                    game_state: state,
                },
                Dropper {
                    style: "position: absolute; left: 19rem; top: 123rem; width: 15rem; height: 24rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dropper { index: 1 },
                    game_state: state,
                },
                Dropper {
                    style: "position: absolute; left: 36rem; top: 123rem; width: 15rem; height: 24rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dropper { index: 2 },
                    game_state: state,
                },
                Dropper {
                    style: "position: absolute; left: 2rem; top: 151rem; width: 15rem; height: 24rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dropper { index: 3 },
                    game_state: state,
                },
                Dropper {
                    style: "position: absolute; left: 19rem; top: 151rem; width: 15rem; height: 24rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dropper { index: 4 },
                    game_state: state,
                },
                Dropper {
                    style: "position: absolute; left: 36rem; top: 151rem; width: 15rem; height: 24rem;
                        display: flex; justify-content: center; align-items: center;",
                    entity: Entity::Dropper { index: 5 },
                    game_state: state,
                },


                Trash {
                    style: "position: absolute; left: 55rem; top: 123rem; width: 40rem; height: 40rem;
                        display: flex; justify-content: center; align-items: center; font-size: 5rem;",
                    entity: Entity::Trash,
                    game_state: state,
                },

                HelpIcon {  
                    style: "position: absolute; left: 75rem; top: 164rem; width: 7rem; height: 11rem;",
                },

                SettingsIcon {  
                    style: "position: absolute; left: 86rem; top: 164.7rem; width: 10rem; height: 10rem;",
                    game_state: state,
                },
            } else {
                Settings {
                    game_state: state,
                }
            }
        }
    }
}