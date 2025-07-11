use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackFillProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn StackFill(props: StackFillProps) -> Element {
    rsx! {
        Stack {
            attrs: props.attrs.unwrap_or_default().try_override(FILL_MOD.to_owned()),
            event: props.event,
            { props.children }
        }
    }
}