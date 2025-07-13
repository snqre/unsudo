use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RectProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Rect(props: RectProps) -> Element {
    rsx!(
        extendable::Node {
            attrs: props.attrs.with_style_before(&format!(
                r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    border-radius: 2px;
                    border-style: solid;
                    border-color: {};
                "#,
                color::CARBON
            )),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    )
}