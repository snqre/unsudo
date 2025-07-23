use dioxus::prelude::*;

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

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    WindowUnavailable,
    DocumentUnavailable,
    ElementIdNotFound
}

modwire::expose!(
    pub coordinate
    pub cursor_coordinate
    pub cursor_x
    pub cursor_y
    pub device
    pub element_coordinate
    pub element_h
    pub element_scroll_offset_x
    pub element_scroll_offset_y
    pub element_scroll_offset
    pub element_size
    pub element_w
    pub element_x
    pub element_y
    pub element
    pub h
    pub interval
    pub scroll_h_percentage
    pub scroll_v_percentage
        support
    pub timeout
    pub w
);