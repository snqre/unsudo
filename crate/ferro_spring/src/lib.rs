use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_timers::callback;
use gloo_utils::format::JsValueSerdeExt as _;

#[derive(Clone)]
#[derive(serde::Serialize)]
pub struct Config {

    /// With higher tension, the spring will resist bouncing and try harder to stop at its end value.
    ///
    /// When tension is zero, no animation occurs.
    ///
    /// @default 170
    pub tension: Option<f64>,

    /// The damping ratio coefficient, or just the damping ratio when `speed` is defined.
    ///
    /// When `speed` is defined, this value should be between 0 and 1.
    ///
    /// Higher friction means the spring will slow down faster.
    ///
    /// @default 26
    pub friction: Option<f64>,

    /// The natural frequency (in seconds), which dictates the number of bounces
    /// per second when no damping exists.
    ///
    /// When defined, `tension` is derived from this, and `friction` is derived
    /// from `tension` and `damping`.
    pub frequency: Option<f64>,

    /// The damping ratio, which dictates how the spring slows down.
    ///
    /// Set to `0` to never slow down. Set to `1` to slow down without bouncing.
    /// Between `0` and `1` is for you to explore.
    ///
    /// Only works when `frequency` is defined.
    ///
    /// @default 1
    pub damping: Option<f32>,

    /// Higher mass means more friction is required to slow down.
    ///
    /// Defaults to 1, which works fine most of the time.
    ///
    /// @default 1
    pub mass: Option<f64>,

    /// The initial velocity of one or more values.
    ///
    /// @default 0
    pub velocity: Option<f64>,

    /// The smallest velocity before the animation is considered "not moving".
    ///
    /// When undefined, `precision` is used instead.
    pub rest_velocity: Option<f64>,

    /// The smallest distance from a value before that distance is essentially zero.
    ///
    /// This helps in deciding when a spring is "at rest". The spring must be within
    /// this distance from its final value, and its velocity must be lower than this
    /// value too (unless `restVelocity` is defined).
    ///
    /// @default 0.01
    pub precision: Option<f64>,

    /// For `duration` animations only. Note: The `duration` is not affected
    /// by this property.
    ///
    /// Defaults to `0`, which means "start from the beginning".
    ///
    /// Setting to `1+` makes an immediate animation.
    ///
    /// Setting to `0.5` means "start from the middle of the easing function".
    ///
    /// Any number `>= 0` and `<= 1` makes sense here.
    pub progress: Option<f64>,

    /// Animation length in number of milliseconds.
    pub duration: Option<f64>,

    /// Avoid overshooting by ending abruptly at the goal value.
    ///
    /// @default false
    pub clamp: Option<bool>,

    /// When above zero, the spring will bounce instead of overshooting when
    /// exceeding its goal value. Its velocity is multiplied by `-1 + bounce`
    /// whenever its current value equals or exceeds its goal. For example,
    /// setting `bounce` to `0.5` chops the velocity in half on each bounce,
    /// in addition to any friction.
    pub bounce: Option<f64>,

    /// "Decay animations" decelerate without an explicit goal value.
    /// Useful for scrolling animations.
    ///
    /// When a `number` between `0` and `1` is given, a lower number makes the
    /// animation slow down faster. And setting to `1` would make an unending
    /// animation.
    ///
    /// @default false
    pub decay: Option<f64>,

    /// While animating, round to the nearest multiple of this number.
    /// The `from` and `to` values are never rounded, as well as any value
    /// passed to the `set` method of an animated value.
    pub round: Option<f64>
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = register)]
    fn register(initial_value: JsValue, config: JsValue) -> u32;

    #[wasm_bindgen(js_name = get)]
    fn get(key: JsValue) -> f64;

    #[wasm_bindgen(js_name = set)]
    fn set(key: JsValue, value: JsValue, config: JsValue);

    #[wasm_bindgen(js_name = cleanup)]
    fn clean_up(key: JsValue);
}

pub fn use_react_spring(initial_value: f64, cfg: Option<Config>, sync_rate: u32) -> Option<Signal<f64>> {
    let sig: Signal<f64> = use_signal(|| initial_value);

    use_hook(move || {
        let cfg_opt: Option<Config> = cfg.clone();
        let mut sig: Signal<f64> = sig.to_owned();
        let key: Option<JsValue> = if let Some(js_cfg) = cfg_opt.and_then(|cfg| cfg_to_js(&cfg)) {
            let cfg: JsValue = js_cfg;
            let val: JsValue = initial_value.into();
            let key: u32 = register(val, cfg);
            let js_key: JsValue = JsValue::from(key);
            let _: callback::Interval = callback::Interval::new(sync_rate, {
                let key: JsValue = js_key.clone();
                move || {
                    let val = get(key.clone());
                    sig.set(val);
                }
            });
            Some(js_key)
        } else {
            None
        };
    
        // Just store the cleanup key, not the interval
        key
    });

    Some(sig)
}

macro_rules! assign {
    ($js_obj:ident, $cfg:ident, $field:ident) => {
        if let Some(value) = $cfg.$field {
            js_sys::Reflect::set(
                &$js_obj,
                &JsValue::from_str(stringify!($field)),
                &JsValue::from_f64(value as f64)
            ).ok()?;
        }
    };
}

fn cfg_to_js(cfg: &Config) -> Option<JsValue> {
    let js_obj: js_sys::Object = js_sys::Object::new();
    assign! { js_obj, cfg, tension }
    assign! { js_obj, cfg, friction }
    assign! { js_obj, cfg, frequency }
    assign! { js_obj, cfg, mass }
    assign! { js_obj, cfg, velocity }
    assign! { js_obj, cfg, rest_velocity }
    assign! { js_obj, cfg, precision }
    assign! { js_obj, cfg, progress }
    assign! { js_obj, cfg, duration }
    assign! { js_obj, cfg, bounce }
    assign! { js_obj, cfg, decay }
    assign! { js_obj, cfg, round }
    if let Some(damping) = cfg.damping {
        let k: &JsValue = &"damping".into();
        let v: &JsValue = &damping.into();
        js_sys::Reflect::set(&js_obj, k, v).ok()?;
    }
    if let Some(clamp) = cfg.clamp {
        let k: &JsValue = &"clamp".into();
        let v: &JsValue = &clamp.into();
        js_sys::Reflect::set(&js_obj, k, v).ok()?;
    }
    let js_obj: JsValue = js_obj.into();
    Some(js_obj)
}