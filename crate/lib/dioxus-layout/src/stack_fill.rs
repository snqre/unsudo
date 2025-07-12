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
            attrs: props.attrs.with_style_before(FILL_STYLE_MOD),
            event: props.event,
            { props.children }
        }
    }
}