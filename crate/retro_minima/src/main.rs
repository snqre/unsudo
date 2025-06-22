use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus::document;

mod color;
mod counter;
mod easing;
mod form;
mod reveal;
mod spiked_button;
mod zip_line;

#[component]
fn Main() -> Element {
    rsx! {
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-skyline" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/stray" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/br-cobane" }
        document::Stylesheet { href: asset!("asset/css/main.css") }
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                min-width: 100px;
                min-height: 100vh;
                background: linear-gradient(to bottom right, {color::OBSIDIAN}, {color::CARBON});
            "#,
            form::DropDown {

            }
        }
    }
}

fn main() {
    launch(Main);
}