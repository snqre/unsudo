use super::*;

pub fn on_timeout(ms: u32, event_handler: &js_sys::Function) {
    let event_handler = function!(move || -> f32 {
        0.0
    });
}




#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        
    };
}

#[macro_export]
macro_rules! bind {
    (async $file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*) -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident($($arg_ident: JsValue),*) -> ::core::result::Result<JsValue, JsValue>;
                }
            }

            pub async fn $fn_ident($($arg_ident: $arg_ty),*) -> ::core::result::Result<$ret_ty, JsValue> {
                to_err(::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident($(to_err(::serde_wasm_bindgen::to_value(&$arg_ident))?),*).await?))
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*)) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident($($arg_ident: JsValue),*) -> Result<JsValue, JsValue>;
                }
            }

            pub async fn $fn_ident($($arg_ident: $arg_ty),*) -> Result<(), ::wasm_bindgen::JsValue> {
                let ret: ::wasm_bindgen::JsValue = [< __ $fn_ident >]::$fn_ident($(to_err(::serde_wasm_bindgen::to_value(&$arg_ident))?),*).await?;
                if !ret.is_undefined() || !ret.is_null() {
                    warn_unexpected_capture!($file_path, $fn_ident, ret);
                }
                Ok(())
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident() -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            pub async fn $fn_ident() -> ::core::result::Result<$ret_ty, ::wasm_bindgen::JsValue> {
                to_err(::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident().await?))
            }
        );
    };
    (async $file_path:literal::$fn_ident:ident()) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub async fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub async fn $fn_ident() -> Result<(), JsValue> {
                let ret: JsValue = [< __ $fn_ident >]::$fn_ident().await?;
                if !ret.is_undefined() || !ret.is_null() {
                    warn!(format!("Expected `undefined` or `null` from `{}` in `{}`, but got: {:?}", stringify!($fn_ident), $file_path, ret));
                    Ok(())
                } else {
                    Ok(())
                }
            }
        );
    };
    ($file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*) -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident($($arg_ident: JsValue),*) -> ::core::result::Result<JsValue, JsValue>;
                }
            }

            pub fn $fn_ident($($arg_ident: $arg_ty),*) -> ::core::result::Result<$ret_ty, ::wasm_bindgen::JsValue> {
                to_err(::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident($(to_err(::serde_wasm_bindgen::to_value(&$arg_ident))?),*)?))
            }
        );
    };
    ($file_path:literal::$fn_ident:ident($($arg_ident:ident: $arg_ty:ty),*)) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident($($arg_ident: JsValue),*) -> ::core::result::Result<JsValue, JsValue>;
                }
            }

            pub fn $fn_ident($($arg_ident: $arg_ty),*) -> ::core::result::Result<(), ::wasm_bindgen::JsValue> {
                let ret = [< __ $fn_ident >]::$fn_ident($(to_err(::serde_wasm_bindgen::to_value(&$arg_ident))?),*)?;
                if !ret.is_undefined() || !ret.is_null() {
                    warn_unexpected_capture!($file_path, $fn_ident, ret);
                }
                Ok(())
            }
        );
    };
    ($file_path:literal::$fn_ident:ident() -> $ret_ty:ty) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub fn $fn_ident() -> Result<$ret_ty, ::wasm_bindgen::JsValue> {
                to_err(::serde_wasm_bindgen::from_value([< __ $fn_ident >]::$fn_ident()?))
            }
        );
    };
    ($file_path:literal::$fn_ident:ident()) => {
        paste::paste!(
            mod [< __ $fn_ident >] {
                use ::wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = $file_path)]
                extern "C" {
                    #[wasm_bindgen(catch)]
                    pub fn $fn_ident() -> Result<JsValue, JsValue>;
                }
            }

            #[inline]
            pub fn $fn_ident() -> Result<(), JsValue> {
                warn_unexpected_capture!($file_path, $fn_ident, [< __ $fn_ident >]::$fn_ident()?)
            }
        );
    };
}



macro_rules! warn_unexpected_capture {
    ($file_path:literal, $fn_ident:ident, $js_val:expr) => {
        warn!(format!("Expected `undefined` or `null` from `{}` in `{}`, but got: `{:?}`", stringify!($fn_ident), $file_path, $js_val));
    };
}

#[inline]
fn to_err<A, B>(ret: ::core::result::Result<A, B>) -> ::core::result::Result<A, ::wasm_bindgen::JsValue> 
where
    B: ::core::fmt::Display {
    ret.map_err(|e| ::wasm_bindgen::JsValue::from_str(&e.to_string()))
}

#[macro_export]
macro_rules! val {
    ($value:expr) => {
        JsValue::from($value)
    };
}

#[macro_export]
macro_rules! function {
    (move || -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move || -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut() -> $ret_ty>)
    };
    (move || $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move || {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut()>)
    };
    (move |$($arg_ident:ident: $arg_ty:ty),*| -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move |$($arg_ident: $arg_ty),*| -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*) -> $ret_ty>)
    };
    (move |$($arg_ident:ident: $arg_ty:ty),*| $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(move |$($arg_ident: $arg_ty),*| {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*)>)
    };
    (|| -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|| -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut() -> $ret_ty>)
    };
    (|| $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|| {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut()>)
    };
    (|$($arg_ident:ident: $arg_ty:ty),*| -> $ret_ty:ty $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|$($arg_ident: $arg_ty),*| -> $ret_ty {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*) -> $ret_ty>)
    };
    (|$($arg_ident:ident: $arg_ty:ty),*| $block:block) => {
        ::wasm_bindgen::closure::Closure::wrap(::std::boxed::Box::new(|$($arg_ident: $arg_ty),*| {
            $block
        }) as ::std::boxed::Box<dyn ::core::ops::FnMut($($arg_ty),*)>)
    };
}

#[cfg(test)]
mod sig_test {
    use super::*;

    fn success() {
        bind!(async "/src/wallet.ts"::test_0(a: u8, b: u8, c: u8) -> u8);
        bind!(async "/src/wallet.ts"::test_1(a: u8, b: u8, c: u8));
        bind!(async "/src/wallet.ts"::test_2() -> u8);
        bind!(async "/src/wallet.ts"::test_3());        
        bind!("/src/wallet.ts"::test_4(a: u8, b: u8, c: u8) -> u8);
        bind!("/src/wallet.ts"::test_5(a: u8, b: u8, c: u8));
        bind!("/src/wallet.ts"::test_6() -> u8);
        bind!("/src/wallet.ts"::test_7());

        function!(move || -> u8 {
            200
        });

        function!(move || {
            
        });

        function!(move |foo: u8| -> u8 {
            foo
        });

        function!(move |_foo: u8| {
            
        });

        function!(|| -> u8 {
            200
        });

        function!(|| {

        });

        function!(|foo: u8| -> u8 {
            foo
        });

        function!(|_foo: u8| {

        });
    }
}