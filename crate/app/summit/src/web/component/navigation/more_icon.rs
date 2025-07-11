use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoreIconProps {
    
}

#[component]
pub fn MoreIcon() -> Element {
    rsx! {
        decor::Icon { size: "{u(2)}px", url: asset!("asset/icon/more_h.svg") }
    }
}

