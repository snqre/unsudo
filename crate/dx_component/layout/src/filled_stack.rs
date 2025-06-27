use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledStackProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn FilledStack(props: FilledStackProps) -> Element {
    rsx! {
        Stack {
            style: r#"
                {fill()}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}