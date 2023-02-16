use crossterm::style::Color;

#[derive(Clone,Copy)]
pub struct ColorScheme {
    command: Color,
    sub_command: Color,
    string: Color,
    flags: Color
}

impl ColorScheme {
    pub fn new(command:Color,sub_command:Color,string:Color,flags:Color) -> Self {
        Self {
            command,
            sub_command,
            string,
            flags
        }
    }
    pub fn command(&self) -> Color {
        self.command
    }
    pub fn sub_command(&self) -> Color {
        self.sub_command
    }
    pub fn string(&self) -> Color {
        self.string
    }
    pub fn flags(&self) -> Color {
        self.flags
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            command: Color::Yellow,
            sub_command: Color::White,
            string: Color::Green,
            flags: Color::DarkGrey
        }
    }
}