use super::*;

/// # Warning
/// This function is intended for use in WASM (browser) environments only.
///
/// There is no guarantee that the event listener will be successfully 
/// attached, or that any updates will occur if browser APIs fail.
/// 
/// Failures happen silently—no errors will be thrown or logged.
pub fn use_cursor_coordinate() -> Signal<Coordinate> {
    let coordinate: Signal<_> = use_signal(|| Coordinate {
        x: 0.0f64,
        y: 0.0f64
    });

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        use ::web_sys;
        use ::web_sys::wasm_bindgen::closure;
        use ::web_sys::wasm_bindgen::JsCast as _;
        let update: _ = {
            let mut coordinate: Signal<_> = coordinate;
            move |e: web_sys::MouseEvent| {
                let x: f64 = e.client_x() as f64;
                let y: f64 = e.client_y() as f64;
                coordinate.set(Coordinate {
                    x,
                    y
                });
            }
        };
        let closure: closure::Closure<_> = closure!(update);
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("mousemove", closure_ref!(closure));
        }
        closure.forget();
    });

    coordinate
}