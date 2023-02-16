use crate::color::ColorScheme;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UD {
    pub scheme: UDColorScheme
}

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
    pub fn load() -> Result<Self,()> {

        let contents = match std::fs::read_to_string(file_path) {
            Ok(_) => c,
            Err(_) => Err(())
        }
        let decoded: UD = toml::from_str(&contents).unwrap();


        // Initialize ColorScheme
        let scheme = match decoded.scheme {
            None => ColorScheme::default(),
            Some(s) => {
                let command = s.command {
                    None => Color::Yellow,
                    Some(c) => c
                };
                let sub_command = s.sub_command {
                    None => Color::White,
                    Some(c) => c
                };
                let string = s.string {
                    None => Color::Green,
                    Some(c) => c
                };
                let flags = s.flags {
                    None => Color::DarkGrey,
                    Some(c) => c
                };

                ColorScheme::new(command,sub_command,string,flags)
            }
        }


        Ok(Self {
            scheme
        })
    }

    pub fn get_scheme(&self) -> ColorScheme {
        self.scheme
    }
}