use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = animate_with_react_spring)]
    fn animate_with_react_spring(key: &str, to: JsValue);
}

fn use_spring() {
    let target = js_sys::Object::new();
    js_sys::Reflect::set(&target, &"opacity".into(), &1.0.into()).unwrap();
    animate_with_react_spring("from-rust", target.into());
}


pub struct Config {

    /// With higher tension, the spring will resist bouncing and try harder to stop at its end value.
    /// 
    /// When tension is zero, no animation occurs.
    /// 
    /// # Default 
    /// `170`
    tension: Option<f64>,

    /// The damping ratio coefficient, or just the damping ratio when `speed` is defined.
    /// 
    /// Higher friction means the spring will slow down faster.
    /// 
    /// # Default
    /// `26`
    friction: f64,

    /// The natural frequency (in seconds), which dictates the number of bounces per second when no damping exists.
    /// 
    /// When defined, `tension` is derived from this, and `friction` is derived from `tension` and `damping`.
    frequency: Option<f64>,

    damping: f32,

    mass: Option<f64>,

    /**
     * The initial velocity of one or more values.
     *
     * @default 0
     */
    velocity: Option<f64>,

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


pub struct Bridge;

impl Bridge {
    pub fn animate_to(&self, )
}