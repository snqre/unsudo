#![allow(clippy::let_with_type_underscore)]
#![allow(unused)]

macro_rules! leak {
    ($($module:ident)*) => {
        $(
            mod $module; pub use $module::*;
        )*
    };
}

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus::document;

mod web;

#[component]
fn Main() -> Element {
    rsx! {
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-skyline" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/stray" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/br-cobane" }
        document::Stylesheet { href: asset!("asset/css/main.css") }
        document::Stylesheet { href: asset!("asset/css/stripe_animation.css") }
        Router::<web::route::Route> {}
    }
}

fn main() {
    launch(Main);
}