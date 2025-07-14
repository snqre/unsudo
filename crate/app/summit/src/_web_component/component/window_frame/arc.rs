use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ArcProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Arc(props: ArcProps) -> Element {
    rsx!(
        Rect {
            attrs: props.attrs.with_style_before(r#"
                border-top-left-radius: 200px;
                border-top-right-radius: 200px;
                border-bottom-left-radius: 2px;
                border-bottom-right-radius: 2px;
            "#),
            event: props.event,
            { props.children }
        }
    )
}