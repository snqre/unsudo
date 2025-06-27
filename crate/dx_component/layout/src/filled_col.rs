use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledColProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn FilledCol(props: FilledColProps) -> Element {
    rsx! {
        Col {
            style: r#"
                {fill()}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}