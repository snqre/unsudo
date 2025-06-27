use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledGridProps {
    pub row: String,
    pub col: String,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn FilledGrid(props: FilledGridProps) -> Element {
    rsx! {
        Grid {
            row: props.row,
            col: props.col,
            style: r#"
                {fill()}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}