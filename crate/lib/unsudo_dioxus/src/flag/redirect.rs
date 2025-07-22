use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RedirectProps {
    pub to: String,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Redirect(props: RedirectProps) -> Element {
    rsx!(
        Link {
            to: props.to,
            class: props.class,
            style: format!(
                r#"
                    all: unset;
                    display: contents;
                    cursor: pointer;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}