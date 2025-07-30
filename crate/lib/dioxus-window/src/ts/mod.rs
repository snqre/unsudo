use super::*;

pub fn on_timeout(ms: u32, event_handler: &js_sys::Function) {
    let event_handler = js_fn!(move || -> f32 {
        0.0
    });
}


#[macro_export]
macro_rules! js_fn {
    (move || -> $ret_ty:ty $block:block) => {
        Closure::wrap(Box::new(move || -> $ret_ty {
            $block
        }) as Box<dyn FnMut() -> $ret_ty>)
    };
    (move |$($arg_ident:ident: $arg_ty:ty),*| -> $ret_ty:ty $block:block) => {
        Closure::wrap(Box::new(move |$($arg_ident: $arg_ty),*| -> $ret_ty {
            $block
        }) as Box<dyn FnMut($($arg_ty),*) -> $ret_ty>)
    };
    (move |$($arg_ident:ident: $arg_ty:ty),*| $block:block) => {
        Closure::wrap(Box::new(move |$($arg_ident: $arg_ty),*| {
            $block
        }) as Box<dyn FnMut($($arg_ty),*)>)
    };
    (|| -> $ret_ty:ty $block:block) => {

    };
    (|$($arg_ident:ident: $arg_ty:ty),*| -> $ret_ty:ty $block:block) => {
        Closure::wrap(Box::new(|$($arg_ident: $arg_ty),*| -> $ret_ty {
            $block
        }) as Box<dyn FnMut($($arg_ty),*) -> $ret_ty>)
    };
    (|$($arg_ident:ident: $arg_ty:ty),*| $block:block) => {
        Closure::wrap(Box::new(|$($arg_ident: $arg_ty),*| {
            $block
        }) as Box<dyn FnMut($($arg_ty),*)>)
    };
}




fn t() {
    js_fn!(move |player_name: String, player_age: u32| {
        
    });
}