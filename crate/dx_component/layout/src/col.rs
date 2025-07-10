use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColProps {
    pub attrs: Option<bw::AttrsProps>,
    pub event: Option<bw::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        bw::Node {
            attrs: props.attrs.unwrap_or_default().merge(bw::AttrsProps {
                display: "flex".into(),
                flex_direction: "column".into(),
                justify_content: "center".into(),
                align_items: "center".into(),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}