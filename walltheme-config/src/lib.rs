use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ConfigOpts {
    pub mix_factor: Option<f64>,
    pub distance_threshold: Option<f64>,
    pub palette_quality: Option<u8>,
    pub palette_max_colors: Option<u8>,
    pub brighter_factor: Option<f64>,
    pub bright_min: Option<u8>,
    pub opacity_target: Option<u8>,
    pub stdout_template: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TomlColorsConfig {
    pub mix_factor: Option<f64>,
    pub distance_threshold: Option<f64>,
    pub brighter_factor: Option<f64>,
    pub bright_min: Option<u8>,
    pub opacity_target: Option<u8>,
    pub rgb: Option<(u8, u8, u8)>,
    pub hex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TomlConfig {
    pub general: Option<ConfigOpts>,
    pub color: Option<HashMap<String, TomlColorsConfig>>,
}
