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
            attrs: props.attrs.with_style_before(r#"
                position: relative
            "#),
            event: props.event,
            { props.children }
        }
    }
}