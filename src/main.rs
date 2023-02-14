//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

mod command;
mod error;

use std::io::Write;
use std::{io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::event::{read, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::style::*;
use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

async fn shell_loop() {
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
                    } => match code {
                        KeyCode::Backspace => {}
                        KeyCode::Enter => {}
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
                            match_char(c, modifiers, kind, state);
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
                    },
                },

                Event::Mouse(_) => {}

                Event::Paste(_) => {}

                Event::Resize(_, _) => {}
            }
        }

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

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    shell_loop().await;

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}

fn match_char(c: char, modifier: KeyModifiers, kind: KeyEventKind, state: KeyEventState) {
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
            queue!(stdout(), Print(c)).unwrap();
        }
    }
}

fn trans_async() {}

fn flush() {
    stdout().flush().unwrap();
}

mod test {
    #[test]
    fn parse_command() {
        let command = crate::command::parse_command("cmake build --build build").unwrap();
        println!("{} {} {:?}",command.command,command.subcommand,command.flags);
    }
}
