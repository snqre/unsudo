use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct HyperlinkProps {
    pub to: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Hyperlink(props: HyperlinkProps) -> Element {
    let fallback: String = "/".into();
    rsx!(
        Link {
            style: format!(
                r#"
                    display: contents;
                    text-decoration: none;
                    color: inherit;
                    background: none;
                    border: none;
                "#
            ),
            to: props.to.unwrap_or(fallback),
            { props.children }
        }
    )
}