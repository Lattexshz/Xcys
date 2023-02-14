//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

mod command;
mod error;

use std::io::Write;
use std::{io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crate::command::{parse_command, ParsedCommand};
use crate::error::CommandParseError;
use crossterm::event::{read, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::style::*;
use crossterm::terminal::*;
use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

fn shell_loop() {
    let mut reader = EventStream::new();

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
            Print("\n"),
            ResetColor
        )
        .unwrap();
        execute!(stdout(), Print("$ "));
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
                                        let command = match parse_command(&input) {
                                            Ok(c) => c,
                                            Err(_) => break 'input,
                                        };
                                        command.run();
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

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    shell_loop();

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}

fn match_char(
    c: char,
    modifier: KeyModifiers,
    kind: KeyEventKind,
    state: KeyEventState,
    input: &mut String,
) {
    // Change the operation according to the Modifier
    {
        // Case insensitivity
        let c = c.to_ascii_lowercase();
        if modifier == KeyModifiers::CONTROL {
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
            highlight(input);
        }
    }
}

fn highlight(input: &mut String) {
    let vec: Vec<char> = input.chars().collect();
    let pos = crossterm::cursor::position().unwrap();
    use crossterm::cursor::MoveTo;
    queue!(stdout(), MoveTo(2, pos.1), Clear(ClearType::FromCursorDown)).unwrap();

    // 0: Command
    // 1: Sub Command
    // 2: String
    // 3: Flags
    let mut status = 0;

    for i in vec {
        match i {
            '"' => {
                if status == 2 {
                    status = 0;
                } else {
                    status = 2;
                }
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
                ).unwrap(),
            _ => queue!(stdout(), SetForegroundColor(Color::Yellow),Print(i),ResetColor).unwrap(),
        };
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
