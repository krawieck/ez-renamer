use regex::Regex;
use structopt::StructOpt;

fn parse_regex(src: &str) -> Result<Regex, regex::Error> {
	Regex::new(&src.to_lowercase())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "ez-renamer")]
pub struct Args {
	/// regular expression for files that should be renamed
	#[structopt(
		parse(try_from_str = "parse_regex"),
		name = "file-match",
		default_value = "."
	)]
	pub file_match: Regex,

	/// directory where should this program look for files
	#[structopt(name = "dir", long, default_value = ".")]
	pub directory: std::path::PathBuf,

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

	/// recursively goes through directories
	#[structopt(short, long)]
	pub recursive: bool,

	/// program is much quieter, it's recommended only if you know what you're doing
	///
	/// -q results in program just asking if u wanna proceed, and -qq results in program not letting anything into stdout
	#[structopt(short, parse(from_occurrences))]
	pub quiet: u8,

	/// confirms the rename, recomended only if you know what you're doing
	#[structopt(short)]
	pub yes: bool,

	/// include directories in renaming process
	#[structopt(long = "include-dirs")]
	pub include_dirs: bool,
}

impl Args {
	#[cfg(test)]
	pub fn new() -> Self {
		use regex::Regex;
		use std::path::PathBuf;
		Args {
			file_match: Regex::new(".").unwrap(),
			directory: PathBuf::new(),
			include_ext: false,
			fix_spaces: String::new(),
			remove_tags: String::new(),
			trim_left_after: String::new(),
			trim_left_with: String::new(),
			trim_right_after: String::new(),
			trim_right_with: String::new(),
			delete: String::new(),
			dont_cleanup: false,
			recursive: false,
			quiet: 0,
			yes: false,
			include_dirs: false,
		}
	}
}
