use super::*;

leak!(
    coordinate
    cursor_coordinate
    device
    scroll_v_percentage
    w
);

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        let msg: ::web_sys::wasm_bindgen::JsValue = $msg.into();
        ::web_sys::console::error_1(&msg);
    };
}

#[macro_export]
macro_rules! closure {
    ($closure:expr) => {
        ::web_sys::wasm_bindgen::closure::Closure::wrap(Box::new($closure) as Box<dyn FnMut(_)>);
    };
}

#[macro_export]
macro_rules! closure_ref {
    ($closure:expr) => {
        $closure.as_ref().unchecked_ref()
    };
}