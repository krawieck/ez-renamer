use crate::args_parser::Args;
use exitcode;
use log::info;

/// Returns `Vec` of all DirEntries that pass requirements
/// given by users, and also in general are valid
pub fn initialize(args: &Args) -> Vec<std::fs::DirEntry> {
	use std::fs::{read_dir, DirEntry};
	use std::{fs, process};

	let entries: Vec<DirEntry> = if args.recursive {
		let mut entries_to_check: Vec<DirEntry> = read_dir(&args.directory)
			.expect("couldn't read given directory")
			.filter_map(|x| {
				info!("surface layer {:?}", x);
				x.ok()
			})
			.collect();

		let mut final_entries: Vec<DirEntry> = vec![];
		// check first element in entries_to_check, if it's a dir,
		loop {
			if entries_to_check.is_empty() {
				break;
			}
			let curr = entries_to_check.swap_remove(0);

			if match curr.file_type() {
				Ok(a) => a.is_dir(),
				Err(_) => false,
			} {
				let mut m: Vec<DirEntry> = match read_dir(curr.path()) {
					Ok(a) => a.filter_map(|x| x.ok()).collect(),
					Err(_) => vec![],
				};
				entries_to_check.append(&mut m);
				if args.include_dirs {
					final_entries.push(curr);
				}
			} else {
				final_entries.push(curr);
			}
		}
		final_entries
	} else {
		match fs::read_dir(&args.directory) {
			Ok(files) => files
				.filter_map(|x| x.ok())
				.filter(|x| match x.file_type() {
					Ok(t) => {
						if args.include_dirs {
							false
						} else {
							!t.is_dir()
						}
					}
					Err(_) => false,
				})
				.collect(),
			Err(error) => {
				eprintln!("Error: {}", error);
				process::exit(error.raw_os_error().unwrap_or(exitcode::IOERR));
			}
		}
	};
	entries
		.into_iter()
		.filter(|x: &DirEntry| {
			args.file_match
				.is_match(&x.file_name().to_str().unwrap().to_lowercase())
		})
		.collect()
}
