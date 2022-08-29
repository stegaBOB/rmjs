use clap::Parser;

use crate::lib::find_and_remove;

mod lib;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the directory to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let Cli { path, debug } = Cli::parse();
    let path_str = path.as_os_str();
    if let Ok(path) = path.canonicalize() {
        if path.is_dir() {
            if let Some(path) = path.to_str() {
                find_and_remove(path, debug);
                std::process::exit(exitcode::OK);
            }
        }
        println!("Given path probably isn't a directory: received {path_str:?}");
        std::process::exit(exitcode::USAGE);
    } else {
        println!("Given path probably isn't valid: received {path_str:?}");
        std::process::exit(exitcode::USAGE);
    }
}
