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
            attrs: props.attrs.with_style_before(r#"
                min-width: 100%;
                max-width: 100%;
                width: 100%;
                min-height: 100%;
                max-height: 100%;
                height: 100%;
            "#),
            event: props.event,
            { props.children }
        }
    }
}