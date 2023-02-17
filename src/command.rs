use crate::error::{CommandParseError, ErrorKind};
use crossterm::style::*;
use crossterm::*;
use futures::io::BufReader;
use futures::{AsyncBufReadExt, StreamExt};
use std::io::stdout;
use std::path::Path;
use std::process::Stdio;

use crate::CommandType;

const BUILTIN_COMMAND_NAME: [&str; 7] = ["cd", "cp", "exit", "help", "rm", "rmdir", "touch"];

pub struct ParsedCommand {
    pub command: String,
    pub subcommand: Vec<String>,

    pub flags: Vec<String>,
}

impl ParsedCommand {
    fn new(command: String, subcommand: Vec<String>, flags: Vec<String>) -> Self {
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
                .args(self.subcommand.as_slice())
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
    pub subcommand: Vec<String>,

    pub flags: Vec<String>,
}

impl BuiltinCommand {
    fn new(command: String, subcommand: Vec<String>, flags: Vec<String>) -> Self {
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
                let p = Path::new(&self.subcommand[0]);
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

            "cp" => {
                if self.subcommand.len() >= 2 {
                    crate::builtin::cp(
                        Path::new(&self.subcommand[0]),
                        Path::new(&self.subcommand[1]),
                    );
                }
            }

            "exit" => {
                std::process::exit(0);
            }

            "help" => {}

            "rm" => {
                for s in &self.subcommand {
                    crate::builtin::rm(Path::new(&s));
                }
            }

            "rmdir" => {
                for s in &self.subcommand {
                    crate::builtin::rmdir(Path::new(&s));
                }
            }

            "touch" => {
                for s in &self.subcommand {
                    crate::builtin::touch(Path::new(&s));
                }
            }

            _ => {}
        }
    }
}

pub fn parse_command(original: &str) -> std::result::Result<CommandType, CommandParseError> {
    let mut subcommand: Vec<String> = vec![];
    let mut flags: Vec<String> = vec![];

    let mut divided: Vec<&str> = original.split_ascii_whitespace().collect();

    let len = divided.len();

    let command: String = if len == 0 {
        return Err(CommandParseError::simple(ErrorKind::Null));
    } else {
        divided[0].parse().unwrap()
    };
    divided.remove(0);

    for i in divided {
        if i.starts_with('-') {
            flags.push(i.parse().unwrap());
        } else {
            subcommand.push(i.parse().unwrap());
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
