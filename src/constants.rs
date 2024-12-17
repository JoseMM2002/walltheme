use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{RgbJson, Theme};

lazy_static! {
    pub static ref OBJECTIVE_THEME: Theme = {
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
