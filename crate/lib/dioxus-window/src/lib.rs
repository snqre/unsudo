use dioxus::prelude::*;
use std::time;
use core::floating_coordinate::point_2d;

modwire::expose!(
    pub wallet
    pub cursor_x
    pub cursor_y
    pub device
    pub element_center_x
    pub element_center_y
    pub element_center
    pub element_coordinate
    pub element_dimension
    pub element_h
    pub element_scroll_offset_x
    pub element_scroll_offset_y
    pub element_scroll_offset
    pub element_scroll_percentage_x
    pub element_scroll_percentage_y
    pub element_w
    pub element_h
    pub element_y
    pub event_listener
    pub height
    pub interval
    pub scroll_percentage_x
    pub scroll_percentage_y
        support
    pub timeout
    pub width
);


#[macro_export(local_inner_macros)]
macro_rules! use_closure {
    ($event:expr => $event_ident:ident $event_ty:ty $event_handler:block) => {
        
        use_effect(move || {
            use web_sys::wasm_bindgen::closure;
            use web_sys::wasm_bindgen::JsCast as _;
            let closure: closure::Closure<_> = closure!(move |$event_ident: $event_ty| $event_handler);
            if let Some(window) = web_sys::window() {
                window.add_event_listener_with_callback($event, closure_ref!(closure));
            }
            closure.forget();
        })
    };
    ($event:expr => $event_handler:block) => {
        #[cfg(target_arch = "wasm32")]
        use_effect(move || {
            use web_sys::wasm_bindgen::closure;
            use web_sys::wasm_bindgen::JsCast as _;
            let closure: closure::Closure<_> = closure!(move |_: web_sys::Event| $event_handler);
            if let Some(window) = web_sys::window() {
                window.add_event_listener_with_callback($event, closure_ref!(closure));
            }
            closure.forget();
        })
    };
}

#[macro_export(local_inner_macros)]
macro_rules! closure {
    ($closure:expr) => {
        ::web_sys::wasm_bindgen::closure::Closure::wrap(Box::new($closure) as Box<dyn FnMut(_)>)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! closure_ref {
    ($closure:expr) => {
        $closure.as_ref().unchecked_ref()
    };
}

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum Device {
    Laptop4K,
    LaptopL,
    Laptop,
    Tablet,
    MobileL,
    MobileM,
    Mobile
}

pub struct Coordinate {
    pub x: f64,
    pub y: f64
}

pub struct Dimension {
    pub width: f64,
    pub height: f64
}

pub fn use_device() -> Signal<Device> {
    let w: Signal<_> = use_w();
    let mut device: Signal<_> = use_signal(|| Device::Laptop);

    match w() {
        w if w >= 2560.0 => device.set(Device::Laptop4K),
        w if w >= 1440.0 => device.set(Device::LaptopL),
        w if w >= 1024.0 => device.set(Device::Laptop),
        w if w >= 768.0 => device.set(Device::Tablet),
        w if w >= 425.0 => device.set(Device::MobileL),
        w if w >= 375.0 => device.set(Device::MobileM),
        _ => device.set(Device::Mobile)
    }

    device
}

pub fn use_cursor_offset_from_element(identifier: &'static str) -> Signal<Coordinate> {
    let mut offset: Signal<Coordinate> = use_signal(|| Coordinate {
        x: 0.0,
        y: 0.0
    });

    use_closure!("mousemove" => mouse_event web_sys::MouseEvent {
        let x = mouse_event.client_x() as f64;
        let y = mouse_event.client_y() as f64;
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(element) = doc.get_element_by_id(&identifier) {
                    let rect = element.get_bounding_client_rect();
                    offset.set(Coordinate {
                        x: x - rect.x(),
                        y: y - rect.y()
                    });
                }
            }
        }
    });

    offset
}

pub fn use_cursor_coordinate() -> Signal<(f64, f64)> {
    #[allow(unused_mut)]
    let mut coordinate: Signal<(f64, f64)> = use_signal(|| (0.0, 0.0));

    use_closure!("mousemove" => mouse_event web_sys::MouseEvent {
        let x: f64 = mouse_event.client_x() as f64;
        let y: f64 = mouse_event.client_y() as f64;
        coordinate.set((x, y));
    });

    coordinate
}

pub fn use_cursor_x() -> Signal<f64> {
    #[allow(unused_mut)]
    let mut ret: Signal<f64> = use_signal(|| 0.0);

    use_closure!("mousemove" => mouse_event web_sys::MouseEvent {
        ret.set(
            mouse_event.client_x() as f64
        );
    });
    
    ret
}

pub fn use_cursor_y() -> Signal<f64> {
    let mut ret: Signal<f64> = use_signal(|| 0.0);

    use_closure!("mousemove" => mouse_event web_sys::MouseEvent {
        ret.set(
            mouse_event.client_y() as f64
        );
    });

    ret
}

pub fn use_size() -> Signal<(f64, f64)> {
    let mut ret: Signal<(f64, f64)> = use_signal(|| (0.0, 0.0));

    let mut update: _ = move || {
        if let Some(window) = web_sys::window() {
            if let (Ok(w), Ok(h)) = (window.inner_width(), window.inner_height()) {
                if let (Some(w), Some(h)) = (w.as_f64(), h.as_f64()) {
                    ret.set((w, h));
                }
            }
        }
    };

    use_effect(move || {
        update();
    });

    use_closure!("resize" => {
        update();
    });

    ret
}

pub fn use_w() -> Signal<f64> {
    let mut ret: Signal<f64> = use_signal(|| 0.0);

    let mut update: _ = move || {
        if let Some(window) = web_sys::window() {
            if let Ok(w) = window.inner_width() {
                if let Some(w) = w.as_f64() {
                    ret.set(w);
                }
            }
        }
    };

    use_effect(move || {
        update();
    });

    use_closure!("resize" => {
        update();
    });

    ret
}

pub fn use_h() -> Signal<f64> {
    let mut ret: Signal<f64> = use_signal(|| 0.0);

    let mut update: _ = move || {
        if let Some(window) = web_sys::window() {
            if let Ok(width) = window.inner_width() {
                if let Some(width) = width.as_f64() {
                    ret.set(width)
                }
            }
        }
    };

    use_effect(move || {
        update();
    });

    use_closure!("resize" => {
        update();
    });

    ret
}

pub fn use_timeout<T>(hook: T, duration: time::Duration) {
    
}

pub fn use_interval<T>(hook: T, ms: u32) {

}

pub fn use_element_coordinate(id: &'static str) -> Signal<(f64, f64)> {
    let coordinate: Signal<(f64, f64)> = use_signal(|| (
        0.0,
        0.0
    ));

    let update: _ = move || {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(element) = doc.get_element_by_id(id) {
                    let rect = element.get_bounding_client_rect();
                    coordinate.set((
                        rect.x(),
                        rect.y()
                    ));
                }
            }
        }
    };

    use_effect(move || {
        update();
    });


}






pub struct Browser;

impl Browser {
    pub fn fetch_window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    pub fn fetch_document() -> Option<web_sys::Document> {
        Self::fetch_window()?.document()
    }

    pub fn fetch_element_pos_x(el_id: &str) {

    }

    pub fn fetch_element_pos_y(el_id: &str) {}

    pub fn fetch_element_pos(el_id: &str) {}

    pub async fn connect_to_polkadot_wallet() {

    }
}


mod browser {
    pub fn fetch_window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    pub fn fetch_document() -> Option<web_sys::Document> {
        fetch_window()?.document()
    }


}


pub struct WebGlue;

impl WebGlue {
    pub fn fetch_window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    
}



pub struct Element;

impl Element {
    pub fn fetch(id: &'static str) -> Option<web_sys::Element> {
        Window::connect_to_document()?.get_element_by_id(id)
    }

    pub fn x() -> f32 {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(element) = doc.get_element_by_id(id) {
                    let rect = element.get_bounding_client_rect();
                    coordinate.set((
                        rect.x(),
                        rect.y()
                    ));
                }
            }
            0.0
        }
    }

    pub fn rect() -> web_sys::DomRect {

    }
}








pub struct Window;

impl Window {
    pub fn connect_to_window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    pub fn connect_to_document() -> Option<web_sys::Document> {
        Self::connect_to_window()?.document()
    }

    
}