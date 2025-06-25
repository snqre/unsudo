use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    pub row: String,
    pub col: String,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: grid;
                grid-template-rows: repeat({props.row});
                grid-template-columns: repeat({props.col});
                {props.style.to_owned().unwrap_or_default()}
            "#
        }
    }
}