use crate::error::{CommandParseError, ErrorKind};

pub struct ParsedCommand {
    pub command: String,
    pub subcommand: String,

    pub flags: Vec<String>,
}

impl ParsedCommand {
    fn new(command: String, subcommand: String, flags: Vec<String>) -> Self {
        Self {
            command,
            subcommand,
            flags,
        }
    }

    pub fn run(&self) {}
}

pub fn parse_command(original:&str) -> Result<ParsedCommand,CommandParseError> {
    let command:String;
    let mut subcommand:String = String::from("");
    let mut flags:Vec<String> = vec![];

    let divided:Vec<&str> = original.split_ascii_whitespace().collect();

    let len = divided.len();

    if len == 0 {
        return Err(CommandParseError::simple(ErrorKind::Null));
    }else {
        command = divided[0].parse().unwrap();
    }

    if len >= 2 {
        subcommand = divided[1].parse().unwrap();
    }

    let mut flags_counted = false;
    for i in divided {
        if flags_counted == true {
            flags.push(i.parse().unwrap());
        }else {
            if i.chars().next().unwrap() == '-' {
                flags.push(i.parse().unwrap());
                flags_counted = true;
            }
        }
    }


    Ok(ParsedCommand::new(command,subcommand,flags))
}
