use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackFillProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn StackFill(props: StackFillProps) -> Element {
    rsx! {
        Stack {
            style: r#"
                {FILL}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}