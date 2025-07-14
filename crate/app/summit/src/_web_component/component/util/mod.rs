use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ToProps {
    pub to: String,
    pub children: Option<Element>
}

#[component]
pub fn To(props: ToProps) -> Element {
    rsx!(
        Link {
            to: props.to,
            style: r#"
                all: unset;
                display: contents;
            "#,
            { props.children }
        }
    )
}