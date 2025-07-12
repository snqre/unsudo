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
            attrs: props.attrs.with_style_before(r#"
                flex-direction: row;
            "#),
            event: props.event,
            { props.children }
        }
    }
}