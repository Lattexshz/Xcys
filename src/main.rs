//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

mod command;
mod error;

use std::fmt::Error;
use std::future::Future;
use std::io::stdout;
use std::io::Write;
use std::path::Path;

use crate::command::{parse_command, BuiltinCommand, ParsedCommand};
use crossterm::event::{read, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::style::*;
use crossterm::terminal::*;
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

pub enum CommandType {
    Executable(ParsedCommand),
    Builtin(BuiltinCommand),
}

fn shell_loop() {
    loop {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(whoami::username()),
            Print("@"),
            Print(whoami::devicename()),
            SetForegroundColor(Color::Yellow),
            Print(" "),
            Print(std::env::current_dir().unwrap().to_str().unwrap()),
            ResetColor
        )
        .unwrap();

        if unsafe {GIT_ENABLED} == true {
            execute!(
                stdout(),
                Print(" "),
                SetForegroundColor(Color::Cyan),
                Print("("),
                Print(std::str::from_utf8(get_git_branch_name().unwrap().as_slice()).unwrap()),
                Print(")"),
                ResetColor
            ).unwrap();
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
                                            highlight(&mut input);
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
        println!();

        // let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        // let mut event = reader.next().fuse();
        //
        //     select! {
        //     //_ = delay => { println!(".\r"); },
        //     maybe_event = event => {
        //         match maybe_event {
        //             Some(Ok(event)) => {
        //                 match event {
        //                     Event::FocusGained => {
        //
        //                     }
        //
        //                     Event::FocusLost => {
        //
        //                     }
        //
        //                     Event::Key(key) => {
        //                         match key {
        //                             KeyEvent {code,modifiers,kind,state} => {
        //                                 match code {
        //                                     KeyCode::Backspace => {},
        //                                     KeyCode::Enter => {},
        //                                     KeyCode::Left => {},
        //                                     KeyCode::Right => {},
        //                                     KeyCode::Up => {},
        //                                     KeyCode::Down => {},
        //                                     KeyCode::Home => {},
        //                                     KeyCode::End => {},
        //                                     KeyCode::PageUp => {},
        //                                     KeyCode::PageDown => {},
        //                                     KeyCode::Tab => {},
        //                                     KeyCode::BackTab => {},
        //                                     KeyCode::Delete => {},
        //                                     KeyCode::Insert => {break;},
        //                                     KeyCode::F(_) => {},
        //                                     KeyCode::Char(c) => {
        //                                         match_char(c,modifiers,kind,state);
        //                                     },
        //                                     KeyCode::Null => {},
        //                                     KeyCode::Esc => {},
        //                                     KeyCode::CapsLock => {},
        //                                     KeyCode::ScrollLock => {},
        //                                     KeyCode::NumLock => {},
        //                                     KeyCode::PrintScreen => {},
        //                                     KeyCode::Menu => {},
        //                                     KeyCode::KeypadBegin => {},
        //                                     KeyCode::Pause => {},
        //                                     KeyCode::Media(_) => {},
        //                                     KeyCode::Modifier(_) => {},
        //                                 }
        //                             }
        //                         }
        //                     }
        //
        //                     Event::Mouse(_) => {
        //
        //                     }
        //
        //                     Event::Paste(_) => {
        //
        //                     }
        //
        //                     Event::Resize(_, _) => {
        //
        //                     }
        //                 }
        //             }
        //             Some(Err(e)) => println!("Error: {:?}\r", e),
        //             None => {continue},
        //         }
        //     }
        //}
    }
}

pub static mut GIT_ENABLED:bool = false;

fn main() -> Result<()> {

    if find("git").is_ok() {
        unsafe {
            GIT_ENABLED = true;
        }
    }

    enable_raw_mode()?;

    shell_loop();

    disable_raw_mode()
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


fn highlight(input: &mut str) {
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
                SetForegroundColor(Color::White),
                Print(i),
                ResetColor
            )
            .unwrap(),

            2 => queue!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(i),
                ResetColor
            )
            .unwrap(),
            3 => queue!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print(i),
                ResetColor
            )
            .unwrap(),
            _ => queue!(
                stdout(),
                SetForegroundColor(Color::Yellow),
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
    #[test]
    fn parse_command() {
        let command = crate::command::parse_command("cargo check").unwrap();
        println!(
            "{} {} {:?}",
            command.command, command.subcommand, command.flags
        );
        command.run();
    }
}
