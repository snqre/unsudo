use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct BlurDotProps {
    pub inner_color: String,
    pub outer_color: String,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn BlurDot(props: BlurDotProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                min-width: 50px;
                aspect-ratio: 1 / 1;
                background-image: radial-gradient(closest-side, {props.inner_color}, {props.outer_color});
                background-position: center;
                background-size: contain;
            "#
        }
    }
}