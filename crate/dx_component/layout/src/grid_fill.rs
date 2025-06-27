use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridFillProps {
    pub row: String,
    pub col: String,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn GridFill(props: GridFillProps) -> Element {
    rsx! {
        Grid {
            row: props.row,
            col: props.col,
            style: r#"
                {FILL}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}