use std::path::Path;
use std::time::SystemTime;

pub fn cp(from: &Path, to: &Path) {
    match std::fs::copy(from, to) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn rm(p: &Path) {
    match std::fs::remove_file(p) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn rmdir(p: &Path) {
    match std::fs::remove_dir(p) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn touch(p: &Path) {
    if p.exists() {
        match std::fs::File::open(p) {
            Ok(f) => match f.set_modified(SystemTime::now()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        match std::fs::File::create(p) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}
