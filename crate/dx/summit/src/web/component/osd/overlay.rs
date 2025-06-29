use super::*;

pub static OVERLAY_Z: u64 = u64::MAX / 2;

#[derive(Props, Clone, PartialEq)]
pub struct OverlayProps {
    pub visible: bool,
    pub background_animation_delay_seconds: f64,
    pub background_animation_duration_seconds: f64,
    pub background_animation_timing_function: String,
    pub background_backdrop_filter_blur: f64,
    pub background_color: String,
    pub background_class: Option<String>,
    pub background_style: Option<String>,
    pub content_class: Option<String>,
    pub content_style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Overlay(props: OverlayProps) -> Element {
    rsx! {
        layout::Stack {
            if props.visible && props.children.is_some() {
                OverlayContent {
                    class: props.content_class,
                    style: format! {
                        r#"
                            {}
                        "#,
                        props.content_style.to_owned().unwrap_or_default()
                    },
                    { props.children }
                }
            }
            OverlayBackground {
                visible: props.visible,
                animation_delay_seconds: props.background_animation_delay_seconds,
                animation_duration_seconds: props.background_animation_duration_seconds,
                animation_timing_function: props.background_animation_timing_function,
                backdrop_filter_blur: props.background_backdrop_filter_blur,
                color: props.background_color,
                class: props.background_class,
                style: format! {
                    r#"
                        {}
                    "#,
                    props.background_style.to_owned().unwrap_or_default()
                }
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct OverlayContentProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn OverlayContent(props: OverlayContentProps) -> Element {
    rsx! {
        layout::StackItem {
            z: OVERLAY_Z,
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100vw;
                    max-width: 100vw;
                    height: 100%;
                    min-height: 100vh;
                    max-height: 100vh;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
            { props.children }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct OverlayBackgroundProps {
    pub visible: bool,
    pub animation_duration_seconds: f64,
    pub animation_delay_seconds: f64,
    pub animation_timing_function: String,
    pub backdrop_filter_blur: f64,
    pub color: String,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
fn OverlayBackground(props: OverlayBackgroundProps) -> Element {
    rsx! {
        layout::StackItem {
            z: OVERLAY_Z - 1,
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100vw;
                    max-width: 100vw;
                    height: 100%;
                    min-height: 100vh;
                    max-height: 100vh;
                    background: {};
                    opacity: {};
                    backdrop-filter: blur({}px);
                    transition-property: opacity, backdrop-filter;
                    transition-duration: {}s;
                    transition-delay: {}s;
                    transition-timing-function: ease;
                    {}
                "#,
                props.color,
                if props.visible {
                    "0.5"
                } else {
                    "0"
                },
                if props.visible {
                    props.backdrop_filter_blur
                } else {
                    0.0
                },
                props.animation_duration_seconds,
                props.animation_delay_seconds,
                props.style.to_owned().unwrap_or_default()
            }
        }
    }
}