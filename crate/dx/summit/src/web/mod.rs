use super::*;

pub mod component;
pub mod route;
pub mod color;
pub mod easing;

#[allow(unused)]
pub fn u(k: u16) -> f64 {
    let base: f64 = 8.0;
    let ratio: f64 = 1.618f64;
    let value: f64 = base * ratio.powf(k as f64);
    value.round()
}

#[allow(unused)]
pub fn vrem(value: f64) -> String {
    "{value}vw + 1rem".to_owned()
}

pub fn sequence(k: u16) -> f64 {
    let base: f64 = 8.0f64;
    let ratio: f64 = 1.618f64;
    let value: f64 = base * ratio.powf(k as f64);
    value.round()
}