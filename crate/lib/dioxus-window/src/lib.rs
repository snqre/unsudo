use dioxus::prelude::*;
use core::floating_coordinate::point_2d;

pub mod prelude {
    use super::*;
    pub use cursor_coordinate::*;
    pub use cursor_x::*;
}

modwire::expose!(
    pub cursor_coordinate
    pub cursor_x
    pub cursor_y
    pub device
    pub element_center_x
    pub element_center_y
    pub element_center
    pub element_coordinate
    pub element_dimension
    pub element_h
    pub element_scroll_offset_x
    pub element_scroll_offset_y
    pub element_scroll_offset
    pub element_w
    pub element_h
    pub element_y
    pub event_listener
    pub h
    pub interval
    pub scroll_percentage_x
    pub scroll_percentage_y
        support
    pub timeout
    pub w
);

#[macro_export(local_inner_macros)]
macro_rules! closure {
    ($closure:expr) => {
        ::web_sys::wasm_bindgen::closure::Closure::wrap(Box::new($closure) as Box<dyn FnMut(_)>)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! closure_ref {
    ($closure:expr) => {
        $closure.as_ref().unchecked_ref()
    };
}

pub type Point2D = point_2d::Point2D<f64>;