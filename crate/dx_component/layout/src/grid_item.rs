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
                
            "#
        }
    }
}