use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct RowFillProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn RowFill(props: RowFillProps) -> Element {
    rsx! {
        Row {
            attrs: props.attrs.unwrap_or_default().try_override(FILL_MOD.to_owned()),
            event: props.event,
            { props.children }
        }
    }
}