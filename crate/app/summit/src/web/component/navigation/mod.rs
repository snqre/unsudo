use super::*;

expose!(
    drop_down_container
    logo
    more_icon
    open_section_button
    scaffold
);

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RootProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub left_attrs: Option<extendable::AttrsProps>,
    pub left_event: Option<extendable::EventProps>,
    pub left: Option<Element>,
    pub right_attrs: Option<extendable::AttrsProps>,
    pub right_event: Option<extendable::EventProps>,
    pub right: Option<Element>,
    pub center_attrs: Option<extendable::AttrsProps>,
    pub center_event: Option<extendable::EventProps>,
    pub children: Option<Element>,
}

#[component]
pub fn Root(props: RootProps) -> Element {
    rsx!(layout::Row {
        attrs: props.attrs.with_style_before(r#"
            justify-content: space-between;
            min-width: 100%;
            width: 100%;
        "#),
        event: props.event,
        if let Some(left) = props.left {
            { left }
        } else {
            div {}
        }
        if let Some(children) = props.children {
            { children }
        } else {
            div {}
        }
        if let Some(right) = props.right {
            { right }
        } else {
            div {}
        }
    })
}



pub fn ItemRack() -> Element {
    rsx!(layout::Row {
        
    })    
}


navigation::Root {
    left: rsx!(navigation::ItemRack {

    }),
}