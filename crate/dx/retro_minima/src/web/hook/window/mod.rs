use super::*;

leak!(
    coordinate
    cursor_coordinate
    device
    h
    scroll_h_percentage
    scroll_v_percentage
    use_interval
    w
);

#[macro_export]
macro_rules! closure {
    ($closure:expr) => {
        ::web_sys::wasm_bindgen::closure::Closure::wrap(Box::new($closure) as Box<dyn FnMut(_)>)
    };
}

#[macro_export]
macro_rules! closure_ref {
    ($closure:expr) => {
        $closure.as_ref().unchecked_ref()
    };
}