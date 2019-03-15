extern crate exitcode;
extern crate log;
extern crate pretty_env_logger;
extern crate regex;
extern crate structopt;

mod args_parser;
mod init;
mod trim;

use args_parser::Args;
use log::{error, info};
use std::{env, fs, io, process};
use structopt::StructOpt;

fn main() {
    use std::path::PathBuf;

    pretty_env_logger::init();
    // pretty_env_logger::formatted_builder();
    let args = Args::from_args();
    // let verbose = args.verbose;
    info!("args: {:#?}", env::args());
    info!("matches: {:#?}", args);
    let dir_content = init::initialize(&args);
    let mut names: Vec<(PathBuf, PathBuf)> = vec![];
    // GO OVER DIRECTORY AND MAKE CHANGES
    for file in dir_content {
        match process_names(file.path(), &args) {
            Ok(f) => names.push(f),
            Err(_) => {}
        }
    }

    // LIST CHANGES AND ASK IF USER THAY WANT TO PROCEED
    if args.quiet < 1 {
        for (from, to) in &names {
            println!(
                "{} -> {}",
                from.as_path().to_str().unwrap(),
                to.as_path().to_str().unwrap()
            );
        }
    }

    // Ask user if they wanna proceed
    if args.quiet < 2 {
        print!("Should I proceed? [Y/n] ");
    }
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let should_i_proceed = helpers::get_input();
    if !(should_i_proceed == "" || should_i_proceed.starts_with("y")) {
        println!("exiting...");
        process::exit(exitcode::OK)
    }

    // RENAMING
    println!("Renaming... ");
    for (a, b) in names {
        info!("{} -> {}", a.to_str().unwrap(), b.to_str().unwrap());
        match fs::rename(a, b) {
            Ok(()) => {
                info!("Ok");
            }
            Err(err) => {
                eprintln!("Error while renaming: {}", err);
                error!("{}", err);
            }
        }
    }
    println!("Done!")
}

fn process_names(
    file: std::path::PathBuf,
    args: &Args,
) -> Result<(std::path::PathBuf, std::path::PathBuf), ()> {
    let (path, filename, extension, original_name) = process_dir_entry(file, args.include_ext);
    info!(
        "path: {}, filename: {}, extension: {}",
        path, filename, extension
    );

    let filename = remove_inside_brackets(&filename, &args.remove_tags);
    let filename = fix_spaces(filename, &args.fix_spaces);
    let filename = delete(&filename, &args.delete);
    let filename = trim::trim(&filename, &args);

    let filename = if !args.dont_cleanup {
        cleanup_spaces(&filename)
    } else {
        filename
    };

    let mut final_name = std::path::PathBuf::from(path);
    final_name.push(filename); // TODO: BUT DOES It WORK ON WINDOWS

    if !args.include_ext {
        final_name.set_extension(extension);
    }

    info!("------");
    return Ok((original_name, final_name));
}

fn cleanup_spaces(input: &str) -> String {
    let mut prev_was_space = false;
    let mut output = String::new();
    for x in input.chars() {
        if x == ' ' {
            if prev_was_space {
                continue;
            } else {
                output.push(x);
                prev_was_space = true;
            }
        } else {
            prev_was_space = false;
            output.push(x);
        }
    }
    output.trim().to_owned()
}

fn fix_spaces(input: String, replacer: &str) -> String {
    let mut output: String = input.clone();
    for x in replacer.chars() {
        output = output.replace(x, " ");
    }
    output
}

fn process_dir_entry(
    entry: std::path::PathBuf,
    include_ext: bool,
) -> (String, String, String, std::path::PathBuf) {
    info!("processing {:?}... ", &entry);

    let result = (
        String::from(entry.parent().unwrap().to_str().unwrap()),
        if include_ext {
            String::from(entry.file_name().unwrap().to_str().unwrap())
        } else {
            String::from(entry.file_stem().unwrap().to_str().unwrap())
        },
        String::from(entry.extension().unwrap_or_default().to_str().unwrap()),
        entry,
    );
    return result;
}

fn remove_inside_brackets(input: &String, brackets: &String) -> String {
    use exitcode;
    use regex::Regex;

    let mut output = input.clone();
    for s in brackets.split_whitespace() {
        output = {
            if s.len() != 2 {
                eprintln!("Error: Brackets are not formatted correctly");
                process::exit(exitcode::CONFIG);
            }
            let s: Vec<char> = s.chars().collect();
            let beg: char = s[0];
            let end: char = s[1];
            let beg = regex::escape(&beg.to_string());
            let end = regex::escape(&end.to_string());

            let mut reg_str = String::from(beg);
            reg_str.push_str(".*?");
            reg_str.push_str(&end);
            let reg = Regex::new(&reg_str).expect("Dev messed sth up with removing brackets");
            reg.replace_all(&output, "").to_string()
        };
    }
    output
}

fn delete(input: &str, to_be_deleted: &str) -> String {
    input.replace(to_be_deleted, "")
}

mod helpers {
    use std::io;
    pub fn get_input() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remove_inside_brackets() {
        let mock = String::from("black_mirror_bandersnatch_[720p]_(x264)");
        let mock = super::remove_inside_brackets(&mock, &String::from("[] ()"));
        assert_eq!(mock, String::from("black_mirror_bandersnatch__"));
    }

    #[test]
    fn test_fix_spaces() {
        let mock_input = String::from("black_mirror_bandersnatch.[720p].(x264)");
        let mock_replacer = "._";
        assert_eq!(
            super::fix_spaces(mock_input, mock_replacer),
            String::from("black mirror bandersnatch [720p] (x264)")
        );
    }

    #[test]
    fn test_cleanup_spaces() {
        assert_eq!(
            super::cleanup_spaces("   black   mirror   bandersnatch  "),
            String::from("black mirror bandersnatch")
        );
    }

    #[test]
    fn test_delete() {
        assert_eq!(
            super::delete(
                "Black Mirror Bandersnatch [x265] [1080p] [EpicRelease]",
                "[x265] [1080p] [EpicRelease]"
            ),
            String::from("Black Mirror Bandersnatch ")
        )
    }
}
