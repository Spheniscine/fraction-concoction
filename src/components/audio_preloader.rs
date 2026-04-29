use std::cell::OnceCell;

use dioxus::{document::Eval, prelude::*};
use enum_map::EnumMap;
use strum::IntoEnumIterator;

use crate::game::Audio;

thread_local! {
    pub static AUDIO: OnceCell<EnumMap<Audio, Eval>> = OnceCell::new();
}

#[component]
pub fn AudioPreloader() -> Element {
    use_effect(|| {
        AUDIO.with(|cell| {
            cell.get_or_init(|| {
                EnumMap::from_fn(|audio: Audio| {
                    document::eval(format!(r#"
                        var howl = new Howl ({{ src: ['{}'] }});
                        while (true) {{
                            let volume = await dioxus.recv();
                            howl.volume(volume);
                            howl.play();
                        }}
                    "#, audio.asset()).as_str())
                })
            });
        })
    });
    rsx! {}
}