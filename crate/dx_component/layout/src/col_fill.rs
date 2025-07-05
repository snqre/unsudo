use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColFillProps {
    pub attrs: Option<div::AttrsProps>,
    pub event: Option<div::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn ColFill(props: ColFillProps) -> Element {
    rsx! {
        Col {
            attrs: props.attrs.unwrap_or_default().merge(div::AttrsProps {
                width: Some("100%"),
                min_width: Some("auto"),
                max_width: Some("auto"),
                height: Some("100%"),
                min_height: Some("auto"),
                max_height: Some("auto"),
                ..Default::default()
            }),
            { props.children }
        }
    }
}