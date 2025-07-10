use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RowProps {
    pub attrs: Option<bw::AttrsProps>,
    pub event: Option<bw::EventProps>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        bw::Node {
            attrs: props.attrs.unwrap_or_default().merge(div::AttrsProps {
                display: Some("flex"),
                flex_direction: Some("row"),
                justify_content: Some("center"),
                align_items: Some("center"),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}