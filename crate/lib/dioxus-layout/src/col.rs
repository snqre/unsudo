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
            attrs: props.attrs.with_style_before(r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
            "#),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}