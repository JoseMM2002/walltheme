use std::fs::{self, create_dir_all};
use std::{collections::HashMap, path::Path};

use color_thief::{get_palette, ColorFormat};
use dirs::home_dir;
use handlebars::Handlebars;
use serde_json::json;
use termion::{color, style};
use walkdir::WalkDir;
use walltheme::constants::{
    CACHE_PATH, DEFAULT_CONFIG_PATH, DEFAULT_TEMPLATES_PATH, OBJECTIVE_THEME,
};
use walltheme::{distance, gen_color_mix, get_configs, MissingHelper, RgbJson, Theme};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let image = image::open(&Path::new(filename)).unwrap();
    let rgb_image = image.to_rgb8();
    let pixels = rgb_image.as_raw();

    let config = get_configs();

    let palette = get_palette(
        pixels,
        ColorFormat::Rgb,
        config.palette_quality,
        config.palette_max_colors,
    );

    if palette.is_err() {
        println!("Error: {:?}", palette.err().unwrap());
        std::process::exit(1);
    }

    let palette = palette.unwrap();
    let mut theme: Theme = HashMap::new();

    for current_color in palette {
        let current_color = RgbJson {
            red: current_color.r,
            green: current_color.g,
            blue: current_color.b,
        };
        for (name, objective_color) in OBJECTIVE_THEME.iter() {
            let theme_color = theme.get(name);

            if theme_color.is_none()
                || distance(current_color, *objective_color)
                    < distance(*objective_color, *theme_color.unwrap())
            {
                theme.insert(name.to_string(), current_color);
            }
        }
    }

    let mut template_theme: HashMap<String, String> = HashMap::new();

    for (name, objective_color) in OBJECTIVE_THEME.iter() {
        let theme_color = theme.get(name).unwrap().clone();
        let theme_distance = distance(theme_color, *objective_color);

        let color = if theme_distance < config.distance_threshold {
            theme_color
        } else {
            gen_color_mix(theme_color, *objective_color, config.mix_factor)
        };

        let bright_color = RgbJson {
            red: (color.red as f64 * config.brighter_factor)
                .min(255.0)
                .max(config.bright_min as f64) as u8,
            green: (color.green as f64 * config.brighter_factor)
                .min(255.0)
                .max(config.bright_min as f64) as u8,
            blue: (color.blue as f64 * config.brighter_factor)
                .min(255.0)
                .max(config.bright_min as f64) as u8,
        };

        theme.insert(format!("{}_bright", name), bright_color);
        theme.insert(name.clone(), color);
    }

    for (name, color) in theme.iter() {
        template_theme.insert(format!("{}_hex", name), color.to_hex());
        template_theme.insert(format!("{}_rgb", name), color.to_string());
    }

    let home = home_dir().expect("Could not find home directory");
    let templates_path = home.join(DEFAULT_TEMPLATES_PATH);
    let cache_path = home.join(DEFAULT_CONFIG_PATH);

    create_dir_all(&templates_path).expect("Could not create templates directory");
    create_dir_all(&cache_path).expect("Could not create cache directory");

    for entry in WalkDir::new(&templates_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let input_path = entry.path();
        let relative_path = input_path.strip_prefix(&templates_path).unwrap();
        let output_path = home.join(CACHE_PATH).join(relative_path);

        if let Some(parent) = output_path.parent() {
            create_dir_all(parent).expect("Could not create parent directories");
        }

        let template_content = fs::read_to_string(input_path).expect("Could not read template");
        let data = json!(template_theme);

        let mut reg = Handlebars::new();
        reg.register_helper("keep", Box::new(MissingHelper));

        let rendered = reg
            .render_template(template_content.as_str(), &data)
            .expect("Could not render template");

        fs::write(output_path, rendered.clone()).expect("Could not write output");

        if config.stdout_template.is_some()
            && input_path.ends_with(config.stdout_template.as_ref().unwrap())
        {
            println!("{}", rendered);
        }
    }

    if config.stdout_template.is_some() {
        return;
    }

    let mut keys: Vec<String> = theme.keys().cloned().collect();
    keys.sort();

    for name in keys.iter() {
        let color = theme.get(name).unwrap();
        println!(
            "{}    {} {}",
            color::Bg(color::Rgb(color.red, color.green, color.blue)),
            style::Reset,
            name
        );
    }
}
