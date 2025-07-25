use wasm_bindgen::prelude::*;


struct Account {
    address: String,

    /// account association with specific chain/network
    genesis_block: Option<String>,

    /// nickname set in the wallet ui
    name: Option<String>,

    /// cryptographic key kind
    kind: Option<KeyPair>
}

#[repr(u8)]
enum KeyPair {
    Ed25519,
    Sr25519,
    Ecdsa,
    Ethereum
}







/// ```rs
/// use wasm_bindgen::prelude::*;
/// 
/// link!(
///     "/src/hello_world.js"::hello_world() -> String
/// );
/// 
/// link!(
///     "/src/hello_world.js"::log_message(message: String) -> bool
/// );
/// ```
macro_rules! import {
    ($($js_path:literal::$js_fn_ident:ident() [async])*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub async fn $js_fn_ident();
                }
            }

            #[wasm_bindgen]
            pub async fn $js_fn_ident() {
                $js_fn_ident::$js_fn_ident().await;
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident())*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub fn $js_fn_ident();
                }
            }

            #[wasm_bindgen]
            pub fn $js_fn_ident() {
                $js_fn_ident::$js_fn_ident();
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident() -> $rs_ret_ty:ty [async])*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub async fn $js_fn_ident() -> JsValue;
                }
            }

            #[wasm_bindgen]
            pub async fn $js_fn_ident() -> Option<$rs_ret_ty> {
                serde_wasm_bindgen::from_value($js_fn_ident().await).ok()
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident() -> $rs_ret_ty:ty)*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub fn $js_fn_ident() -> JsValue;
                }
            }

            #[wasm_bindgen]
            pub fn $js_fn_ident() -> Option<$rs_ret_ty> {
                serde_wasm_bindgen::from_value($js_fn_ident::$js_fn_ident()).ok()
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident($rs_arg_ident:ident: $rs_arg_ty:ty) [async])*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub async fn $js_fn_ident(js_data: JsValue);
                }
            }

            #[wasm_bindgen]
            pub async fn $js_fn_ident($rs_arg_ident: $rs_arg_ty) {
                if let Some(js_arg) = serde_wasm_bindgen::to_value(&$rs_arg_ident).ok() {
                    $js_fn_ident::$js_fn_ident(js_arg).await;
                }
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident($rs_arg_ident:ident: $rs_arg_ty:ty))*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub fn $js_fn_ident(js_data: JsValue);
                }
            }

            #[wasm_bindgen]
            pub fn $js_fn_ident($rs_arg_ident: $rs_arg_ty) {
                if let Some(js_arg) = serde_wasm_bindgen::to_value(&$rs_arg_ident).ok() {
                    $js_fn_ident::$js_fn_ident(js_arg);
                }
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident($rs_arg_ident:ident: $rs_arg_ty:ty) -> $rs_ret_ty:ty [async])*) => {
        $(
            mod $js_fn_ident {
                #[super::wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub async fn $js_fn_ident(js_data: JsValue) -> JsValue;
                }
            }

            #[wasm_bindgen]
            pub async fn $rs_fn_ident($rs_arg_ident: $rs_arg_ty) -> Option<$rs_ret_ty> {
                let js_arg: _ = serde_wasm_bindgen::to_value(&$rs_arg_ident).ok()?;
                let js_ret: _ = $js_fn_ident::$js_fn_ident(js_arg).await;
                serde_wasm_bindgen::from_value(js_ret).ok()
            }
        )*
    };

    ($($js_path:literal::$js_fn_ident:ident($rs_arg_ident:ident: $rs_arg_ty:ty) -> $rs_ret_ty:ty)*) => {
        $(
            mod $js_fn_ident {
                use super::*;

                #[wasm_bindgen(module = $js_path)]
                extern "C" {
                    pub fn $js_fn_ident(js_data: JsValue) -> JsValue;
                }
            }

            #[wasm_bindgen]
            pub fn $js_fn_ident($rs_arg_ident: $rs_arg_ty) -> Option<$rs_ret_ty> {
                let js_arg: _ = serde_wasm_bindgen::to_value(&$rs_arg_ident).ok()?;
                let js_ret: _ = $js_fn_ident::$js_fn_ident(js_arg);
                serde_wasm_bindgen::from_value(js_ret).ok()
            }            
        )*
    };
}



import!(
    "/src/connect.ts"::do_something(message: String) [async]
);

import!(
    "/src/connect.ts"::hello(message: String)
);

import!(
    "/src/connect.ts"::hello_world() -> String
);

import!(
    "/src/connect.ts"::foo_bar(message: String) -> u8
);