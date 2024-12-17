use constants::{DEFAULT_CONFIG, DEFAULT_CONFIG_PATH};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

pub mod constants;

pub type Theme = HashMap<String, RgbJson>;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RgbJson {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbJson {
    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.red, self.green, self.blue)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ConfigOpts {
    pub mix_factor: Option<f64>,
    pub distance_threshold: Option<f64>,
    pub palette_quality: Option<u8>,
    pub palette_max_colors: Option<u8>,
    pub brighter_factor: Option<f64>,
    pub bright_min: Option<u8>,
}

pub struct Config {
    pub mix_factor: f64,
    pub distance_threshold: f64,
    pub palette_quality: u8,
    pub palette_max_colors: u8,
    pub brighter_factor: f64,
    pub bright_min: u8,
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

pub fn get_configs() -> Config {
    let home = home_dir().expect("Could not find home directory");
    let config_path = home.join(DEFAULT_CONFIG_PATH).join("config.toml");

    if !config_path.exists() {
        return DEFAULT_CONFIG;
    }

    let config_content = fs::read_to_string(config_path).expect("Could not read config file");
    let user_config: ConfigOpts = toml::from_str(&config_content).expect("Could not parse config");

    if user_config.brighter_factor.is_some() && user_config.brighter_factor.unwrap() < 1.0 {
        panic!("Brighter factor must be greater than 1.0");
    }

    Config {
        mix_factor: user_config.mix_factor.unwrap_or(DEFAULT_CONFIG.mix_factor),
        distance_threshold: user_config
            .distance_threshold
            .unwrap_or(DEFAULT_CONFIG.distance_threshold),
        palette_quality: user_config
            .palette_quality
            .unwrap_or(DEFAULT_CONFIG.palette_quality),
        palette_max_colors: user_config
            .palette_max_colors
            .unwrap_or(DEFAULT_CONFIG.palette_max_colors),
        brighter_factor: user_config
            .brighter_factor
            .unwrap_or(DEFAULT_CONFIG.brighter_factor),
        bright_min: user_config.bright_min.unwrap_or(DEFAULT_CONFIG.bright_min),
    }
}
