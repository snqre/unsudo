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




pub struct Point2D {
    pub x: f64,
    pub y: f64
}

pub trait DragEventExtension {
    fn sync_offset(&self, offset: Signal<(f64, f64)>);
    fn sync_offset_x(&self, offset: Signal<(f64, f64)>);
    fn sync_offset_y(&self, offset: Signal<(f64, f64)>);
    fn sync_offset_within_bounds(
        &self,
        offset: Signal<(f64, f64)>,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64
    );

    ///
    /// ```rs
    /// fn Foo() -> Element {
    ///     let mut begin = use_signal(|| Point2D {
    ///         x: 0.0,
    ///         y: 0.0
    ///     });
    ///     let mut delta = use_signal(|| Point2D {
    ///         x: 0.0,
    ///         y: 0.0
    ///     });
    ///     let mut offset = use_signal(|| Point2D {
    ///         x: 0.0,
    ///         y: 0.0
    ///     });
    ///     
    /// }
    /// ```
    fn calculate_update(
        &self,
        begin_x: f64,
        begin_y: f64,
        delta_x: f64,
        delta_y: f64
    ) -> (f64, f64, f64, f64);

    fn sync_delta_x(&self, begin: Signal<Point2D>, delta: )
}

impl DragEventExtension for DragEvent {

    ///
    /// # Example
    /// ```rs
    /// fn Foo() -> Element {
    ///     let mut offset: Signal<(f64, f64)> = use_signal(|| (0.0, 0.0));
    /// 
    ///     rsx!(
    ///         div {
    ///             ondrag: |event| {
    ///                 event.sync_offset(offset);
    ///             },
    ///             draggable: true,
    ///             style: format!(
    ///                 r#"
    ///                     transform: translate(
    ///                         {}px,
    ///                         {}px
    ///                     );
    ///                     cursor: grab;
    ///                 "#,
    ///                 offset().0,
    ///                 offset().1
    ///             ),
    ///             "Bar"
    ///         }
    ///     )
    /// }
    /// ```
    ///
    fn sync_offset(&self, mut offset: Signal<(f64, f64)>) {
        let point: geometry::ClientPoint = self.client_coordinates();
        offset.set((
            point.x, 
            point.y
        ));
    }

    fn sync_offset_x(&self, mut offset: Signal<(f64, f64)>) {
        let point: geometry::ClientPoint = self.client_coordinates();
        offset.set((
            point.x,
            offset().1
        ));
    }

    fn sync_offset_y(&self, mut offset: Signal<(f64, f64)>) {
        let point: geometry::ClientPoint = self.client_coordinates();
        offset.set((
            offset().0,
            point.y
        ));
    }

    fn sync_offset_within_bounds(
            &self,
            mut offset: Signal<(f64, f64)>,
            min_x: f64,
            max_x: f64,
            min_y: f64,
            max_y: f64
        ) {
        let point: geometry::ClientPoint = self.client_coordinates();
        let x = point.x.clamp(min_x, max_x);
        let y = point.y.clamp(min_y, max_y);
        offset.set((x, y));
    }

    fn calculate_update(&self, begin: Signal<Point2D>, delta: Signal<Point2D>) {
        let point: geometry::ClientPoint = self.client_coordinates();
        offset.set((
            point.x, 
            point.y
        ));
    }
}



pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    WindowUnavailable,
    DocumentUnavailable,
    ElementIdNotFound
}


pub fn use_element_dimension(id: &str) -> Option<(f64, f64)> {
    let document = web_sys::window()?.document()?;
    let element = document.get_element_by_id(id)?;
    let element_rect = element.get_bounding_client_rect();
    let element_w = element_rect.width();
    let element_h = element_rect.height();
    Some((element_w, element_h))
}

pub fn use_element_position(id: &str) -> Option<(f64, f64)> {
    let document = web_sys::window()?.document()?;
    let element = document.get_element_by_id(id)?;
    let rect = element.get_bounding_client_rect();
    Some((rect.x(), rect.y()))   
}

pub fn use_element_center(id: &str) -> Option<(f64, f64)> {
    let document = web_sys::window()?.document()?;
    let element = document.get_element_by_id(id)?;
    let rect = element.get_bounding_client_rect();
    Some((
        rect.x() + rect.width() / 2.0,
        rect.y() + rect.height() / 2.0
    ))   
}

pub fn use_element_scroll_offset_x(identifier: &str) -> Signal<i32> {
    element(identifier).unwrap().scroll_left();
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