use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColFillProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn ColFill(props: ColFillProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.unwrap_or_default().try_override(FILL_MOD.to_owned()),
            event: props.event,
            { props.children }
        }
    }
}