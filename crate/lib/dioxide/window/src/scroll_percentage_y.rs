use super::*;

/// Tracks vertical scroll progress of the page as a percentage (0..100).
/// As the user reaches the bottom of the page they get closer to 100%.
pub fn use_scroll_percentage_y() -> Signal<f64> {
    let mut percentage: Signal<f64> = use_signal(|| 0.0);

    use_effect(move || {
        add_window_event_listener("scroll", move |_| {
            if let Some(document) = document() {
                if let Some(element) = document.document_element() {
                    let scroll_top: f64 = element.scroll_top().into();
                    let scroll_height: f64 = element.scroll_height().into();
                    let client_height: f64 = element.client_height().into();
                    let new_percentage: f64 = ((scroll_top / (scroll_height - client_height)) * 100.0).clamp(0.0, 100.0);
                    percentage.set(new_percentage);
                }
            }
        });
    });

    percentage
}