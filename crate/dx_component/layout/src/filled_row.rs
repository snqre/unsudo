use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledRowProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn FilledRow(props: FilledRowProps) -> Element {
    rsx! {
        Row {
            style: r#"
                {fill()}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}