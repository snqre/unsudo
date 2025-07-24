use super::*;

pub fn use_element_dimension(identifier: &'static str) -> Signal<Point2D> {
    let point: Signal<_> = use_signal(|| point_2d::Point2D {
        x: 0.0,
        y: 0.0
    });
    
    use_effect({
        let mut point: Signal<_> = point.to_owned();
        move || {
            add_window_event_listener("resize", move |_| {
                if let Some(element) = element(identifier) {
                    let dom_rect: web_sys::DomRect = element.get_bounding_client_rect();
                    let dom_rect_w: f64 = dom_rect.width();
                    let dom_rect_h: f64 = dom_rect.height();
                    point.set(point_2d::Point2D {
                        x: dom_rect_w,
                        y: dom_rect_h
                    });
                }
            });
        }
    });

    point
}