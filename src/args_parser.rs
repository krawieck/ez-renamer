use clap;

#[derive(Debug)]
pub struct Args {
    pub file_match: String,
    pub directory: String,
    pub verbose: bool,
    pub include_ext: bool,
    pub fix_spaces: String,
    pub remove_tags: String,
    pub trim_right_from: String,
    pub trim_right_with: String,
    pub trim_left_from: String,
    pub trim_left_with: String,
    pub dont_cleanup: bool,
    pub delete: String,
}

impl Args {
    pub fn from(matches: clap::ArgMatches) -> Self {
        Args {
            file_match: matches.value_of("file_match").unwrap_or(".").to_string(),
            directory: matches.value_of("directory").unwrap_or(".").to_string(),
            verbose: matches.is_present("verbose"),
            include_ext: matches.is_present("include_ext"),
            fix_spaces: matches.value_of("fix_spaces").unwrap_or("").to_string(),
            remove_tags: matches.value_of("remove_tags").unwrap_or("").to_string(),
            trim_right_from: matches
                .value_of("trim_right_from")
                .unwrap_or("")
                .to_string(),
            trim_right_with: matches
                .value_of("trim_right_with")
                .unwrap_or("")
                .to_string(),
            trim_left_from: matches.value_of("trim_left_from").unwrap_or("").to_string(),
            trim_left_with: matches.value_of("trim_left_with").unwrap_or("").to_string(),
            dont_cleanup: matches.is_present("dont_cleanup_spaces"),
            delete: matches.value_of("delete").unwrap_or("").to_string(),
        }
    }
}
