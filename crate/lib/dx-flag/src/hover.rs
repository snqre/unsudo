use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct HoverProps {
    pub on_mouse_enter: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_leave: Option<EventHandler<Event<MouseData>>>,
    pub children: Option<Element>
}

#[component]
pub fn Hover(props: HoverProps) -> Element {
    rsx!(
        div {
            style: r#"
                display: contents;
                cursor: pointer;
            "#,
            onmouseenter: move |data| if let Some(on_mouse_enter) = props.on_mouse_enter {
                on_mouse_enter(data)
            },
            onmouseleave: move |data| if let Some(on_mouse_leave) = props.on_mouse_leave {
                on_mouse_leave(data)
            },
            { props.children }
        }
    )
}