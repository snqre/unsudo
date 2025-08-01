use super::*;

pub mod color;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Url {
    External(String),
    Internal(Asset)
}
impl ToString for Url {
    fn to_string(&self) -> String {
        match self {
            Self::External(x) => x.to_string(),
            Self::Internal(x) => x.to_string()
        }
    }
}

#[derive(Routable)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Route {
    #[route("/")]
    Home {}
}

mod scale {
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
}

mod color {
    pub static OBSIDIAN: &str = "#121212";
    pub static CARBON: &str = "#202020";
    pub static JET: &str = "#404040";
    pub static AQUA: &str = "#357DED";
    pub static OFFICE_BLUE: &str = "#383F51";
    pub static STEEL: &str = "#9D9D9C";
    pub static SILVER: &str = "#EFE6DD";
    pub static SPRING: &str = "#00A676";
    pub static HONEY: &str = "#FF8000";
    pub static IMPERIAL_RED: &str = "#FF0004";

    pub fn interpolate(range: (&str, &str), t: f32) -> String {
        let (rx, gx, bx) = hex_to_rgb(range.0);
        let (ry, gy, by) = hex_to_rgb(range.1);
        let r = rx as f32 + (ry as f32 - rx as f32) * t;
        let r = r.round() as u8;
        let g = gx as f32 + (gy as f32 - gx as f32) * t;
        let g = g.round() as u8;
        let b = bx as f32 + (by as f32 - bx as f32) * t;
        let b = b.round() as u8;
        rgb_to_hex(r, g, b)
    }

    fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    } 
}