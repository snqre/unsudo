use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct RowFillProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn RowFill(props: RowFillProps) -> Element {
    rsx! {
        Row {
            style: r#"
                {FILL}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}