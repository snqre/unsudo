use super::*;

/// # Warning
/// This function is intended for use in WASM (browser) environments only.
///
/// There is no guarantee that the event listener will be successfully 
/// attached, or that any updates will occur if browser APIs fail.
/// 
/// Failures happen silently—no errors will be thrown or logged.
pub fn use_width() -> Signal<f64> {
    #[allow(unused_mut)]
    let mut w: Signal<f64> = use_signal(|| 0.0f64);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use ::web_sys::wasm_bindgen::JsCast as _;
        let mut update: _ = move || {
            if let Some(window) = web_sys::window() {
                if let Ok(value) = window.inner_width() {
                    if let Some(value) = value.as_f64() {
                        width.set(value);
                    }
                }
            }
        };
        update();
        let closure: _ = closure!(move |_: web_sys::Event| {
            update();
        });
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("resize", closure_ref!(closure));
        }
        closure.forget();
    });

    w
}