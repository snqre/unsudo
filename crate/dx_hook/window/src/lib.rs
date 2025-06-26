use ::prelude::*;
use ::dioxus::prelude::*;

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

package!(
    coordinate
    cursor_coordinate
    device
    h
    interval
    scroll_h_percentage
    scroll_v_percentage
    timeout
    w
);