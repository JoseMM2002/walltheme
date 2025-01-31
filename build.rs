use std::env;
use std::fs;
use std::path::Path;
use walltheme_config::TomlConfig;

fn main() {
    let schema = schemars::schema_for!(TomlConfig);

    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let config_dir = Path::new(&home_dir).join(".config/walltheme");
    let config_file = config_dir.join("config.toml");

    fs::create_dir_all(&config_dir).expect("Failed to create configuration directory");

    let schema_path = config_dir.join("walltheme.schema.json");

    fs::write(&schema_path, serde_json::to_string_pretty(&schema).unwrap())
        .expect("Failed to write walltheme.schema.json");

    if !config_file.exists() {
        fs::write(&config_file, r#"#:schema ./walltheme.schema.json"#)
            .expect("Failed to create default config.toml");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
