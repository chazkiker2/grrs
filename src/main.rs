use std::fs::File;
use std::io::{self, BufReader, prelude::*, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use log::{info, warn};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn log_matches<I>(
    content: I,
    pattern: &str,
    mut writer: impl Write,
) -> Result<()>
    where I: Iterator<Item=Result<String, std::io::Error>>
{
    for line in content {
        let line = line.unwrap();
        if line.contains(&pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();
    env_logger::init();
    info!("starting up");
    warn!("oops! little warning");

    // try to open file
    let path = args.path.to_owned();
    let file = File::open(args.path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    // wrap `stdout` in `BufWriter` for higher performance printing
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    // read file and write (to wrapped `stdout`) all lines which match the given pattern
    let reader = BufReader::new(file);
    log_matches(reader.lines(), &args.pattern, &mut handle)?;

    // print all matching lines
    handle.flush().unwrap();

    info!("full program run");
    Ok(())
}

#[test]
fn sanity_check() {
    assert_eq!(42, 42)
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let file = File::open("test.txt");
    assert!(file.is_ok());

    let reader = BufReader::new(file.unwrap());
    let response = log_matches(reader.lines(), "in", &mut result);
    assert!(response.is_ok());
    assert_eq!(result, b"This is inside test.txt!\n");
}
