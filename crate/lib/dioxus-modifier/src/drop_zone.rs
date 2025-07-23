use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct DropZoneProps {
    pub on_drag_enter: Option<EventHandler<DragEvent>>,
    pub on_drag_leave: Option<EventHandler<DragEvent>>,
    pub on_drag_over: Option<EventHandler<DragEvent>>,
    pub on_drop: Option<EventHandler<DragEvent>>,
    pub children: Option<Element>
}

#[component]
pub fn DropZone(props: DropZoneProps) -> Element {
    rsx!(
        div {
            ondragenter: props.on_drag_enter.unwrap_or_default(),
            ondragleave: props.on_drag_leave.unwrap_or_default(),
            ondragover: move |event| {
                event.prevent_default();
                if let Some(on_drag_over) = &props.on_drag_over {
                    on_drag_over(event);
                }
            },
            ondrop: props.on_drop.unwrap_or_default(),
            style: format!(
                r#"
                    display: contents;
                "#
            ),
            { props.children }
        }
    )
}