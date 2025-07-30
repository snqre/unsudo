use super::*;

pub fn on_timeout(ms: u32, event_handler: &js_sys::Function) {
    let event_handler = js_fn!(move || -> f32 {
        0.0
    });
}


use ::wasm_bindgen::prelude::*;
use ::serde_wasm_bindgen;

#[macro_export]
macro_rules! js_include {
    ($js_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*) -> $ret_ty:ty) => {
        paste::paste!(
            mod $fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident($($arg_ident: JsValue),*) -> Result<JsValue, JsValue>;
                }
            }

            pub fn $fn_ident($($arg_ident: $arg_ty),*) -> Result<$ret_ty, JsValue> {
                let ret = $fn_ident::$fn_ident(
                    $(serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                        JsValue::from_str(&e.to_string())
                    })?),*
                )?;
                serde_wasm_bindgen::from_value(ret).map_err(|e| {
                    JsValue::from_str(&e.to_string())
                })
            }
        );
    };
}



js_include!("src/ts/event.ts"::hello_world(status: u8) -> u8);






#[macro_export]
macro_rules! js_value {
    ($value:expr) => {
        JsValue::from($value)
    };
}

#[macro_export]
macro_rules! js_fn {
    (move || -> $ret_ty:ty $block:block) => {
        Closure::wrap(Box::new(move || -> $ret_ty {
            $block
        }) as Box<dyn FnMut() -> $ret_ty>)
    };
    (move || $block:block) => {
        Closure::wrap(Box::new(move || {
            $block
        }) as Box<dyn FnMut()>)
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
        Closure::wrap(Box::new(|| -> $ret_ty {
            $block
        }) as Box<dyn FnMut() -> $ret_ty>)
    };
    (|| $block:block) => {
        Closure::wrap(Box::new(|| {
            $block
        }) as Box<dyn FnMut()>)
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

#[cfg(test)]
mod sig_test {
    use super::*;

    fn success() {
        js_fn!(move || -> u8 {
            200
        });

        js_fn!(move || {
            
        });

        js_fn!(move |foo: u8| -> u8 {
            foo
        });

        js_fn!(move |_foo: u8| {
            
        });

        js_fn!(|| -> u8 {
            200
        });

        js_fn!(|| {

        });

        js_fn!(|foo: u8| -> u8 {
            foo
        });

        js_fn!(|_foo: u8| {

        });
    }
}