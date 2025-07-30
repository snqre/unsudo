use dioxus::prelude::*;
use web_sys::js_sys::Function;
use std::time;
use core::floating_coordinate::point_2d;

mod ts;

::modwire::expose!(
    
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



pub struct Coordinate {
    pub x: f64,
    pub y: f64
}

pub struct Dimension {
    pub width: f64,
    pub height: f64
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






pub fn use_element_coordinate(identifier: &'static str) -> Signal<(f64, f64)> {
    let mut ret: Signal<(f64, f64)> = use_signal(|| (0.0, 0.0));

    use_animation_frame(move || {
        web_sys::window()
            .and_then(|c| c.document())
            .and_then(|c| c.get_element_by_id(identifier))
            .map(|c| c.get_bounding_client_rect())
            .map(|c| (
                c.x(),
                c.y()
            ))
            .map(|(x, y)| ret.set((x, y)));
    });

    ret
}



    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast as _;

mod animation {
    use super::*;

    use std::{cell::RefCell, rc::Rc};


    macro_rules! to_js_fn {
        ($block:block) => {
            Closure::wrap(Box::new(move || $block) as Box<dyn FnMut()>).as_ref().unchecked_ref()
        };
    }

    pub fn use_animation_frame<A, B>(hook: B) 
    where
        B: 'static,
        B: FnMut() {
        let mut keep_alive: Signal<bool> = use_signal(|| true);
        
        let update: _ = move || {
            if keep_alive() {
                web_sys::window().unwrap().request_animation_frame(to_js_fn!({
                    update();
                }));
            }
        };

        use_effect(move || {
            web_sys::window().unwrap().request_animation_frame(to_js_fn!({
                

            }));
        });
    }
}



#[wasm_bindgen(module = "/src/wallet.ts")]
extern "C" {
    #[wasm_bindgen(js_name = on_animation_frame)]
    fn on_animation_frame(event_handler: &Closure<dyn FnMut() -> bool>) -> Function;
}

/// Starts an animation frame loop and returns a function that stops it.
pub fn on<F>(mut event_listener: F) -> impl FnOnce()
where
    F: FnMut() -> bool + 'static,
{
    let closure = Closure::wrap(Box::new(move || {
        event_listener()
    }) as Box<dyn FnMut() -> bool>);

    let stop_fn = on_animation_frame(&closure);
    
    // Leak the closure if you want it to live for the loop duration.
    // If you want to cancel safely later, store it instead.
    closure.forget();

    // Return a kill switch that stops the animation loop.
    move || {
        let _ = stop_fn.call0(&JsValue::NULL);
    }
}



pub fn use_animation_frame<A, B>(mut hook: B) -> Signal<A> 
where
    B: 'static,
    B: FnMut() -> A,
    B: Clone {

    use std::rc;
    use std::cell;

    type ClosureRef = rc::Rc<cell::RefCell<Option<Closure<dyn FnMut()>>>>;
    
    let ret: Signal<A> = use_signal(|| hook());
    let c_ref: ClosureRef = rc::Rc::new(cell::RefCell::new(None)); 
    let c_ref_clone: ClosureRef = c_ref.to_owned();
    let c_ref_clone_0: ClosureRef = c_ref_clone.to_owned();

    use_effect(move || {
        let mut ret: Signal<A> = ret.to_owned();
        let mut hook: B = hook.to_owned();
        let c_ref: ClosureRef = c_ref_clone.to_owned();
        let c_ref_clone: ClosureRef = c_ref_clone.to_owned();
        let c: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            ret.set(hook());
            if let Some(window) = web_sys::window() {
                let _ = window.request_animation_frame(
                    c_ref
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                );
            }
        }) as Box<dyn FnMut()>);
        *c_ref_clone.borrow_mut() = Some(c);
        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(
                c_ref_clone
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            );
        }
    });

    use_drop(move || {
        c_ref_clone_0.borrow_mut().take();
    });

    ret
}