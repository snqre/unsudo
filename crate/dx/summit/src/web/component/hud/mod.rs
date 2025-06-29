use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OverlayProps {
    pub background_animation_delay_seconds: f64,
    pub background_animation_duration_seconds: f64,
    pub background_animation_timing_function: String,
    pub background_backdrop_filter_blur: f64,
    pub background_style: Option<String>,
    pub content_style: Option<String>,
    pub visible: bool,
    pub children: Option<Element>
}

#[component]
pub fn Overlay(props: OverlayProps) -> Element {
    rsx! {
        layout::Stack {
            if props.visible && props.children.is_some() {
                OverlayContent {
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
                animation_delay_seconds: props.background_animation_delay_seconds,
                animation_duration_seconds: props.background_animation_duration_seconds,
                animation_timing_function: props.background_animation_timing_function,
                backdrop_filter_blur: props.background_backdrop_filter_blur,
                visible: props.visible,
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


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OverlayContentProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn OverlayContent(props: OverlayContentProps) -> Element {
    rsx! {
        layout::StackItem {
            z: u64::MAX,
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


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OverlayBackgroundProps {
    pub animation_duration_seconds: f64,
    pub animation_delay_seconds: f64,
    pub animation_timing_function: String,
    pub backdrop_filter_blur: f64,
    pub visible: bool,
    pub style: Option<String>
}

#[component]
pub fn OverlayBackground(props: OverlayBackgroundProps) -> Element {
    rsx! {
        layout::StackItem {
            z: u64::MAX - 1,
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
                color::OBSIDIAN,
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