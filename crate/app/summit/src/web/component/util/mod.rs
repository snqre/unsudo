use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct LinkProps {
    pub to: String,
    pub children: Option<Element>
}

#[component]
pub fn Link(props: LinkProps) -> Element {
    rsx!(
        dioxus::prelude::Link {
            to: props.to,
            style: r#"
                all: unset;
                display: contents;
            "#,
            { props.children }
        }
    )
}