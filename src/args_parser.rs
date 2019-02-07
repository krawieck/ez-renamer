use clap;

#[derive(Debug)]
pub struct Args {
    pub directory: String,
    pub verbose: bool,
    pub include_ext: bool,
    pub fix_spaces: bool,
    pub remove_tags: String,
}

impl Args {
    pub fn from(matches: clap::ArgMatches) -> Self {
        Args {
            directory: matches.value_of("directory").unwrap_or(".").to_string(),
            verbose: matches.is_present("verbose"),
            include_ext: matches.is_present("include_ext"),
            fix_spaces: matches.is_present("fix_spaces"),
            remove_tags: matches.value_of("remove_tags").unwrap_or("").to_string(),
        }
    }
}
