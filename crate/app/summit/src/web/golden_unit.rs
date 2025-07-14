pub fn from(k: u16) -> f64 {
    let base: f64 = 8.0;
    let ratio: f64 = 1.618f64;
    let value: f64 = base * ratio.powf(k as f64);
    value.round()
}