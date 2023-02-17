#![feature(file_set_times)]

mod builtin;
mod color;
mod command;
mod error;
mod toml;

use std::io::stdout;
use std::io::Write;
use std::path::Path;

use crate::color::ColorScheme;
use crate::command::{parse_command, BuiltinCommand, ParsedCommand};
use crate::toml::Config;
use crossterm::event::{read, KeyEventKind, KeyModifiers};
use crossterm::style::*;
use crossterm::terminal::*;
use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use octocrab::models::repos::Tag;
use octocrab::Page;

pub enum CommandType {
    Executable(ParsedCommand),
    Builtin(BuiltinCommand),
}

fn shell_loop(scheme: ColorScheme) {
    loop {
        let path = to_suitable_style(std::env::current_dir().unwrap().to_str().unwrap());
        execute!(
            stdout(),
            crossterm::terminal::SetTitle(&path),
            SetForegroundColor(Color::Green),
            Print(whoami::username()),
            Print("@"),
            Print(whoami::devicename()),
            SetForegroundColor(Color::Yellow),
            Print(" "),
            Print(path),
            ResetColor
        )
        .unwrap();

        if unsafe { GIT_ENABLED } {
            match get_git_branch_name() {
                Ok(b) => {
                    execute!(
                        stdout(),
                        Print(" "),
                        SetForegroundColor(Color::Cyan),
                        Print("("),
                        Print(to_suitable_style(
                            std::str::from_utf8(b.as_slice()).unwrap()
                        )),
                        Print(")"),
                        ResetColor
                    )
                    .unwrap();
                }
                Err(_) => {}
            }
        }
        execute!(stdout(), Print("\n")).unwrap();
        execute!(stdout(), Print("$ ")).unwrap();
        let mut input = String::from("");
        let mut screen_size = crossterm::terminal::size().unwrap();
        let y = crossterm::cursor::position().unwrap().1;
        'input: loop {
            let event = read().unwrap();

            match event {
                Event::FocusGained => {}

                Event::FocusLost => {}

                Event::Key(key) => match key {
                    KeyEvent {
                        code,
                        modifiers,
                        kind,
                        state,
                    } => {
                        if kind == KeyEventKind::Release {
                            match code {
                                KeyCode::Backspace => {
                                    if !input.is_empty() {
                                        input.remove(input.len() - 1);

                                        // Move
                                        let pos = crossterm::cursor::position().unwrap();
                                        if pos.0 == 0 && pos.1 != y {
                                            execute!(
                                                stdout(),
                                                crossterm::cursor::MoveTo(screen_size.0, pos.1 - 1)
                                            )
                                            .unwrap();
                                            continue;
                                        } else if pos.0 == 2 && pos.1 == y {
                                            continue;
                                        }
                                        execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();

                                        execute!(
                                            stdout(),
                                            crossterm::terminal::Clear(
                                                crossterm::terminal::ClearType::FromCursorDown
                                            )
                                        )
                                        .unwrap();
                                    }
                                }
                                KeyCode::Enter => match kind {
                                    KeyEventKind::Press => {}
                                    KeyEventKind::Repeat => {}
                                    KeyEventKind::Release => {
                                        println!();
                                        match parse_command(&input) {
                                            Ok(c) => match c {
                                                CommandType::Executable(e) => match e.run() {
                                                    Ok(_) => {}
                                                    Err(e) => execute!(
                                                        stdout(),
                                                        SetForegroundColor(Color::Red),
                                                        Print("Error: "),
                                                        ResetColor,
                                                        Print(e)
                                                    )
                                                    .unwrap(),
                                                },
                                                CommandType::Builtin(b) => {
                                                    b.run();
                                                }
                                            },
                                            Err(_) => break 'input,
                                        };
                                        input.clear();
                                        break 'input;
                                    }
                                },
                                KeyCode::Left => {}
                                KeyCode::Right => {}
                                KeyCode::Up => {}
                                KeyCode::Down => {}
                                KeyCode::Home => {}
                                KeyCode::End => {}
                                KeyCode::PageUp => {}
                                KeyCode::PageDown => {}
                                KeyCode::Tab => {}
                                KeyCode::BackTab => {}
                                KeyCode::Delete => {}
                                KeyCode::Insert => {
                                    break;
                                }
                                KeyCode::F(_) => {}
                                KeyCode::Char(c) => {
                                    {
                                        // Case insensitivity
                                        let c = c.to_ascii_lowercase();
                                        if modifiers == KeyModifiers::CONTROL {
                                            match c {
                                                'c' => {
                                                    std::process::exit(0);
                                                }

                                                _ => {}
                                            }
                                        }
                                    }

                                    match kind {
                                        KeyEventKind::Press => {}
                                        KeyEventKind::Repeat => {}
                                        KeyEventKind::Release => {
                                            input.push(c);
                                            highlight(&mut input, scheme);
                                        }
                                    }
                                    flush();
                                }
                                KeyCode::Null => {}
                                KeyCode::Esc => {}
                                KeyCode::CapsLock => {}
                                KeyCode::ScrollLock => {}
                                KeyCode::NumLock => {}
                                KeyCode::PrintScreen => {}
                                KeyCode::Menu => {}
                                KeyCode::KeypadBegin => {}
                                KeyCode::Pause => {}
                                KeyCode::Media(_) => {}
                                KeyCode::Modifier(_) => {}
                            }
                        }
                    }
                },

                Event::Mouse(_) => {}

                Event::Paste(_) => {}

                Event::Resize(_, _) => {}
            }
        }
        println!();
    }
}

pub static mut GIT_ENABLED: bool = false;

fn main() -> Result<()> {
    execute!(stdout(), crossterm::terminal::SetTitle("XCYS Shell")).unwrap();

    // Get Latest release name.
    let tag = get_version();
    let version = &tag.items[0].name;

    // Load config
    let config = match Config::load() {
        Ok(c) => c,
        Err(_) => Config::default(),
    };

    // Find out if Git is available. If not available,
    // do not display the branch name (this has the effect of eliminating wasteful processing!)
    if find("git").is_ok() {
        unsafe {
            GIT_ENABLED = true;
        }
    }

    // Run shell
    enable_raw_mode()?;

    execute!(
        stdout(),
        Print("XCYS V"),
        Print(env!("CARGO_PKG_VERSION")),
        Print("\n"),
        SetAttribute(Attribute::RapidBlink),
        Print("The latest source code is available at "),
        SetForegroundColor(Color::DarkBlue),
        SetAttribute(Attribute::Underlined),
        Print("https://github.com/Lattexshz/Xcys\n\n"),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();

    // Notify updates when the current version does not match the latest version
    if version != env!("CARGO_PKG_VERSION") {
        execute!(
            stdout(),
            SetAttribute(Attribute::RapidBlink),
            SetForegroundColor(Color::Yellow),
            Print("New version now available! "),
            SetForegroundColor(Color::DarkBlue),
            SetAttribute(Attribute::Underlined),
            Print("https://github.com/Lattexshz/Xcys/releases/"),
            Print(version),
            Print("\n\n"),
            SetAttribute(Attribute::Reset)
        )
        .unwrap();
    }

    shell_loop(config.get_scheme());

    disable_raw_mode()
}

#[tokio::main]
// Get latest release from GitHub
async fn get_version() -> Page<Tag> {
    let t = octocrab::instance()
        .repos("Lattexshz", "Xcys")
        .list_tags()
        .send()
        .await;
    t.unwrap()
}

fn to_suitable_style(s: &str) -> String {
    String::from(s).replace(':', "").replace('\\', "/")
}

fn find(program: &str) -> std::result::Result<Vec<u8>, ()> {
    use std::process::Command;
    let mut output = match Command::new("where").args([program]).output() {
        Ok(o) => o,
        Err(_) => {
            return Err(());
        }
    };

    if output.stdout.is_empty() {
        return Err(());
    }

    output.stdout.remove(output.stdout.len() - 1);
    output.stdout.remove(output.stdout.len() - 1);

    Ok(output.stdout)
}

fn get_git_branch_name() -> std::result::Result<Vec<u8>, bool> {
    use std::process::Command;
    let git = Path::new(".git");
    if git.is_dir() && !git.exists() {
        return Err(false);
    }
    if unsafe { crate::GIT_ENABLED } {
        let mut output = match Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "@"])
            .output()
        {
            Ok(o) => o,
            Err(_) => {
                return Err(false);
            }
        };

        if output.stdout.is_empty() {
            return Err(false);
        }

        output.stdout.remove(output.stdout.len() - 1);

        Ok(output.stdout)
    } else {
        Err(false)
    }
}

fn highlight(input: &mut str, scheme: ColorScheme) {
    let vec: Vec<char> = input.chars().collect();
    let pos = crossterm::cursor::position().unwrap();
    use crossterm::cursor::MoveTo;
    queue!(stdout(), MoveTo(2, pos.1), Clear(ClearType::FromCursorDown)).unwrap();

    // 0: Command
    // 1: Sub Command
    // 2: String
    // 3: Flags
    let mut status = 0;

    let mut d_quotation_count = 0;

    for i in vec {
        match i {
            '"' => {
                status = 2;
                d_quotation_count += 1
            }

            '-' => {
                if status != 2 {
                    status = 3;
                }
            }

            ' ' => {
                if status != 2 {
                    status = 1;
                }
            }

            _ => {}
        }

        match status {
            1 => queue!(
                stdout(),
                SetForegroundColor(scheme.sub_command()),
                Print(i),
                ResetColor
            )
            .unwrap(),

            2 => queue!(
                stdout(),
                SetForegroundColor(scheme.string()),
                Print(i),
                ResetColor
            )
            .unwrap(),
            3 => queue!(
                stdout(),
                SetForegroundColor(scheme.flags()),
                Print(i),
                ResetColor
            )
            .unwrap(),
            _ => queue!(
                stdout(),
                SetForegroundColor(scheme.command()),
                Print(i),
                ResetColor
            )
            .unwrap(),
        };
        if d_quotation_count == 2 && status == 2 {
            status = 1;
            d_quotation_count = 0;
        }
    }
}

fn flush() {
    stdout().flush().unwrap();
}

mod test {
    use crate::CommandType;

    #[test]
    fn parse_command() {
        let command = crate::command::parse_command("cargo check").unwrap();

        match command {
            CommandType::Executable(command) => {
                println!(
                    "{} {:?} {:?}",
                    command.command, command.subcommand, command.flags
                );
            }
            CommandType::Builtin(_) => {}
        }
    }
}
