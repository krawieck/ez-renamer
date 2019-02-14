#[macro_use]
extern crate clap;
extern crate regex;

mod args_parser;
mod init;

use clap::{App, Arg};
use regex::Regex;
use std::{env, fs, io, process};

fn main() {
    let matches: clap::ArgMatches<'_> = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("file_match")
                .help("regular expression for files to rename")
                .index(1)
                .required(false),
        )
        .args(&[
            Arg::with_name("fix_spaces")
                .help("replaces whatever's given to space")
                .short("s")
                .long("fixspaces")
                .takes_value(true),
            Arg::with_name("remove_tags")
                .help(
                    "remove tags, they're usually inside [] or (). \
                     Syntax for this argument should be '<opening bracket><closing \
                     bracket> <repeat>', example: '{} () []' i don't give a fuck if \
                     u didn't format it correctly. i'm not gonna do a lot of error \
                     checking here",
                )
                .short("t")
                .long("rmtags")
                .takes_value(true),
            Arg::with_name("verbose")
                .help("spits out more output")
                .short("v")
                .long("verbose")
                .takes_value(false),
            Arg::with_name("directory")
                .help("where should I rename files")
                .long("dir")
                .takes_value(true),
            Arg::with_name("include_ext")
                .help("include extension in renaming")
                .short("e")
                .long("include-ext")
                .takes_value(true),
            Arg::with_name("dont_cleanup_spaces")
                .help(
                    "by default ez-renamer removes multiple spaces (cleans up) \
                     after it's done. This flag stops him from doing that",
                )
                .long("dont-cleanup"), // TODO: add short?
            Arg::with_name("trim_right_from")
                .help(
                    "Trim from the given sequence to the right
example:
ezr --trim-right-from [1080p]
\"Mind Field S03E02 [1080p] [x265] [YIFY].mkv\" -> \"Mind Field S03E02 [1080p].mkv\"",
                )
                .long("trim-right-from")
                .takes_value(true),
            Arg::with_name("trim_left_from")
                .help(
                    "Trim with the given sequence to the left.
example:
ezr --trim-left-with mind
\"[HorribleSubs] Mind Field S03E02.mkv\" -> \"Mind Field S03E02.mkv\"",
                )
                .long("trim-left-from")
                .takes_value(true),
            Arg::with_name("trim_right_with")
                .help(
                    "Trim with the given sequence to the right
example:
ezr --trim-right-with [1080p]
\"Mind Field S03E02 [1080p] [x265] [YIFY].mkv\" -> \"Mind Field S03E02 .mkv\"",
                )
                .long("trim-right-with")
                .takes_value(true),
            Arg::with_name("trim_left_with")
                .help(
                    "Trim with the given sequence to the left.
example:
ezr --trim-left-with ubs]
\"[HorribleSubs] Mind Field S03E02.mkv\" -> \"Mind Field S03E02.mkv\"",
                )
                .long("trim-left-with")
                .takes_value(true),
            Arg::with_name("delete")
                .help("deletes the anything containing given phrase")
                .short("d")
                .long("delete")
                .takes_value(true),
        ])
        .get_matches();
    let args = args_parser::Args::from(matches);
    let verbose = args.verbose;
    if verbose {
        println!("args: {:?}", env::args());
        println!("matches: {:?}", args);
    }
    let dir: fs::ReadDir = init::initialize(&args.directory, verbose);
    let mut names: Vec<(std::path::PathBuf, std::path::PathBuf)> = vec![];

    let reg = Regex::new(&args.file_match).expect("file match ain't valid");

    // GO OVER DIRECTORY AND MAKE CHANGES
    for file in dir {
        let (path, filename, extension, original_name) =
            match process_dir_entry(&file, args.include_ext, &reg, verbose) {
                Ok(e) => e,
                Err(_) => continue,
            };
        if verbose {
            println!(
                "path: {}, filename: {}, extension: {}",
                path, filename, extension
            );
        }

        let filename = remove_inside_brackets(&filename, &args.remove_tags);
        let filename = fix_spaces(filename, &args.fix_spaces);
        let filename = delete(&filename, &args.delete);

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

        names.push((original_name, final_name));
    }

    // LIST CHANGES AND ASK IF USER THAY WANT TO PROCEED
    for (from, to) in &names {
        println!(
            "{} -> {}",
            from.as_path().to_str().unwrap(),
            to.as_path().to_str().unwrap()
        );
    }

    print!("Should I proceed? [Y/n] ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let should_i_proceed = helpers::get_input();
    if !(should_i_proceed == "" || should_i_proceed.starts_with("y")) {
        println!("exiting...");
        process::exit(1)
    }

    // RENAMING
    println!("Renaming... ");
    for (a, b) in names {
        if verbose {
            print!("{} -> {}", a.to_str().unwrap(), b.to_str().unwrap());
        }
        match fs::rename(a, b) {
            Ok(()) => {
                if verbose {
                    println!()
                }
            }
            Err(err) => println!("Error while renaming: {}", err),
        }
    }
    println!("Done!")
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
    entry: &std::result::Result<std::fs::DirEntry, std::io::Error>,
    include_ext: bool,
    file_match: &Regex,
    verbose: bool,
) -> Result<(String, String, String, std::path::PathBuf), ()> {
    use std::path::PathBuf;
    if verbose {
        print!("processing {:?}... ", &entry);
    }

    let entry: PathBuf = match entry {
        Ok(entry) => {
            match entry.file_type() {
                Ok(file_type) => {
                    if !file_type.is_file() {
                        if verbose {
                            println!("skipped");
                        }
                        return Err(());
                    }
                }
                Err(_) => return Err(()),
            }
            if !file_match.is_match(entry.file_name().to_str().unwrap()) {
                return Err(());
            }

            entry.path()
        }
        Err(err) => {
            eprintln!("Error happened when reading dir: \"{}\"", err);
            return Err(());
        }
    };

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
    return Ok(result);
}

fn remove_inside_brackets(input: &String, brackets: &String) -> String {
    use regex::Regex;

    let mut output = input.clone();
    for s in brackets.split_whitespace() {
        output = {
            if s.len() != 2 {
                eprintln!("Error: Brackets are not formatted correctly");
                process::exit(1)
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
