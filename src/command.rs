use std::path::Path;
use crate::error::{CommandParseError, ErrorKind};
use futures::io::BufReader;
use futures::{AsyncBufReadExt, StreamExt};
use std::process::Stdio;
use crate::CommandType;

const BUILTIN_COMMAND_NAME:[&str;2] = ["cd","exit"];

pub struct ParsedCommand {
    pub command: String,
    pub subcommand: String,

    pub flags: Vec<String>,
}

impl ParsedCommand {
    fn new(command: String, subcommand: String, flags: Vec<String>) -> Self {
        println!("{:?}",flags);

        Self {
            command,
            subcommand,
            flags,
        }
    }

    #[tokio::main]
    pub async fn run(&self) {
        use async_process::Command;
        let mut child = if !self.subcommand.is_empty() {
            Command::new(&self.command)
                .args(self.flags.as_slice())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        }else {
            Command::new(&self.command)
                .arg(&self.subcommand)
                .args(self.flags.as_slice())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        };

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
        match self.command.as_str() {
            "cd" => {
                println!("{} {}",&self.command, &self.subcommand);
                let p = Path::new(&self.subcommand);
                std::env::set_current_dir(p).unwrap();
                match p.exists() {
                    true => {

                    }
                    false => {}
                }
            },

            "exit" => {
                std::process::exit(0);
            }
            c => {

            }
        }
    }
}


pub fn parse_command(original: &str) -> Result<CommandType, CommandParseError> {
    let command: String;
    let mut subcommand: String = String::new();
    let mut flags: Vec<String> = vec![];

    let divided: Vec<&str> = original.split_ascii_whitespace().collect();

    let len = divided.len();

    if len == 0 {
        return Err(CommandParseError::simple(ErrorKind::Null));
    } else {
        command = divided[0].parse().unwrap();
    }

    if len >= 2 {
        if !subcommand.starts_with('-') {
            subcommand = divided[1].parse().unwrap();
        }
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


    for i in BUILTIN_COMMAND_NAME {
        if command.to_ascii_lowercase() == i {
            return Ok(CommandType::Builtin(BuiltinCommand::new(command, subcommand, flags)))
        }
    }

    Ok(CommandType::Executable(ParsedCommand::new(command, subcommand, flags)))
}
