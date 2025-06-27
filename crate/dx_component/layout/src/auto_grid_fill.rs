use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct AutoGridFillProps {
    pub row_count: u32,
    pub col_count: u32,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn AutoGridFill(props: AutoGridFillProps) -> Element {
    rsx! {
        AutoGrid {
            row_count: props.row_count,
            col_count: props.col_count,
            style: r#"
                {FILL}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}