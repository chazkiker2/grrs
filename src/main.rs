use structopt::StructOpt;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::path::{PathBuf};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    // optimized solution
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())

}

