use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Stack(props: StackProps) -> Element {
    rsx! {
        Col {
            style: r#"
                position: relative;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}