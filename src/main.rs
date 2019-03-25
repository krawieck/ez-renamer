extern crate exitcode;
extern crate log;
extern crate pretty_env_logger;
extern crate regex;
extern crate structopt;

mod args_parser;
mod init;
mod name_processing;
mod trim;

fn main() -> Result<(), std::io::Error> {
    use args_parser::Args;
    use log::{error, info};
    use name_processing::process_name;
    use std::io;
    use std::path::PathBuf;
    use std::{env, fs};
    use structopt::StructOpt;

    pretty_env_logger::init(); // RUST_LOG=ezr before command to enable logging
    let args = Args::from_args();
    info!("args: {:#?}", env::args());
    info!("matches: {:#?}", args);

    // GO OVER DIRECTORY AND MAKE CHANGES
    let names = init::initialize(&args)
        .iter()
        .map(|a| process_name(a.path(), &args))
        .filter_map(|a| a.ok())
        .collect::<Vec<(PathBuf, PathBuf)>>();

    // LIST CHANGES AND ASK IF USER THAY WANT TO PROCEED
    if args.quiet < 1 {
        for (from, to) in &names {
            println!(
                "{} -> {}",
                from.as_path().to_str().unwrap(),
                to.as_path().to_str().unwrap()
            );
        }
    }

    if !args.yes {
        // Ask user if they wanna proceed
        if args.quiet < 2 {
            print!("Should I proceed? [Y/n] ");
        }
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let proceed = {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_goes_into_input_above) => {}
                Err(_no_updates_is_fine) => {}
            }
            input.trim().chars().next().unwrap_or('y')
        };
        if !(proceed == 'Y' || proceed == 'y') {
            println!("exiting...");
            return Ok(());
        }
    }

    // RENAMING

    if args.quiet < 1 {
        println!("Renaming... ");
    }
    for (a, b) in names {
        info!("{} -> {}", a.to_str().unwrap(), b.to_str().unwrap());
        match fs::rename(a, b) {
            Ok(()) => {
                info!("Ok");
            }
            Err(err) => {
                if args.quiet < 2 {
                    eprintln!("Error while renaming: {}", err);
                }
                error!("{}", err);
            }
        }
    }
    if args.quiet < 1 {
        println!("Done!")
    };
    Ok(())
}
