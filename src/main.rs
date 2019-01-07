#[macro_use]
extern crate clap;

extern crate regex;

use clap::{App, Arg};
use std::{env, fs};

fn initialize(verbose: bool) -> std::fs::ReadDir {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Couldn't get the path,\nhere's error message: {}", error);
            std::process::exit(1)
        }
    };

    if verbose {
        println!("current dir: {:?}", &current_dir);
    }

    match fs::read_dir(current_dir) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("coudn't read files in current dir\n{}", error);
            std::process::exit(1)
        }
    }
}

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
                .long("directory")
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

    let dir = initialize(verbose);
    // let remove:String = matches.value_of("remove_tags").unwrap_or_default(String::from("").to_string());
    // println!("{:?}", remove_tags);

    // going over directory
    for file in dir {
        let original_name = match file {
            Ok(path) => path,
            Err(err) => {
                eprintln!("Error happened when reading dir: \"{}\"", err);
                continue;
            }
        };
        let (path, filename, extension) = process_dir_entry(&original_name.path(), include_ext);

        if verbose {
            println!(
                "path: {}, filename: {}, extension: {}",
                path, filename, extension
            );
        }

        // DO THE RENAMING

        let mut final_name = std::path::PathBuf::from(path);
        final_name.set_file_name(filename);
        if !include_ext {
            final_name.set_extension(extension);
        }
    }
}

fn process_dir_entry(entry: &std::path::PathBuf, include_ext: bool) -> (String, String, String) {
    let err_msg = "Dev fucked somthing up with extracting filename and extension from path";
    return (
        String::from(entry.parent().expect(err_msg).to_str().expect(err_msg)),
        if include_ext {
            String::from(entry.file_stem().expect(err_msg).to_str().expect(err_msg))
        } else {
            String::from(entry.file_name().expect(err_msg).to_str().expect(err_msg))
        },
        String::from(entry.extension().expect(err_msg).to_str().expect(err_msg)),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process_dir_entry() {
        let mock_entry =
            std::path::PathBuf::from("/home/user/porn/brazzers_1_[x264]_(1080p)_{2018-05-14}.mkv");

        // let result = super::process_dir_entry(&mock_entry, true);

        assert_eq!(
            super::process_dir_entry(&mock_entry, true),
            (
                String::from("/home/user/porn"),
                String::from("brazzers_1_[x264]_(1080p)_{2018-05-14}"),
                String::from("mkv"),
            )
        );
    }

}
