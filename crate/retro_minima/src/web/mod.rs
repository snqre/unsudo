use super::*;

pub mod component;
pub mod route;
pub mod semantic;


pub mod window;
pub mod color;
pub mod easing;



pub fn sequence(k: u16) -> f64 {
    let base: f64 = 8.0f64;
    let ratio: f64 = 1.618f64;
    let value: f64 = base * ratio.powf(k as f64);
    value.round()
}