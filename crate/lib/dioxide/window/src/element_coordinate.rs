use super::*;

pub fn use_element_coordinate(identifier: &'static str) -> Signal<Coordinate> {
    let point: Signal<_> = use_signal(|| (0.0, 0.0));

    use_effect({
        let mut point: Signal<_> = point.to_owned();
        move || {
            add_window_event_listener("resize", move |_| {
                if let Some(element) = element(identifier) {
                    let dom_rect: web_sys::DomRect = element.get_bounding_client_rect();
                    let dom_rect_x: f64 = dom_rect.x();
                    let dom_rect_y: f64 = dom_rect.y();
                    point.set((dom_rect_x, dom_rect_y));
                }
            });            
        }
    });

    point
}