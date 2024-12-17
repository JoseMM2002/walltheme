use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod constants;

pub type Theme = HashMap<&'static str, RgbJson>;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RgbJson {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn distance(color1: RgbJson, color2: RgbJson) -> f64 {
    let r = color1.red as f64 - color2.red as f64;
    let g = color1.green as f64 - color2.green as f64;
    let b = color1.blue as f64 - color2.blue as f64;

    (r.powi(2) + g.powi(2) + b.powi(2)).sqrt()
}

pub fn interpolate(start: u8, end: u8, factor: f64) -> u8 {
    (start as f64 + (end as f64 - start as f64) * factor) as u8
}

pub fn gen_color_mix(color1: RgbJson, color2: RgbJson, factor: f64) -> RgbJson {
    RgbJson {
        red: interpolate(color1.red, color2.red, factor),
        green: interpolate(color1.green, color2.green, factor),
        blue: interpolate(color1.blue, color2.blue, factor),
    }
}
