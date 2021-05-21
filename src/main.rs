use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufReader, prelude::*, Write};
use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();

    // try to open file
    let path = args.path.to_owned();
    let file = File::open(args.path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    // read file and write (to wrapped stdout) all lines which match the given pattern
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }
    // print all matching lines
    handle.flush().unwrap();

    Ok(())
}

