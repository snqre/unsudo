use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct BlurDotProps {
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
            class: r#"
                w-min-02
                {props.class.to_owned().unwrap_or_default()}
            "#,
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                min-width: {web::sequence(5u16)}px;
                aspect-ratio: 1 / 1;
                background-image: radial-gradient(closest-side, {props.inner_color}, {props.outer_color});
                background-position: center;
                background-size: contain;
            "#
        }
    }
}