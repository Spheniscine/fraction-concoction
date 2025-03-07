use dioxus::prelude::*;

use crate::components::math::Math;


const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TEST_SVG: Asset = asset!("/assets/test.svg");

#[component]
pub fn Hero() -> Element {
    // let opts = katex::Opts::builder().output_type(katex::OutputType::Html).build().unwrap();
    let html = katex::render(r#"1 \large\frac {12} {34} \Omega"#).unwrap();

    rsx! {
        div {
            id: "hero",
            div {
                style: "position: absolute; left: 1.25rem; top: 2rem; width: 17.5rem; height: 40rem; background-color: #f00",
            },
            div {
                style: "position: absolute; left: 21.25rem; top: 2rem; width: 17.5rem; height: 40rem; background-color: #0f0",
            },
            div {
                style: "position: absolute; left: 41.25rem; top: 2rem; width: 17.5rem; height: 40rem; background-color: #00f",
            },
            div {
                style: "position: absolute; left: 61.25rem; top: 2rem; width: 17.5rem; height: 40rem; background-color: #ff0",
            },
            div {
                style: "position: absolute; left: 81.25rem; top: 2rem; width: 17.5rem; height: 40rem; background-color: #0ff",
            },
            Math {
                style: "font-size: 5rem",
                tex: r#"1 \large\frac {{12}} {{34}} \Omega"#,
            }
        }
        // div {
        //     id: "hero",
        //     img { src: HEADER_SVG, id: "header" }
        //     div { id: "links",
        //         span {
        //             class: "select-none",
        //             dangerous_inner_html: html
        //         }
        //         a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
        //         a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
        //         a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
        //         a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
        //         a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
        //         a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        //     }
        // }
    }
}