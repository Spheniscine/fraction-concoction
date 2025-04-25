use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::game::{Difficulty, Feedback, GameState};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    let mut state = use_signal(|| {
        game_state.read().new_settings_state()
    });

    let mut difficulty_changed = move |evt: Event<FormData>| {
        state.write().difficulty = evt.value().parse().unwrap_or(Difficulty::Easy);
        if state.read().difficulty != game_state.read().difficulty {
            state.write().reset_level = true;
        }
    };

    let reset_level_changed = move |evt: Event<FormData>| {
        state.write().reset_level = evt.checked();
    };

    let adaptive_difficulty_changed = move |evt: Event<FormData>| {
        state.write().adaptive_difficulty = evt.checked();
    };

    let keep_dropper_selection_settings_changed = move |evt: Event<FormData>| {
        state.write().keep_dropper_selection = evt.checked();
    };

    let audio_settings_changed = move |evt: Event<FormData>| {
        state.write().audio_state = evt.checked();
    };

    let mut ok = move |_| {
        game_state.write().apply_settings(&state.read());
        game_state.write().show_settings = false;
    };
    let mut cancel = move |_| {
        game_state.write().show_settings = false;
    };

    let mut onmounted = async move |e: Event<MountedData>| {
        e.set_focus(true).await;
    };
    let mut onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                game_state.write().apply_settings(&state.read());
                game_state.write().show_settings = false;
            }
            Key::Escape => {
                game_state.write().show_settings = false;
            }
            _ => {}
        }
    };

    rsx! {
        style {
            "#settingsDialog:focus {{ outline: none; }}"
        }
        div {
            id: "settingsDialog",
            style: "margin: 2.5%; padding: 5rem; width: 85%; height: 91.5%; background-color: #ccc; font-size: 5rem; line-height: 10rem;
                border-radius: 2rem;",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,
            
            p { "Difficulty options: ",
                select {
                    style: "font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onchange: difficulty_changed,
                    for difficulty in Difficulty::iter() {
                        option { 
                            selected: difficulty == state.read().difficulty,
                            "{difficulty}" 
                        },
                    }
                },
            },

            p { 
                "Reset current level: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().reset_level,
                    disabled: state.read().difficulty != game_state.read().difficulty,
                    onchange: reset_level_changed
                }
            },

            p { 
                "Adaptive difficulty: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().adaptive_difficulty,
                    onchange: adaptive_difficulty_changed
                }
            },

            p { 
                "Keep vial selected after pouring: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().keep_dropper_selection,
                    onchange: keep_dropper_selection_settings_changed
                }
            },

            p { 
                "Audio: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().audio_state,
                    onchange: audio_settings_changed
                }
            },

            p { 
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: ok,
                    "OK"
                },
                " ",
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: cancel,
                    "Cancel"
                },
            },

            p {
                style: "position: absolute; bottom: 1.5rem; font-size: 3rem;",
                "© OnlineMathLearning.com"
            },
        }
    }
}