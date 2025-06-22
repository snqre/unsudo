pub enum Direction {
    In,
    Out,
    Symmetric
}


pub enum BounceFalloff {
    Expo(f32),
    Linear(f32),
    Log(f32),
    Custom(fn(f32) -> f32)
}

pub struct Bounce {

    /// The initial height (amplitude) of the bounce.
    /// 
    /// Controls how strong the initial bounce is. Higher values will cause the object to
    /// jump higher with each bounce. If `normalize_amplitude` is enabled, this will always
    /// start as `1.0`, but it can still be adjusted.
    pub amplitude: f32,

    /// The phase shift for the bounce's timing.
    ///
    /// This value affects when the bounce "starts". By adjusting the phase, you can make the
    /// bounce start earlier or later within the cycle, simulating effects like delayed impacts
    /// or offsets in a timed sequence. A phase of `0.0` means no shift, and values greater than
    /// `0.0` will shift the bounce in time.
    pub phase: Option<f32>,

    /// Skew factor to distort the bounce curve.
    ///
    /// This is a power-based adjustment that allows you to modify the "tightness" of the bounce's
    /// timing. A `skew` value of `1.0` results in a uniform bounce curve, while values greater than `1.0`
    /// will make the bounce curve more delayed (later bounces). Values between `0.0` and `1.0` will make the
    /// bounce more "snappy" and quicker early on, but slower as it progresses.
    pub skew: Option<f32>,

    /// Direction of the bounce.
    ///
    /// - `Direction::In`: The bounce starts strong but decays toward the end.
    /// - `Direction::Out`: The bounce starts weak but grows stronger over time.
    /// - `Direction::Symmetric`: A combination of both, where the bounce first decays, then builds strength again.
    ///
    /// This property controls how the bounce progresses over time and can significantly change the feel
    /// of the animation. For example, "in-out" can be useful for smoother, more natural effects.
    pub direction: Direction,
    
    /// Number of bounces (oscillations).
    ///
    /// Determines how many times the object will bounce before it either stabilizes or stops. A higher
    /// value results in more complex oscillations, while a lower value will make the bounce stop more
    /// quickly. This field can be used in combination with `falloff_curve` for more controlled, natural-looking
    /// damping.
    pub bounce_count: u32,

    /// Frequency scaling factor.
    ///
    /// Multiplies the frequency of the bounce, controlling the "tightness" of the bounces.
    /// Higher values will make the bounces happen more frequently (tighter waves), and lower values
    /// will spread them out, creating a slower, more drawn-out effect. This allows designers to make bounces
    /// that feel either quick or long, based on the visual need.
    pub frequency: Option<f32>,

    /// Falloff curve that controls how the bounce decays over time.
    ///
    /// The falloff controls the rate at which the bounce diminishes:
    /// - `Falloff::Exponential(f32)`: The bounce decays exponentially based on the given stiffness parameter.
    /// - `Falloff::Linear(f32)`: Linear decay that diminishes steadily across bounces.
    /// - `Falloff::Logarithmic(f32)`: Logarithmic decay which has a slower start and a sharp end.
    /// - `Falloff::Custom(fn(f32) -> f32)`: A custom function that defines how the decay behaves.
    ///
    /// This property is key to customizing the physicality of the bounce. For example, an exponential falloff
    /// will give a quick, sharp decay, while a linear falloff can provide a more uniform decrease.
    pub falloff: BounceFalloff,

    /// Vertical offset of the bounce.
    ///
    /// This value allows the designer to add a constant vertical shift to the bounce's output. By adjusting the
    /// `offset`, the bounce can start higher or lower than the baseline (the "floor"), effectively raising or
    /// lowering the entire bounce sequence. This is useful for placing the bouncing object at a specific location
    /// or for creating visual effects like impacts or jumps.
    pub offset: Option<f32>,

    /// Floor value where the bounce stops.
    ///
    /// The bounce will stop once it reaches the `floor` value. This is used to simulate objects that will not
    /// bounce below a certain point, like a ball hitting the ground and not going further down. It can be used
    /// to prevent unnatural jittering as the amplitude approaches zero.
    pub floor: Option<f32>,

    /// The minimum threshold under which the bounce is considered to have "snapped" to its final value.
    ///
    /// If the bounce amplitude falls below this threshold, the motion is stopped abruptly and the final value
    /// is returned. This is especially useful for avoiding "micro-jitters" in cases where the bounce has nearly
    /// ended, but is still oscillating slightly due to the decay curve.
    pub snap_threshold: Option<f32>,

    /// Whether to clamp the output of the bounce to the range `[s, s + c]`.
    ///
    /// When `clamp_output` is enabled, the bounce value is guaranteed to stay within the provided bounds
    /// (start `s` and change `c`). This ensures the bounce never exceeds the expected range, even with heavy
    /// oscillation or damping. This is useful when animating UI elements or game objects that should not leave
    /// a specific region of the screen or world.
    pub clamp_out: Option<bool>,

    /// Whether to normalize the amplitude of the bounce, making the first bounce always equal to 1.0.
    ///
    /// If enabled, the first bounce will always have an amplitude of `1.0`, and subsequent bounces will be
    /// scaled down proportionally. This is useful when you want the "first hit" to feel consistent, regardless
    /// of the parameters used.    
    pub normalize_amplitude: Option<bool>,

    /// Whether to mirror odd-numbered bounces.
    ///
    /// If enabled, odd bounces will be reversed, creating a more unpredictable and bouncy effect. This simulates
    /// a more chaotic physical bounce, as if the object is bouncing off multiple surfaces or experiencing
    /// irregular forces.
    pub mirror_odd_bounce: Option<bool>
}

impl Default for Bounce {
    fn default() -> Self {
        Self {
            amplitude: 1.0f32,
            bounce_count: 3u32,
            falloff: BounceFalloff::Expo(3.0f32),
            direction: Direction::Out,
            phase: None,
            skew: None,
            frequency: None,
            offset: None,
            floor: None,
            snap_threshold: None,
            clamp_out: None,
            normalize_amplitude: None,
            mirror_odd_bounce: None
        }
    }
}

impl From<Bounce> for Box<dyn Fn(f32, f32, f32, f32) -> f32> {
    fn from(value: Bounce) -> Self {
        let Bounce {
            amplitude,
            phase,
            skew,
            direction,
            bounce_count,
            frequency,
            falloff,
            offset,
            floor,
            snap_threshold,
            clamp_out,
            normalize_amplitude,
            mirror_odd_bounce
        } = value;
        let phase: f32 = phase.unwrap_or(0.0f32);
        let skew: f32 = skew.unwrap_or(1.0f32);
        let frequency: f32 = frequency.unwrap_or(1.0f32);
        let offset: f32 = offset.unwrap_or(0.0f32);
        let floor: f32 = floor.unwrap_or(0.0f32);
        let snap_threshold: f32 = snap_threshold.unwrap_or(0.001f32);
        let clamp_out: bool = clamp_out.unwrap_or(false);
        let normalize_amplitude: bool = normalize_amplitude.unwrap_or(false);
        let mirror_odd_bounce: bool = mirror_odd_bounce.unwrap_or(true);
        let falloff_fn: Box<dyn Fn(f32) -> f32> = match falloff {
            BounceFalloff::Expo(k) => Box::new(move |x| {
                let z: f32 = 2.0f32.powf(-k * x);
                amplitude * z
            }),
            BounceFalloff::Linear(k) => Box::new(move |x| {
                let z: f32 = (1.0f32 - k * x).max(0.0f32);
                amplitude * z
            }),
            BounceFalloff::Log(k) => Box::new(move |x| {
                let log: f32 = (x * k + 1.0f32).ln();
                let z: f32 = (k + 1.0f32).ln();
                let z: f32 = 1.0f32 - log / z;
                z.max(0.0f32)
            }),
            BounceFalloff::Custom(f) => Box::new(move |x| amplitude * f(x))
        };
        Box::new(move |p: f32, s: f32, c: f32, d: f32| -> f32 {
            if p >= d {
                return s + c
            }
            let t: f32 = p / d;
            let t: f32 = ((t + phase) % 1.0f32).powf(skew);
            let t: f32 = t * frequency;
            let t: f32 = t;
            let bounce_index_raw: f32 = t * bounce_count as f32;
            let bounce_index: f32 = bounce_index_raw.floor();
            let loc_t: f32 = bounce_index_raw - bounce_index;
            let is_odd: bool = (bounce_index as u32) % 2u32 != 0u32;
            let is_mirrored: bool = mirror_odd_bounce && is_odd;
            let falloff: f32 = falloff_fn(bounce_index / bounce_count as f32);
            if falloff.abs() < snap_threshold {
                return s + c
            }
            let parabola: f32 = if is_mirrored {
                let k: f32 = (0.5f32 - loc_t).powi(2i32);
                -falloff * (4.0f32 * k - 1.0f32)
            } else {
                let k: f32 = (loc_t - 0.5f32).powi(2i32);
                falloff * (4.0 * k - 1.0)
            };
            let y: f32 = match direction {
                Direction::In => (1.0f32 - t).powf(2.0f32) * parabola,
                Direction::Out => t.powf(2.0f32) * parabola,
                Direction::Symmetric => {
                    let k = if t < 0.5f32 {
                        2.0f32 * t
                    } else {
                        2.0f32 * (1.0f32 - t)
                    };
                    k.powf(2.0f32) * parabola
                }
            };
            let mut out: f32 = s + offset + if normalize_amplitude {
                y
            } else {
                y * c
            };
            if clamp_out {
                out = out.clamp(s, s + c);
            }
            out = out.max(s + floor);
            out
        })
    }
}


pub fn ease_in(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c *
    (p / d) *
    (p / d) + s
}

pub fn ease_in_cubic(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c *
    (p / d) *
    (p / d) *
    (p / d) + s
}

pub fn ease_in_quart(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c *
    (p / d) *
    (p / d) *
    (p / d) * 
    (p / d) + s
}

pub fn ease_in_quint(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c * 
    (p / d) *
    (p / d) *
    (p / d) *
    (p / d) *
    (p / d) + s
}


pub fn ease_out(p: f32, s: f32, c: f32, d: f32) -> f32 {
    -c * (p / d) * ((p / d) - 2.0f32) + s
}

/// # Feel
/// Very shart deceleration.
pub fn ease_out_expo(p: f32, s: f32, c: f32, d: f32) -> f32 {
    if p == d {
        return s + c
    }
    let k: f32 = -2f32.powf(-10.0f32 * p / d) + 1.0f32;
    c * k + s
}

pub fn ease_out_cubic(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c * (
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) + 1.0f32
    ) + s
}

pub fn ease_out_quart(p: f32, s: f32, c: f32, d: f32) -> f32 {
    -c * (
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) - 1.0f32
    ) + s
}

pub fn ease_out_quint(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c * (
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) + 1.0f32
    ) + s
}