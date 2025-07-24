use dioxus::prelude::*;
use core::floating_coordinate::point_2d;

modwire::expose!(
    pub cursor_coordinate
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
        #[cfg(target_arch = "wasm32")]
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
    let w: Signal<_> = use_width();
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

pub fn use_cursor_coordinate() -> Signal<Coordinate> {
    #[allow(unused_mut)]
    let mut ret: Signal<Coordinate> = use_signal(|| Coordinate {
        x: 0.0,
        y: 0.0
    });

    use_closure!("mousemove" => event web_sys::MouseEvent {
        let x: f64 = e.client_x() as f64;
        let y: f64 = e.client_y() as f64;
        ret.set(Coordinate { x, y });
    });

    ret
}

pub fn use_dimension() -> Signal<Dimension> {

}

pub fn use_width() -> Signal<f64> {
    #[allow(unused_mut)]
    let mut ret: Signal<f64> = use_signal(|| 0.0);

    use_closure!("resize" => __ web_sys::Event {
        if let Some(window) = web_sys::window() {
            if let Ok(height) = window.inner_width() {
                if let Some(height) = height.as_f64() {
                    ret.set(height);
                }
            }
        }
    });

    ret
}

pub fn use_height() -> Signal<f64> {
    #[allow(unused_mut)]
    let mut ret: Signal<f64> = use_signal(|| 0.0);

    use_closure!("resize" => __ web_sys::Event {
        if let Some(window) = web_sys::window() {
            if let Ok(width) = window.inner_width() {
                if let Some(width) = width.as_f64() {
                    ret.set(width)
                }
            }
        }
    });

    ret
}

pub fn use_timeout() {}

pub fn use_interval<T>(hook: T, ms: u32) {

}

pub fn use_element_coordinate(id: &'static str) {

}


// import
// macro

// trait
// type

// ...
// | data
// | impl
// | impl .. for data
// ...

// free fn