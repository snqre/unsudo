use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Stack(props: StackProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
                position: "relative".into(),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}