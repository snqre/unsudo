use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct BlurDotProps {
    pub size: String,
    pub inner_color: String,
    pub outer_color: String,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn BlurDot(props: BlurDotProps) -> Element {
    rsx!(
        layout::Col {
            class: props.class,
            style: format!(
                r#"
                    width: {};
                    aspect-ratio: 1 / 1;
                    background-image: radial-gradient(
                        closest-side,
                        {},
                        {}
                    );
                    background-position: center;
                    background-size: contain;
                    opacity: 0.05;
                    {}
                "#,
                props.size,
                props.inner_color,
                props.outer_color,
                props.style.unwrap_or_default()
            )
        }
    )
}