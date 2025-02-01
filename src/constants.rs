use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::config::{ColorsConfig, GeneralConfig};
use crate::theme::{RgbJson, Theme};

pub const DEFAULT_COLORS_CONFIG: ColorsConfig = ColorsConfig {
    mix_factor: 0.1,
    distance_threshold: 50.0,
    brighter_factor: 1.4,
    bright_min: 20,
    opacity_target: None,
};

pub const DEFAULT_CONFIG_PATH: &str = ".config/walltheme/";
pub const DEFAULT_TEMPLATES_PATH: &str = ".config/walltheme/templates/";
pub const CACHE_PATH: &str = ".cache/walltheme/";

lazy_static! {
    pub static ref DEFAULT_CONFIG: GeneralConfig = GeneralConfig {
        palette_quality: 1,
        palette_max_colors: 255,
        stdout_template: None,
        colors: HashMap::new(),
    };
    pub static ref OBJECTIVE_THEME: Theme = {
        HashMap::from([
            (
                "black".to_string(),
                RgbJson {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
            ),
            (
                "red".to_string(),
                RgbJson {
                    red: 255,
                    green: 0,
                    blue: 0,
                },
            ),
            (
                "green".to_string(),
                RgbJson {
                    red: 0,
                    green: 255,
                    blue: 0,
                },
            ),
            (
                "blue".to_string(),
                RgbJson {
                    red: 0,
                    green: 0,
                    blue: 255,
                },
            ),
            (
                "yellow".to_string(),
                RgbJson {
                    red: 255,
                    green: 255,
                    blue: 0,
                },
            ),
            (
                "magenta".to_string(),
                RgbJson {
                    red: 255,
                    green: 0,
                    blue: 255,
                },
            ),
            (
                "cyan".to_string(),
                RgbJson {
                    red: 0,
                    green: 255,
                    blue: 255,
                },
            ),
            (
                "white".to_string(),
                RgbJson {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
            ),
            (
                "orange".to_string(),
                RgbJson {
                    red: 255,
                    green: 165,
                    blue: 0,
                },
            ),
        ])
    };
}
