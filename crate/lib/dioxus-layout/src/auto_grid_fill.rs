use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct AutoGridFillProps {
    pub row_count: u32,
    pub col_count: u32,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn AutoGridFill(props: AutoGridFillProps) -> Element {
    rsx!(
        AutoGrid {
            row_count: props.row_count,
            col_count: props.col_count,
            attrs: props.attrs.with_style_before(FILL_STYLE_MOD),
            event: props.event,
            { props.children }
        }
    )
}