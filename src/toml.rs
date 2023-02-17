use crate::color::ColorScheme;
use crossterm::style::Color;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct UD {
    pub ColorScheme: Option<UDColorScheme>,
}

#[derive(Debug, Deserialize)]
pub struct UDColorScheme {
    command: Option<String>,
    sub_command: Option<String>,
    string: Option<String>,
    flags: Option<String>,
}

pub struct Config {
    ColorScheme: ColorScheme,
}

impl Config {
    pub fn load() -> Result<Self, ()> {
        let file_path = Path::new("./xcys.toml");
        let contents = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => {
                return Err(());
            }
        };
        let decoded: UD = toml::from_str(&contents).unwrap();

        // Initialize ColorScheme
        let scheme = match decoded.ColorScheme {
            None => {
                println!("Use default");
                ColorScheme::default()
            }
            Some(s) => {
                let command = match s.command {
                    None => Color::Yellow,
                    Some(c) => get_color_from_name(&c),
                };
                let sub_command = match s.sub_command {
                    None => Color::White,
                    Some(c) => get_color_from_name(&c),
                };
                let string = match s.string {
                    None => Color::Green,
                    Some(c) => get_color_from_name(&c),
                };
                let flags = match s.flags {
                    None => Color::DarkGrey,
                    Some(c) => get_color_from_name(&c),
                };

                ColorScheme::new(command, sub_command, string, flags)
            }
        };

        Ok(Self {
            ColorScheme: scheme,
        })
    }

    pub fn get_scheme(&self) -> ColorScheme {
        self.ColorScheme
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ColorScheme: ColorScheme::default(),
        }
    }
}

fn get_color_from_name(color: &str) -> Color {
    match color.to_ascii_lowercase().as_str() {
        "white" => Color::White,
        "black" => Color::Black,
        "grey" => Color::Grey,
        "red" => Color::Red,
        "darkred" => Color::DarkRed,
        "green" => Color::Green,
        "darkgreen" => Color::DarkGreen,
        "blue" => Color::Blue,
        "darkblue" => Color::DarkBlue,
        "darkgrey" => Color::DarkGrey,
        _ => Color::White,
    }
}
