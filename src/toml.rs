use crate::color::ColorScheme;

pub struct Config {
    scheme: ColorScheme
}

impl Config {
    pub fn load() -> Self {
        Self {
            scheme: ColorScheme::default()
        }
    }

    pub fn get_scheme(&self) -> ColorScheme {
        self.scheme
    }
}