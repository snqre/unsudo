use super::*;
use crate::web;

pub struct RevealOnVScrollPercentageProps {
    pub threshold: f32
}

#[component]
pub fn RevealOnVScrollPercentage(props: RevealOnVScrollPercentageProps) -> Element {
    let scroll: Signal<_> = web::window::use_scroll_v_percentage();
    rsx! {
        if scroll() >= props.threshold {
            div {
                style: r#"
                    display: contents;
                "#
            }
        } else {
            div {
                style: r#"
                    display: contents;
                "#
            }
        }
    }
}