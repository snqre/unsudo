use super::*;

enum NavbarState {
    Idle,
}

#[component]
pub fn Build() -> Element {
    let device: Signal<_> = ::window::use_device();

    rsx! {
        if device() == ::window::Device::MobileL || device() == ::window::Device::MobileM || device() == ::window::Device::Mobile {
            Scaffold {
                left: rsx! {
                    Logo {}
                },
                right: rsx! {
                    MoreIcon {}
                }
            }
        } else {
            Scaffold {
                left: rsx! {
                    Logo {}
                },
                layout::Row {
                    OpenSectionButton {}
                    OpenSectionButton {}
                    OpenSectionButton {}
                }
            }
        }
    }
}