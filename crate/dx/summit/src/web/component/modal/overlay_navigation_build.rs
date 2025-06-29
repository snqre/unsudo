use super::*;

#[derive(Clone, PartialEq)]
pub enum OverlayNavigationBuildState {
    Minimal,
    FullScreen
}

#[component]
pub fn OverlayNavigationBuild() -> Element {
    let state: Signal<_> = use_signal(|| OverlayNavigationBuildState::Minimal);

    rsx! {
        OverlayBuild {
            visible: state() == OverlayNavigationBuildState::FullScreen,
            layout::ColFill {
                layout::Row {}
                layout::Col {}
            }
        }
    }
}