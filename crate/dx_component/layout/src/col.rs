use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColProps {
    pub attrs: Option<div::AttrsProps>,
    pub event: Option<div::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div::Div {
            attrs: props.attrs.unwrap_or_default().merge(div::AttrsProps {
                display: Some("flex"),
                flex_direction: Some("column"),
                justify_content: Some("center"),
                align_items: Some("center"),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}