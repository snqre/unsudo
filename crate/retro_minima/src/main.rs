use dioxus::prelude::*;
use dioxus::document;

mod color;
mod counter;
mod form;
mod spiked_button;
mod zip_line;

#[component]
fn Main() -> Element {
    rsx! {
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
            spiked_button::SpikedButton {}
        }
    }
}

fn main() {
    launch(Main);
}