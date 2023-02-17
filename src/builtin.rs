

use std::path::Path;
use std::time::SystemTime;

pub fn rm(p: &Path) {
    if p.exists() && p.is_file() {
        match std::fs::remove_file(p) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

pub fn touch(p: &Path) {
    if p.exists() {
        match std::fs::File::open(p) {
            Ok(f) => {
                f.set_modified(SystemTime::now());
            }
            Err(e) => {
                eprintln!("{}",e);
            }
        }
    }else {
        match std::fs::File::create(p) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}",e)
            }
        }
    }
}