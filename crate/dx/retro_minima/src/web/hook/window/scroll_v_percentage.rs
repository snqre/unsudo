use super::*;

/// # Warning
/// This function is intended for use in WASM (browser) environments only.
///
/// There is no guarantee that the event listener will be successfully 
/// attached, or that any updates will occur if browser APIs fail.
/// 
/// Failures happen silently—no errors will be thrown or logged.
pub fn use_scroll_v_percentage() -> Signal<f64> {
    let scroll: Signal<_> = use_signal(|| 0.0f64);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use ::web_sys;
        use ::web_sys::wasm_bindgen::closure;
        use ::web_sys::wasm_bindgen::JsCast as _;
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
        let closure: closure::Closure<_> = closure!(move |_: web_sys::Event| {
            update();
        });
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("scroll", closure_ref!(closure));
        }
        closure.forget();
    });

    scroll
}