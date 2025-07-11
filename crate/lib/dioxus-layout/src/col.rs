use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        extendable::Node {
            attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
                display: "flex".into(),
                flex_direction: "column".into(),
                justify_content: "center".into(),
                align_items: "center".into(),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}