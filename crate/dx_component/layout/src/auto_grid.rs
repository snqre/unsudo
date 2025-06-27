use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct AutoGridProps {
    pub row_count: u32,
    pub col_count: u32,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn AutoGrid(props: AutoGridProps) -> Element {
    rsx! {
        Grid {
            row: "repeat(1fr, {props.row_count})",
            col: "repeat(1fr, {props.col_count})",
            style: r#"{props.style.to_owned().unwrap_or_default()}"#,
            { props.children }
        }
    }
}