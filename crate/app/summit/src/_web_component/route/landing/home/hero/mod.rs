

#[component]
pub fn Hero() -> Element {
    rsx!(
        layout::PageItem {

        }
    )
}





#[component]
fn CaptionCard() -> Element {
    rsx!(

    )
}

#[component]
fn SocialIconGroup() -> Element {
    rsx!(

    )
}

#[derive(Props, Clone, PartialEq)]
struct SocialIconProps {
    pub url: Url,
    pub to: String
}
#[component]
fn SocialIcon() -> Element {
    rsx!(
        util::Link {
            to: props.to,
            decor::Icon {
                url: props.url,
                size: format!("{}px", u(1))
            }
        }
    )
}