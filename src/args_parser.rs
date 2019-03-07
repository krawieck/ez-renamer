use clap;

#[derive(Debug)]
pub struct Args {
    pub file_match: String,
    pub directory: String,
    pub verbose: bool,
    pub include_ext: bool,
    pub fix_spaces: String,
    pub remove_tags: String,
    pub trim_right_after: String,
    pub trim_right_with: String,
    pub trim_left_after: String,
    pub trim_left_with: String,
    pub dont_cleanup: bool,
    pub delete: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            file_match: String::new(),
            directory: String::new(),
            verbose: false,
            include_ext: false,
            fix_spaces: String::new(),
            remove_tags: String::new(),
            trim_left_after: String::new(),
            trim_left_with: String::new(),
            trim_right_after: String::new(),
            trim_right_with: String::new(),
            delete: String::new(),
            dont_cleanup: false,
        }
    }

    pub fn from(matches: clap::ArgMatches) -> Self {
        Args {
            file_match: matches.value_of("file_match").unwrap_or(".").to_string(),
            directory: matches.value_of("directory").unwrap_or(".").to_string(),
            verbose: matches.is_present("verbose"),
            include_ext: matches.is_present("include_ext"),
            fix_spaces: matches.value_of("fix_spaces").unwrap_or("").to_string(),
            remove_tags: matches.value_of("remove_tags").unwrap_or("").to_string(),
            trim_right_after: matches
                .value_of("trim_right_after")
                .unwrap_or("")
                .to_string(),
            trim_right_with: matches
                .value_of("trim_right_with")
                .unwrap_or("")
                .to_string(),
            trim_left_after: matches
                .value_of("trim_left_after")
                .unwrap_or("")
                .to_string(),
            trim_left_with: matches.value_of("trim_left_with").unwrap_or("").to_string(),
            dont_cleanup: matches.is_present("dont_cleanup_spaces"),
            delete: matches.value_of("delete").unwrap_or("").to_string(),
        }
    }
    pub fn parse() -> Self {
        use clap::{App, Arg};
        // TODO: move clap declaration to .yaml file or at least sth less cluttered
        Args::from(
            App::new(crate_name!())
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
                    Arg::with_name("trim_right_after")
                        .help(
                            "Trim after the given sequence to the right
example:
ezr --trim-right-after [1080p]
\"Mind Field S03E02 [1080p] [x265] [YIFY].mkv\" -> \"Mind Field S03E02 [1080p].mkv\"",
                        )
                        .long("trim-right-after")
                        .takes_value(true),
                    Arg::with_name("trim_left_after")
                        .help(
                            "Trim after the given sequence to the left.
example:
ezr --trim-left-with mind
\"[HorribleSubs] Mind Field S03E02.mkv\" -> \"Mind Field S03E02.mkv\"",
                        )
                        .long("trim-left-after")
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
                        .help("deletes this phrase(s) from names")
                        .short("d")
                        .long("delete")
                        .takes_value(true),
                ])
                .get_matches(),
        )
    }
}
