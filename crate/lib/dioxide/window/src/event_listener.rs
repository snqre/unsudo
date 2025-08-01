use super::*;

pub fn add_window_event_listener<A, B>(event: A, mut event_handler: B)
where
    A: Into<String>,
    B: FnMut(web_sys::Event) + 'static {
    use web_sys::wasm_bindgen::prelude::*;

    let event: &str = &event.into();
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event_handler(event);
    }) as Box<dyn FnMut(_)>);
    if let Some(window) = web_sys::window() {
        window.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref()).unwrap();
    }
    closure.forget()
}