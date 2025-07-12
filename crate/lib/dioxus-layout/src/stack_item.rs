use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct StackItemProps {
    pub z: u64,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn StackItem(props: StackItemProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.with_style_before(&format!(r#"
                position: absolute;
                z-index: {};
            "#,
                props.z    
            )),
            event: props.event,
            { props.children }
        }
    }
}