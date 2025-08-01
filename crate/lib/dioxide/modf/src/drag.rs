use dioxus::html::geometry;

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
            draggable: true,
            style: r#"
                display: contents;
            "#,
            { props.children }
        }
    )
}








#[derive(Props, Clone, PartialEq)]
pub struct DragDeltaProps {
    pub disable_x_axis: Option<bool>,
    pub y_axis: Option<bool>,
    pub upper_x_bound: Option<f64>,

    pub children: Option<Element>
}

#[component]
pub fn DragDelta(props: DragDeltaProps) -> Element {
    let mut begin_x: Signal<f64> = use_signal(|| 0.0);
    let mut begin_y: Signal<f64> = use_signal(|| 0.0);
    let mut delta_x: Signal<f64> = use_signal(|| 0.0);
    let mut delta_y: Signal<f64> = use_signal(|| 0.0);

    rsx!(
        div {
            ondragstart: move |event| {
                let point: geometry::ClientPoint = event.client_coordinates();
                begin_x.set(point.x);
                begin_y.set(point.y);
            },
            ondrag: move |event| {
                let point: geometry::ClientPoint = event.client_coordinates();
                let begin_x = begin_x();
                let begin_y = begin_y();
                delta_x.set(point.x - begin_x);
                delta_y.set(point.y - begin_y);
            },
            ondragend: move |_| {
                delta_x.set(0.0);
                delta_y.set(0.0);
            },
            draggable: true,
            style: format!(
                r#"
                    display: flex;
                    flex-direction: column;
                    justify-contents: center;
                    align-items: center;
                    transform: translate(
                        {}px,
                        {}px
                    );
                    cursor: grab;
                "#,
                delta_x(),
                delta_y()
            ),
            { props.children }
        }
    )
}