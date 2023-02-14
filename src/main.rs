//! Demonstrates how to read events asynchronously with async-std.
//!
//! cargo run --features="event-stream" --example event-stream-async-std

use std::{io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

const HELP: &str = r#"EventStream based on futures_util::stream::Stream with async-std
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

async fn print_events() {
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        match event {
                            Event::FocusGained => {

                            }

                            Event::FocusLost => {

                            }

                            Event::Key(k) => {
                                match k {
                                    _ => {
                                        
                                    }
                                }
                            }

                            Event::Mouse(_) => {

                            }

                            Event::Paste(_) => {

                            }

                            Event::Resize(_,_) => {

                            }
                        }

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    async_std::task::block_on(print_events());

    execute!(stdout, DisableMouseCapture)
}