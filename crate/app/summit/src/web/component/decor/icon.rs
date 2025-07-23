use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct IconProps {
    pub url: Url,
    pub w: String,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx!(
        dioxus_layout::Row {
            class: props.class,
            style: format!(
                r#"
                    background-image: url({});
                    background-size: contain;
                    background-position: center;
                    background-repeat: no-repeat;
                    width: {};
                    aspect-ratio: 1 / 1;
                    {}
                "#,
                props.url.to_string(),
                props.w,
                props.style.unwrap_or_default()
            )
        }
    )
}