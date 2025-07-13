use ::dioxus::prelude::*;
use ::dioxus::document;
use ::modwire::*;

mod web;

#[component]
fn Main() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("asset/css/animation/faulty_neon.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit_scrollbar.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit_thumb_hover.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit_thumb.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit_track.css") }
        document::Stylesheet { href: asset!("asset/css/color.css") }
        document::Stylesheet { href: asset!("asset/css/reset.css") }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-skyline" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/stray" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/br-cobane" }
        document::Stylesheet { href: asset!("asset/css/main.css") }
        document::Stylesheet { href: asset!("asset/css/stripe_animation.css") }
        Router::<web::route::Route> {}
    )
}

fn main() {
    launch(Main);
}