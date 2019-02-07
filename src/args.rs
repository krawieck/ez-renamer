use clap;

pub struct Args {
    directory: String,
    verbose: bool,
    include_ext: bool,
    fix_spaces: bool,
    remove_tags: String,
}

impl Args {
    fn from(matches: clap::ArgMatches<'_>) -> Self {
        Args {
            directory: matches.value_of("directory").unwrap_or("").to_string(),
            verbose: matches.is_present("verbose"),
            include_ext: matches.is_present("include_ext"),
            fix_spaces: matches.is_present("fix_spaces"),
            remove_tags: matches.value_of("remove_tags").unwrap_or("").to_string(),
        }
    }
}
