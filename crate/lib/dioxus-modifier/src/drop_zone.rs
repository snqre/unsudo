use super::*;

pub struct DropZoneProps {
    pub on_drag_enter: Option<EventHandler<DragEvent>>,
    pub on_drag_leave: Option<EventHandler<DragEvent>>,
    pub on_drag_over: Option<EventHandler<DragEvent>>,
    pub on_drop: Option<EventHandler<DragEvent>>,
    pub children: Option<Element>
}