use crate::error::{CommandParseError, ErrorKind};
use crossterm::style::*;
use crossterm::*;
use futures::io::BufReader;
use futures::{AsyncBufReadExt, StreamExt};
use std::io::stdout;
use std::path::Path;
use std::process::Stdio;

use crate::CommandType;

const BUILTIN_COMMAND_NAME: [&str; 2] = ["cd", "exit"];

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
    pub async fn run(&self) -> std::result::Result<(), std::io::Error> {
        use async_process::Command;
        let r_child = if self.subcommand.is_empty() && !self.flags.is_empty() {
            Command::new(&self.command)
                .args(self.flags.as_slice())
                .stdout(Stdio::piped())
                .spawn()
        } else {
            Command::new(&self.command)
                .arg(&self.subcommand)
                .args(self.flags.as_slice())
                .stdout(Stdio::piped())
                .spawn()
        };

        let mut child = match r_child {
            Ok(c) => c,
            Err(e) => {
                return Err(e);
            }
        };

        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

        while let Some(line) = lines.next().await {
            println!("{}", line.unwrap());
        }

        Ok(())
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
                let p = Path::new(&self.subcommand);
                match std::env::set_current_dir(p) {
                    Ok(_) => {}
                    Err(e) => {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Red),
                            Print("Error: "),
                            ResetColor,
                            Print(e)
                        )
                        .unwrap();
                    }
                }
            }

            "exit" => {
                std::process::exit(0);
            }
            c => {}
        }
    }
}

pub fn parse_command(original: &str) -> std::result::Result<CommandType, CommandParseError> {
    let mut subcommand: String = String::new();
    let mut flags: Vec<String> = vec![];

    let divided: Vec<&str> = original.split_ascii_whitespace().collect();

    let len = divided.len();

    let command: String = if len == 0 {
        return Err(CommandParseError::simple(ErrorKind::Null));
    } else {
        divided[0].parse().unwrap()
    };

    if len >= 2 {
        if !divided[1].starts_with('-') {
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
            return Ok(CommandType::Builtin(BuiltinCommand::new(
                command, subcommand, flags,
            )));
        }
    }

    Ok(CommandType::Executable(ParsedCommand::new(
        command, subcommand, flags,
    )))
}
