use args_parser::Args;

/// Changes names of the files based on the user's requirements
/// and also returns touple with original name and changed name
pub fn process_name(
	entry: std::path::PathBuf,
	args: &Args,
) -> Result<(std::path::PathBuf, std::path::PathBuf), ()> {
	use log::info;
	use std::path::PathBuf;
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
	info!("after remove_inside_brackets: {}", &filename);
	let filename = fix_spaces(&filename, &args.fix_spaces);
	info!("after fix_spaces: {}", &filename);
	let filename = delete(&filename, &args.delete);
	info!("after delete: {}", &filename);
	let filename = trim(&filename, &args);
	info!("after trim: {}", &filename);

	let filename = if !args.dont_cleanup {
		cleanup_spaces(&filename)
	} else {
		filename
	};
	info!("after cleanup_spaces: {}", &filename);

	let filename = if !extension.is_empty() {
		format!("{}.{}", filename, extension)
	} else {
		filename
	};
	let mut final_name = PathBuf::from(path);

	final_name.push(filename);
	info!("after final_name.push: {:?}", &final_name);

	info!("------");
	Ok((entry, final_name))
}

/// Removes trailing spaces and any double spaces
///
/// example:
/// `cleanup_spaces("  asdfasdf  sd   asdfk   ") -> "asdfasdf sd asdfk"`
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
/// `fix_spaces("Love_Death_and_Robots_S01E14.Zima.Blue", "_.") -> "Love Death Robots S01E14 Zima Blue"`
fn fix_spaces(input: &str, replacer: &str) -> String {
	let mut output: String = input.to_string();
	for x in replacer.chars() {
		output = output.replace(x, " ");
	}
	output
}

/// Removes contents inside brackets (with brackets)
///
/// example:
/// `remove_inside_brackets("black mirror s03e01 (2018) [x265] [1080p]", "[]()") -> "black mirror s03e01   "`
fn remove_inside_brackets(input: &str, brackets: &str) -> String {
	use regex::Regex;
	use std::{process, str};

	let mut output = input.to_owned();
	for s in brackets
		.as_bytes()
		.chunks(2)
		.map(str::from_utf8)
		.filter_map(|a| a.ok())
	{
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

			let mut reg_str = beg;
			reg_str.push_str(".*?");
			reg_str.push_str(&end);
			let reg = Regex::new(&reg_str).expect("Dev messed sth up with removing brackets");
			reg.replace_all(&output, "").to_string()
		};
	}
	output
}

/// Deletes some phrase
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
		let mock = super::remove_inside_brackets(&mock, &String::from("[]()"));
		assert_eq!(mock, String::from("black_mirror_bandersnatch__"));
		let mock = String::from("black_mirror_bandersnatch");
		let mock = super::remove_inside_brackets(&mock, &String::from(""));
		assert_eq!(mock, String::from("black_mirror_bandersnatch"));
		let mock = String::from("black_mirrór_bandersnątćh");
		let mock = super::remove_inside_brackets(&mock, &String::from(""));
		assert_eq!(mock, String::from("black_mirrór_bandersnątćh"));
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

	#[test]
	fn test_hyphen() {
		assert_eq!(super::delete("Foo -[bar]", "-[bar]"), String::from("Foo "))
	}
}
