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

modwire::expose!(
    pub coordinate
    pub cursor_coordinate
    pub device
    pub h
    pub interval
    pub scroll_h_percentage
    pub scroll_v_percentage
    pub timeout
    pub w
);