use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    pub attrs: Option<bw::AttrsProps>,
    pub event: Option<bw::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Stack(props: StackProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.unwrap_or_default().merge(bw::AttrsProps {
                position: "relative".into(),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}