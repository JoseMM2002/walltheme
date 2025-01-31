use std::{collections::HashMap, fs};

use dirs::home_dir;
use walltheme_config::{TomlColorsConfig, TomlConfig};

use crate::constants::{DEFAULT_COLORS_CONFIG, DEFAULT_CONFIG, DEFAULT_CONFIG_PATH};

#[derive(Clone, Debug)]
pub struct ColorsConfig {
    pub mix_factor: f64,
    pub distance_threshold: f64,
    pub brighter_factor: f64,
    pub bright_min: u8,
    pub opacity_target: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct GeneralConfig {
    pub palette_quality: u8,
    pub palette_max_colors: u8,
    pub stdout_template: Option<String>,
    pub colors: HashMap<String, TomlColorsConfig>,
}

pub fn get_configs() -> (GeneralConfig, ColorsConfig) {
    let home = home_dir().expect("Could not find home directory");
    let config_path = home.join(DEFAULT_CONFIG_PATH).join("config.toml");

    if !config_path.exists() {
        return (DEFAULT_CONFIG.clone(), DEFAULT_COLORS_CONFIG);
    }

    let config_content = fs::read_to_string(config_path).expect("Could not read config file");
    let toml_config: TomlConfig = toml::from_str(&config_content).expect("Could not parse config");

    let user_config = match toml_config.general {
        Some(config) => {
            if config.brighter_factor.is_some() && config.brighter_factor.unwrap() < 1.0 {
                panic!("Brighter factor must be greater than 1.0");
            }
            config
        }
        None => {
            return (DEFAULT_CONFIG.clone(), DEFAULT_COLORS_CONFIG);
        }
    };

    (
        GeneralConfig {
            palette_quality: user_config
                .palette_quality
                .unwrap_or(DEFAULT_CONFIG.palette_quality),
            palette_max_colors: user_config
                .palette_max_colors
                .unwrap_or(DEFAULT_CONFIG.palette_max_colors),
            stdout_template: user_config.stdout_template,
            colors: toml_config.color.unwrap_or_default(),
        },
        ColorsConfig {
            mix_factor: user_config
                .mix_factor
                .unwrap_or(DEFAULT_COLORS_CONFIG.mix_factor),
            distance_threshold: user_config
                .distance_threshold
                .unwrap_or(DEFAULT_COLORS_CONFIG.distance_threshold),
            brighter_factor: user_config
                .brighter_factor
                .unwrap_or(DEFAULT_COLORS_CONFIG.brighter_factor),
            bright_min: user_config
                .bright_min
                .unwrap_or(DEFAULT_COLORS_CONFIG.bright_min),
            opacity_target: user_config.opacity_target,
        },
    )
}

pub fn get_color_config(
    name: &str,
    general_config: &GeneralConfig,
    colors_config: &ColorsConfig,
) -> ColorsConfig {
    if let Some(color_config) = general_config.colors.get(name) {
        ColorsConfig {
            distance_threshold: color_config
                .distance_threshold
                .unwrap_or(colors_config.distance_threshold),
            mix_factor: color_config.mix_factor.unwrap_or(colors_config.mix_factor),
            brighter_factor: color_config
                .brighter_factor
                .unwrap_or(colors_config.brighter_factor),
            bright_min: color_config.bright_min.unwrap_or(colors_config.bright_min),
            opacity_target: color_config.opacity_target.or(colors_config.opacity_target),
        }
    } else {
        colors_config.clone()
    }
}
