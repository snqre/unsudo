use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct OverlayBuildProps {
    pub visible: bool,
    pub background_class: Option<String>,
    pub background_style: Option<String>,
    pub content_class: Option<String>,
    pub content_style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn OverlayBuild(props: OverlayBuildProps) -> Element {
    rsx! {
        Overlay {
            visible: props.visible,
            background_animation_delay_seconds: 0.0,
            background_animation_duration_seconds: 3.0,
            background_animation_timing_function: "ease",
            background_backdrop_filter_blur: u(1),
            background_color: color::OBSIDIAN,
            background_class: props.background_class,
            background_style: props.background_style,
            content_class: props.content_class,
            content_style: props.content_style,
            { props.children }
        }
    }
}