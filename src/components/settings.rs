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
    let close = move |_| {
        game_state.write().apply_settings(&state.read());
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
                    checked: state.read().audio_state,
                    onchange: audio_settings_changed
                }
            }

            p { 
                button {
                    r#type: "button",
                    onclick: close,
                    "Close"
                }
            }
        }
    }
}