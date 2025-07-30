use super::*;

pub fn on_timeout(ms: u32, event_handler: &js_sys::Function) {
    let event_handler = js_fn!(move || -> f32 {
        0.0
    });
}


use ::wasm_bindgen::prelude::*;
use ::serde_wasm_bindgen;

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        
    };
}

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
    ($js_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*)) => {
        paste::paste!(
            mod $fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident($($arg_ident: JsValue),*) -> Result<(), JsValue>;
                }
            }

            pub fn $fn_ident($($arg_ident: $arg_ty),*) -> Result<(), JsValue> {
                $fn_ident::$fn_ident($(serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                    JsValue::from_str(&e.to_string())
                })?),*)
            }
        );
    };
    ($js_path:literal::$fn_ident:ident() -> $ret_ty:ty) => {
        paste::paste!(
            mod $fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            pub fn $fn_ident() -> Result<$ret_ty, JsValue> {
                let ret = $fn_ident::$fn_ident();
                serde_wasm_bindgen::from_value(ret).map_err(|e| {
                    JsValue::from_str(&e.to_string())
                })
            }
        );
    };
    ($js_path:literal::$fn_ident:ident()) => {
        paste::paste!(
            mod $fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub fn $fn_ident() -> Result<(), JsValue> {
                let ret = $fn_ident::$fn_ident()?;
                if ret.is_undefined() || ret.is_null() {
                    Ok(())
                } else {
                    warn!("...")
                    Ok(())   
                }
            }
        );
    };







    (async $file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*)) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use super::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident($($arg_ident: JsValue),*) -> Result<JsValue, JsValue>;
                }
            }

            pub async fn $fn_ident($($arg_ident: $arg_ty),*) -> Result<(), JsValue> {
                let ret = [< __ $fn_ident >]::$fn_ident($(serde_wasm_bindgen::to_value(&$arg_ident).map_err(|e| {
                    JsValue::from_str(&e.to_string())
                })?),*).await?;
                if !ret.is_undefined() || !ret.is_null() {
                    warn!("");
                    Ok(())
                } else {
                    Ok(())
                }
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident()) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use super::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub async fn $fn_ident() -> Result<(), JsValue> {
                let ret = [< __ $fn_ident >]::$fn_ident().await?;
                if !ret.is_undefined() || !ret.is_null() {
                    warn!("...");
                    Ok(())
                } else {
                    Ok(())
                }
            }
        );
    };
    
}

#[macro_export]
macro_rules! val {
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