use std::{collections::HashMap, path::Path};

use color_thief::{get_palette, ColorFormat};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

fn distance(color1: RgbJson, color2: RgbJson) -> f64 {
    let r = color1.red as f64 - color2.red as f64;
    let g = color1.green as f64 - color2.green as f64;
    let b = color1.blue as f64 - color2.blue as f64;

    (r * r + g * g + b * b).sqrt()
}

fn interpolate(start: u8, end: u8, factor: f64) -> u8 {
    (start as f64 + (end as f64 - start as f64) * factor) as u8
}

fn gen_color_mix(color1: RgbJson, color2: RgbJson, factor: f64) -> RgbJson {
    RgbJson {
        red: interpolate(color1.red, color2.red, factor),
        green: interpolate(color1.green, color2.green, factor),
        blue: interpolate(color1.blue, color2.blue, factor),
    }
}

type Theme = HashMap<&'static str, RgbJson>;

lazy_static! {
    static ref OBJECTIVE_THEME: Theme = {
        let mut theme: Theme = HashMap::new();

        theme.insert(
            "black",
            RgbJson {
                red: 0,
                green: 0,
                blue: 0,
            },
        );
        theme.insert(
            "red",
            RgbJson {
                red: 255,
                green: 0,
                blue: 0,
            },
        );
        theme.insert(
            "green",
            RgbJson {
                red: 0,
                green: 255,
                blue: 0,
            },
        );
        theme.insert(
            "blue",
            RgbJson {
                red: 0,
                green: 0,
                blue: 255,
            },
        );
        theme.insert(
            "yellow",
            RgbJson {
                red: 255,
                green: 255,
                blue: 0,
            },
        );
        theme.insert(
            "magenta",
            RgbJson {
                red: 255,
                green: 0,
                blue: 255,
            },
        );
        theme.insert(
            "cyan",
            RgbJson {
                red: 0,
                green: 255,
                blue: 255,
            },
        );
        theme.insert(
            "white",
            RgbJson {
                red: 255,
                green: 255,
                blue: 255,
            },
        );

        theme
    };
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct RgbJson {
    red: u8,
    green: u8,
    blue: u8,
}

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

    let palette = get_palette(pixels, ColorFormat::Rgb, 1, 255);

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
                theme.insert(name, current_color);
            }
        }
    }

    for (name, objective_color) in OBJECTIVE_THEME.iter() {
        let theme_color = theme.get(name).unwrap();
        let theme_distance = distance(*theme_color, *objective_color);

        if theme_distance < 150.0 {
            continue;
        }

        let mix = gen_color_mix(*theme_color, *objective_color, 0.2);
        theme.insert(name, mix);
    }
}
