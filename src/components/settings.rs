use dioxus::prelude::*;

use crate::game::{Feedback, GameState};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    let mut state = use_signal(|| {
        game_state.read().new_settings_state()
    });

    let audio_settings_changed = move |evt: Event<FormData>| {
        state.write().audio_state = evt.checked();
    };

    let ok = move |_| {
        game_state.write().apply_settings(&state.read());
        game_state.write().show_settings = false;
    };
    let cancel = move |_| {
        game_state.write().show_settings = false;
    };
    rsx! {
        div {
            style: "margin: 2.5%; padding: 5rem; width: 85%; height: 91.5%; background-color: #ccc; font-size: 5rem; line-height: 10rem;
                border-radius: 2rem;",
            p { "Difficulty options: Coming soon!" }
            p { 
                "Audio: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().audio_state,
                    onchange: audio_settings_changed
                }
            }

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
            }
        }
    }
}