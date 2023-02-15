use crate::color::ColorScheme;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UDColorScheme {
    command: Option<String>,
    sub_command: Option<String>,
    string: Option<String>,
    flags: Option<String>
}


pub struct Config {
    scheme: ColorScheme
}

impl Config {
    pub fn load() -> Self {

        let decoded: UDColorScheme = toml::from_str(toml_str).unwrap();

        Self {
            scheme: ColorScheme::default()
        }
    }

    pub fn get_scheme(&self) -> ColorScheme {
        self.scheme
    }
}