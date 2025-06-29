use super::*;

pub static OVERLAY_BUILD_IS_VISIBLE: GlobalSignal<bool> = GlobalSignal::new(|| false);

#[derive(Props, Clone, PartialEq)]
pub struct OverlayBuildProps {
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
            visible: OVERLAY_BUILD_IS_VISIBLE(),
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