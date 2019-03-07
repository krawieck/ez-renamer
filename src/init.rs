pub fn initialize(dir: &std::path::PathBuf) -> std::fs::ReadDir {
    use std::{fs, process};

    match fs::read_dir(dir) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("coudn't read files in current dir\n{}", error);
            process::exit(1)
        }
    }
}
