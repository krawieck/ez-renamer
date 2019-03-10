use clap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ez-renamer")]
pub struct Args {
    /// regular expression for files that should be renamed
    #[structopt(name = "file-match", default_value = ".")]
    pub file_match: String,
    /// directory where should this program look for files
    #[structopt(name = "dir", long, default_value = ".")]
    pub directory: std::path::PathBuf,
    /// becomes a bit louder
    #[structopt(long, short)]
    pub verbose: bool,
    /// includes extensions in renaming process
    #[structopt(name = "include-ext", long, short = "e")]
    pub include_ext: bool,
    /// whatever you give is replaced by space (but only single chars)
    ///
    /// example:
    ///
    /// `--fix-spaces="_"` results in:
    ///
    /// "the_office_[720p]_[x265]" -> "the office [720p] [x265]"
    #[structopt(name = "fix-spaces", long, short = "s", default_value = "")]
    pub fix_spaces: String,
    /// remove tags, they're usually inside [] or (). e.g. -s "() []"
    ///
    /// Syntax for this argument should be '<opening bracket><closing
    /// bracket> <repeat>'
    ///
    /// example:
    ///
    /// ezr -s "[] ()"
    ///
    /// "Mind Field S03E02 (2018) [1080p] [x265] [YIFY].mkv" -> "Mind Field S03E02.mkv"
    #[structopt(name = "remove-tags", long = "rmtags", short = "t", default_value = "")]
    pub remove_tags: String,
    /// Trim after the given sequence to the right
    ///
    /// example:
    ///
    /// ezr --trim-right-after [1080p]
    ///
    /// "Mind Field S03E02 [1080p] [x265] [YIFY].mkv" -> "Mind Field S03E02 [1080p].mkv"
    #[structopt(name = "trim-right-after", long, default_value = "")]
    pub trim_right_after: String,
    /// Trim with the given sequence to the right
    ///
    /// example:
    ///
    /// ezr --trim-right-with [1080p]
    ///
    /// "Mind Field S03E02 [1080p] [x265] [YIFY].mkv" -> "Mind Field S03E02 .mkv"
    #[structopt(name = "trim-right-with", long, default_value = "")]
    pub trim_right_with: String,
    /// Trim after the given sequence to the left.
    ///
    /// example:
    ///
    /// ezr --trim-left-with Mind
    ///
    /// "[HorribleSubs] Mind Field S03E02.mkv" -> "Mind Field S03E02.mkv"
    #[structopt(name = "trim-left-after", long, default_value = "")]
    pub trim_left_after: String,
    /// Trim with the given sequence to the left.
    ///
    /// example:
    ///
    /// ezr --trim-left-with ubs]
    ///
    /// "[HorribleSubs] Mind Field S03E02.mkv" -> "Mind Field S03E02.mkv"
    #[structopt(name = "trim-left-with", long, default_value = "")]
    pub trim_left_with: String,
    /// By default ez-renamer removes multiple spaces (cleans up)
    /// after it's done. This flag stops him from doing that
    #[structopt(name = "dont-cleanup", long)]
    pub dont_cleanup: bool,
    /// deletes this phrase(s) from names
    ///
    /// example:
    ///
    /// ezr -d "[WEBRip] [720p] [YTS.AM]"
    ///
    /// "Green Book (2018) [WEBRip] [720p] [YTS.AM]" -> "Green Book (2018)"
    #[structopt(long, short, default_value = "")]
    pub delete: String,
}

impl Args {
    #[cfg(test)]
    pub fn new() -> Self {
        use std::path::PathBuf;
        Args {
            file_match: String::new(),
            directory: PathBuf::new(),
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
        use std::path::PathBuf;
        Args {
            file_match: matches.value_of("file_match").unwrap_or(".").to_string(),
            directory: PathBuf::from(matches.value_of("directory").unwrap_or(".").to_string()),
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
        use clap::{crate_authors, crate_name, crate_version, App, Arg};
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
