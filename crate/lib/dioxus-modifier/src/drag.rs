use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DragProps {
    pub on_drag_start: Option<EventHandler<DragEvent>>,
    pub on_drag: Option<EventHandler<DragEvent>>,
    pub on_drag_end: Option<EventHandler<DragEvent>>,
    pub children: Option<Element>
}

#[component]
pub fn Drag(props: DragProps) -> Element {
    rsx!(
        div {
            ondragstart: props.on_drag_start.unwrap_or_default(),
            ondrag: props.on_drag.unwrap_or_default(),
            ondragend: props.on_drag_end.unwrap_or_default(),
            ondragenter: props.on_drag_enter.unwrap_or_default(),
            ondragleave: props.on_drag_leave.unwrap_or_default(),
            ondragover: props.on_drag_over.unwrap_or_default(),
            ondrop: props.on_drop.unwrap_or_default(),
            draggable: true,
            style: r#"
                display: contents;
            "#,
            { props.children }
        }
    )
}