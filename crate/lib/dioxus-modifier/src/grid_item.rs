use super::*;

#[derive(Clone, PartialEq)]
pub enum GridItemCoverage {
    Auto,
    Span(Option<u32>, Option<u32>),

    /// Manually specify grid coordinate
    /// 
    /// * col-start x 0
    /// * row-start y 0
    /// * col-end x 1
    /// * row-end y 1
    Coordinate(
        Option<u32>, 
        Option<u32>, 
        Option<u32>, 
        Option<u32>
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct GridItemProps {
    pub coverage: Option<GridItemCoverage>,
    pub children: Option<Element>
}

#[component]
pub fn GridItem(props: GridItemProps) -> Element {
    rsx!(
        match props.coverage {
            Some(GridItemCoverage::Auto) | None => rsx!(
                div {
                    style: format!(
                        r#"
                            display: contents;
                        "#
                    ),
                    { props.children }
                }
            ),
            Some(GridItemCoverage::Span(w, h)) => rsx!(
                div {
                    style: format!(
                        r#"
                            display: contents;
                            {}
                            {}
                        "#,
                        w.map(|v| format!("grid-column-end: span {};", v)).unwrap_or_default(),
                        h.map(|v| format!("grid-row-end: span {};", v)).unwrap_or_default()
                    ),
                    { props.children }
                }
            ),
            Some(GridItemCoverage::Coordinate(x0, y0, x1, y1)) => rsx!(
                div {
                    style: format!(
                        r#"
                            display: contents;
                            {}
                            {}
                            {}
                            {}
                        "#,
                        x0.map(|v| format!("grid-column-start: {};", v)).unwrap_or_default(),
                        x1.map(|v| format!("grid-column-end: {};", v)).unwrap_or_default(),
                        y0.map(|v| format!("grid-row-start: {};", v)).unwrap_or_default(),
                        y1.map(|v| format!("grid-row-end: {};", v)).unwrap_or_default()
                    ),
                    { props.children }
                }
            )
        }
    )
}