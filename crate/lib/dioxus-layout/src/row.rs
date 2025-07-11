use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RowProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
                flex_direction: "row".into(),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}