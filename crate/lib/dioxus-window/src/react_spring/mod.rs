use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "target/js/hello.js")]
extern "C" {
    fn on_interval(ms: &JsValue, event_handler: &js_sys::Function);
    fn on_timeout(ms: &JsValue, event_handler: &js_sys::Function) -> js_sys::Function;
}