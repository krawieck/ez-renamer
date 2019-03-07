use exitcode;

pub fn initialize(dir: &std::path::PathBuf) -> std::fs::ReadDir {
    use std::{fs, process};

    match fs::read_dir(dir) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(error.raw_os_error().unwrap_or(exitcode::IOERR));
        }
    }
}
