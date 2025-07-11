use super::*;

pub static PAGE_OVERLAY_Z: u64 = u64::MAX / 2;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageProps {
    pub overlay_is_visible: bool,
    pub overlay_background_animation_delay_seconds: f64,
    pub overlay_background_animation_duration_seconds: f64,
    pub overlay_background_animation_timing_function: String,
    pub overlay_background_animation_backdrop_filter_blur: f64,
    pub overlay_background_color: String,
    pub overlay_background_class: Option<String>,
    pub overlay_background_style: Option<String>,
    pub overlay_content_class: Option<String>,
    pub overlay_content_style: Option<String>,
    pub overlay_class: Option<String>,
    pub overlay_style: Option<String>,
    pub overlay_holder_class: Option<String>,
    pub overlay_holder_style: Option<String>,
    pub overlay: Option<Element>,
    pub content_class: Option<String>,
    pub content_style: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        Holder {
            class: props.class,
            style: props.style,
            OverlayHolder {
                class: props.overlay_holder_class,
                style: props.overlay_holder_style,
                Overlay {
                    visible: props.overlay_is_visible,
                    background_animation_delay_seconds: props.overlay_background_animation_delay_seconds,
                    background_animation_duration_seconds: props.overlay_background_animation_duration_seconds,
                    background_animation_timing_function: props.overlay_background_animation_timing_function,
                    background_backdrop_filter_blur: props.overlay_background_animation_backdrop_filter_blur,
                    background_color: props.overlay_background_color,
                    background_class: props.overlay_background_class,
                    background_style: props.overlay_background_style,
                    content_class: props.overlay_content_class,
                    content_style: props.overlay_content_style,
                    class: props.overlay_class,
                    style: props.overlay_style,
                    { props.overlay }
                }
            }
            Content {
                class: props.content_class,
                style: props.content_style,
                { props.children }
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct HolderProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Holder(props: HolderProps) -> Element {
    rsx! {
        Stack {
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
            { props.children }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct ContentProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Content(props: ContentProps) -> Element {
    rsx! {
        StackItem {
            z: 0,
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: auto;
                    scroll-snap-type: y mandatory;
                    scroll-behaviour: smooth;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct OverlayHolderProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn OverlayHolder(props: OverlayHolderProps) -> Element {
    rsx! {
        StackItem {
            z: 0,
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: hidden;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
            { props.children }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct OverlayProps {
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
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Overlay(props: OverlayProps) -> Element {
    rsx! {
        Stack {
            class: props.class,
            style: props.style,
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
        StackItem {
            z: PAGE_OVERLAY_Z,
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
        StackItem {
            z: PAGE_OVERLAY_Z - 1,
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