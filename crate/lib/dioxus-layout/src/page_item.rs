use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageItemProps {
    pub top_attrs: Option<extendable::AttrsProps>,
    pub top_event: Option<extendable::EventProps>,
    pub top: Option<Element>,
    pub bottom_attrs: Option<extendable::AttrsProps>,
    pub bottom_event: Option<extendable::EventProps>,
    pub bottom: Option<Element>,
    pub bg_attrs: Option<extendable::AttrsProps>,
    pub bg_event: Option<extendable::EventProps>,
    pub bg: Option<Element>,
    pub wrapper_attrs: Option<extendable::AttrsProps>,
    pub wrapper_event: Option<extendable::EventProps>,
    pub content_attrs: Option<extendable::AttrsProps>,
    pub content_event: Option<extendable::EventProps>,
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn PageItem(props: PageItemProps) -> Element {
    rsx!(
        Stack {
            attrs: props.attrs.with_style_before(VIEW_FILL_STYLE_MOD),
            event: props.event,
            StackItem {
                z: 0,
                attrs: props.bg_attrs
                    .with_style_before(ABSOLUTE_POSITION_RESET_STYLE_MOD)
                    .with_style_before(VIEW_FILL_STYLE_MOD),
                event: props.bg_event,
                { props.bg }
            }
            StackItem {
                z: 1,
                attrs: props.wrapper_attrs
                    .with_style_before(ABSOLUTE_POSITION_RESET_STYLE_MOD)
                    .with_style_before(VIEW_FILL_STYLE_MOD)
                    .with_style_before("justify-content: space-between; scroll-snap-align: start;"),
                event: props.wrapper_event,
                if let Some(top) = props.top {
                    Col {
                        attrs: props.top_attrs.with_style_before("width: 100%;"),
                        event: props.top_event,
                        { top }
                    }
                }
                if let Some(children) = props.children {
                    Col {
                        attrs: props.content_attrs
                            .with_style_before(FILL_STYLE_MOD)
                            .with_style_before("justify-content: space-between;"),
                        event: props.content_event,
                        { children }
                    }
                }
                if let Some(bottom) = props.bottom {
                    Col {
                        attrs: props.bottom_attrs.with_style_before("width: 100%;"),
                        event: props.bottom_event,
                        { bottom }
                    }
                }
            }
        }
    )
}