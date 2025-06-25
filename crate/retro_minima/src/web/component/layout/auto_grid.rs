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
        div {
            style: r#"
                display: grid;
                grid-template-rows: repeat({props.row_count}, 1fr);
                grid-template-columns: repeat({props.col_count}, 1fr);
                width: 100%;
                min-width: 100%;
                max-width: 100%;
                height: 100%;
                min-height: 100%;
                max-height: 100%;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}