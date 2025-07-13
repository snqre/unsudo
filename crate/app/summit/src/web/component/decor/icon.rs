use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct IconProps {
    pub url: Url,
    pub size: String,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx!(
        layout::Row {
            attrs: props.attrs.with_style_before(&format!(
                r#"
                    background-image: url({}),
                    background-size: contain;
                    background-position: center;
                    background-repeat: no-repeat;
                    cursor: pointer;
                    width: {};
                    aspect-ratio: 1 / 1;
                "#,
                props.url.to_string(),
                props.size
            )),
            event: props.event.unwrap_or_default()
        }
    )
}