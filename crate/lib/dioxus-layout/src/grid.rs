use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    pub row: String,
    pub col: String,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    rsx!(
        extendable::Node {
            attrs: props.attrs.with_style_before(&format!(r#"
                display: grid;
                grid-template-rows: repeat({});
                grid-template-columns: repeat({});
            "#,
                props.row,
                props.col
            )),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    )
}