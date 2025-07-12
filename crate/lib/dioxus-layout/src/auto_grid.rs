use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct AutoGridProps {
    pub row_count: u32,
    pub col_count: u32,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn AutoGrid(props: AutoGridProps) -> Element {
    rsx! {
        Grid {
            row: format!("repeat(1fr, {})", props.row_count),
            col: format!("repeat(1fr, {})", props.col_count),
            attrs: props.attrs,
            event: props.event,
            { props.children }
        }
    }
}