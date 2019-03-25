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
use std::{env, fs, process};
use structopt::StructOpt;

fn main() {
    use std::path::PathBuf;
    pretty_env_logger::init(); // RUST_LOG=ezr before command to enable logging
    let args = Args::from_args();
    info!("args: {:#?}", env::args());
    info!("matches: {:#?}", args);

    // GO OVER DIRECTORY AND MAKE CHANGES
    let names = init::initialize(&args)
        .iter()
        .map(|a| process_names(a.path(), &args))
        .filter_map(|a| a.ok())
        .collect::<Vec<(PathBuf, PathBuf)>>();

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

    if !args.yes {
        use std::io;
        // Ask user if they wanna proceed
        if args.quiet < 2 {
            print!("Should I proceed? [Y/n] ");
        }
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let proceed = {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_goes_into_input_above) => {}
                Err(_no_updates_is_fine) => {}
            }
            input.trim().chars().next().unwrap_or('y')
        };
        if !(proceed == 'Y' || proceed == 'y') {
            println!("exiting...");
            process::exit(exitcode::OK)
        }
    }

    // RENAMING

    if args.quiet < 1 {
        println!("Renaming... ");
    }
    for (a, b) in names {
        info!("{} -> {}", a.to_str().unwrap(), b.to_str().unwrap());
        match fs::rename(a, b) {
            Ok(()) => {
                info!("Ok");
            }
            Err(err) => {
                if args.quiet < 2 {
                    eprintln!("Error while renaming: {}", err);
                }
                error!("{}", err);
            }
        }
    }
    if args.quiet < 1 {
        println!("Done!")
    }
}

fn process_names(
    entry: std::path::PathBuf,
    args: &Args,
) -> Result<(std::path::PathBuf, std::path::PathBuf), ()> {
    use trim::trim;
    let (path, filename, extension) = {
        (
            String::from(entry.parent().unwrap().to_str().unwrap()),
            if args.include_ext {
                String::from(entry.file_name().unwrap().to_str().unwrap())
            } else {
                String::from(entry.file_stem().unwrap().to_str().unwrap())
            },
            String::from(entry.extension().unwrap_or_default().to_str().unwrap()),
        )
    };

    info!(
        "path: {}, filename: {}, extension: {}",
        path, filename, extension
    );

    let filename = remove_inside_brackets(&filename, &args.remove_tags);
    let filename = fix_spaces(&filename, &args.fix_spaces);
    let filename = delete(&filename, &args.delete);
    let filename = trim(&filename, &args);

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
    Ok((entry, final_name))
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

/// Replaces dots or underscores or any other character with spaces
///
/// example use:
/// `fix_spaces("Love_Death_and_Robots_S01E14.Zima.Blue", "_.") -> "Love Death Robots S01E14 Zima Blue"`b
///
fn fix_spaces(input: &str, replacer: &str) -> String {
    let mut output: String = input.to_string();
    for x in replacer.chars() {
        output = output.replace(x, " ");
    }
    output
}

fn remove_inside_brackets(input: &str, brackets: &String) -> String {
    use exitcode;
    use regex::Regex;

    let mut output = input.to_owned();
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
    output.to_owned()
}

/// deletes some phrase
///
/// example:
/// `delete("LDR S01E2 720p x265-PSA", "720p x265-PSA") -> "LDR S01E2 "`
fn delete(input: &str, to_be_deleted: &str) -> String {
    input.replace(to_be_deleted, "")
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
            super::fix_spaces(&mock_input, mock_replacer),
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
