use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct BlurDotProps {
    pub size: String,
    pub inner_color: String,
    pub outer_color: String,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn BlurDot(props: BlurDotProps) -> Element {
    rsx! {
        div {
            class: r#"{props.class.to_owned().unwrap_or_default()}"#,
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: {props.size};
                aspect-ratio: 1 / 1;
                background-image: radial-gradient(closest-side, {props.inner_color}, {props.outer_color});
                background-position: center;
                background-size: contain;
                opacity: 0.05;
                {props.style.to_owned().unwrap_or_default()}
            "#
        }
    }
}