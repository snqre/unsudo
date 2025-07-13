use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct BlurDotProps {
    pub size: String,
    pub inner_color: String,
    pub outer_color: String,
    pub attrs: Box<Option<extendable::AttrsProps>>,
    pub event: Box<Option<extendable::EventProps>>,
    pub children: Option<Element>
}

#[component]
pub fn BlurDot(props: BlurDotProps) -> Element {
    rsx!(
        extendable::Node {
            attrs: props.attrs.with_style_before(&format!(
                r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
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
                "#,
                props.size,
                props.inner_color,
                props.outer_color
            )),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    )
}