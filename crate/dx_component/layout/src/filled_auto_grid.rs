use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledAutoGridProps {
    pub row_count: u32,
    pub col_count: u32,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn FilledAutoGrid(props: FilledAutoGridProps) -> Element {
    rsx! {
        AutoGrid {
            row_count: props.row_count,
            col_count: props.col_count,
            style: r#"
                {fill()}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}