use super::*;

pub fn use_w() -> Signal<f64> {
    let mut w: Signal<f64> = use_signal(|| 0.0f64);

    use_effect(move || {
        use ::web_sys::wasm_bindgen;
        use ::web_sys::wasm_bindgen::JsCast as _;
        use ::web_sys::wasm_bindgen::closure;
        use ::web_sys::console;
        let mut update: _ = move || {
            if let Some(window) = web_sys::window() {
                match window.inner_width() {
                    Ok(value) => {
                        if let Some(value) = value.as_f64() {
                            w.set(value);
                        } else {
                            log!("Unable to convert `inner_width` to `f64`. The value may be in an unexpected format.");
                        }
                    },
                    Err(e) => {
                        let e: Option<String> = e.try_into().ok();
                        if let Some(e) = e {
                            log!(format!("[use_w]: Unable to fetch `window.inner_width`: {e}"));
                        } else {
                            log!("[use_w]: Unable to fetch `window.inner_width`");
                        }
                    }
                }
            } else {
                log!("[use_w]: Window not found while updating. No update has taken place.");
            }
        };
        update();
        let closure: _ = closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            update(); // Call update_width on resize event
        }) as Box<dyn FnMut(_)>);
        match web_sys::window() {
            Some(window) => {

            },
            _ => {
                log!("`window` was not found while an adding event an event listener for `resize`.");
            }
        }



        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });

    w
}