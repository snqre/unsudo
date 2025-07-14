use dioxus::prelude::*;
use dioxus::document;








mod web;
mod web_component;
mod web_route;

#[component]
fn Main() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("asset/css/animation/faulty-neon.css") }
        document::Stylesheet { href: asset!("asset/css/animation/stripe.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-scrollbar.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-thumb-hover.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-thumb.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-track.css") }
        document::Stylesheet { href: asset!("asset/css/color.css") }
        document::Stylesheet { href: asset!("asset/css/reset.css") }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-skyline" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/stray" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/br-cobane" }
        document::Stylesheet { href: asset!("asset/css/main.css") }
        Router::<web_route::Route> {}
    )
}

fn main() {
    launch(Main);
}