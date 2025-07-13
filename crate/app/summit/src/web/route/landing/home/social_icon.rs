use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SocialIconProps {
    pub size: String,
    pub url: Url,
    pub to: String,
    pub style: Option<String>
}

#[component]
pub fn SocialIcon(props: SocialIconProps) -> Element {
    rsx! {

        Link {
            to: props.to,
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                background-image: url({props.url});
                background-size: contain;
                background-position: center;
                background-repeat: no-repeat;
                cursor: pointer;
                width: {props.size};
                aspect-ratio: 1 / 1;
                {props.style.to_owned().unwrap_or_default()}
            "#
        }
    }

    rsx!(
        util::Link {
            to: props.to,
            decor::Icon {
                url: props.url,
                size: props.size,
            }
        }
    )
}