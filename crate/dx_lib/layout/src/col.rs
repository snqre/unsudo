use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColProps {
    pub attrs: Option<extender::AttrsProps>,
    pub event: Option<extender::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        extender::Node {
            attrs: props.attrs.unwrap_or_default().merge(extender::AttrsProps {
                display: "flex".into(),
                flex_direction: "column".into(),
                justify_content: "center".into(),
                align_items: "center".into(),
                ..Default::default()
            }),
            event: props.event,
            { props.children }
        }
    }
}