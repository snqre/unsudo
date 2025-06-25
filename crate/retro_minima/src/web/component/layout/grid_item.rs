use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridItemProps {
    pub x_0: u64,
    pub y_0: u64,
    pub x_1: u64,
    pub y_1: u64,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn GridItem(props: GridItemProps) -> Element {
    rsx! {
        div {
            style: r#"
                grid-column: {props.x_0} / {props.x_1}:
                grid-row: {props.y_0} / {props.y_1}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}