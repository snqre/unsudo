use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct StackItemProps {
    pub z: u64,
    pub attrs: Option<bw::AttrsProps>,
    pub event: Option<bw::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn StackItem(props: StackItemProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.unwrap_or_default().merge(bw::AttrsProps {
                position: "absolute".into(),
                z_index: "{props.z}".into(),
                ..Default::default()
            }),
            event: props.event,
            { props.children }
        }
    }
}