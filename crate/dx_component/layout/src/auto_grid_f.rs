use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct AutoGridFProps {
    pub row_count: u32,
    pub col_count: u32,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn AutoGridF(props: AutoGridFProps) -> Element {
    rsx! {
        AutoGrid {
            row_count: props.row_count,
            col_count: props.col_count,
            style: r#"
                width: 100%;
                min-width: auto;
                max-width: auto;
                height: 100%;
                min-height: auto;
                max-height: auto;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}