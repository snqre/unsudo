use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct GridFillProps {
    pub row_count: u8,
    pub row_gap: Option<String>,
    pub col_count: u8,
    pub col_gap: Option<String>,
    pub gap: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>,
}

#[component]
pub fn GridFill(props: GridFillProps) -> Element {
    rsx!(
        Grid {
            row_count: props.row_count,
            row_gap: props.row_gap,
            col_count: props.col_count,
            col_gap: props.col_gap,
            gap: props.gap,
            class: props.class,
            style: format!(
                r#"
                    {}
                    {}
                "#,
                stylesheet::FILL,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}