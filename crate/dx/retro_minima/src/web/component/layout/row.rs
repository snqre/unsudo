use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        Col {
            style: r#"
                flex-direction: row;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}