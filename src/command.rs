use crate::error::{CommandParseError, ErrorKind};
use futures::io::BufReader;
use futures::{AsyncBufReadExt, StreamExt};
use std::process::Stdio;
use crate::CommandType;

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

    #[tokio::main]
    pub async fn run(&self) {
        use async_process::Command;
        let mut child = Command::new(&self.command)
            .arg(&self.subcommand)
            .args(self.flags.as_slice())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

        while let Some(line) = lines.next().await {
            println!("{}", line.unwrap());
        }
    }
}


pub struct BuiltinCommand {
    pub command: String,
    pub subcommand: String,

    pub flags: Vec<String>,
}

impl BuiltinCommand {
    fn new(command: String, subcommand: String, flags: Vec<String>) -> Self {
        Self {
            command,
            subcommand,
            flags,
        }
    }

    #[tokio::main]
    pub async fn run(&self) {
        use async_process::Command;
        let mut child = Command::new(&self.command)
            .arg(&self.subcommand)
            .args(self.flags.as_slice())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

        while let Some(line) = lines.next().await {
            println!("{}", line.unwrap());
        }
    }
}


pub fn parse_command(original: &str) -> Result<CommandType, CommandParseError> {
    let command: String;
    let mut subcommand: String = String::from("");
    let mut flags: Vec<String> = vec![];

    let divided: Vec<&str> = original.split_ascii_whitespace().collect();

    let len = divided.len();

    if len == 0 {
        return Err(CommandParseError::simple(ErrorKind::Null));
    } else {
        command = divided[0].parse().unwrap();
    }

    if len >= 2 {
        subcommand = divided[1].parse().unwrap();
    }

    let mut flags_counted = false;
    for i in divided {
        if flags_counted {
            flags.push(i.parse().unwrap());
        } else {
            if i.starts_with('-') {
                flags.push(i.parse().unwrap());
                flags_counted = true;
            }
        }
    }

    Ok(CommandType::Executable(ParsedCommand::new(command, subcommand, flags)))
}
