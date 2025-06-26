use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridItemProps {
    pub from: Coordinate,
    pub to: Coordinate,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn GridItem(props: GridItemProps) -> Element {
    rsx! {
        div {
            style: r#"
                grid-column: {props.from.x} / {props.to.x};
                grid-row: {props.from.y} / {props.to.y};
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}