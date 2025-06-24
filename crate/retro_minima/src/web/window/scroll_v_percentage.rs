use super::*;

pub fn use_scroll_v_percentage() -> Signal<f64> {
    let scroll: Signal<_> = use_signal(|| 0.0f64);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use ::web_sys;
        use ::web_sys::wasm_bindgen::closure;
        use ::web_sys::wasm_bindgen::JsCast as _;
        use ::web_sys::js_sys;
        use ::web_sys::console;
        let mut update: _ = {
            let mut scroll: Signal<_> = scroll;
            move || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.document_element() {
                            let scroll_top: f64 = element.scroll_top() as f64;
                            let scroll_h: f64 = element.scroll_height() as f64;
                            let client_h: f64 = element.client_height() as f64;
                            let percentage: f64 = (scroll_top / (scroll_h - client_h)) * 100.0f64;
                            let percentage: f64 = percentage.clamp(0.0f64, 100.0f64);
                            scroll.set(percentage);
                        }
                    }
                }
            }
        };
        update();
        let closure: closure::Closure<_> = closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            update();
        }) as Box<dyn FnMut(_)>);
        if let Some(window) = web_sys::window() {
            let closure: &js_sys::Function = closure.as_ref().unchecked_ref();
            if window.add_event_listener_with_callback("scroll", closure).is_err() {
                let message: wasm_bindgen::JsValue = "Unable to hook event listener for scroll.".into();
                console::error_1(&message);
            }
        } else {
            let message: wasm_bindgen::JsValue = ERR_NO_WINDOW.into();
            console::error_1(&message);
        }
        closure.forget();
    });

    scroll
}