#[macro_use]
extern crate extension_traits;

use dioxus::prelude::*;

use components::Hero;

mod components;
mod utils;
mod game;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: &str = include_str!("../assets/styling/main.css");
const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css",
            integrity: "sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib",
            crossorigin: "anonymous"
        }

        document::Link { rel: "icon", href: FAVICON }
        document::Style { {MAIN_CSS} }
        document::Style { {TAILWIND_CSS} }

        Hero {}
    }
}
