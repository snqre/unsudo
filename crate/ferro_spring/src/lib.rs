use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt as _;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = animate_with_react_spring)]
    fn animate() -> js_sys::Array;
}

pub type Get = Box<dyn Fn() -> f64>;
pub type Set = Box<dyn Fn(f64, Config)>;

pub fn sync() -> (Get, Set) {
    let ar: js_sys::Array = animate();
    let js_get: js_sys::Function = ar.get(0).unchecked_into();
    let js_set: js_sys::Function = ar.get(1).unchecked_into();
    let get: Get = Box::new(move || {
        let val: JsValue = js_get.call0(&JsValue::NULL).unwrap();
        val.as_f64().unwrap()
    });
    let set: Set = Box::new(move |new_value: f64, cfg: Config| {
        let cfg_js: JsValue = JsValue::from_serde(&cfg).unwrap();
        js_set.call2(&JsValue::NULL, &JsValue::from(new_value), &cfg_js).unwrap();
    });
    (get, set)
}

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

pub fn use_spring() -> (Signal<f64>, Box<dyn Fn(f64, Config)>) {
    let signal: Signal<f64> = use_signal(|| 0.0f64);
    let ar = animate();
    let js_get: js_sys::Function = ar.get(0).unchecked_into();
    let js_set: js_sys::Function = ar.get(1).unchecked_into();
    let get = {
        let js_get = js_get.to_owned();
        let signal = signal.to_owned();
        
        use_effect(move || {
            let internal = Interval::new(16, move || {

            });
        });
    };
    let set = {
        let js_set = js_set.to_owned();
        Box::new(move |new_value: f64, cfg: Config| {
            let js_cfg = JsValue::from_serde(&cfg).unwrap();
            js_cfg.call2(&JsValue::NULL, &JsValue::from_f64(new_value), &js_cfg);
        }) as Box<dyn Fn(f64, Config)>
    };
    (signal, set)
}