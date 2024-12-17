use std::{collections::HashMap, path::Path};

use color_thief::{get_palette, ColorFormat};
use termion::{color, style};
use walltheme::constants::OBJECTIVE_THEME;
use walltheme::{distance, gen_color_mix, RgbJson, Theme};

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

    for (name, color) in theme.iter() {
        println!(
            "{}{}{}",
            color::Bg(color::Rgb(color.red, color.green, color.blue)),
            name,
            style::Reset
        );
    }
}
