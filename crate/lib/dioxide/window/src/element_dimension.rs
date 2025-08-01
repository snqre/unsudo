use super::*;

/// width (0) & height (1)
pub fn use_element_dimension(identifier: &'static str) -> Signal<(f64, f64)> {
    let point: Signal<_> = use_signal(|| (0.0, 0.0));
    
    use_effect({
        let mut point: Signal<_> = point.to_owned();
        move || {
            add_window_event_listener("resize", move |_| {
                if let Some(element) = element(identifier) {
                    let dom_rect: web_sys::DomRect = element.get_bounding_client_rect();
                    let dom_rect_width: f64 = dom_rect.width();
                    let dom_rect_height: f64 = dom_rect.height();
                    point.set((dom_rect_width, dom_rect_height));
                }
            });
        }
    });

    point
}