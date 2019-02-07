#[macro_use]
extern crate clap;

extern crate regex;
use clap::{App, Arg};
use std::{env, fs, process};
mod init;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let regex = &args[1];
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("file_match")
                .help("regular expression for files to rename")
                .index(1)
                .required(true),
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
                .short("d")
                .long("dir")
                .takes_value(true),
            Arg::with_name("include_ext")
                .help("include extension in renaming")
                .short("e")
                .long("include-ext")
                .takes_value(true),
        ])
        .get_matches();

    let verbose = matches.is_present("verbose");
    let fix_spaces = matches.is_present("fix_spaces");
    let include_ext = matches.is_present("include_ext");
    if verbose {
        println!("args: {:?}", args);
        println!("matches: {:?}", matches);
    }
    let remove_tags = matches.value_of("remove_tags").unwrap_or("");
    if verbose {
        println!("remove tags {}", remove_tags);
    }
    let dir: fs::ReadDir = {
        if matches.is_present("directory") {
            init::initialize(matches.value_of("directory").unwrap(), verbose)
        } else {
            init::initialize(".", verbose)
        }
    };
    // let remove:String = matches.value_of("remove_tags").unwrap_or_default(String::from("").to_string());
    // println!("{:?}", remove_tags);
    let mut names: Vec<(std::path::PathBuf, std::path::PathBuf)> = vec![];

    // GO OVER DIRECTORY AND MAKE CHANGES
    for file in dir {
        let (path, filename, extension, original_name) =
            match process_dir_entry(&file, include_ext, verbose) {
                Ok(e) => e,
                Err(_) => continue,
            };
        if verbose {
            println!(
                "path: {}, filename: {}, extension: {}",
                path, filename, extension
            );
        }

        let filename = remove_inside_brackets(&filename, remove_tags.to_owned());
        // let filename =

        let mut final_name = std::path::PathBuf::from(path);
        final_name.push(filename); // TODO: BUT DOES It WORK ON WINDOWS

        if !include_ext {
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

    use std::io::{self, Write};
    // let writer = std::io::stdout();

    io::stdout().write(b"Should I proceed? [Y/n] ").unwrap();
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

fn process_dir_entry(
    entry: &std::result::Result<std::fs::DirEntry, std::io::Error>,
    include_ext: bool,
    verbose: bool,
) -> Result<(String, String, String, std::path::PathBuf), ()> {
    use std::ffi::OsStr;
    use std::path::PathBuf;
    // let err_msg = "Dev fucked something up with extracting filename and extension from path... \
    //                or the somehow the path is wrong, idk, it's just a pre-written message";
    if verbose {
        print!("processing {:?} ... ", &entry);
        println!("{:?}", entry);
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
            String::from(entry.file_stem().unwrap().to_str().unwrap())
        } else {
            String::from(entry.file_name().unwrap().to_str().unwrap())
        },
        String::from(entry.extension().unwrap_or_default().to_str().unwrap()),
        entry,
    );
    if verbose {
        println!("ok");
    }
    return Ok(result);
}
fn remove_inside_brackets(input: &String, brackets: String) -> String {
    use regex::Regex;

    let mut output = input.clone();
    for s in brackets.split_whitespace() {
        output = {
            if s.len() != 2 {
                panic!("Brackets are not formatted correctly");
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
    // SEEMS LIKE UNTESTABLE FUNCTION
    // #[test]
    // fn test_process_dir_entry() {
    //     let entry = std::fs::DirEntry::from();
    //     let mock_entry: std::fs::DirEntry = (std::path::PathBuf::from(
    //         "/home/user/porn/brazzers_1_[x264]_(1080p)_{2018-05-14}.mkv",
    //     ));

    //     assert_eq!(
    //         super::process_dir_entry(Ok(&mock_entry), true),
    //         (
    //             String::from("/home/user/porn"),
    //             String::from("brazzers_1_[x264]_(1080p)_{2018-05-14}"),
    //             String::from("mkv"),
    //             mock_entry
    //         )
    //     );
    // }
    #[test]
    fn test_remove_inside_brackets() {
        let mock = String::from("black_mirror_bandersnatch_[720p]_(x264)");
        let mock = super::remove_inside_brackets(&mock, String::from("[] ()"));
        assert_eq!(mock, String::from("black_mirror_bandersnatch__"));
    }
}
