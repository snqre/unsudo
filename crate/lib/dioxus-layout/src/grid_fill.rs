use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridFillProps {
    pub row: String,
    pub col: String,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>,
}

#[component]
pub fn GridFill(props: GridFillProps) -> Element {
    rsx! {
        Grid {
            row: props.row,
            col: props.col,
            attrs: props.attrs.with_style_before(FILL_STYLE_MOD),
            event: props.event,
            { props.children }
        }
    }
}