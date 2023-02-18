use crossterm::queue;
use crossterm::style::*;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::SystemTime;

pub fn cp(from: &Path, to: &Path) {
    match std::fs::copy(from, to) {
        Ok(_) => {}
        Err(e) => {
            queue!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("Error: "),
                ResetColor,
                Print(e)
            )
            .unwrap();
            stdout().flush().unwrap()
        }
    }
}

pub fn rm(p: &Path) {
    match std::fs::remove_file(p) {
        Ok(_) => {}
        Err(e) => {
            queue!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("Error: "),
                ResetColor,
                Print(e)
            )
            .unwrap();
            stdout().flush().unwrap()
        }
    }
}

pub fn rmdir(p: &Path) {
    match std::fs::remove_dir(p) {
        Ok(_) => {}
        Err(e) => {
            queue!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("Error: "),
                ResetColor,
                Print(e)
            )
            .unwrap();
            stdout().flush().unwrap()
        }
    }
}

pub fn touch(p: &Path) {
    if p.exists() {
        match std::fs::File::open(p) {
            Ok(f) => match f.set_modified(SystemTime::now()) {
                Ok(_) => {}
                Err(e) => {
                    queue!(
                        stdout(),
                        SetForegroundColor(Color::Red),
                        Print("Error: "),
                        ResetColor,
                        Print(e)
                    )
                    .unwrap();
                    stdout().flush().unwrap()
                }
            },
            Err(e) => {
                queue!(
                    stdout(),
                    SetForegroundColor(Color::Red),
                    Print("Error: "),
                    ResetColor,
                    Print(e)
                )
                .unwrap();
                stdout().flush().unwrap()
            }
        }
    } else {
        match std::fs::File::create(p) {
            Ok(_) => {}
            Err(e) => {
                queue!(
                    stdout(),
                    SetForegroundColor(Color::Red),
                    Print("Error: "),
                    ResetColor,
                    Print(e)
                )
                .unwrap();
                stdout().flush().unwrap()
            }
        }
    }
}
